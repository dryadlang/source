#include <gtest/gtest.h>
#include "dryad/compiler/token.h"

using namespace dryad;

namespace {

Token make_token(TokenType type) {
    return Token(type, "", SourceLocation());
}

} // namespace

TEST(TokenTest, IsLiteralCoversAllLiteralTypes) {
    EXPECT_TRUE(make_token(TokenType::IntegerLiteral).is_literal());
    EXPECT_TRUE(make_token(TokenType::FloatLiteral).is_literal());
    EXPECT_TRUE(make_token(TokenType::StringLiteral).is_literal());
    EXPECT_TRUE(make_token(TokenType::BooleanLiteral).is_literal());
    EXPECT_TRUE(make_token(TokenType::NullLiteral).is_literal());
}

TEST(TokenTest, IsLiteralFalseForNonLiterals) {
    EXPECT_FALSE(make_token(TokenType::Identifier).is_literal());
    EXPECT_FALSE(make_token(TokenType::Plus).is_literal());
    EXPECT_FALSE(make_token(TokenType::KeywordLet).is_literal());
    EXPECT_FALSE(make_token(TokenType::EndOfFile).is_literal());
}

TEST(TokenTest, IsOperatorCoversAllOperatorRanges) {
    // Plus .. Percent
    EXPECT_TRUE(make_token(TokenType::Plus).is_operator());
    EXPECT_TRUE(make_token(TokenType::Percent).is_operator());
    // Equal .. GreaterEqual
    EXPECT_TRUE(make_token(TokenType::Equal).is_operator());
    EXPECT_TRUE(make_token(TokenType::GreaterEqual).is_operator());
    // PlusEqual .. SlashEqual
    EXPECT_TRUE(make_token(TokenType::PlusEqual).is_operator());
    EXPECT_TRUE(make_token(TokenType::SlashEqual).is_operator());
    // Bang .. GreaterGreater
    EXPECT_TRUE(make_token(TokenType::Bang).is_operator());
    EXPECT_TRUE(make_token(TokenType::AmpersandAmpersand).is_operator());
    EXPECT_TRUE(make_token(TokenType::GreaterGreater).is_operator());
}

TEST(TokenTest, IsOperatorFalseForNonOperators) {
    EXPECT_FALSE(make_token(TokenType::Identifier).is_operator());
    EXPECT_FALSE(make_token(TokenType::LeftParen).is_operator());
    EXPECT_FALSE(make_token(TokenType::KeywordLet).is_operator());
    EXPECT_FALSE(make_token(TokenType::IntegerLiteral).is_operator());
}

TEST(TokenTest, IsKeywordCoversTheFullRange) {
    EXPECT_TRUE(make_token(TokenType::KeywordLet).is_keyword());
    EXPECT_TRUE(make_token(TokenType::KeywordExtern).is_keyword());
    EXPECT_TRUE(make_token(TokenType::KeywordAsync).is_keyword());
}

TEST(TokenTest, IsKeywordFalseForNonKeywords) {
    EXPECT_FALSE(make_token(TokenType::Identifier).is_keyword());
    EXPECT_FALSE(make_token(TokenType::Plus).is_keyword());
    EXPECT_FALSE(make_token(TokenType::Error).is_keyword());
}

