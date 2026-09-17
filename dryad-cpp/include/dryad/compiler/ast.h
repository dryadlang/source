#ifndef DRYAD_COMPILER_AST_H
#define DRYAD_COMPILER_AST_H

#include "dryad/compiler/execution_mode.h"
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
    FunctionExpression,
    ThisExpression,
    SuperExpression,
    AwaitExpression,
    ErrorExpression,
    
    VariableDeclaration,
    FunctionDeclaration,
    ClassDeclaration,
    InterfaceDeclaration,
    IntrinsicDeclaration,
    
    BlockStatement,
    ExpressionStatement,
    ReturnStatement,
    IfStatement,
    WhileStatement,
    ForStatement,
    BreakStatement,
    ContinueStatement,
    CompileBlockStatement,
    
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
    std::string type_annotation;
    std::unique_ptr<Expression> initializer;
    bool is_const;
    
    VariableDeclaration(std::string name, std::string type_annotation, std::unique_ptr<Expression> initializer, bool is_const, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::VariableDeclaration, location), name(std::move(name)), type_annotation(std::move(type_annotation)), initializer(std::move(initializer)), is_const(is_const) {}
};

struct Parameter {
    std::string name;
    std::string type_annotation;
    std::unique_ptr<Expression> default_value;

    Parameter() = default;
    Parameter(std::string name, std::string type_annotation,
              std::unique_ptr<Expression> default_value = nullptr)
        : name(std::move(name)),
          type_annotation(std::move(type_annotation)),
          default_value(std::move(default_value)) {}

    Parameter(Parameter&&) = default;
    Parameter& operator=(Parameter&&) = default;
    Parameter(const Parameter&) = delete;
    Parameter& operator=(const Parameter&) = delete;
};

class FunctionDeclaration : public Statement {
public:
    std::string name;
    std::vector<Parameter> parameters;
    std::unique_ptr<BlockStatement> body;
    std::string return_type;
    bool is_internal;
    bool is_async;
    ExecutionMode compile_mode;
    
    FunctionDeclaration(std::string name, std::vector<Parameter> parameters, std::unique_ptr<BlockStatement> body, std::string return_type = "", bool is_internal = false, bool is_async = false, ExecutionMode compile_mode = ExecutionMode::Inherit, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::FunctionDeclaration, location), name(std::move(name)), parameters(std::move(parameters)), body(std::move(body)), return_type(std::move(return_type)), is_internal(is_internal), is_async(is_async), compile_mode(compile_mode) {}
};

class CompileBlockStatement : public Statement {
public:
    ExecutionMode mode;
    std::unique_ptr<BlockStatement> body;

    CompileBlockStatement(ExecutionMode mode, std::unique_ptr<BlockStatement> body,
                          SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::CompileBlockStatement, location),
          mode(mode),
          body(std::move(body)) {}
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
    bool is_abstract;
    bool is_getter;
    bool is_setter;
    
    MethodDeclaration(std::string name, std::vector<Parameter> parameters, std::unique_ptr<BlockStatement> body, bool is_constructor = false, bool is_abstract = false, bool is_getter = false, bool is_setter = false)
        : name(std::move(name)), parameters(std::move(parameters)), body(std::move(body)), is_constructor(is_constructor), is_abstract(is_abstract), is_getter(is_getter), is_setter(is_setter) {}
};

class PropertyDeclaration {
public:
    std::string name;
    std::string type_annotation;
    bool is_readonly;
    bool is_abstract;
    bool has_getter;
    bool has_setter;
    bool getter_is_auto;
    bool setter_is_auto;
    std::string setter_parameter;
    std::unique_ptr<BlockStatement> getter_body;
    std::unique_ptr<BlockStatement> setter_body;
    
    PropertyDeclaration(std::string name, std::string type_annotation, bool is_readonly = false, bool is_abstract = false)
        : name(std::move(name)), type_annotation(std::move(type_annotation)), is_readonly(is_readonly), is_abstract(is_abstract),
          has_getter(false), has_setter(false), getter_is_auto(false), setter_is_auto(false), setter_parameter("value") {}
};

