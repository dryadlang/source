#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"

using namespace dryad;

std::unique_ptr<Program> parse(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    return parser.parse();
}

TEST(ParserTest, EmptyProgram) {
    auto program = parse("");
    
    ASSERT_NE(program, nullptr);
    EXPECT_EQ(program->statements.size(), 0);
}

TEST(ParserTest, IntegerLiteral) {
    auto program = parse("42;");
    
    ASSERT_NE(program, nullptr);
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    ASSERT_NE(expr_stmt, nullptr);
    
    auto* literal = dynamic_cast<IntegerLiteral*>(expr_stmt->expression.get());
    ASSERT_NE(literal, nullptr);
    EXPECT_EQ(literal->value, 42);
}

TEST(ParserTest, FloatLiteral) {
    auto program = parse("3.14;");
    
    ASSERT_NE(program, nullptr);
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* literal = dynamic_cast<FloatLiteral*>(expr_stmt->expression.get());
    
    ASSERT_NE(literal, nullptr);
    EXPECT_DOUBLE_EQ(literal->value, 3.14);
}

TEST(ParserTest, StringLiteral) {
    auto program = parse("\"hello\";");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* literal = dynamic_cast<StringLiteral*>(expr_stmt->expression.get());
    
    ASSERT_NE(literal, nullptr);
    EXPECT_EQ(literal->value, "hello");
}

TEST(ParserTest, BooleanLiterals) {
    auto program = parse("true; false;");
    
    ASSERT_EQ(program->statements.size(), 2);
    
    auto* true_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* true_lit = dynamic_cast<BooleanLiteral*>(true_stmt->expression.get());
    ASSERT_NE(true_lit, nullptr);
    EXPECT_TRUE(true_lit->value);
    
    auto* false_stmt = dynamic_cast<ExpressionStatement*>(program->statements[1].get());
    auto* false_lit = dynamic_cast<BooleanLiteral*>(false_stmt->expression.get());
    ASSERT_NE(false_lit, nullptr);
    EXPECT_FALSE(false_lit->value);
}