TEST(TokenTest, TokenTypeToStringCoversEveryCategory) {
    EXPECT_EQ(token_type_to_string(TokenType::EndOfFile), "EOF");
    EXPECT_EQ(token_type_to_string(TokenType::Identifier), "Identifier");
    EXPECT_EQ(token_type_to_string(TokenType::IntegerLiteral), "IntegerLiteral");
    EXPECT_EQ(token_type_to_string(TokenType::FloatLiteral), "FloatLiteral");
    EXPECT_EQ(token_type_to_string(TokenType::StringLiteral), "StringLiteral");
    EXPECT_EQ(token_type_to_string(TokenType::BooleanLiteral), "BooleanLiteral");
    EXPECT_EQ(token_type_to_string(TokenType::NullLiteral), "NullLiteral");

    EXPECT_EQ(token_type_to_string(TokenType::Plus), "+");
    EXPECT_EQ(token_type_to_string(TokenType::Minus), "-");
    EXPECT_EQ(token_type_to_string(TokenType::Star), "*");
    EXPECT_EQ(token_type_to_string(TokenType::Slash), "/");
    EXPECT_EQ(token_type_to_string(TokenType::Percent), "%");

    EXPECT_EQ(token_type_to_string(TokenType::Equal), "=");
    EXPECT_EQ(token_type_to_string(TokenType::EqualEqual), "==");
    EXPECT_EQ(token_type_to_string(TokenType::BangEqual), "!=");
    EXPECT_EQ(token_type_to_string(TokenType::Less), "<");
    EXPECT_EQ(token_type_to_string(TokenType::LessEqual), "<=");
    EXPECT_EQ(token_type_to_string(TokenType::Greater), ">");
    EXPECT_EQ(token_type_to_string(TokenType::GreaterEqual), ">=");

    EXPECT_EQ(token_type_to_string(TokenType::PlusEqual), "+=");
    EXPECT_EQ(token_type_to_string(TokenType::MinusEqual), "-=");
    EXPECT_EQ(token_type_to_string(TokenType::StarEqual), "*=");
    EXPECT_EQ(token_type_to_string(TokenType::SlashEqual), "/=");

    EXPECT_EQ(token_type_to_string(TokenType::Bang), "!");
    EXPECT_EQ(token_type_to_string(TokenType::AmpersandAmpersand), "&&");
    EXPECT_EQ(token_type_to_string(TokenType::PipePipe), "||");
    EXPECT_EQ(token_type_to_string(TokenType::Ampersand), "&");
    EXPECT_EQ(token_type_to_string(TokenType::Pipe), "|");
    EXPECT_EQ(token_type_to_string(TokenType::Caret), "^");
    EXPECT_EQ(token_type_to_string(TokenType::Tilde), "~");
    EXPECT_EQ(token_type_to_string(TokenType::LessLess), "<<");
    EXPECT_EQ(token_type_to_string(TokenType::GreaterGreater), ">>");

    EXPECT_EQ(token_type_to_string(TokenType::LeftParen), "(");
    EXPECT_EQ(token_type_to_string(TokenType::RightParen), ")");
    EXPECT_EQ(token_type_to_string(TokenType::LeftBrace), "{");
    EXPECT_EQ(token_type_to_string(TokenType::RightBrace), "}");
    EXPECT_EQ(token_type_to_string(TokenType::LeftBracket), "[");
    EXPECT_EQ(token_type_to_string(TokenType::RightBracket), "]");

    EXPECT_EQ(token_type_to_string(TokenType::Comma), ",");
    EXPECT_EQ(token_type_to_string(TokenType::Dot), ".");
    EXPECT_EQ(token_type_to_string(TokenType::Colon), ":");
    EXPECT_EQ(token_type_to_string(TokenType::Semicolon), ";");
    EXPECT_EQ(token_type_to_string(TokenType::Arrow), "->");
    EXPECT_EQ(token_type_to_string(TokenType::Question), "?");
    EXPECT_EQ(token_type_to_string(TokenType::At), "@");
    EXPECT_EQ(token_type_to_string(TokenType::Hash), "#");

    EXPECT_EQ(token_type_to_string(TokenType::KeywordLet), "let");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordConst), "const");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordFunction), "function");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordReturn), "return");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordIf), "if");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordElse), "else");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordWhile), "while");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordFor), "for");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordBreak), "break");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordContinue), "continue");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordClass), "class");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordInterface), "interface");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordAbstract), "abstract");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordExtends), "extends");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordImplements), "implements");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordReadonly), "readonly");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordGet), "get");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordSet), "set");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordDelete), "delete");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordNew), "new");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordThis), "this");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordSuper), "super");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordImport), "import");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordExport), "export");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordFrom), "from");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordAs), "as");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordAsync), "async");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordAwait), "await");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordTry), "try");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordCatch), "catch");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordFinally), "finally");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordThrow), "throw");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordTrue), "true");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordFalse), "false");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordNull), "null");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordInternal), "internal");
    EXPECT_EQ(token_type_to_string(TokenType::KeywordExtern), "extern");

    EXPECT_EQ(token_type_to_string(TokenType::Error), "Error");
}

TEST(TokenTest, TokenTypeToStringFallsBackToUnknownForUnmappedValues) {
    // Every declared TokenType has a map entry; an out-of-range raw value
    // exercises the ternary's "not found" branch instead.
    auto bogus = static_cast<TokenType>(250);
    EXPECT_EQ(token_type_to_string(bogus), "Unknown");
}
