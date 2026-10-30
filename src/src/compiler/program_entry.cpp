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

std::unique_ptr<FunctionDeclaration> synthesize_script_entry_function(Program* program) {
    if (!program || !program_has_script_body(program)) {
        return nullptr;
    }

    auto body = std::make_unique<BlockStatement>(std::vector<std::unique_ptr<Statement>>{});
    std::vector<std::unique_ptr<Statement>> remaining;
    for (auto& stmt : program->statements) {
        if (is_script_level_statement(stmt->type)) {
            body->statements.push_back(std::move(stmt));
        } else {
            remaining.push_back(std::move(stmt));
        }
    }
    program->statements = std::move(remaining);

    return std::make_unique<FunctionDeclaration>(
        kScriptEntryFunctionName,
        std::vector<Parameter>{},
        std::move(body));
}

} // namespace dryad