TEST(ParserTest, NullLiteral) {
    auto program = parse("null;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* literal = dynamic_cast<NullLiteral*>(expr_stmt->expression.get());
    
    ASSERT_NE(literal, nullptr);
}

TEST(ParserTest, Identifier) {
    auto program = parse("foo;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* ident = dynamic_cast<Identifier*>(expr_stmt->expression.get());
    
    ASSERT_NE(ident, nullptr);
    EXPECT_EQ(ident->name, "foo");
}

TEST(ParserTest, BinaryExpression) {
    auto program = parse("1 + 2;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* binary = dynamic_cast<BinaryExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(binary, nullptr);
    EXPECT_EQ(binary->op, TokenType::Plus);
    
    auto* left = dynamic_cast<IntegerLiteral*>(binary->left.get());
    auto* right = dynamic_cast<IntegerLiteral*>(binary->right.get());
    
    ASSERT_NE(left, nullptr);
    ASSERT_NE(right, nullptr);
    EXPECT_EQ(left->value, 1);
    EXPECT_EQ(right->value, 2);
}

TEST(ParserTest, OperatorPrecedence) {
    auto program = parse("1 + 2 * 3;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* add = dynamic_cast<BinaryExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(add, nullptr);
    EXPECT_EQ(add->op, TokenType::Plus);
    
    auto* left = dynamic_cast<IntegerLiteral*>(add->left.get());
    EXPECT_EQ(left->value, 1);
    
    auto* mult = dynamic_cast<BinaryExpression*>(add->right.get());
    ASSERT_NE(mult, nullptr);
    EXPECT_EQ(mult->op, TokenType::Star);
}

TEST(ParserTest, UnaryExpression) {
    auto program = parse("-42;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* unary = dynamic_cast<UnaryExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(unary, nullptr);
    EXPECT_EQ(unary->op, TokenType::Minus);
    
    auto* operand = dynamic_cast<IntegerLiteral*>(unary->operand.get());
    ASSERT_NE(operand, nullptr);
    EXPECT_EQ(operand->value, 42);
}

TEST(ParserTest, LogicalOperators) {
    auto program = parse("true && false || true;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* or_expr = dynamic_cast<BinaryExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(or_expr, nullptr);
    EXPECT_EQ(or_expr->op, TokenType::PipePipe);
    
    auto* and_expr = dynamic_cast<BinaryExpression*>(or_expr->left.get());
    ASSERT_NE(and_expr, nullptr);
    EXPECT_EQ(and_expr->op, TokenType::AmpersandAmpersand);
}

TEST(ParserTest, CallExpression) {
    auto program = parse("foo(1, 2);");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* call = dynamic_cast<CallExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(call, nullptr);
    EXPECT_EQ(call->arguments.size(), 2);
    
    auto* callee = dynamic_cast<Identifier*>(call->callee.get());
    ASSERT_NE(callee, nullptr);
    EXPECT_EQ(callee->name, "foo");
}

TEST(ParserTest, MemberExpression) {
    auto program = parse("obj.prop;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* member = dynamic_cast<MemberExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(member, nullptr);
    EXPECT_FALSE(member->computed);
    
    auto* object = dynamic_cast<Identifier*>(member->object.get());
    auto* property = dynamic_cast<Identifier*>(member->property.get());
    
    ASSERT_NE(object, nullptr);
    ASSERT_NE(property, nullptr);
    EXPECT_EQ(object->name, "obj");
    EXPECT_EQ(property->name, "prop");
}

TEST(ParserTest, IndexExpression) {
    auto program = parse("arr[0];");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* member = dynamic_cast<MemberExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(member, nullptr);
    EXPECT_TRUE(member->computed);
}

TEST(ParserTest, ArrayLiteral) {
    auto program = parse("[1, 2, 3];");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* array = dynamic_cast<ArrayExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(array, nullptr);
    EXPECT_EQ(array->elements.size(), 3);
}

TEST(ParserTest, VariableDeclaration) {
    auto program = parse("let x = 42;");
    
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* var_decl = dynamic_cast<VariableDeclaration*>(program->statements[0].get());
    ASSERT_NE(var_decl, nullptr);
    EXPECT_EQ(var_decl->name, "x");
    EXPECT_FALSE(var_decl->is_const);
    
    auto* init = dynamic_cast<IntegerLiteral*>(var_decl->initializer.get());
    ASSERT_NE(init, nullptr);
    EXPECT_EQ(init->value, 42);
}

TEST(ParserTest, ConstDeclaration) {
    auto program = parse("const PI = 3.14;");
    
    auto* var_decl = dynamic_cast<VariableDeclaration*>(program->statements[0].get());
    ASSERT_NE(var_decl, nullptr);
    EXPECT_EQ(var_decl->name, "PI");
    EXPECT_TRUE(var_decl->is_const);
}

TEST(ParserTest, FunctionDeclaration) {
    auto program = parse("function add(a: number, b: number): number { return a + b; }");
    
    ASSERT_EQ(program->statements.size(), 1);
    
    auto* func_decl = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(func_decl, nullptr);
    EXPECT_EQ(func_decl->name, "add");
    EXPECT_EQ(func_decl->parameters.size(), 2);
    EXPECT_EQ(func_decl->parameters[0].name, "a");
    EXPECT_EQ(func_decl->parameters[0].type_annotation, "number");
    EXPECT_EQ(func_decl->return_type, "number");
    ASSERT_NE(func_decl->body, nullptr);
}

TEST(ParserTest, ReturnStatement) {
    auto program = parse("function test() { return 42; }");
    
    auto* func_decl = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(func_decl, nullptr);
    ASSERT_EQ(func_decl->body->statements.size(), 1);
    
    auto* ret_stmt = dynamic_cast<ReturnStatement*>(func_decl->body->statements[0].get());
    ASSERT_NE(ret_stmt, nullptr);
    
    auto* value = dynamic_cast<IntegerLiteral*>(ret_stmt->value.get());
    ASSERT_NE(value, nullptr);
    EXPECT_EQ(value->value, 42);
}

TEST(ParserTest, IfStatement) {
    auto program = parse("if (x > 0) { return 1; }");
    
    auto* if_stmt = dynamic_cast<IfStatement*>(program->statements[0].get());
    ASSERT_NE(if_stmt, nullptr);
    
    auto* condition = dynamic_cast<BinaryExpression*>(if_stmt->condition.get());
    ASSERT_NE(condition, nullptr);
    EXPECT_EQ(condition->op, TokenType::Greater);
    
    auto* then_branch = dynamic_cast<BlockStatement*>(if_stmt->then_branch.get());
    ASSERT_NE(then_branch, nullptr);
    EXPECT_EQ(then_branch->statements.size(), 1);
}

TEST(ParserTest, IfElseStatement) {
    auto program = parse("if (x > 0) { return 1; } else { return -1; }");
    
    auto* if_stmt = dynamic_cast<IfStatement*>(program->statements[0].get());
    ASSERT_NE(if_stmt, nullptr);
    ASSERT_NE(if_stmt->else_branch, nullptr);
    
    auto* else_branch = dynamic_cast<BlockStatement*>(if_stmt->else_branch.get());
    ASSERT_NE(else_branch, nullptr);
}

TEST(ParserTest, WhileStatement) {
    auto program = parse("while (i < 10) { foo(); }");
    
    auto* while_stmt = dynamic_cast<WhileStatement*>(program->statements[0].get());
    ASSERT_NE(while_stmt, nullptr);
    
    auto* condition = dynamic_cast<BinaryExpression*>(while_stmt->condition.get());
    ASSERT_NE(condition, nullptr);
    
    auto* body = dynamic_cast<BlockStatement*>(while_stmt->body.get());
    ASSERT_NE(body, nullptr);
}

TEST(ParserTest, BlockStatement) {
    auto program = parse("{ let x = 1; let y = 2; }");
    
    auto* block = dynamic_cast<BlockStatement*>(program->statements[0].get());
    ASSERT_NE(block, nullptr);
    EXPECT_EQ(block->statements.size(), 2);
}

TEST(ParserTest, CompleteProgram) {
    auto program = parse(R"(
        function fib(n: number): number {
            if (n <= 1) {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        }
        
        let result = fib(10);
    )");
    
    ASSERT_NE(program, nullptr);
    EXPECT_EQ(program->statements.size(), 2);
    
    auto* func = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(func, nullptr);
    EXPECT_EQ(func->name, "fib");
    
    auto* var = dynamic_cast<VariableDeclaration*>(program->statements[1].get());
    ASSERT_NE(var, nullptr);
    EXPECT_EQ(var->name, "result");
}

TEST(ParserTest, ChainedCalls) {
    auto program = parse("foo().bar().baz();");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* outer_call = dynamic_cast<CallExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(outer_call, nullptr);
    
    auto* member = dynamic_cast<MemberExpression*>(outer_call->callee.get());
    ASSERT_NE(member, nullptr);
}

TEST(ParserTest, NestedMemberAccess) {
    auto program = parse("a.b.c.d;");
    
    auto* expr_stmt = dynamic_cast<ExpressionStatement*>(program->statements[0].get());
    auto* member = dynamic_cast<MemberExpression*>(expr_stmt->expression.get());
    
    ASSERT_NE(member, nullptr);
    
    auto* prop = dynamic_cast<Identifier*>(member->property.get());
    ASSERT_NE(prop, nullptr);
    EXPECT_EQ(prop->name, "d");
}
