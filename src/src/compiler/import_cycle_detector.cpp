#include "dryad/compiler/import_cycle_detector.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/runtime/module_loader.h"
#include <unordered_map>
#include <unordered_set>

namespace dryad {

namespace {

std::vector<std::string> collect_import_specifiers(const Program& program) {
    std::vector<std::string> specifiers;
    for (const auto& stmt : program.statements) {
        if (stmt->type == ASTNodeType::ImportDeclaration) {
            auto* import_stmt = static_cast<ImportDeclaration*>(stmt.get());
            if (!import_stmt->module_path.empty()) {
                specifiers.push_back(import_stmt->module_path);
            }
        } else if (stmt->type == ASTNodeType::ExportDeclaration) {
            auto* export_stmt = static_cast<ExportFromDeclaration*>(stmt.get());
            if (!export_stmt->module_path.empty()) {
                specifiers.push_back(export_stmt->module_path);
            }
        }
    }
    return specifiers;
}

void dfs(const std::string& specifier,
         const std::string& resolved_path,
         ModuleLoader& loader,
         std::unordered_map<std::string, std::string>& resolved_by_specifier,
         std::unordered_set<std::string>& visiting,
         std::unordered_set<std::string>& visited,
         std::vector<std::string>& stack,
         ImportCycleReport& report) {
    if (report.has_cycle) {
        return;
    }

    const auto visit_it = visiting.find(resolved_path);
    if (visit_it != visiting.end()) {
        report.has_cycle = true;
        report.cycle = stack;
        report.cycle.push_back(specifier);
        return;
    }
    if (visited.count(resolved_path) != 0) {
        return;
    }

    visiting.insert(resolved_path);
    stack.push_back(specifier);
    resolved_by_specifier[specifier] = resolved_path;

    loader.push_import_context(resolved_path);
    try {
        auto program = parse_file(resolved_path);
        for (const auto& next_specifier : collect_import_specifiers(*program)) {
            std::string next_resolved;
            try {
                next_resolved = loader.resolve_specifier(next_specifier);
            } catch (...) {
                continue;
            }
            dfs(next_specifier,
                next_resolved,
                loader,
                resolved_by_specifier,
                visiting,
                visited,
                stack,
                report);
        }
    } catch (...) {
        loader.pop_import_context();
        visiting.erase(resolved_path);
        if (!stack.empty()) {
            stack.pop_back();
        }
        return;
    }
    loader.pop_import_context();

    visiting.erase(resolved_path);
    visited.insert(resolved_path);
    if (!stack.empty()) {
        stack.pop_back();
    }
}

} // namespace

ImportCycleReport detect_import_cycles(const std::string& entry_path, ModuleLoader& loader) {
    ImportCycleReport report;
    std::unordered_map<std::string, std::string> resolved_by_specifier;
    std::unordered_set<std::string> visiting;
    std::unordered_set<std::string> visited;
    std::vector<std::string> stack;

    dfs(entry_path, entry_path, loader, resolved_by_specifier, visiting, visited, stack, report);
    return report;
}

} // namespace dryad
