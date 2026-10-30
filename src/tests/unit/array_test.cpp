#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(ArrayTest, SimpleLiteral) {
    std::string code = R"(
        let arr = [1, 2, 3];
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto arr = interpreter.global_env()->get("arr");
    ASSERT_TRUE(arr.is_array());
}

TEST(ArrayTest, IndexAccess) {
    std::string code = R"(
        let arr = [10, 20, 30];
        let val = arr[1];
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto val = interpreter.global_env()->get("val");
    ASSERT_TRUE(val.is_integer());
    EXPECT_EQ(val.as_integer(), 20);
}

