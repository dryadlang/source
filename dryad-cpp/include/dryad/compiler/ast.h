#ifndef DRYAD_COMPILER_AST_H
#define DRYAD_COMPILER_AST_H

#include "dryad/compiler/token.h"
#include "dryad/runtime/value.h"
#include <memory>
#include <vector>
#include <string>

namespace dryad {

enum class ASTNodeType {
    Program,
    
    IntegerLiteral,
    FloatLiteral,
    StringLiteral,
    BooleanLiteral,
    NullLiteral,
    Identifier,
    
    BinaryExpression,
    UnaryExpression,
    AssignmentExpression,
    CallExpression,
    MemberExpression,
    IndexExpression,
    ArrayExpression,
    ObjectExpression,
    NewExpression,
    ThisExpression,
    
    VariableDeclaration,
    FunctionDeclaration,
    ClassDeclaration,
    IntrinsicDeclaration,
    
    BlockStatement,
    ExpressionStatement,
    ReturnStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    BreakStatement,
    ContinueStatement,
    
    ImportDeclaration,
    ExportDeclaration
};

class ASTNode {
public:
    ASTNodeType type;
    SourceLocation location;
    
    explicit ASTNode(ASTNodeType type, SourceLocation location = SourceLocation())
        : type(type), location(location) {}
    
    virtual ~ASTNode() = default;
};

class Expression : public ASTNode {
public:
    explicit Expression(ASTNodeType type, SourceLocation location = SourceLocation())
        : ASTNode(type, location) {}
};

class Statement : public ASTNode {
public:
    explicit Statement(ASTNodeType type, SourceLocation location = SourceLocation())
        : ASTNode(type, location) {}
};

class IntegerLiteral : public Expression {
public:
    int64_t value;
    
    IntegerLiteral(int64_t value, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::IntegerLiteral, location), value(value) {}
};

class FloatLiteral : public Expression {
public:
    double value;
    
    FloatLiteral(double value, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::FloatLiteral, location), value(value) {}
};

class StringLiteral : public Expression {
public:
    std::string value;
    
    StringLiteral(std::string value, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::StringLiteral, location), value(std::move(value)) {}
};

class BooleanLiteral : public Expression {
public:
    bool value;
    
    BooleanLiteral(bool value, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::BooleanLiteral, location), value(value) {}
};

class NullLiteral : public Expression {
public:
    explicit NullLiteral(SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::NullLiteral, location) {}
};

class Identifier : public Expression {
public:
    std::string name;
    
    Identifier(std::string name, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::Identifier, location), name(std::move(name)) {}
};

class BinaryExpression : public Expression {
public:
    TokenType op;
    std::unique_ptr<Expression> left;
    std::unique_ptr<Expression> right;
    
    BinaryExpression(TokenType op, std::unique_ptr<Expression> left, std::unique_ptr<Expression> right, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::BinaryExpression, location), op(op), left(std::move(left)), right(std::move(right)) {}
};

class UnaryExpression : public Expression {
public:
    TokenType op;
    std::unique_ptr<Expression> operand;
    
    UnaryExpression(TokenType op, std::unique_ptr<Expression> operand, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::UnaryExpression, location), op(op), operand(std::move(operand)) {}
};

class AssignmentExpression : public Expression {
public:
    std::unique_ptr<Expression> target;
    std::unique_ptr<Expression> value;
    
    AssignmentExpression(std::unique_ptr<Expression> target, std::unique_ptr<Expression> value, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::AssignmentExpression, location), target(std::move(target)), value(std::move(value)) {}
};

class CallExpression : public Expression {
public:
    std::unique_ptr<Expression> callee;
    std::vector<std::unique_ptr<Expression>> arguments;
    
    CallExpression(std::unique_ptr<Expression> callee, std::vector<std::unique_ptr<Expression>> arguments, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::CallExpression, location), callee(std::move(callee)), arguments(std::move(arguments)) {}
};

class MemberExpression : public Expression {
public:
    std::unique_ptr<Expression> object;
    std::unique_ptr<Expression> property;
    bool computed;
    
    MemberExpression(std::unique_ptr<Expression> object, std::unique_ptr<Expression> property, bool computed, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::MemberExpression, location), object(std::move(object)), property(std::move(property)), computed(computed) {}
};

class ArrayExpression : public Expression {
public:
    std::vector<std::unique_ptr<Expression>> elements;
    
    ArrayExpression(std::vector<std::unique_ptr<Expression>> elements, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::ArrayExpression, location), elements(std::move(elements)) {}
};

class BlockStatement : public Statement {
public:
    std::vector<std::unique_ptr<Statement>> statements;
    
