#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(ExceptionsTest, GetStackTraceIntrinsic) {
    std::string code = R"(
        let trace = __runtime_get_stack_trace();
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
    
    auto intrinsic_fn = interpreter.global_env()->get("__runtime_get_stack_trace");
    ASSERT_TRUE(intrinsic_fn.is_function());
}

// The factories themselves are covered against the real @std/core module in
// tests/stdlib/core/exceptions_test.dryad (StdlibDryadTest).
