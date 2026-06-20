#ifndef DRYAD_COMPILER_PARSER_H
#define DRYAD_COMPILER_PARSER_H

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
    
private:
    std::vector<Token> tokens_;
    size_t current_;
    bool has_error_;
    std::string error_message_;
    
    Token peek() const;
    Token previous() const;
    Token advance();
    bool is_at_end() const;
    bool check(TokenType type) const;
    bool match(TokenType type);
    bool match(std::initializer_list<TokenType> types);
    Token consume(TokenType type, const std::string& message);
    
    void error(const std::string& message);
    void synchronize();
    
    std::unique_ptr<Statement> parse_statement();
    std::unique_ptr<Statement> parse_declaration();
    std::unique_ptr<Statement> parse_variable_declaration();
    std::unique_ptr<Statement> parse_function_declaration(bool is_internal = false);
    std::unique_ptr<Statement> parse_class_declaration(bool is_internal = false);
    std::unique_ptr<Statement> parse_block_statement();
    std::unique_ptr<Statement> parse_expression_statement();
    std::unique_ptr<Statement> parse_if_statement();
    std::unique_ptr<Statement> parse_while_statement();
    std::unique_ptr<Statement> parse_return_statement();
    
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
