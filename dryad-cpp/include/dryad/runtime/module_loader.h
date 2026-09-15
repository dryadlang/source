#ifndef DRYAD_RUNTIME_MODULE_LOADER_H
#define DRYAD_RUNTIME_MODULE_LOADER_H

#include "dryad/compiler/ast.h"
#include <memory>
#include <string>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace dryad {

class Interpreter;
class Environment;

class ModuleLoader {
public:
    explicit ModuleLoader(const std::string& stdlib_root);
    
    void set_stdlib_root(const std::string& stdlib_root);
    void execute_import(ImportDeclaration* stmt, Interpreter* interpreter);
    
private:
    std::string stdlib_root_;
    std::unordered_map<std::string, std::shared_ptr<Environment>> modules_;
    std::unordered_map<std::string, std::unique_ptr<Program>> programs_;
    std::unordered_set<std::string> loading_;
    
    std::string resolve_module_path(const std::string& specifier) const;
    static std::string read_file(const std::string& path);
    std::shared_ptr<Environment> load_module(const std::string& specifier, Interpreter* interpreter);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_MODULE_LOADER_H
