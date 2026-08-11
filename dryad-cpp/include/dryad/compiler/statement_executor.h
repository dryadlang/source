#ifndef DRYAD_COMPILER_STATEMENT_EXECUTOR_H
#define DRYAD_COMPILER_STATEMENT_EXECUTOR_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/environment.h"
#include <memory>

namespace dryad {

class Interpreter;

class StatementExecutor {
public:
    explicit StatementExecutor(Interpreter* interpreter);
    
    void execute(Statement* stmt);
    void execute_block(const std::vector<std::unique_ptr<Statement>>& statements, 
                      std::shared_ptr<Environment> env);
    
private:
    Interpreter* interpreter_;
    
    void execute_expression(ExpressionStatement* stmt);
    void execute_variable_declaration(VariableDeclaration* stmt);
    void execute_function_declaration(FunctionDeclaration* stmt);
    void execute_class_declaration(ClassDeclaration* stmt);
    void execute_block_statement(BlockStatement* stmt);
    void execute_if(IfStatement* stmt);
    void execute_while(WhileStatement* stmt);
    void execute_return(ReturnStatement* stmt);
};

} // namespace dryad

#endif // DRYAD_COMPILER_STATEMENT_EXECUTOR_H