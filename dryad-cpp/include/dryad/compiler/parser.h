#ifndef DRYAD_COMPILER_PARSER_H
#define DRYAD_COMPILER_PARSER_H

#include "dryad/compiler/diagnostic.h"
#include "dryad/compiler/token.h"
#include "dryad/compiler/ast.h"
#include <vector>
#include <memory>
#include <string>

namespace dryad {

class Parser {
public:
    explicit Parser(std::vector<Token> tokens);
    
    std::unique_ptr<Program> parse();
    
    bool has_error() const { return has_error_; }
    const std::string& error_message() const { return error_message_; }
    const std::vector<std::string>& errors() const { return errors_; }
    const std::vector<Diagnostic>& diagnostics() const { return diagnostics_; }
    
private:
    std::vector<Token> tokens_;
    size_t current_;
    bool has_error_;
    std::string error_message_;
    std::vector<std::string> errors_;
    std::vector<Diagnostic> diagnostics_;
    
    Token peek() const;
    Token previous() const;
    Token advance();
    bool is_at_end() const;
    bool check(TokenType type) const;
    bool match(TokenType type);
    bool match(std::initializer_list<TokenType> types);
    Token consume(TokenType type, const std::string& message);
    Token consume_member_name(const std::string& message);
    
    void error(const std::string& message);
    void error_at(const SourceLocation& location, const std::string& message);
    void synchronize();
    void synchronize_class_member();
    void synchronize_expression();
    
    std::unique_ptr<Statement> parse_statement();
    std::unique_ptr<Statement> parse_declaration();
    std::unique_ptr<Statement> parse_variable_declaration();
    std::unique_ptr<Statement> parse_function_declaration(bool is_internal = false, bool is_async = false);
    std::unique_ptr<Statement> parse_class_declaration(bool is_internal = false, bool is_abstract = false);
    std::unique_ptr<Statement> parse_interface_declaration(bool is_internal = false);
    std::unique_ptr<PropertyDeclaration> parse_property_declaration(bool is_abstract, bool is_readonly, const Token& name, SourceLocation loc);
    void parse_property_accessors(PropertyDeclaration& property, SourceLocation loc);
    std::unique_ptr<MethodDeclaration> parse_method_declaration(bool is_abstract, const Token& name, SourceLocation loc, bool is_getter = false, bool is_setter = false);
    std::unique_ptr<Statement> parse_intrinsic_declaration();
    std::unique_ptr<Statement> parse_block_statement();
    std::unique_ptr<Statement> parse_expression_statement();
    std::unique_ptr<Statement> parse_if_statement();
    std::unique_ptr<Statement> parse_while_statement();
    std::unique_ptr<Statement> parse_return_statement();
    std::unique_ptr<Statement> parse_import_declaration();
    
    std::string parse_intrinsic_name();
    std::vector<Parameter> parse_parameters();
    std::string parse_return_type();
    std::unique_ptr<FunctionExpression> parse_function_expression();
    
    std::unique_ptr<Expression> parse_expression();
    std::unique_ptr<Expression> parse_assignment();
    std::unique_ptr<Expression> parse_logical_or();
    std::unique_ptr<Expression> parse_logical_and();
    std::unique_ptr<Expression> parse_equality();
    std::unique_ptr<Expression> parse_comparison();
    std::unique_ptr<Expression> parse_addition();
    std::unique_ptr<Expression> parse_multiplication();
    std::unique_ptr<Expression> parse_unary();
    std::unique_ptr<Expression> parse_postfix();
    std::unique_ptr<Expression> parse_primary();
    std::unique_ptr<Expression> parse_array_literal();
};

} // namespace dryad

#endif // DRYAD_COMPILER_PARSER_H
