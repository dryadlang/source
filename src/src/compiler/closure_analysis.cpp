#include "dryad/compiler/closure_analysis.h"

namespace dryad {

namespace {

void collect_declared(const Statement* stmt, std::set<std::string>& out) {
    if (!stmt) {
        return;
    }
    switch (stmt->type) {
        case ASTNodeType::VariableDeclaration:
            out.insert(static_cast<const VariableDeclaration*>(stmt)->name);
            break;
        case ASTNodeType::BlockStatement:
            for (const auto& inner : static_cast<const BlockStatement*>(stmt)->statements) {
                collect_declared(inner.get(), out);
            }
            break;
        case ASTNodeType::CompileBlockStatement:
            collect_declared(static_cast<const CompileBlockStatement*>(stmt)->body.get(), out);
            break;
        case ASTNodeType::IfStatement: {
            const auto* if_stmt = static_cast<const IfStatement*>(stmt);
            collect_declared(if_stmt->then_branch.get(), out);
            collect_declared(if_stmt->else_branch.get(), out);
            break;
        }
        case ASTNodeType::WhileStatement:
            collect_declared(static_cast<const WhileStatement*>(stmt)->body.get(), out);
            break;
        case ASTNodeType::ForStatement: {
            const auto* for_stmt = static_cast<const ForStatement*>(stmt);
            collect_declared(for_stmt->initializer.get(), out);
            collect_declared(for_stmt->body.get(), out);
            break;
        }
        default:
            break;
    }
}

void collect_referenced(const Statement* stmt, std::set<std::string>& out);

void collect_referenced(const Expression* expr, std::set<std::string>& out) {
    if (!expr) {
        return;
    }
    switch (expr->type) {
        case ASTNodeType::Identifier:
            out.insert(static_cast<const Identifier*>(expr)->name);
            break;
        case ASTNodeType::BinaryExpression: {
            const auto* binary = static_cast<const BinaryExpression*>(expr);
            collect_referenced(binary->left.get(), out);
            collect_referenced(binary->right.get(), out);
            break;
        }
        case ASTNodeType::UnaryExpression:
            collect_referenced(static_cast<const UnaryExpression*>(expr)->operand.get(), out);
            break;
        case ASTNodeType::AssignmentExpression: {
            const auto* assign = static_cast<const AssignmentExpression*>(expr);
            collect_referenced(assign->target.get(), out);
            collect_referenced(assign->value.get(), out);
            break;
        }
        case ASTNodeType::CallExpression: {
            const auto* call = static_cast<const CallExpression*>(expr);
            collect_referenced(call->callee.get(), out);
            for (const auto& arg : call->arguments) {
                collect_referenced(arg.get(), out);
            }
            break;
        }
        case ASTNodeType::MemberExpression: {
            const auto* member = static_cast<const MemberExpression*>(expr);
            collect_referenced(member->object.get(), out);
            // A non-computed property is a field name, not a variable.
            if (member->computed) {
                collect_referenced(member->property.get(), out);
            }
            break;
        }
        case ASTNodeType::ArrayExpression:
            for (const auto& element : static_cast<const ArrayExpression*>(expr)->elements) {
                collect_referenced(element.get(), out);
            }
            break;
        case ASTNodeType::ObjectExpression:
            for (const auto& property : static_cast<const ObjectExpression*>(expr)->properties) {
                collect_referenced(property.value.get(), out);
            }
            break;
        case ASTNodeType::NewExpression:
            for (const auto& arg : static_cast<const NewExpression*>(expr)->arguments) {
                collect_referenced(arg.get(), out);
            }
            break;
        case ASTNodeType::AwaitExpression:
            collect_referenced(static_cast<const AwaitExpression*>(expr)->operand.get(), out);
            break;
        case ASTNodeType::FunctionExpression: {
            const auto* lambda = static_cast<const FunctionExpression*>(expr);
            for (const std::string& name :
                 free_names(lambda->parameters, block_statements_of(lambda->body.get()))) {
                out.insert(name);
            }
            break;
        }
        default:
            break;
    }
}

void collect_referenced(const Statement* stmt, std::set<std::string>& out) {
    if (!stmt) {
        return;
    }
    switch (stmt->type) {
        case ASTNodeType::VariableDeclaration:
            collect_referenced(static_cast<const VariableDeclaration*>(stmt)->initializer.get(), out);
            break;
        case ASTNodeType::ExpressionStatement:
            collect_referenced(static_cast<const ExpressionStatement*>(stmt)->expression.get(), out);
            break;
        case ASTNodeType::ReturnStatement:
            collect_referenced(static_cast<const ReturnStatement*>(stmt)->value.get(), out);
            break;
        case ASTNodeType::BlockStatement:
            for (const auto& inner : static_cast<const BlockStatement*>(stmt)->statements) {
                collect_referenced(inner.get(), out);
            }
            break;
        case ASTNodeType::CompileBlockStatement:
            collect_referenced(static_cast<const CompileBlockStatement*>(stmt)->body.get(), out);
            break;
        case ASTNodeType::IfStatement: {
            const auto* if_stmt = static_cast<const IfStatement*>(stmt);
            collect_referenced(if_stmt->condition.get(), out);
            collect_referenced(if_stmt->then_branch.get(), out);
            collect_referenced(if_stmt->else_branch.get(), out);
            break;
        }
        case ASTNodeType::WhileStatement: {
            const auto* while_stmt = static_cast<const WhileStatement*>(stmt);
            collect_referenced(while_stmt->condition.get(), out);
            collect_referenced(while_stmt->body.get(), out);
            break;
        }
        case ASTNodeType::ForStatement: {
            const auto* for_stmt = static_cast<const ForStatement*>(stmt);
            collect_referenced(for_stmt->initializer.get(), out);
            collect_referenced(for_stmt->condition.get(), out);
            collect_referenced(for_stmt->increment.get(), out);
            collect_referenced(for_stmt->body.get(), out);
            break;
        }
        default:
            break;
    }
}

void collect_lambdas(const Statement* stmt, std::vector<const FunctionExpression*>& out);

void collect_lambdas(const Expression* expr, std::vector<const FunctionExpression*>& out) {
    if (!expr) {
        return;
    }
    switch (expr->type) {
        case ASTNodeType::FunctionExpression:
            // Only direct children; deeper lambdas surface through free_names.
            out.push_back(static_cast<const FunctionExpression*>(expr));
            break;
        case ASTNodeType::BinaryExpression: {
            const auto* binary = static_cast<const BinaryExpression*>(expr);
            collect_lambdas(binary->left.get(), out);
            collect_lambdas(binary->right.get(), out);
            break;
        }
        case ASTNodeType::UnaryExpression:
            collect_lambdas(static_cast<const UnaryExpression*>(expr)->operand.get(), out);
            break;
        case ASTNodeType::AssignmentExpression: {
            const auto* assign = static_cast<const AssignmentExpression*>(expr);
            collect_lambdas(assign->target.get(), out);
            collect_lambdas(assign->value.get(), out);
            break;
        }
        case ASTNodeType::CallExpression: {
            const auto* call = static_cast<const CallExpression*>(expr);
            collect_lambdas(call->callee.get(), out);
            for (const auto& arg : call->arguments) {
                collect_lambdas(arg.get(), out);
            }
            break;
        }
        case ASTNodeType::MemberExpression: {
            const auto* member = static_cast<const MemberExpression*>(expr);
            collect_lambdas(member->object.get(), out);
            if (member->computed) {
                collect_lambdas(member->property.get(), out);
            }
            break;
        }
        case ASTNodeType::ArrayExpression:
            for (const auto& element : static_cast<const ArrayExpression*>(expr)->elements) {
                collect_lambdas(element.get(), out);
            }
            break;
        case ASTNodeType::ObjectExpression:
            for (const auto& property : static_cast<const ObjectExpression*>(expr)->properties) {
                collect_lambdas(property.value.get(), out);
            }
            break;
        case ASTNodeType::NewExpression:
            for (const auto& arg : static_cast<const NewExpression*>(expr)->arguments) {
                collect_lambdas(arg.get(), out);
            }
            break;
        case ASTNodeType::AwaitExpression:
            collect_lambdas(static_cast<const AwaitExpression*>(expr)->operand.get(), out);
            break;
        default:
            break;
    }
}

void collect_lambdas(const Statement* stmt, std::vector<const FunctionExpression*>& out) {
    if (!stmt) {
        return;
    }
    switch (stmt->type) {
        case ASTNodeType::VariableDeclaration:
            collect_lambdas(static_cast<const VariableDeclaration*>(stmt)->initializer.get(), out);
            break;
        case ASTNodeType::ExpressionStatement:
            collect_lambdas(static_cast<const ExpressionStatement*>(stmt)->expression.get(), out);
            break;
        case ASTNodeType::ReturnStatement:
            collect_lambdas(static_cast<const ReturnStatement*>(stmt)->value.get(), out);
            break;
        case ASTNodeType::BlockStatement:
            for (const auto& inner : static_cast<const BlockStatement*>(stmt)->statements) {
                collect_lambdas(inner.get(), out);
            }
            break;
        case ASTNodeType::CompileBlockStatement:
            collect_lambdas(static_cast<const CompileBlockStatement*>(stmt)->body.get(), out);
            break;
        case ASTNodeType::IfStatement: {
            const auto* if_stmt = static_cast<const IfStatement*>(stmt);
            collect_lambdas(if_stmt->condition.get(), out);
            collect_lambdas(if_stmt->then_branch.get(), out);
            collect_lambdas(if_stmt->else_branch.get(), out);
            break;
        }
        case ASTNodeType::WhileStatement: {
            const auto* while_stmt = static_cast<const WhileStatement*>(stmt);
            collect_lambdas(while_stmt->condition.get(), out);
            collect_lambdas(while_stmt->body.get(), out);
            break;
        }
        case ASTNodeType::ForStatement: {
            const auto* for_stmt = static_cast<const ForStatement*>(stmt);
            collect_lambdas(for_stmt->initializer.get(), out);
            collect_lambdas(for_stmt->condition.get(), out);
            collect_lambdas(for_stmt->increment.get(), out);
            collect_lambdas(for_stmt->body.get(), out);
            break;
        }
        default:
            break;
    }
}

} // namespace

std::vector<Statement*> block_statements_of(const BlockStatement* block) {
    std::vector<Statement*> statements;
    if (!block) {
        return statements;
    }
    statements.reserve(block->statements.size());
    for (const auto& stmt : block->statements) {
        statements.push_back(stmt.get());
    }
    return statements;
}

std::set<std::string> declared_names(const std::vector<Parameter>& parameters,
                                     const std::vector<Statement*>& body) {
    std::set<std::string> declared;
    for (const auto& param : parameters) {
        declared.insert(param.name);
    }
    for (const Statement* stmt : body) {
        collect_declared(stmt, declared);
    }
    return declared;
}

std::vector<std::string> free_names(const std::vector<Parameter>& parameters,
                                    const std::vector<Statement*>& body) {
    std::set<std::string> referenced;
    for (const auto& param : parameters) {
        collect_referenced(param.default_value.get(), referenced);
    }
    for (const Statement* stmt : body) {
        collect_referenced(stmt, referenced);
    }
    const std::set<std::string> declared = declared_names(parameters, body);
    std::vector<std::string> free;
    for (const std::string& name : referenced) {
        if (declared.count(name) == 0) {
            free.push_back(name);
        }
    }
    return free;
}

std::set<std::string> captured_names(const std::vector<Parameter>& parameters,
                                     const std::vector<Statement*>& body) {
    std::vector<const FunctionExpression*> lambdas;
    for (const auto& param : parameters) {
        collect_lambdas(param.default_value.get(), lambdas);
    }
    for (const Statement* stmt : body) {
        collect_lambdas(stmt, lambdas);
    }
    const std::set<std::string> declared = declared_names(parameters, body);
    std::set<std::string> captured;
    for (const FunctionExpression* lambda : lambdas) {
        for (const std::string& name :
             free_names(lambda->parameters, block_statements_of(lambda->body.get()))) {
            if (declared.count(name) != 0) {
                captured.insert(name);
            }
        }
    }
    return captured;
}

} // namespace dryad
