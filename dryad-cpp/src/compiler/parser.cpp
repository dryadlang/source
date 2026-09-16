#include "dryad/compiler/parser.h"
#include "dryad/common/utils.h"
#include <sstream>

namespace dryad {

Parser::Parser(std::vector<Token> tokens)
    : tokens_(std::move(tokens)), current_(0), has_error_(false) {}

std::unique_ptr<Program> Parser::parse() {
    std::vector<std::unique_ptr<Statement>> statements;
    
    while (!is_at_end()) {
        try {
            auto stmt = parse_declaration();
            if (stmt) {
                statements.push_back(std::move(stmt));
            }
        } catch (const DryadException& e) {
            error(e.what());
            synchronize();
        }
    }
    
    return std::make_unique<Program>(std::move(statements));
}

Token Parser::peek() const {
    return tokens_[current_];
}

Token Parser::previous() const {
    return tokens_[current_ - 1];
}

Token Parser::advance() {
    if (!is_at_end()) current_++;
    return previous();
}

bool Parser::is_at_end() const {
    return peek().type == TokenType::EndOfFile;
}

bool Parser::check(TokenType type) const {
    if (is_at_end()) return false;
    return peek().type == type;
}

bool Parser::match(TokenType type) {
    if (check(type)) {
        advance();
        return true;
    }
    return false;
}

bool Parser::match(std::initializer_list<TokenType> types) {
    for (TokenType type : types) {
        if (match(type)) {
            return true;
        }
    }
    return false;
}

Token Parser::consume(TokenType type, const std::string& message) {
    if (check(type)) return advance();
    
    std::stringstream ss;
    ss << message << " Got '" << token_type_to_string(peek().type) << "'";
    throw DryadException(ss.str());
}

Token Parser::consume_member_name(const std::string& message) {
    if (check(TokenType::Identifier)) {
        return advance();
    }
    if (match(TokenType::KeywordGet) || match(TokenType::KeywordSet) || match(TokenType::KeywordDelete)) {
        return previous();
    }

    std::stringstream ss;
    ss << message << " Got '" << token_type_to_string(peek().type) << "'";
    throw DryadException(ss.str());
}

void Parser::error(const std::string& message) {
    error_at(peek().location, message);
}

void Parser::error_at(const SourceLocation& location, const std::string& message) {
    has_error_ = true;
    errors_.push_back(message);
    diagnostics_.emplace_back(location, DiagnosticSeverity::Error, message);
    if (error_message_.empty()) {
        error_message_ = message;
    } else {
        error_message_ += "\n" + message;
    }
}

void Parser::synchronize_expression() {
    while (!is_at_end()) {
        switch (peek().type) {
            case TokenType::Comma:
            case TokenType::Semicolon:
            case TokenType::RightParen:
            case TokenType::RightBracket:
            case TokenType::RightBrace:
                return;
            default:
                break;
        }
        advance();
    }
}

void Parser::synchronize() {
    advance();
    
    while (!is_at_end()) {
        if (previous().type == TokenType::Semicolon) return;
        
        switch (peek().type) {
            case TokenType::KeywordClass:
            case TokenType::KeywordInterface:
            case TokenType::KeywordImport:
            case TokenType::KeywordFunction:
            case TokenType::KeywordLet:
            case TokenType::KeywordConst:
            case TokenType::KeywordFor:
            case TokenType::KeywordIf:
            case TokenType::KeywordWhile:
            case TokenType::KeywordReturn:
                return;
            default:
                break;
        }
        
        advance();
    }
}

void Parser::synchronize_class_member() {
    while (!is_at_end()) {
        if (peek().type == TokenType::RightBrace) {
            return;
        }
        if (peek().type == TokenType::Semicolon) {
            advance();
            return;
        }
        switch (peek().type) {
            case TokenType::KeywordAbstract:
            case TokenType::KeywordReadonly:
            case TokenType::KeywordGet:
            case TokenType::KeywordSet:
            case TokenType::KeywordDelete:
            case TokenType::Identifier:
                return;
            default:
                break;
        }
        advance();
    }
}

std::unique_ptr<Statement> Parser::parse_declaration() {
    if (match(TokenType::KeywordImport)) {
        return parse_import_declaration();
    }

    // Check for @intrinsic decorator
    if (match(TokenType::At)) {
        Token decorator = consume(TokenType::Identifier, "Expected decorator name after '@'");
        if (decorator.lexeme == "intrinsic") {
            return parse_intrinsic_declaration();
        } else {
            throw DryadException("Unknown decorator '@" + decorator.lexeme + "'");
        }
    }
    
    bool is_internal = false;
    bool is_abstract = false;
    while (true) {
        if (match(TokenType::KeywordInternal)) {
            is_internal = true;
            continue;
        }
        if (match(TokenType::KeywordAbstract)) {
            is_abstract = true;
            continue;
        }
        break;
    }
    
    if (match(TokenType::KeywordLet) || match(TokenType::KeywordConst)) {
        if (is_internal || is_abstract) {
            throw DryadException("'internal' and 'abstract' modifiers cannot be used with variable declarations");
        }
        return parse_variable_declaration();
    }
    
    if (match(TokenType::KeywordFunction)) {
        if (is_abstract) {
            throw DryadException("'abstract' modifier can only be used with classes");
        }
        return parse_function_declaration(is_internal);
    }
    
    if (match(TokenType::KeywordInterface)) {
        if (is_abstract) {
            throw DryadException("'abstract' modifier cannot be used with interfaces");
        }
        return parse_interface_declaration(is_internal);
    }

    if (match(TokenType::KeywordClass)) {
        return parse_class_declaration(is_internal, is_abstract);
    }

    if (is_internal) {
        throw DryadException("'internal' modifier can only be used with functions, classes, or interfaces");
    }
    if (is_abstract) {
        throw DryadException("'abstract' modifier can only be used with classes");
    }
    
    return parse_statement();
}

std::unique_ptr<Statement> Parser::parse_variable_declaration() {
    bool is_const = previous().type == TokenType::KeywordConst;
    SourceLocation loc = previous().location;
    
    Token name = consume(TokenType::Identifier, "Expected variable name");

    std::string type_annotation;
    if (match(TokenType::Colon)) {
        Token type = consume(TokenType::Identifier, "Expected type annotation");
        type_annotation = type.lexeme;
    }
    
    std::unique_ptr<Expression> initializer = nullptr;
    if (match(TokenType::Equal)) {
        initializer = parse_expression();
    }
    
    consume(TokenType::Semicolon, "Expected ';' after variable declaration");
    
    return std::make_unique<VariableDeclaration>(name.lexeme, type_annotation, std::move(initializer), is_const, loc);
}

std::unique_ptr<Statement> Parser::parse_function_declaration(bool is_internal) {
    SourceLocation loc = previous().location;
    
    Token name = consume(TokenType::Identifier, "Expected function name");
    
    consume(TokenType::LeftParen, "Expected '(' after function name");
    
    std::vector<Parameter> parameters = parse_parameters();
    
    consume(TokenType::RightParen, "Expected ')' after parameters");
    
    std::string return_type = parse_return_type();
    
    consume(TokenType::LeftBrace, "Expected '{' before function body");
    auto body = parse_block_statement();
    
    return std::make_unique<FunctionDeclaration>(
        name.lexeme,
        std::move(parameters),
        std::unique_ptr<BlockStatement>(static_cast<BlockStatement*>(body.release())),
        return_type,
        is_internal,
        loc
    );
}

std::unique_ptr<Statement> Parser::parse_statement() {
    if (match(TokenType::LeftBrace)) {
        return parse_block_statement();
    }
    
    if (match(TokenType::KeywordIf)) {
        return parse_if_statement();
    }
    
    if (match(TokenType::KeywordWhile)) {
        return parse_while_statement();
    }
    
    if (match(TokenType::KeywordReturn)) {
        return parse_return_statement();
    }
    
    return parse_expression_statement();
}

std::unique_ptr<Statement> Parser::parse_block_statement() {
    SourceLocation loc = previous().location;
    std::vector<std::unique_ptr<Statement>> statements;
    
    while (!check(TokenType::RightBrace) && !is_at_end()) {
        statements.push_back(parse_declaration());
    }
    
    consume(TokenType::RightBrace, "Expected '}' after block");
    
    return std::make_unique<BlockStatement>(std::move(statements), loc);
}

std::unique_ptr<Statement> Parser::parse_expression_statement() {
    SourceLocation loc = peek().location;
    auto expr = parse_expression();
    consume(TokenType::Semicolon, "Expected ';' after expression");
    return std::make_unique<ExpressionStatement>(std::move(expr), loc);
}

std::unique_ptr<Statement> Parser::parse_if_statement() {
    SourceLocation loc = previous().location;
    
    consume(TokenType::LeftParen, "Expected '(' after 'if'");
    auto condition = parse_expression();
    consume(TokenType::RightParen, "Expected ')' after condition");
    
    auto then_branch = parse_statement();
    
    std::unique_ptr<Statement> else_branch = nullptr;
    if (match(TokenType::KeywordElse)) {
        else_branch = parse_statement();
    }
    
    return std::make_unique<IfStatement>(std::move(condition), std::move(then_branch), std::move(else_branch), loc);
}

std::unique_ptr<Statement> Parser::parse_while_statement() {
    SourceLocation loc = previous().location;
    
    consume(TokenType::LeftParen, "Expected '(' after 'while'");
    auto condition = parse_expression();
    consume(TokenType::RightParen, "Expected ')' after condition");
    
    auto body = parse_statement();
    
    return std::make_unique<WhileStatement>(std::move(condition), std::move(body), loc);
}

std::unique_ptr<Statement> Parser::parse_return_statement() {
    SourceLocation loc = previous().location;
    
    std::unique_ptr<Expression> value = nullptr;
    if (!check(TokenType::Semicolon)) {
        value = parse_expression();
    }
    
    consume(TokenType::Semicolon, "Expected ';' after return value");
    
    return std::make_unique<ReturnStatement>(std::move(value), loc);
}

std::unique_ptr<Expression> Parser::parse_expression() {
    return parse_assignment();
}

std::unique_ptr<Expression> Parser::parse_assignment() {
    auto expr = parse_logical_or();
    
    if (match(TokenType::Equal)) {
        auto value = parse_assignment();
        return std::make_unique<AssignmentExpression>(std::move(expr), std::move(value), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_logical_or() {
    auto expr = parse_logical_and();
    
    while (match(TokenType::PipePipe)) {
        TokenType op = previous().type;
        auto right = parse_logical_and();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_logical_and() {
    auto expr = parse_equality();
    
    while (match(TokenType::AmpersandAmpersand)) {
        TokenType op = previous().type;
        auto right = parse_equality();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_equality() {
    auto expr = parse_comparison();
    
    while (match({TokenType::EqualEqual, TokenType::BangEqual})) {
        TokenType op = previous().type;
        auto right = parse_comparison();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_comparison() {
    auto expr = parse_addition();
    
    while (match({TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual})) {
        TokenType op = previous().type;
        auto right = parse_addition();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_addition() {
    auto expr = parse_multiplication();
    
    while (match({TokenType::Plus, TokenType::Minus})) {
        TokenType op = previous().type;
        auto right = parse_multiplication();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_multiplication() {
    auto expr = parse_unary();
    
    while (match({TokenType::Star, TokenType::Slash, TokenType::Percent})) {
        TokenType op = previous().type;
        auto right = parse_unary();
        expr = std::make_unique<BinaryExpression>(op, std::move(expr), std::move(right), previous().location);
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_unary() {
    if (match({TokenType::Bang, TokenType::Minus})) {
        TokenType op = previous().type;
        auto operand = parse_unary();
        return std::make_unique<UnaryExpression>(op, std::move(operand), previous().location);
    }
    
    return parse_postfix();
}

std::unique_ptr<Expression> Parser::parse_postfix() {
    auto expr = parse_primary();
    
    while (true) {
        if (match(TokenType::LeftParen)) {
            std::vector<std::unique_ptr<Expression>> arguments;
            
            if (!check(TokenType::RightParen)) {
                do {
                    arguments.push_back(parse_expression());
                } while (match(TokenType::Comma));
            }
            
            consume(TokenType::RightParen, "Expected ')' after arguments");
            expr = std::make_unique<CallExpression>(std::move(expr), std::move(arguments), previous().location);
        }
        else if (match(TokenType::Dot)) {
            Token name = consume_member_name("Expected property name after '.'");
            auto property = std::make_unique<Identifier>(name.lexeme, name.location);
            expr = std::make_unique<MemberExpression>(std::move(expr), std::move(property), false, previous().location);
        }
        else if (match(TokenType::LeftBracket)) {
            auto index = parse_expression();
            consume(TokenType::RightBracket, "Expected ']' after index");
            expr = std::make_unique<MemberExpression>(std::move(expr), std::move(index), true, previous().location);
        }
        else {
            break;
        }
    }
    
    return expr;
}

std::unique_ptr<Expression> Parser::parse_primary() {
    if (match(TokenType::KeywordTrue)) {
        return std::make_unique<BooleanLiteral>(true, previous().location);
    }
    
    if (match(TokenType::KeywordFalse)) {
        return std::make_unique<BooleanLiteral>(false, previous().location);
    }
    
    if (match(TokenType::KeywordNull)) {
        return std::make_unique<NullLiteral>(previous().location);
    }
    
    if (match(TokenType::IntegerLiteral)) {
        int64_t value = std::stoll(previous().lexeme);
        return std::make_unique<IntegerLiteral>(value, previous().location);
    }
    
    if (match(TokenType::FloatLiteral)) {
        double value = std::stod(previous().lexeme);
        return std::make_unique<FloatLiteral>(value, previous().location);
    }
    
    if (match(TokenType::StringLiteral)) {
        return std::make_unique<StringLiteral>(previous().lexeme, previous().location);
    }
    
    if (match(TokenType::KeywordNew)) {
        SourceLocation loc = previous().location;
        Token class_name = consume(TokenType::Identifier, "Expected class name after 'new'");
        consume(TokenType::LeftParen, "Expected '(' after class name");
        
        std::vector<std::unique_ptr<Expression>> arguments;
        if (!check(TokenType::RightParen)) {
            do {
                arguments.push_back(parse_expression());
            } while (match(TokenType::Comma));
        }
        
        consume(TokenType::RightParen, "Expected ')' after arguments");
        return std::make_unique<NewExpression>(class_name.lexeme, std::move(arguments), loc);
    }
    
    if (match(TokenType::KeywordFunction)) {
        return parse_function_expression();
    }

    if (match(TokenType::KeywordThis)) {
        return std::make_unique<ThisExpression>(previous().location);
    }

    if (match(TokenType::KeywordSuper)) {
        return std::make_unique<SuperExpression>(previous().location);
    }
    
    if (match(TokenType::Identifier)) {
        return std::make_unique<Identifier>(previous().lexeme, previous().location);
    }
    
    if (match(TokenType::LeftBracket)) {
        return parse_array_literal();
    }
    
    if (match(TokenType::LeftParen)) {
        auto expr = parse_expression();
        consume(TokenType::RightParen, "Expected ')' after expression");
        return expr;
    }
    
    SourceLocation loc = peek().location;
    error_at(loc, "Expected expression");
    synchronize_expression();
    return std::make_unique<ErrorExpression>("Expected expression", loc);
}

std::unique_ptr<Expression> Parser::parse_array_literal() {
    SourceLocation loc = previous().location;
    std::vector<std::unique_ptr<Expression>> elements;
    
    if (!check(TokenType::RightBracket)) {
        do {
            elements.push_back(parse_expression());
        } while (match(TokenType::Comma));
    }
    
    consume(TokenType::RightBracket, "Expected ']' after array elements");
    
    return std::make_unique<ArrayExpression>(std::move(elements), loc);
}

void Parser::parse_property_accessors(PropertyDeclaration& property, SourceLocation loc) {
    consume(TokenType::LeftBrace, "Expected '{' before property accessors");

    while (!check(TokenType::RightBrace) && !is_at_end()) {
        if (match(TokenType::KeywordGet)) {
            property.has_getter = true;
            if (match(TokenType::Semicolon)) {
                property.getter_is_auto = true;
            } else {
                consume(TokenType::LeftBrace, "Expected '{' before getter body");
                std::vector<std::unique_ptr<Statement>> body_statements;
                while (!check(TokenType::RightBrace) && !is_at_end()) {
                    body_statements.push_back(parse_declaration());
                }
                consume(TokenType::RightBrace, "Expected '}' after getter body");
                property.getter_body = std::make_unique<BlockStatement>(std::move(body_statements), loc);
            }
        } else if (match(TokenType::KeywordSet)) {
            property.has_setter = true;
            if (match(TokenType::LeftParen)) {
                Token param = consume(TokenType::Identifier, "Expected setter parameter name");
                property.setter_parameter = param.lexeme;
                consume(TokenType::RightParen, "Expected ')' after setter parameter");
            }
            if (match(TokenType::Semicolon)) {
                property.setter_is_auto = true;
            } else {
                consume(TokenType::LeftBrace, "Expected '{' before setter body");
                std::vector<std::unique_ptr<Statement>> body_statements;
                while (!check(TokenType::RightBrace) && !is_at_end()) {
                    body_statements.push_back(parse_declaration());
                }
                consume(TokenType::RightBrace, "Expected '}' after setter body");
                property.setter_body = std::make_unique<BlockStatement>(std::move(body_statements), loc);
            }
        } else {
            error("Expected 'get' or 'set' in property accessor block");
            synchronize_class_member();
        }
    }

    consume(TokenType::RightBrace, "Expected '}' after property accessors");
}

std::unique_ptr<PropertyDeclaration> Parser::parse_property_declaration(bool is_abstract, bool is_readonly,
                                                                       const Token& name, SourceLocation loc) {
    consume(TokenType::Colon, "Expected ':' after property name");

    std::string type_annotation;
    if (check(TokenType::Identifier)) {
        type_annotation = advance().lexeme;
    }

    auto property = std::make_unique<PropertyDeclaration>(name.lexeme, type_annotation, is_readonly, is_abstract);

    if (match(TokenType::Semicolon)) {
        return property;
    }

    if (check(TokenType::LeftBrace)) {
        if (is_abstract) {
            throw DryadException("Abstract properties cannot declare accessor bodies");
        }
        parse_property_accessors(*property, loc);
        return property;
    }

    throw DryadException("Expected ';' or '{' after property type");
}

std::unique_ptr<MethodDeclaration> Parser::parse_method_declaration(bool is_abstract, const Token& method_name,
                                                                    SourceLocation loc, bool is_getter,
                                                                    bool is_setter) {
    consume(TokenType::LeftParen, "Expected '(' after method name");

    std::vector<Parameter> parameters = parse_parameters();

    consume(TokenType::RightParen, "Expected ')' after parameters");

    if (is_setter && parameters.size() != 1) {
        throw DryadException("Setter '" + method_name.lexeme + "' must have exactly one parameter");
    }
    if (is_getter && !parameters.empty()) {
        throw DryadException("Getter '" + method_name.lexeme + "' cannot have parameters");
    }

    parse_return_type();

    bool is_constructor = (method_name.lexeme == "constructor" && !is_getter && !is_setter);
    if (is_abstract && is_constructor) {
        throw DryadException("Constructors cannot be abstract");
    }

    std::unique_ptr<BlockStatement> body;
    if (is_abstract) {
        consume(TokenType::Semicolon, "Expected ';' after abstract method declaration");
    } else {
        consume(TokenType::LeftBrace, "Expected '{' before method body");
        auto body_statements = std::vector<std::unique_ptr<Statement>>();

        while (!check(TokenType::RightBrace) && !is_at_end()) {
            body_statements.push_back(parse_declaration());
        }

        consume(TokenType::RightBrace, "Expected '}' after method body");
        body = std::make_unique<BlockStatement>(std::move(body_statements), loc);
    }

    return std::make_unique<MethodDeclaration>(
        method_name.lexeme, std::move(parameters), std::move(body), is_constructor, is_abstract, is_getter, is_setter);
}

std::unique_ptr<Statement> Parser::parse_class_declaration(bool is_internal, bool is_abstract) {
    SourceLocation loc = previous().location;
    (void)is_internal;

    Token name = consume(TokenType::Identifier, "Expected class name");

    std::string super_class;
    if (match(TokenType::KeywordExtends)) {
        Token super = consume(TokenType::Identifier, "Expected superclass name");
        super_class = super.lexeme;
    }

    std::vector<std::string> interfaces;
    if (match(TokenType::KeywordImplements)) {
        do {
            Token iface = consume(TokenType::Identifier, "Expected interface name");
            interfaces.push_back(iface.lexeme);
        } while (match(TokenType::Comma));
    }

    consume(TokenType::LeftBrace, "Expected '{' before class body");

    std::vector<std::unique_ptr<PropertyDeclaration>> properties;
    std::vector<std::unique_ptr<MethodDeclaration>> methods;

    while (!check(TokenType::RightBrace) && !is_at_end()) {
        try {
            bool member_is_abstract = false;
            bool member_is_readonly = false;
            bool member_is_getter = false;
            bool member_is_setter = false;

            Token member_name = Token(TokenType::Error, "", SourceLocation());
            if (match(TokenType::KeywordGet)) {
                if (check(TokenType::Identifier)) {
                    member_is_getter = true;
                    member_name = advance();
                } else {
                    member_name = previous();
                }
            } else if (match(TokenType::KeywordSet)) {
                if (check(TokenType::Identifier)) {
                    member_is_setter = true;
                    member_name = advance();
                } else {
                    member_name = previous();
                }
            } else if (match(TokenType::KeywordDelete)) {
                if (check(TokenType::LeftParen)) {
                    member_name = previous();
                } else {
                    throw DryadException("Expected '(' after method name 'delete'");
                }
            } else {
                if (match(TokenType::KeywordAbstract)) {
                    member_is_abstract = true;
                }
                if (match(TokenType::KeywordReadonly)) {
                    member_is_readonly = true;
                }
                member_name = consume_member_name("Expected class member name");
            }

            if (member_is_getter || member_is_setter) {
                methods.push_back(parse_method_declaration(false, member_name, loc, member_is_getter, member_is_setter));
                continue;
            }

            if (check(TokenType::Colon)) {
                if (member_name.lexeme == "constructor") {
                    throw DryadException("Constructors cannot be declared as properties");
                }
                if (member_is_abstract && !is_abstract) {
                    throw DryadException("Abstract properties are only allowed in abstract classes");
                }
                properties.push_back(parse_property_declaration(member_is_abstract, member_is_readonly, member_name, loc));
                continue;
            }

            if (member_is_readonly) {
                throw DryadException("'readonly' modifier can only be used with properties");
            }

            methods.push_back(parse_method_declaration(member_is_abstract, member_name, loc));
        } catch (const DryadException& e) {
            error(e.what());
            synchronize_class_member();
        }
    }

    consume(TokenType::RightBrace, "Expected '}' after class body");

    return std::make_unique<ClassDeclaration>(
        name.lexeme, super_class, std::move(interfaces), std::move(properties), std::move(methods), is_abstract, loc);
}

std::unique_ptr<Statement> Parser::parse_interface_declaration(bool is_internal) {
    SourceLocation loc = previous().location;
    (void)is_internal;

    Token name = consume(TokenType::Identifier, "Expected interface name");
    consume(TokenType::LeftBrace, "Expected '{' before interface body");

    std::vector<std::unique_ptr<PropertyDeclaration>> properties;
    std::vector<std::unique_ptr<MethodDeclaration>> methods;

    while (!check(TokenType::RightBrace) && !is_at_end()) {
        bool member_is_readonly = false;

        if (match(TokenType::KeywordReadonly)) {
            member_is_readonly = true;
        }

        Token member_name = consume(TokenType::Identifier, "Expected interface member name");

        if (check(TokenType::Colon)) {
            properties.push_back(parse_property_declaration(true, member_is_readonly, member_name, loc));
            continue;
        }

        if (match(TokenType::KeywordAbstract)) {
            throw DryadException("Interface methods cannot use the 'abstract' modifier");
        }

        methods.push_back(parse_method_declaration(true, member_name, loc));
    }

    consume(TokenType::RightBrace, "Expected '}' after interface body");

    return std::make_unique<InterfaceDeclaration>(
        name.lexeme, std::move(methods), std::move(properties), loc);
}

std::unique_ptr<Statement> Parser::parse_intrinsic_declaration() {
    SourceLocation loc = previous().location;
    
    consume(TokenType::LeftParen, "Expected '(' after @intrinsic");
    Token intrinsic_name_token = consume(TokenType::StringLiteral, "Expected intrinsic name as string literal");
    std::string intrinsic_name = intrinsic_name_token.lexeme;
    consume(TokenType::RightParen, "Expected ')' after intrinsic name");
    
    consume(TokenType::KeywordExtern, "Expected 'extern' after @intrinsic(...) decorator");
    consume(TokenType::KeywordFunction, "Expected 'function' after 'extern'");
    
    Token function_name = consume(TokenType::Identifier, "Expected function name");
    
    consume(TokenType::LeftParen, "Expected '(' after function name");
    
    std::vector<Parameter> parameters = parse_parameters();
    
    consume(TokenType::RightParen, "Expected ')' after parameters");
    
    std::string return_type = parse_return_type();
    
    consume(TokenType::Semicolon, "Expected ';' after intrinsic declaration");
    
    return std::make_unique<IntrinsicDeclaration>(
        intrinsic_name,
        function_name.lexeme,
        std::move(parameters),
        return_type,
        loc
    );
}

std::vector<Parameter> Parser::parse_parameters() {
    std::vector<Parameter> parameters;
    
    if (!check(TokenType::RightParen)) {
        do {
            Token param_name = consume(TokenType::Identifier, "Expected parameter name");
            
            std::string type_annotation;
            if (match(TokenType::Colon)) {
                Token type = consume(TokenType::Identifier, "Expected type annotation");
                type_annotation = type.lexeme;
            }

            std::unique_ptr<Expression> default_value = nullptr;
            if (match(TokenType::Equal)) {
                default_value = parse_expression();
            }
            
            parameters.push_back(Parameter{param_name.lexeme, type_annotation, std::move(default_value)});
        } while (match(TokenType::Comma));
    }
    
    return parameters;
}

std::unique_ptr<FunctionExpression> Parser::parse_function_expression() {
    SourceLocation loc = previous().location;

    consume(TokenType::LeftParen, "Expected '(' after 'function'");
    std::vector<Parameter> parameters = parse_parameters();
    consume(TokenType::RightParen, "Expected ')' after parameters");

    std::string return_type = parse_return_type();

    consume(TokenType::LeftBrace, "Expected '{' before function body");
    auto body = parse_block_statement();

    return std::make_unique<FunctionExpression>(
        std::move(parameters),
        std::unique_ptr<BlockStatement>(static_cast<BlockStatement*>(body.release())),
        return_type,
        loc
    );
}

std::string Parser::parse_return_type() {
    std::string return_type;
    
    if (match(TokenType::Colon)) {
        Token type = consume(TokenType::Identifier, "Expected return type");
        return_type = type.lexeme;
    }
    
    return return_type;
}

std::unique_ptr<Statement> Parser::parse_import_declaration() {
    SourceLocation loc = previous().location;

    consume(TokenType::LeftBrace, "Expected '{' after import");
    std::vector<std::string> names;

    if (!check(TokenType::RightBrace)) {
        do {
            Token name = consume(TokenType::Identifier, "Expected imported symbol name");
            names.push_back(name.lexeme);
        } while (match(TokenType::Comma));
    }

    consume(TokenType::RightBrace, "Expected '}' after import list");
    consume(TokenType::KeywordFrom, "Expected 'from' after import list");

    Token module_token = consume(TokenType::StringLiteral, "Expected module path string");
    consume(TokenType::Semicolon, "Expected ';' after import declaration");

    return std::make_unique<ImportDeclaration>(std::move(names), module_token.lexeme, loc);
}

} // namespace dryad
