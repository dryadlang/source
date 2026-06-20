#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

Value run(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    return Value();
}

Value evaluate(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    if (!program->statements.empty()) {
        auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
        if (expr_stmt) {
            return interp.evaluate(expr_stmt->expression.get());
        }
    }
    
    return Value();
}

TEST(InterpreterTest, IntegerLiteral) {
    Value result = evaluate("42;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 42);
}

TEST(InterpreterTest, FloatLiteral) {
    Value result = evaluate("3.14;");
    EXPECT_TRUE(result.is_float());
    EXPECT_DOUBLE_EQ(result.as_float(), 3.14);
}

TEST(InterpreterTest, StringLiteral) {
    Value result = evaluate("\"hello\";");
    EXPECT_TRUE(result.is_string());
    EXPECT_EQ(result.as_string(), "hello");
}

TEST(InterpreterTest, BooleanLiterals) {
    Value true_val = evaluate("true;");
    EXPECT_TRUE(true_val.is_boolean());
    EXPECT_TRUE(true_val.as_boolean());
    
    Value false_val = evaluate("false;");
    EXPECT_TRUE(false_val.is_boolean());
    EXPECT_FALSE(false_val.as_boolean());
}

TEST(InterpreterTest, NullLiteral) {
    Value result = evaluate("null;");
    EXPECT_TRUE(result.is_null());
}

TEST(InterpreterTest, Addition) {
    Value result = evaluate("1 + 2;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 3);
}

TEST(InterpreterTest, Subtraction) {
    Value result = evaluate("10 - 3;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 7);
}

TEST(InterpreterTest, Multiplication) {
    Value result = evaluate("4 * 5;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 20);
}

TEST(InterpreterTest, Division) {
    Value result = evaluate("15 / 3;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 5);
}

TEST(InterpreterTest, Modulo) {
    Value result = evaluate("17 % 5;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 2);
}

TEST(InterpreterTest, FloatArithmetic) {
    Value result = evaluate("3.5 + 2.5;");
    EXPECT_TRUE(result.is_float());
    EXPECT_DOUBLE_EQ(result.as_float(), 6.0);
}

TEST(InterpreterTest, MixedArithmetic) {
    Value result = evaluate("5 + 2.5;");
    EXPECT_TRUE(result.is_float());
    EXPECT_DOUBLE_EQ(result.as_float(), 7.5);
}

TEST(InterpreterTest, StringConcatenation) {
    Value result = evaluate("\"hello\" + \" world\";");
    EXPECT_TRUE(result.is_string());
    EXPECT_EQ(result.as_string(), "hello world");
}

TEST(InterpreterTest, OperatorPrecedence) {
    Value result = evaluate("2 + 3 * 4;");
    EXPECT_EQ(result.as_integer(), 14);
}

TEST(InterpreterTest, UnaryMinus) {
    Value result = evaluate("-42;");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), -42);
}

TEST(InterpreterTest, UnaryNot) {
    Value result1 = evaluate("!true;");
    EXPECT_TRUE(result1.is_boolean());
    EXPECT_FALSE(result1.as_boolean());
    
    Value result2 = evaluate("!false;");
    EXPECT_TRUE(result2.as_boolean());
}

TEST(InterpreterTest, Comparison) {
    EXPECT_TRUE(evaluate("5 > 3;").as_boolean());
    EXPECT_FALSE(evaluate("3 > 5;").as_boolean());
    EXPECT_TRUE(evaluate("5 >= 5;").as_boolean());
    EXPECT_TRUE(evaluate("3 < 5;").as_boolean());
    EXPECT_FALSE(evaluate("5 < 3;").as_boolean());
    EXPECT_TRUE(evaluate("5 <= 5;").as_boolean());
}

TEST(InterpreterTest, Equality) {
    EXPECT_TRUE(evaluate("5 == 5;").as_boolean());
    EXPECT_FALSE(evaluate("5 == 3;").as_boolean());
    EXPECT_TRUE(evaluate("5 != 3;").as_boolean());
    EXPECT_FALSE(evaluate("5 != 5;").as_boolean());
    
    EXPECT_TRUE(evaluate("\"hello\" == \"hello\";").as_boolean());
    EXPECT_FALSE(evaluate("\"hello\" == \"world\";").as_boolean());
}

TEST(InterpreterTest, LogicalAnd) {
    EXPECT_TRUE(evaluate("true && true;").as_boolean());
    EXPECT_FALSE(evaluate("true && false;").as_boolean());
    EXPECT_FALSE(evaluate("false && true;").as_boolean());
    EXPECT_FALSE(evaluate("false && false;").as_boolean());
}

TEST(InterpreterTest, LogicalOr) {
    EXPECT_TRUE(evaluate("true || true;").as_boolean());
    EXPECT_TRUE(evaluate("true || false;").as_boolean());
    EXPECT_TRUE(evaluate("false || true;").as_boolean());
    EXPECT_FALSE(evaluate("false || false;").as_boolean());
}

TEST(InterpreterTest, VariableDeclaration) {
    run("let x = 42;");
}

TEST(InterpreterTest, VariableAccess) {
    Lexer lexer("let x = 42; x;");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    Value result = interp.global_env()->get("x");
    EXPECT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 42);
}

TEST(InterpreterTest, MultipleVariables) {
    Lexer lexer("let x = 10; let y = 20;");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("x").as_integer(), 10);
    EXPECT_EQ(interp.global_env()->get("y").as_integer(), 20);
}

TEST(InterpreterTest, VariableExpression) {
    Lexer lexer("let a = 5; let b = 10; let c = a + b;");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("c").as_integer(), 15);
}

TEST(InterpreterTest, IfStatementTrue) {
    Lexer lexer("if (true) { let x = 1; }");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
}

TEST(InterpreterTest, IfStatementCondition) {
    Lexer lexer("if (5 > 3) { let result = 1; }");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
}

TEST(InterpreterTest, BlockScope) {
    Lexer lexer("let x = 1; { let x = 2; }");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("x").as_integer(), 1);
}

