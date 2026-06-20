#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(DisposableTest, PlaceholderExists) {
    std::string code = R"(
        function IDisposable() {
            return "IDisposable interface placeholder";
        }
        let result = IDisposable();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto result = interpreter.global_env()->get("result");
    ASSERT_TRUE(result.is_string());
    EXPECT_EQ(result.as_string(), "IDisposable interface placeholder");
}

// Test that the dispose pattern can be implemented with functions
// (This is the pattern we'll use until class support is available)
TEST(DisposableTest, DisposePatternWithFunctions) {
    std::string code = R"(
        let disposed = false;
        
        function dispose() {
            disposed = true;
        }
        
        // Simulate using a resource
        dispose();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto disposed = interpreter.global_env()->get("disposed");
    ASSERT_TRUE(disposed.is_boolean());
    EXPECT_TRUE(disposed.as_boolean());
}
