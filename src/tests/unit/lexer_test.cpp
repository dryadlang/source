#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"

using namespace dryad;

TEST(LexerTest, EmptySource) {
    Lexer lexer("");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 1);
    EXPECT_EQ(tokens[0].type, TokenType::EndOfFile);
}

TEST(LexerTest, Whitespace) {
    Lexer lexer("   \t\n\r  ");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 1);
    EXPECT_EQ(tokens[0].type, TokenType::EndOfFile);
}

TEST(LexerTest, LineComment) {
    Lexer lexer("// this is a comment\n42");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 2);
    EXPECT_EQ(tokens[0].type, TokenType::IntegerLiteral);
    EXPECT_EQ(tokens[0].lexeme, "42");
}

TEST(LexerTest, BlockComment) {
    Lexer lexer("/* block comment */ 42");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 2);
    EXPECT_EQ(tokens[0].type, TokenType::IntegerLiteral);
    EXPECT_EQ(tokens[0].lexeme, "42");
}

TEST(LexerTest, IntegerLiterals) {
    Lexer lexer("0 42 123456");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 4);
    EXPECT_EQ(tokens[0].type, TokenType::IntegerLiteral);
    EXPECT_EQ(tokens[0].lexeme, "0");
    EXPECT_EQ(tokens[1].type, TokenType::IntegerLiteral);
    EXPECT_EQ(tokens[1].lexeme, "42");
    EXPECT_EQ(tokens[2].type, TokenType::IntegerLiteral);
    EXPECT_EQ(tokens[2].lexeme, "123456");
}

TEST(LexerTest, FloatLiterals) {
    Lexer lexer("3.14 0.5 123.456");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 4);
    EXPECT_EQ(tokens[0].type, TokenType::FloatLiteral);
    EXPECT_EQ(tokens[0].lexeme, "3.14");
    EXPECT_EQ(tokens[1].type, TokenType::FloatLiteral);
    EXPECT_EQ(tokens[1].lexeme, "0.5");
    EXPECT_EQ(tokens[2].type, TokenType::FloatLiteral);
    EXPECT_EQ(tokens[2].lexeme, "123.456");
}

TEST(LexerTest, StringLiterals) {
    Lexer lexer(R"("hello" "world" "")");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 4);
    EXPECT_EQ(tokens[0].type, TokenType::StringLiteral);
    EXPECT_EQ(tokens[0].lexeme, "hello");
    EXPECT_EQ(tokens[1].type, TokenType::StringLiteral);
    EXPECT_EQ(tokens[1].lexeme, "world");
    EXPECT_EQ(tokens[2].type, TokenType::StringLiteral);
    EXPECT_EQ(tokens[2].lexeme, "");
}

TEST(LexerTest, StringEscapes) {
    Lexer lexer(R"("hello\nworld" "tab\there" "quote\"test")");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 4);
    EXPECT_EQ(tokens[0].lexeme, "hello\nworld");
    EXPECT_EQ(tokens[1].lexeme, "tab\there");
    EXPECT_EQ(tokens[2].lexeme, "quote\"test");
}

TEST(LexerTest, UnterminatedString) {
    Lexer lexer(R"("unterminated)");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 1);
    EXPECT_EQ(tokens[0].type, TokenType::Error);
    EXPECT_TRUE(lexer.has_error());
}

TEST(LexerTest, Identifiers) {
    Lexer lexer("foo bar_baz test123 _private");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 5);
    EXPECT_EQ(tokens[0].type, TokenType::Identifier);
    EXPECT_EQ(tokens[0].lexeme, "foo");
    EXPECT_EQ(tokens[1].type, TokenType::Identifier);
    EXPECT_EQ(tokens[1].lexeme, "bar_baz");
    EXPECT_EQ(tokens[2].type, TokenType::Identifier);
    EXPECT_EQ(tokens[2].lexeme, "test123");
    EXPECT_EQ(tokens[3].type, TokenType::Identifier);
    EXPECT_EQ(tokens[3].lexeme, "_private");
}

TEST(LexerTest, Keywords) {
    Lexer lexer("let const function return if else while for");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 9);
    EXPECT_EQ(tokens[0].type, TokenType::KeywordLet);
    EXPECT_EQ(tokens[1].type, TokenType::KeywordConst);
    EXPECT_EQ(tokens[2].type, TokenType::KeywordFunction);
    EXPECT_EQ(tokens[3].type, TokenType::KeywordReturn);
    EXPECT_EQ(tokens[4].type, TokenType::KeywordIf);
    EXPECT_EQ(tokens[5].type, TokenType::KeywordElse);
    EXPECT_EQ(tokens[6].type, TokenType::KeywordWhile);
    EXPECT_EQ(tokens[7].type, TokenType::KeywordFor);
}

TEST(LexerTest, BooleanKeywords) {
    Lexer lexer("true false null");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 4);
    EXPECT_EQ(tokens[0].type, TokenType::KeywordTrue);
    EXPECT_EQ(tokens[1].type, TokenType::KeywordFalse);
    EXPECT_EQ(tokens[2].type, TokenType::KeywordNull);
}

TEST(LexerTest, SingleCharOperators) {
    Lexer lexer("+ - * / % = ! < >");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 10);
    EXPECT_EQ(tokens[0].type, TokenType::Plus);
    EXPECT_EQ(tokens[1].type, TokenType::Minus);
    EXPECT_EQ(tokens[2].type, TokenType::Star);
    EXPECT_EQ(tokens[3].type, TokenType::Slash);
    EXPECT_EQ(tokens[4].type, TokenType::Percent);
    EXPECT_EQ(tokens[5].type, TokenType::Equal);
    EXPECT_EQ(tokens[6].type, TokenType::Bang);
    EXPECT_EQ(tokens[7].type, TokenType::Less);
    EXPECT_EQ(tokens[8].type, TokenType::Greater);
}

