#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(ObjectTest, CreateSimpleObject) {
    std::string code = R"(
        let obj = __create_object();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto obj = interpreter.global_env()->get("obj");
    ASSERT_TRUE(obj.is_object());
}
