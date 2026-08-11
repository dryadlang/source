#ifndef DRYAD_COMPILER_TOKEN_H
#define DRYAD_COMPILER_TOKEN_H

#include <string>
#include <unordered_map>

namespace dryad {

enum class TokenType {
    EndOfFile,
    
    Identifier,
    
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,
    
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    
    Bang,
    AmpersandAmpersand,
    PipePipe,
    
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    Question,
    At,          // @ decorator marker
    Hash,        // # module/intrinsic name prefix
    
    KeywordLet,
    KeywordConst,
    KeywordFunction,
    KeywordReturn,
    KeywordIf,
    KeywordElse,
    KeywordWhile,
    KeywordFor,
    KeywordBreak,
    KeywordContinue,
    KeywordClass,
    KeywordExtends,
    KeywordNew,
    KeywordThis,
    KeywordSuper,
    KeywordImport,
    KeywordExport,
    KeywordFrom,
    KeywordAs,
    KeywordAsync,
    KeywordAwait,
    KeywordTry,
    KeywordCatch,
    KeywordFinally,
    KeywordThrow,
    KeywordTrue,
    KeywordFalse,
    KeywordNull,
    KeywordInternal,
    KeywordExtern,   // extern for intrinsic functions
    
    Error
};

struct SourceLocation {
    size_t line;
    size_t column;
    size_t offset;
    
    SourceLocation(size_t line = 1, size_t column = 1, size_t offset = 0)
        : line(line), column(column), offset(offset) {}
};

struct Token {
    TokenType type;
    std::string lexeme;
    SourceLocation location;
    
    Token(TokenType type, std::string lexeme, SourceLocation location)
        : type(type), lexeme(std::move(lexeme)), location(location) {}
    
    bool is_literal() const;
    bool is_operator() const;
    bool is_keyword() const;
};

std::string token_type_to_string(TokenType type);

} // namespace dryad

#endif // DRYAD_COMPILER_TOKEN_H
