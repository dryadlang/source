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

void Parser::error(const std::string& message) {
    has_error_ = true;
    if (error_message_.empty()) {
        error_message_ = message;
    }
}

void Parser::synchronize() {
    advance();
    
    while (!is_at_end()) {
        if (previous().type == TokenType::Semicolon) return;
        
        switch (peek().type) {
            case TokenType::KeywordClass:
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

std::unique_ptr<Statement> Parser::parse_declaration() {
    // Check for 'internal' modifier
    bool is_internal = false;
    if (match(TokenType::KeywordInternal)) {
        is_internal = true;
    }
    
    if (match(TokenType::KeywordLet) || match(TokenType::KeywordConst)) {
        if (is_internal) {
            throw DryadException("'internal' modifier cannot be used with variable declarations");
        }
        return parse_variable_declaration();
    }
    
    if (match(TokenType::KeywordFunction)) {
        return parse_function_declaration(is_internal);
    }
    
    if (match(TokenType::KeywordClass)) {
        return parse_class_declaration(is_internal);
    }
    
    // If we saw 'internal' but no function/class follows, error
    if (is_internal) {
        throw DryadException("'internal' modifier can only be used with functions or classes");
    }
    
    return parse_statement();
}

std::unique_ptr<Statement> Parser::parse_variable_declaration() {
    bool is_const = previous().type == TokenType::KeywordConst;
    SourceLocation loc = previous().location;
    
    Token name = consume(TokenType::Identifier, "Expected variable name");
    
    if (match(TokenType::Colon)) {
        advance();
    }
    
    std::unique_ptr<Expression> initializer = nullptr;
    if (match(TokenType::Equal)) {
        initializer = parse_expression();
    }
    
    consume(TokenType::Semicolon, "Expected ';' after variable declaration");
    
    return std::make_unique<VariableDeclaration>(name.lexeme, std::move(initializer), is_const, loc);
}

std::unique_ptr<Statement> Parser::parse_function_declaration(bool is_internal) {
    SourceLocation loc = previous().location;
    
    Token name = consume(TokenType::Identifier, "Expected function name");
    
    consume(TokenType::LeftParen, "Expected '(' after function name");
    
    std::vector<Parameter> parameters;
    if (!check(TokenType::RightParen)) {
        do {
            Token param_name = consume(TokenType::Identifier, "Expected parameter name");
            
            std::string type_annotation;
            if (match(TokenType::Colon)) {
                Token type = consume(TokenType::Identifier, "Expected type annotation");
                type_annotation = type.lexeme;
            }
            
            parameters.push_back({param_name.lexeme, type_annotation});
        } while (match(TokenType::Comma));
    }
    
    consume(TokenType::RightParen, "Expected ')' after parameters");
    
    std::string return_type;
    if (match(TokenType::Colon)) {
        Token type = consume(TokenType::Identifier, "Expected return type");
        return_type = type.lexeme;
    }
    
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
            Token name = consume(TokenType::Identifier, "Expected property name after '.'");
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
    
    if (match(TokenType::KeywordThis)) {
        return std::make_unique<ThisExpression>(previous().location);
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
    
    throw DryadException("Expected expression");
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

std::unique_ptr<Statement> Parser::parse_class_declaration(bool is_internal) {
    SourceLocation loc = previous().location;
    (void)is_internal;
    
    Token name = consume(TokenType::Identifier, "Expected class name");
    
    std::string super_class;
    if (match(TokenType::Identifier) && previous().lexeme == "extends") {
        Token super = consume(TokenType::Identifier, "Expected superclass name");
        super_class = super.lexeme;
    }
    
    consume(TokenType::LeftBrace, "Expected '{' before class body");
    
    std::vector<std::unique_ptr<MethodDeclaration>> methods;
    
    while (!check(TokenType::RightBrace) && !is_at_end()) {
        Token method_name = consume(TokenType::Identifier, "Expected method name");
        
        consume(TokenType::LeftParen, "Expected '(' after method name");
        
        std::vector<Parameter> parameters;
        if (!check(TokenType::RightParen)) {
            do {
                Token param_name = consume(TokenType::Identifier, "Expected parameter name");
                parameters.push_back({param_name.lexeme, ""});
                
                if (match(TokenType::Colon)) {
                    advance();
                }
            } while (match(TokenType::Comma));
        }
        
        consume(TokenType::RightParen, "Expected ')' after parameters");
        
        if (match(TokenType::Colon)) {
            advance();
        }
        
        consume(TokenType::LeftBrace, "Expected '{' before method body");
        auto body_statements = std::vector<std::unique_ptr<Statement>>();
        
        while (!check(TokenType::RightBrace) && !is_at_end()) {
            body_statements.push_back(parse_declaration());
        }
        
        consume(TokenType::RightBrace, "Expected '}' after method body");
        
        auto body = std::make_unique<BlockStatement>(std::move(body_statements), loc);
        bool is_constructor = (method_name.lexeme == "constructor");
        
        methods.push_back(std::make_unique<MethodDeclaration>(
            method_name.lexeme, std::move(parameters), std::move(body), is_constructor
        ));
    }
    
    consume(TokenType::RightBrace, "Expected '}' after class body");
    
    return std::make_unique<ClassDeclaration>(name.lexeme, super_class, std::move(methods), loc);
}

} // namespace dryad