TEST(InterpreterTest, ComplexExpression) {
    Value result = evaluate("(5 + 3) * (10 - 2);");
    EXPECT_EQ(result.as_integer(), 64);
}

TEST(InterpreterTest, TruthyValues) {
    EXPECT_TRUE(evaluate("true;").is_truthy());
    EXPECT_FALSE(evaluate("false;").is_truthy());
    EXPECT_FALSE(evaluate("null;").is_truthy());
    EXPECT_FALSE(evaluate("0;").is_truthy());
    EXPECT_TRUE(evaluate("1;").is_truthy());
    EXPECT_FALSE(evaluate("\"\";").is_truthy());
    EXPECT_TRUE(evaluate("\"hello\";").is_truthy());
}

TEST(InterpreterTest, AssignmentExpression) {
    Lexer lexer("let x = 10; x = 20;");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("x").as_integer(), 20);
}

TEST(InterpreterTest, FunctionDeclaration) {
    Lexer lexer("function add(a, b) { return a + b; }");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_TRUE(interp.global_env()->get("add").is_function());
}

TEST(InterpreterTest, FunctionCall) {
    Lexer lexer("function double(x) { return x * 2; } let result = double(5);");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("result").as_integer(), 10);
}

TEST(InterpreterTest, FunctionWithMultipleParameters) {
    Lexer lexer("function add(a, b) { return a + b; } let sum = add(3, 7);");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("sum").as_integer(), 10);
}

TEST(InterpreterTest, FunctionClosure) {
    Lexer lexer("let x = 10; function getX() { return x; } let result = getX();");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("result").as_integer(), 10);
}

TEST(InterpreterTest, RecursiveFunction) {
    Lexer lexer(R"(
        function factorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * factorial(n - 1);
        }
        let result = factorial(5);
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    EXPECT_EQ(interp.global_env()->get("result").as_integer(), 120);
}

TEST(InterpreterTest, NativePrintFunction) {
    Lexer lexer("print(\"Hello\", \"World\");");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    std::string output = testing::internal::GetCapturedStdout();
    
    EXPECT_EQ(output, "Hello World\n");
}

