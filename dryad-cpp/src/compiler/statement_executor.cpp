#include "dryad/compiler/statement_executor.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/common/utils.h"

namespace dryad {

StatementExecutor::StatementExecutor(Interpreter* interpreter)
    : interpreter_(interpreter) {}

void StatementExecutor::execute(Statement* stmt) {
    switch (stmt->type) {
        case ASTNodeType::ExpressionStatement:
            execute_expression(static_cast<ExpressionStatement*>(stmt));
            break;
        
        case ASTNodeType::VariableDeclaration:
            execute_variable_declaration(static_cast<VariableDeclaration*>(stmt));
            break;
        
        case ASTNodeType::FunctionDeclaration:
            execute_function_declaration(static_cast<FunctionDeclaration*>(stmt));
            break;
        
        case ASTNodeType::ClassDeclaration:
            execute_class_declaration(static_cast<ClassDeclaration*>(stmt));
            break;
        
        case ASTNodeType::BlockStatement:
            execute_block_statement(static_cast<BlockStatement*>(stmt));
            break;
        
        case ASTNodeType::IfStatement:
            execute_if(static_cast<IfStatement*>(stmt));
            break;
        
        case ASTNodeType::WhileStatement:
            execute_while(static_cast<WhileStatement*>(stmt));
            break;
        
        case ASTNodeType::ReturnStatement:
            execute_return(static_cast<ReturnStatement*>(stmt));
            break;
        
        default:
            throw DryadException("Unknown statement type");
    }
}

void StatementExecutor::execute_expression(ExpressionStatement* stmt) {
    interpreter_->evaluate(stmt->expression.get());
}

void StatementExecutor::execute_variable_declaration(VariableDeclaration* stmt) {
    Value value;
    if (stmt->initializer) {
        value = interpreter_->evaluate(stmt->initializer.get());
    }
    interpreter_->current_env()->define(stmt->name, value);
}

void StatementExecutor::execute_function_declaration(FunctionDeclaration* stmt) {
    auto func = std::make_shared<DryadFunction>(stmt, interpreter_->current_env());
    interpreter_->current_env()->define(stmt->name, Value(func));
}

void StatementExecutor::execute_block_statement(BlockStatement* stmt) {
    auto block_env = std::make_shared<Environment>(interpreter_->current_env());
    execute_block(stmt->statements, block_env);
}

void StatementExecutor::execute_block(const std::vector<std::unique_ptr<Statement>>& statements,
                                     std::shared_ptr<Environment> env) {
    auto previous = interpreter_->current_env();
    interpreter_->set_current_env(env);
    
    try {
        for (auto& statement : statements) {
            execute(statement.get());
        }
    } catch (...) {
        interpreter_->set_current_env(previous);
        throw;
    }
    
    interpreter_->set_current_env(previous);
}

void StatementExecutor::execute_if(IfStatement* stmt) {
    Value condition = interpreter_->evaluate(stmt->condition.get());
    
    if (condition.is_truthy()) {
        execute(stmt->then_branch.get());
    } else if (stmt->else_branch) {
        execute(stmt->else_branch.get());
    }
}

void StatementExecutor::execute_while(WhileStatement* stmt) {
    while (true) {
        Value condition = interpreter_->evaluate(stmt->condition.get());
        if (!condition.is_truthy()) break;
        
        try {
            execute(stmt->body.get());
        } catch (const BreakException&) {
            break;
        } catch (const ContinueException&) {
            continue;
        }
    }
}

void StatementExecutor::execute_return(ReturnStatement* stmt) {
    Value value;
    if (stmt->value) {
        value = interpreter_->evaluate(stmt->value.get());
    }
    throw ReturnException(value);
}

void StatementExecutor::execute_class_declaration(ClassDeclaration* stmt) {
    auto klass = std::make_shared<DryadClass>(stmt->name);
    
    for (auto& method_decl : stmt->methods) {
        auto func_decl = std::make_unique<FunctionDeclaration>(
            method_decl->name,
            std::move(method_decl->parameters),
            std::move(method_decl->body),
            "",
            false
        );
        
        auto method_func = std::make_shared<DryadFunction>(
            func_decl.get(),
            interpreter_->current_env()
        );
        
        if (method_decl->is_constructor) {
            klass->constructor = method_func;
        } else {
            klass->methods[method_decl->name] = method_func;
        }
        
        klass->owned_declarations.push_back(std::move(func_decl));
    }
    
    interpreter_->current_env()->define(stmt->name, Value(klass));
}

} // namespace dryad