class ClassDeclaration : public Statement {
public:
    std::string name;
    std::string super_class;
    std::vector<std::string> interfaces;
    std::vector<std::unique_ptr<PropertyDeclaration>> properties;
    std::vector<std::unique_ptr<MethodDeclaration>> methods;
    bool is_abstract;
    
    ClassDeclaration(std::string name, std::string super_class, std::vector<std::string> interfaces,
                     std::vector<std::unique_ptr<PropertyDeclaration>> properties,
                     std::vector<std::unique_ptr<MethodDeclaration>> methods, bool is_abstract = false,
                     SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ClassDeclaration, location), name(std::move(name)), super_class(std::move(super_class)),
          interfaces(std::move(interfaces)), properties(std::move(properties)), methods(std::move(methods)),
          is_abstract(is_abstract) {}
};

class InterfaceDeclaration : public Statement {
public:
    std::string name;
    std::vector<std::unique_ptr<MethodDeclaration>> methods;
    std::vector<std::unique_ptr<PropertyDeclaration>> properties;
    
    InterfaceDeclaration(std::string name, std::vector<std::unique_ptr<MethodDeclaration>> methods,
                         std::vector<std::unique_ptr<PropertyDeclaration>> properties,
                         SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::InterfaceDeclaration, location), name(std::move(name)), methods(std::move(methods)),
          properties(std::move(properties)) {}
};

class NewExpression : public Expression {
public:
    std::string class_name;
    std::vector<std::unique_ptr<Expression>> arguments;
    
    NewExpression(std::string class_name, std::vector<std::unique_ptr<Expression>> arguments, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::NewExpression, location), class_name(std::move(class_name)), arguments(std::move(arguments)) {}
};

class FunctionExpression : public Expression {
public:
    std::vector<Parameter> parameters;
    std::unique_ptr<BlockStatement> body;
    std::string return_type;

    FunctionExpression(std::vector<Parameter> parameters, std::unique_ptr<BlockStatement> body,
                       std::string return_type = "", SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::FunctionExpression, location),
          parameters(std::move(parameters)),
          body(std::move(body)),
          return_type(std::move(return_type)) {}
};

class ThisExpression : public Expression {
public:
    ThisExpression(SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::ThisExpression, location) {}
};

class SuperExpression : public Expression {
public:
    SuperExpression(SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::SuperExpression, location) {}
};

class AwaitExpression : public Expression {
public:
    std::unique_ptr<Expression> operand;

    AwaitExpression(std::unique_ptr<Expression> operand, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::AwaitExpression, location), operand(std::move(operand)) {}
};

class ErrorExpression : public Expression {
public:
    std::string message;

    ErrorExpression(std::string message, SourceLocation location = SourceLocation())
        : Expression(ASTNodeType::ErrorExpression, location), message(std::move(message)) {}
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

class BreakStatement : public Statement {
public:
    BreakStatement(SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::BreakStatement, location) {}
};

class ContinueStatement : public Statement {
public:
    ContinueStatement(SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ContinueStatement, location) {}
};

class ForStatement : public Statement {
public:
    std::unique_ptr<Statement> initializer;
    std::unique_ptr<Expression> condition;
    std::unique_ptr<Expression> increment;
    std::unique_ptr<Statement> body;

    ForStatement(std::unique_ptr<Statement> initializer,
                 std::unique_ptr<Expression> condition,
                 std::unique_ptr<Expression> increment,
                 std::unique_ptr<Statement> body,
                 SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ForStatement, location),
          initializer(std::move(initializer)),
          condition(std::move(condition)),
          increment(std::move(increment)),
          body(std::move(body)) {}
};

class ImportDeclaration : public Statement {
public:
    std::vector<std::string> names;
    std::string module_path;
    
    ImportDeclaration(std::vector<std::string> names, std::string module_path, SourceLocation location = SourceLocation())
        : Statement(ASTNodeType::ImportDeclaration, location),
          names(std::move(names)),
          module_path(std::move(module_path)) {}
};

class Program : public ASTNode {
public:
    std::vector<std::unique_ptr<Statement>> statements;
    
    Program(std::vector<std::unique_ptr<Statement>> statements = {})
        : ASTNode(ASTNodeType::Program), statements(std::move(statements)) {}
};

} // namespace dryad

#endif // DRYAD_COMPILER_AST_H
