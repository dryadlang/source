#include "dryad/compiler/lexer.h"
#include <cctype>

namespace dryad {

std::unordered_map<std::string, TokenType> Lexer::keywords_ = {
    {"let", TokenType::KeywordLet},
    {"const", TokenType::KeywordConst},
    {"function", TokenType::KeywordFunction},
    {"return", TokenType::KeywordReturn},
    {"if", TokenType::KeywordIf},
    {"else", TokenType::KeywordElse},
    {"while", TokenType::KeywordWhile},
    {"for", TokenType::KeywordFor},
    {"break", TokenType::KeywordBreak},
    {"continue", TokenType::KeywordContinue},
    {"class", TokenType::KeywordClass},
    {"extends", TokenType::KeywordExtends},
    {"new", TokenType::KeywordNew},
    {"this", TokenType::KeywordThis},
    {"super", TokenType::KeywordSuper},
    {"import", TokenType::KeywordImport},
    {"export", TokenType::KeywordExport},
    {"from", TokenType::KeywordFrom},
    {"as", TokenType::KeywordAs},
    {"async", TokenType::KeywordAsync},
    {"await", TokenType::KeywordAwait},
    {"try", TokenType::KeywordTry},
    {"catch", TokenType::KeywordCatch},
    {"finally", TokenType::KeywordFinally},
    {"throw", TokenType::KeywordThrow},
    {"true", TokenType::KeywordTrue},
    {"false", TokenType::KeywordFalse},
    {"null", TokenType::KeywordNull},
    {"internal", TokenType::KeywordInternal}
};

Lexer::Lexer(std::string source)
    : source_(std::move(source)),
      current_(0),
      line_(1),
      column_(1),
      line_start_(0),
      has_error_(false) {}

std::vector<Token> Lexer::tokenize() {
    std::vector<Token> tokens;
    
    while (true) {
        Token token = next_token();
        tokens.push_back(token);
        
        if (token.type == TokenType::Error) {
            has_error_ = true;
            error_message_ = token.lexeme;
            break;
        }
        
        if (token.type == TokenType::EndOfFile) {
            break;
        }
    }
    
    return tokens;
}

Token Lexer::next_token() {
    skip_whitespace();
    
    if (is_at_end()) {
        return make_token(TokenType::EndOfFile, "");
    }
    
    return scan_token();
}

bool Lexer::is_at_end() const {
    return current_ >= source_.length();
}

char Lexer::peek() const {
    if (is_at_end()) return '\0';
    return source_[current_];
}

char Lexer::peek_next() const {
    if (current_ + 1 >= source_.length()) return '\0';
    return source_[current_ + 1];
}

char Lexer::advance() {
    char c = source_[current_++];
    if (c == '\n') {
        line_++;
        line_start_ = current_;
        column_ = 1;
    } else {
        column_++;
    }
    return c;
}

bool Lexer::match(char expected) {
    if (is_at_end()) return false;
    if (source_[current_] != expected) return false;
    advance();
    return true;
}

void Lexer::skip_whitespace() {
    while (!is_at_end()) {
        char c = peek();
        
        if (c == ' ' || c == '\t' || c == '\r' || c == '\n') {
            advance();
        } else if (c == '/' && peek_next() == '/') {
            skip_line_comment();
        } else if (c == '/' && peek_next() == '*') {
            skip_block_comment();
        } else {
            break;
        }
    }
}

void Lexer::skip_line_comment() {
    while (!is_at_end() && peek() != '\n') {
        advance();
    }
}

void Lexer::skip_block_comment() {
    advance();
    advance();
    
    while (!is_at_end()) {
        if (peek() == '*' && peek_next() == '/') {
            advance();
            advance();
            return;
        }
        advance();
    }
}

SourceLocation Lexer::current_location() const {
    return SourceLocation(line_, column_, current_);
}

Token Lexer::make_token(TokenType type, const std::string& lexeme) {
    return Token(type, lexeme, current_location());
}

Token Lexer::make_error(const std::string& message) {
    return Token(TokenType::Error, message, current_location());
}

Token Lexer::scan_token() {
    char c = advance();
    SourceLocation start_loc = current_location();
    
    switch (c) {
        case '(': return Token(TokenType::LeftParen, "(", start_loc);
        case ')': return Token(TokenType::RightParen, ")", start_loc);
        case '{': return Token(TokenType::LeftBrace, "{", start_loc);
        case '}': return Token(TokenType::RightBrace, "}", start_loc);
        case '[': return Token(TokenType::LeftBracket, "[", start_loc);
        case ']': return Token(TokenType::RightBracket, "]", start_loc);
        case ',': return Token(TokenType::Comma, ",", start_loc);
        case '.': return Token(TokenType::Dot, ".", start_loc);
        case ':': return Token(TokenType::Colon, ":", start_loc);
        case ';': return Token(TokenType::Semicolon, ";", start_loc);
        case '?': return Token(TokenType::Question, "?", start_loc);
        
        case '+':
            if (match('=')) return Token(TokenType::PlusEqual, "+=", start_loc);
            return Token(TokenType::Plus, "+", start_loc);
        
        case '-':
            if (match('=')) return Token(TokenType::MinusEqual, "-=", start_loc);
            if (match('>')) return Token(TokenType::Arrow, "->", start_loc);
            return Token(TokenType::Minus, "-", start_loc);
        
        case '*':
            if (match('=')) return Token(TokenType::StarEqual, "*=", start_loc);
            return Token(TokenType::Star, "*", start_loc);
        
        case '/':
            if (match('=')) return Token(TokenType::SlashEqual, "/=", start_loc);
            return Token(TokenType::Slash, "/", start_loc);
        
        case '%': return Token(TokenType::Percent, "%", start_loc);
        
        case '!':
            if (match('=')) return Token(TokenType::BangEqual, "!=", start_loc);
            return Token(TokenType::Bang, "!", start_loc);
        
        case '=':
            if (match('=')) return Token(TokenType::EqualEqual, "==", start_loc);
            return Token(TokenType::Equal, "=", start_loc);
        
        case '<':
            if (match('=')) return Token(TokenType::LessEqual, "<=", start_loc);
            return Token(TokenType::Less, "<", start_loc);
        
        case '>':
            if (match('=')) return Token(TokenType::GreaterEqual, ">=", start_loc);
            return Token(TokenType::Greater, ">", start_loc);
        
        case '&':
            if (match('&')) return Token(TokenType::AmpersandAmpersand, "&&", start_loc);
            return make_error("Unexpected character '&'");
        
        case '|':
            if (match('|')) return Token(TokenType::PipePipe, "||", start_loc);
            return make_error("Unexpected character '|'");
        
        case '"': {
            current_--;
            column_--;
            return scan_string();
        }
        
        default:
            if (std::isdigit(c)) {
                current_--;
                column_--;
                return scan_number();
            }
            
            if (std::isalpha(c) || c == '_') {
                current_--;
                column_--;
                return scan_identifier();
            }
            
            return make_error(std::string("Unexpected character '") + c + "'");
    }
}

Token Lexer::scan_string() {
    SourceLocation start_loc = current_location();
    std::string value;
    
    advance();
    
    while (!is_at_end() && peek() != '"') {
        if (peek() == '\\') {
            advance();
            if (is_at_end()) {
                return make_error("Unterminated string");
            }
            
            char escaped = advance();
            switch (escaped) {
                case 'n': value += '\n'; break;
                case 't': value += '\t'; break;
                case 'r': value += '\r'; break;
                case '\\': value += '\\'; break;
                case '"': value += '"'; break;
                default:
                    value += '\\';
                    value += escaped;
                    break;
            }
        } else {
            value += advance();
        }
    }
    
    if (is_at_end()) {
        return make_error("Unterminated string");
    }
    
    advance();
    
    return Token(TokenType::StringLiteral, value, start_loc);
}

Token Lexer::scan_number() {
    SourceLocation start_loc = current_location();
    std::string number;
    
    while (!is_at_end() && std::isdigit(peek())) {
        number += advance();
    }
    
    if (!is_at_end() && peek() == '.' && std::isdigit(peek_next())) {
        number += advance();
        
        while (!is_at_end() && std::isdigit(peek())) {
            number += advance();
        }
        
        return Token(TokenType::FloatLiteral, number, start_loc);
    }
    
    return Token(TokenType::IntegerLiteral, number, start_loc);
}

Token Lexer::scan_identifier() {
    SourceLocation start_loc = current_location();
    std::string identifier;
    
    while (!is_at_end() && (std::isalnum(peek()) || peek() == '_')) {
        identifier += advance();
    }
    
    auto it = keywords_.find(identifier);
    if (it != keywords_.end()) {
        return Token(it->second, identifier, start_loc);
    }
    
    return Token(TokenType::Identifier, identifier, start_loc);
}

} // namespace dryad