TEST(LexerTest, TwoCharOperators) {
    Lexer lexer("== != <= >= += -= *= /= -> && ||");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 12);
    EXPECT_EQ(tokens[0].type, TokenType::EqualEqual);
    EXPECT_EQ(tokens[1].type, TokenType::BangEqual);
    EXPECT_EQ(tokens[2].type, TokenType::LessEqual);
    EXPECT_EQ(tokens[3].type, TokenType::GreaterEqual);
    EXPECT_EQ(tokens[4].type, TokenType::PlusEqual);
    EXPECT_EQ(tokens[5].type, TokenType::MinusEqual);
    EXPECT_EQ(tokens[6].type, TokenType::StarEqual);
    EXPECT_EQ(tokens[7].type, TokenType::SlashEqual);
    EXPECT_EQ(tokens[8].type, TokenType::Arrow);
    EXPECT_EQ(tokens[9].type, TokenType::AmpersandAmpersand);
    EXPECT_EQ(tokens[10].type, TokenType::PipePipe);
}

TEST(LexerTest, Delimiters) {
    Lexer lexer("( ) { } [ ] , . : ; ?");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 12);
    EXPECT_EQ(tokens[0].type, TokenType::LeftParen);
    EXPECT_EQ(tokens[1].type, TokenType::RightParen);
    EXPECT_EQ(tokens[2].type, TokenType::LeftBrace);
    EXPECT_EQ(tokens[3].type, TokenType::RightBrace);
    EXPECT_EQ(tokens[4].type, TokenType::LeftBracket);
    EXPECT_EQ(tokens[5].type, TokenType::RightBracket);
    EXPECT_EQ(tokens[6].type, TokenType::Comma);
    EXPECT_EQ(tokens[7].type, TokenType::Dot);
    EXPECT_EQ(tokens[8].type, TokenType::Colon);
    EXPECT_EQ(tokens[9].type, TokenType::Semicolon);
    EXPECT_EQ(tokens[10].type, TokenType::Question);
}

TEST(LexerTest, SourceLocation) {
    Lexer lexer("let x = 42;\nlet y = 10;");
    auto tokens = lexer.tokenize();
    
    EXPECT_EQ(tokens[0].location.line, 1);
    EXPECT_EQ(tokens[0].location.column, 1);
    
    EXPECT_EQ(tokens[5].location.line, 2);
}

TEST(LexerTest, CompleteProgram) {
    Lexer lexer(R"(
        function add(a: number, b: number): number {
            return a + b;
        }
        
        let result = add(10, 20);
    )");
    
    auto tokens = lexer.tokenize();
    
    EXPECT_FALSE(lexer.has_error());
    EXPECT_GT(tokens.size(), 20);
    
    EXPECT_EQ(tokens[0].type, TokenType::KeywordFunction);
    EXPECT_EQ(tokens[1].type, TokenType::Identifier);
    EXPECT_EQ(tokens[1].lexeme, "add");
}

TEST(LexerTest, ImportStatement) {
    Lexer lexer(R"(import { readFile } from "@std/io";)");
    auto tokens = lexer.tokenize();
    
    EXPECT_FALSE(lexer.has_error());
    
    int import_count = 0;
    int from_count = 0;
    for (const auto& token : tokens) {
        if (token.type == TokenType::KeywordImport) import_count++;
        if (token.type == TokenType::KeywordFrom) from_count++;
    }
    
    EXPECT_EQ(import_count, 1);
    EXPECT_EQ(from_count, 1);
}

TEST(LexerTest, ClassDeclaration) {
    Lexer lexer(R"(
        class Foo extends Bar {
            constructor() {
                super();
                this.value = 42;
            }
        }
    )");
    
    auto tokens = lexer.tokenize();
    
    EXPECT_FALSE(lexer.has_error());
    
    bool has_class = false;
    bool has_extends = false;
    bool has_super = false;
    bool has_this = false;
    
    for (const auto& token : tokens) {
        if (token.type == TokenType::KeywordClass) has_class = true;
        if (token.type == TokenType::KeywordExtends) has_extends = true;
        if (token.type == TokenType::KeywordSuper) has_super = true;
        if (token.type == TokenType::KeywordThis) has_this = true;
    }
    
    EXPECT_TRUE(has_class);
    EXPECT_TRUE(has_extends);
    EXPECT_TRUE(has_super);
    EXPECT_TRUE(has_this);
}

TEST(LexerTest, AsyncAwait) {
    Lexer lexer("async function fetch() { await getData(); }");
    auto tokens = lexer.tokenize();
    
    EXPECT_FALSE(lexer.has_error());
    
    bool has_async = false;
    bool has_await = false;
    
    for (const auto& token : tokens) {
        if (token.type == TokenType::KeywordAsync) has_async = true;
        if (token.type == TokenType::KeywordAwait) has_await = true;
    }
    
    EXPECT_TRUE(has_async);
    EXPECT_TRUE(has_await);
}

TEST(LexerTest, TryCatchFinally) {
    Lexer lexer("try { risky(); } catch (e) { handle(); } finally { cleanup(); }");
    auto tokens = lexer.tokenize();
    
    EXPECT_FALSE(lexer.has_error());
    
    int try_count = 0;
    int catch_count = 0;
    int finally_count = 0;
    
    for (const auto& token : tokens) {
        if (token.type == TokenType::KeywordTry) try_count++;
        if (token.type == TokenType::KeywordCatch) catch_count++;
        if (token.type == TokenType::KeywordFinally) finally_count++;
    }
    
    EXPECT_EQ(try_count, 1);
    EXPECT_EQ(catch_count, 1);
    EXPECT_EQ(finally_count, 1);
}
