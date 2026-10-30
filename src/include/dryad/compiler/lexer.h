#ifndef DRYAD_COMPILER_LEXER_H
#define DRYAD_COMPILER_LEXER_H

#include "dryad/compiler/token.h"
#include <string>
#include <vector>
#include <unordered_map>

namespace dryad {

class Lexer {
public:
    explicit Lexer(std::string source);
    
    std::vector<Token> tokenize();
    Token next_token();
    
    bool has_error() const { return has_error_; }
    const std::string& error_message() const { return error_message_; }
    
private:
    std::string source_;
    size_t current_;
    size_t line_;
    size_t column_;
    size_t line_start_;
    
    bool has_error_;
    std::string error_message_;
    
    static std::unordered_map<std::string, TokenType> keywords_;
    
    bool is_at_end() const;
    char peek() const;
    char peek_next() const;
    char advance();
    bool match(char expected);
    
    void skip_whitespace();
    void skip_line_comment();
    void skip_block_comment();
    
    Token make_token(TokenType type, const std::string& lexeme);
    Token make_error(const std::string& message);
    
    Token scan_token();
    Token scan_string();
    Token scan_number();
    Token scan_identifier();
    
    SourceLocation current_location() const;
};

} // namespace dryad

#endif // DRYAD_COMPILER_LEXER_H