    BlockStatement(std::vector<std::unique_ptr<Statement>> statements, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::BlockStatement, location), statements(std::move(statements)) {}
};

class ExpressionStatement : public Statement {
public:
    std::unique_ptr<Expression> expression;
    
    ExpressionStatement(std::unique_ptr<Expression> expression, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ExpressionStatement, location), expression(std::move(expression)) {}
};

class VariableDeclaration : public Statement {
public:
    std::string name;
    std::unique_ptr<Expression> initializer;
    bool is_const;
    
    VariableDeclaration(std::string name, std::unique_ptr<Expression> initializer, bool is_const, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::VariableDeclaration, location), name(std::move(name)), initializer(std::move(initializer)), is_const(is_const) {}
};

struct Parameter {
    std::string name;
    std::string type_annotation;
};

class FunctionDeclaration : public Statement {
public:
    std::string name;
    std::vector<Parameter> parameters;
    std::unique_ptr<BlockStatement> body;
    std::string return_type;
    bool is_internal;
    
    FunctionDeclaration(std::string name, std::vector<Parameter> parameters, std::unique_ptr<BlockStatement> body, std::string return_type = "", bool is_internal = false, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::FunctionDeclaration, location), name(std::move(name)), parameters(std::move(parameters)), body(std::move(body)), return_type(std::move(return_type)), is_internal(is_internal) {}
};

class IntrinsicDeclaration : public Statement {
public:
    std::string intrinsic_name;
    std::string function_name;
    std::vector<Parameter> parameters;
    std::string return_type;
    
    IntrinsicDeclaration(std::string intrinsic_name, std::string function_name, std::vector<Parameter> parameters, std::string return_type = "", SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::IntrinsicDeclaration, location), 
          intrinsic_name(std::move(intrinsic_name)), 
          function_name(std::move(function_name)), 
          parameters(std::move(parameters)), 
          return_type(std::move(return_type)) {}
};

class MethodDeclaration {
public:
    std::string name;
    std::vector<Parameter> parameters;
    std::unique_ptr<BlockStatement> body;
    bool is_constructor;
    
    MethodDeclaration(std::string name, std::vector<Parameter> parameters, std::unique_ptr<BlockStatement> body, bool is_constructor = false)
        : name(std::move(name)), parameters(std::move(parameters)), body(std::move(body)), is_constructor(is_constructor) {}
};

class ClassDeclaration : public Statement {
public:
    std::string name;
    std::string super_class;
    std::vector<std::unique_ptr<MethodDeclaration>> methods;
    
    ClassDeclaration(std::string name, std::string super_class, std::vector<std::unique_ptr<MethodDeclaration>> methods, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ClassDeclaration, location), name(std::move(name)), super_class(std::move(super_class)), methods(std::move(methods)) {}
};

class NewExpression : public Expression {
public:
    std::string class_name;
    std::vector<std::unique_ptr<Expression>> arguments;
    
    NewExpression(std::string class_name, std::vector<std::unique_ptr<Expression>> arguments, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::NewExpression, location), class_name(std::move(class_name)), arguments(std::move(arguments)) {}
};

class ThisExpression : public Expression {
public:
    ThisExpression(SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::ThisExpression, location) {}
};

class ReturnStatement : public Statement {
public:
    std::unique_ptr<Expression> value;
    
    ReturnStatement(std::unique_ptr<Expression> value, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ReturnStatement, location), value(std::move(value)) {}
};

class IfStatement : public Statement {
public:
    std::unique_ptr<Expression> condition;
    std::unique_ptr<Statement> then_branch;
    std::unique_ptr<Statement> else_branch;
    
    IfStatement(std::unique_ptr<Expression> condition, std::unique_ptr<Statement> then_branch, std::unique_ptr<Statement> else_branch, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::IfStatement, location), condition(std::move(condition)), then_branch(std::move(then_branch)), else_branch(std::move(else_branch)) {}
};

class WhileStatement : public Statement {
public:
    std::unique_ptr<Expression> condition;
    std::unique_ptr<Statement> body;
    
    WhileStatement(std::unique_ptr<Expression> condition, std::unique_ptr<Statement> body, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::WhileStatement, location), condition(std::move(condition)), body(std::move(body)) {}
};

class Program : public ASTNode {
public:
    std::vector<std::unique_ptr<Statement>> statements;
    
    Program(std::vector<std::unique_ptr<Statement>> statements = {})
        : ASTNode(ASTNodeType::Program), statements(std::move(statements)) {}
};

} // namespace dryad

#endif // DRYAD_COMPILER_AST_H
