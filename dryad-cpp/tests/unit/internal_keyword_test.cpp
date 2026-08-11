#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(InternalKeywordTest, LexerRecognizesInternalKeyword) {
    Lexer lexer("internal");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 2);
    EXPECT_EQ(tokens[0].type, TokenType::KeywordInternal);
    EXPECT_EQ(tokens[0].lexeme, "internal");
    EXPECT_EQ(tokens[1].type, TokenType::EndOfFile);
}

TEST(InternalKeywordTest, ParserRecognizesInternalFunction) {
    Lexer lexer("internal function test() { return 42; }");
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* func_decl = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(func_decl, nullptr);
    EXPECT_TRUE(func_decl->is_internal);
    EXPECT_EQ(func_decl->name, "test");
}

TEST(InternalKeywordTest, ParserRecognizesPublicFunction) {
    Lexer lexer("function test() { return 42; }");
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* func_decl = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(func_decl, nullptr);
    EXPECT_FALSE(func_decl->is_internal);
    EXPECT_EQ(func_decl->name, "test");
}

TEST(InternalKeywordTest, ParserRejectsInternalWithoutFunction) {
    Lexer lexer("internal let x = 5;");
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    EXPECT_TRUE(parser.has_error());
    EXPECT_NE(parser.error_message().find("internal"), std::string::npos);
}

TEST(InternalKeywordTest, InterpreterExecutesInternalFunction) {
    std::string code = R"(
        internal function getValue() {
            return 123;
        }
        
        let result = getValue();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto result = interpreter.global_env()->get("result");
    ASSERT_TRUE(result.is_number());
    EXPECT_EQ(result.as_integer(), 123);
}
