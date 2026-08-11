#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(ClassTest, SimpleClassDeclaration) {
    std::string code = R"(
        class Point {
            constructor(x, y) {
                this.x = x;
                this.y = y;
            }
            
            getX() {
                return this.x;
            }
        }
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
}

TEST(ClassTest, NewExpression) {
    std::string code = R"(
        let p = new Point(10, 20);
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
}

TEST(ClassTest, ClassInstantiationAndMethodCall) {
    std::string code = R"(
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
    
    getX() {
        return this.x;
    }
}

let p = new Point(10, 20);
let result = p.getX();
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error()) << "Parser error: " << parser.error_message();
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto p = interpreter.global_env()->get("p");
    ASSERT_TRUE(p.is_instance());
    
    auto result = interpreter.global_env()->get("result");
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 10);
}
