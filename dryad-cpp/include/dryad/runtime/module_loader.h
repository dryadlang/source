#ifndef DRYAD_RUNTIME_MODULE_LOADER_H
#define DRYAD_RUNTIME_MODULE_LOADER_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/library_module.h"
#include <memory>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace dryad {

class Interpreter;
class Environment;

ModuleMetadata collect_module_metadata(const Program* program);

class ModuleLoader {
public:
    explicit ModuleLoader(const std::string& stdlib_root);
    
    void set_stdlib_root(const std::string& stdlib_root);
    void set_library_paths(const std::vector<std::string>& paths);
    void add_library_path(const std::string& path);
    void set_prefer_source_modules(bool prefer_source);
    void push_import_context(const std::string& path);
    void pop_import_context();
    void execute_import(ImportDeclaration* stmt, Interpreter* interpreter);
    void execute_export_from(ExportFromDeclaration* stmt, Interpreter* interpreter);
    std::shared_ptr<Environment> load_module(const std::string& specifier, Interpreter* interpreter);
    const ModuleMetadata* module_metadata(const std::string& specifier) const;
    
private:
    std::string stdlib_root_;
    std::vector<std::string> library_paths_;
    bool prefer_source_modules_ = false;
    std::unordered_map<std::string, std::shared_ptr<Environment>> modules_;
    std::unordered_map<std::string, std::unique_ptr<Program>> programs_;
    std::unordered_map<std::string, ModuleMetadata> metadata_;
    std::unordered_map<std::string, std::shared_ptr<CompiledLibrary>> libraries_;
    std::unordered_map<std::string, std::shared_ptr<LibraryMaterialization>> materializations_;
    std::unordered_set<std::string> loading_;
    std::vector<std::string> import_context_stack_;
    std::unordered_map<std::string, std::string> module_source_paths_;
    
    std::string import_context_path() const;
    std::string resolve_specifier(const std::string& specifier) const;
    std::string resolve_cache_key(const std::string& specifier) const;
    std::string module_cache_key(const std::string& specifier, const std::string& resolved_path) const;
    std::string resolve_module_path(const std::string& specifier) const;
    std::string resolve_library_path(const std::string& specifier) const;
    static std::string read_file(const std::string& path);
    static void validate_import(const std::string& module_path,
                                const std::string& name,
                                const ModuleMetadata& metadata);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_MODULE_LOADER_H
