#include "dryad/compiler/token.h"

namespace dryad {

bool Token::is_literal() const {
    return type == TokenType::IntegerLiteral ||
           type == TokenType::FloatLiteral ||
           type == TokenType::StringLiteral ||
           type == TokenType::BooleanLiteral ||
           type == TokenType::NullLiteral;
}

bool Token::is_operator() const {
    return (type >= TokenType::Plus && type <= TokenType::Percent) ||
           (type >= TokenType::Equal && type <= TokenType::GreaterEqual) ||
           (type >= TokenType::PlusEqual && type <= TokenType::SlashEqual) ||
           (type >= TokenType::Bang && type <= TokenType::PipePipe);
}

bool Token::is_keyword() const {
    return type >= TokenType::KeywordLet && type <= TokenType::KeywordNull;
}

std::string token_type_to_string(TokenType type) {
    static const std::unordered_map<TokenType, std::string> names = {
        {TokenType::EndOfFile, "EOF"},
        {TokenType::Identifier, "Identifier"},
        {TokenType::IntegerLiteral, "IntegerLiteral"},
        {TokenType::FloatLiteral, "FloatLiteral"},
        {TokenType::StringLiteral, "StringLiteral"},
        {TokenType::BooleanLiteral, "BooleanLiteral"},
        {TokenType::NullLiteral, "NullLiteral"},
        {TokenType::Plus, "+"},
        {TokenType::Minus, "-"},
        {TokenType::Star, "*"},
        {TokenType::Slash, "/"},
        {TokenType::Percent, "%"},
        {TokenType::Equal, "="},
        {TokenType::EqualEqual, "=="},
        {TokenType::BangEqual, "!="},
        {TokenType::Less, "<"},
        {TokenType::LessEqual, "<="},
        {TokenType::Greater, ">"},
        {TokenType::GreaterEqual, ">="},
        {TokenType::PlusEqual, "+="},
        {TokenType::MinusEqual, "-="},
        {TokenType::StarEqual, "*="},
        {TokenType::SlashEqual, "/="},
        {TokenType::Bang, "!"},
        {TokenType::AmpersandAmpersand, "&&"},
        {TokenType::PipePipe, "||"},
        {TokenType::LeftParen, "("},
        {TokenType::RightParen, ")"},
        {TokenType::LeftBrace, "{"},
        {TokenType::RightBrace, "}"},
        {TokenType::LeftBracket, "["},
        {TokenType::RightBracket, "]"},
        {TokenType::Comma, ","},
        {TokenType::Dot, "."},
        {TokenType::Colon, ":"},
        {TokenType::Semicolon, ";"},
        {TokenType::Arrow, "->"},
        {TokenType::Question, "?"},
        {TokenType::KeywordLet, "let"},
        {TokenType::KeywordConst, "const"},
        {TokenType::KeywordFunction, "function"},
        {TokenType::KeywordReturn, "return"},
        {TokenType::KeywordIf, "if"},
        {TokenType::KeywordElse, "else"},
        {TokenType::KeywordWhile, "while"},
        {TokenType::KeywordFor, "for"},
        {TokenType::KeywordBreak, "break"},
        {TokenType::KeywordContinue, "continue"},
        {TokenType::KeywordClass, "class"},
        {TokenType::KeywordExtends, "extends"},
        {TokenType::KeywordNew, "new"},
        {TokenType::KeywordThis, "this"},
        {TokenType::KeywordSuper, "super"},
        {TokenType::KeywordImport, "import"},
        {TokenType::KeywordExport, "export"},
        {TokenType::KeywordFrom, "from"},
        {TokenType::KeywordAs, "as"},
        {TokenType::KeywordAsync, "async"},
        {TokenType::KeywordAwait, "await"},
        {TokenType::KeywordTry, "try"},
        {TokenType::KeywordCatch, "catch"},
        {TokenType::KeywordFinally, "finally"},
        {TokenType::KeywordThrow, "throw"},
        {TokenType::KeywordTrue, "true"},
        {TokenType::KeywordFalse, "false"},
        {TokenType::KeywordNull, "null"},
        {TokenType::KeywordInternal, "internal"},
        {TokenType::Error, "Error"}
    };
    
    auto it = names.find(type);
    return it != names.end() ? it->second : "Unknown";
}

} // namespace dryad
