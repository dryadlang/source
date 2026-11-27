#include "dryad/compiler/program_entry.h"

namespace dryad {

bool is_script_level_statement(ASTNodeType type) {
    switch (type) {
        case ASTNodeType::FunctionDeclaration:
        case ASTNodeType::ClassDeclaration:
        case ASTNodeType::InterfaceDeclaration:
        case ASTNodeType::ImportDeclaration:
        case ASTNodeType::ExportDeclaration:
        case ASTNodeType::IntrinsicDeclaration:
            return false;
        default:
            return true;
    }
}

bool program_has_user_functions(const Program* program) {
    if (!program) {
        return false;
    }
    for (const auto& stmt : program->statements) {
        if (stmt->type == ASTNodeType::FunctionDeclaration) {
            return true;
        }
    }
    return false;
}

bool program_has_script_body(const Program* program) {
    if (!program) {
        return false;
    }
    for (const auto& stmt : program->statements) {
        if (is_script_level_statement(stmt->type)) {
            return true;
        }
    }
    return false;
}

std::vector<Statement*> script_level_statements(const Program* program) {
    std::vector<Statement*> statements;
    if (!program) {
        return statements;
    }
    for (const auto& stmt : program->statements) {
        if (is_script_level_statement(stmt->type)) {
            statements.push_back(stmt.get());
        }
    }
    return statements;
}

} // namespace dryad
