#include "dryad/runtime/module_loader.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/function.h"
#include "dryad/common/utils.h"
#include <fstream>
#include <sstream>

namespace dryad {

ModuleLoader::ModuleLoader(const std::string& stdlib_root)
    : stdlib_root_(stdlib_root) {}

void ModuleLoader::set_stdlib_root(const std::string& stdlib_root) {
    stdlib_root_ = stdlib_root;
}

std::string ModuleLoader::resolve_module_path(const std::string& specifier) const {
    if (specifier.empty()) {
        throw DryadException("Module specifier cannot be empty");
    }
    if (specifier.rfind("@std/", 0) == 0) {
        return stdlib_root_ + "/" + specifier + ".dryad";
    }
    if (specifier.size() > 7 && specifier.substr(specifier.size() - 7) == ".dryad") {
        return specifier;
    }
    return specifier + ".dryad";
}

std::string ModuleLoader::read_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw DryadException("Cannot open module file: " + path);
    }
    std::ostringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

std::shared_ptr<Environment> ModuleLoader::load_module(const std::string& specifier,
                                                       Interpreter* interpreter) {
    auto cached = modules_.find(specifier);
    if (cached != modules_.end()) {
        return cached->second;
    }

    if (loading_.count(specifier) != 0) {
        throw DryadException("Circular module import detected: " + specifier);
    }

    const std::string path = resolve_module_path(specifier);
    const std::string source = read_file(path);

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw DryadException("Parser error in module " + specifier + ": " + parser.error_message());
    }

    loading_.insert(specifier);

    auto module_env = std::make_shared<Environment>(interpreter->global_env());
    auto previous = interpreter->current_env();
    interpreter->set_current_env(module_env);

    try {
        for (auto& stmt : program->statements) {
            interpreter->execute_statement(stmt.get());
        }
    } catch (...) {
        interpreter->set_current_env(previous);
        loading_.erase(specifier);
        throw;
    }

    interpreter->set_current_env(previous);
    loading_.erase(specifier);

    programs_[specifier] = std::move(program);
    modules_[specifier] = module_env;
    return module_env;
}

void ModuleLoader::execute_import(ImportDeclaration* stmt, Interpreter* interpreter) {
    auto module_env = load_module(stmt->module_path, interpreter);

    for (const auto& name : stmt->names) {
        if (!module_env->has(name)) {
            throw DryadException("Module '" + stmt->module_path + "' does not export '" + name + "'");
        }

        Value value = module_env->get(name);
        if (value.is_function()) {
            auto dryad_fn = std::dynamic_pointer_cast<DryadFunction>(value.as_function());
            if (dryad_fn && dryad_fn->declaration() && dryad_fn->declaration()->is_internal) {
                throw DryadException("Cannot import internal member '" + name + "' from '" +
                                     stmt->module_path + "'");
            }
        }

        interpreter->current_env()->define(name, value);
    }
}

} // namespace dryad
