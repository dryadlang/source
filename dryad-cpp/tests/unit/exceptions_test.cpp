#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(ExceptionsTest, GetStackTraceIntrinsic) {
    std::string code = R"(
        let trace = __get_stack_trace();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto trace = interpreter.global_env()->get("trace");
    ASSERT_TRUE(trace.is_string());
    EXPECT_FALSE(trace.as_string().empty());
    EXPECT_NE(trace.as_string().find("Stack trace"), std::string::npos);
}

TEST(ExceptionsTest, IntrinsicRegistered) {
    Interpreter interpreter;
    
    auto intrinsic_fn = interpreter.global_env()->get("__get_stack_trace");
    ASSERT_TRUE(intrinsic_fn.is_function());
}

TEST(ExceptionsTest, ArgumentNullExceptionFactory) {
    std::string code = R"(
        function ArgumentNullException(paramName) {
            return paramName + " cannot be null";
        }
        let ex = ArgumentNullException("param");
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto ex = interpreter.global_env()->get("ex");
    ASSERT_TRUE(ex.is_string());
    EXPECT_EQ(ex.as_string(), "param cannot be null");
}

TEST(ExceptionsTest, ObjectDisposedExceptionFactory) {
    std::string code = R"(
        function ObjectDisposedException(objectName) {
            return "Cannot access disposed object: " + objectName;
        }
        let ex = ObjectDisposedException("FileStream");
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto ex = interpreter.global_env()->get("ex");
    ASSERT_TRUE(ex.is_string());
    EXPECT_EQ(ex.as_string(), "Cannot access disposed object: FileStream");
}

