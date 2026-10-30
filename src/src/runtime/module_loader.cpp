#include "dryad/runtime/module_loader.h"
#include "dryad/runtime/library_module.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/function.h"
#include "dryad/common/utils.h"
#include <cstdlib>
#include <filesystem>
#include <fstream>
#include <sstream>

namespace dryad {

namespace {

bool file_exists(const std::string& path) {
    std::ifstream file(path);
    return file.good();
}

bool has_source_extension(const std::string& path) {
    return ends_with(path, ".dryad") || ends_with(path, ".dyd");
}

bool is_relative_specifier(const std::string& specifier) {
    return starts_with(specifier, "./") || starts_with(specifier, "../");
}

std::string resolve_with_extensions(const std::string& base_path) {
    if (has_source_extension(base_path)) {
        if (file_exists(base_path)) {
            return base_path;
        }
        throw DryadException("Cannot open module file: " + base_path);
    }

    const std::string dyd_path = base_path + ".dyd";
    if (file_exists(dyd_path)) {
        return dyd_path;
    }

    const std::string dryad_path = base_path + ".dryad";
    if (file_exists(dryad_path)) {
        return dryad_path;
    }

    throw DryadException("Cannot open module file: " + base_path + " (.dyd or .dryad)");
}

std::vector<std::string> library_paths_from_env() {
    std::vector<std::string> paths;
    const char* env = std::getenv("DRYAD_LIBRARY_PATH");
    if (env && env[0] != '\0') {
        paths = split(env, ':');
    }
    if (file_exists("dryad_modules")) {
        paths.push_back("dryad_modules");
    }
    return paths;
}

} // namespace

ModuleLoader::ModuleLoader(const std::string& stdlib_root)
    : stdlib_root_(stdlib_root), library_paths_(library_paths_from_env()) {}

void ModuleLoader::set_stdlib_root(const std::string& stdlib_root) {
    stdlib_root_ = stdlib_root;
}

void ModuleLoader::set_library_paths(const std::vector<std::string>& paths) {
    library_paths_ = paths;
}

void ModuleLoader::add_library_path(const std::string& path) {
    library_paths_.push_back(path);
}

void ModuleLoader::set_prefer_source_modules(bool prefer_source) {
    prefer_source_modules_ = prefer_source;
}

void ModuleLoader::push_import_context(const std::string& path) {
    import_context_stack_.push_back(path);
}

void ModuleLoader::pop_import_context() {
    if (!import_context_stack_.empty()) {
        import_context_stack_.pop_back();
    }
}

std::string ModuleLoader::import_context_path() const {
    if (import_context_stack_.empty()) {
        return "";
    }
    return import_context_stack_.back();
}

std::string ModuleLoader::module_cache_key(const std::string& specifier,
                                         const std::string& resolved_path) const {
    if (is_relative_specifier(specifier) || specifier.rfind("@", 0) != 0) {
        return std::filesystem::path(resolved_path).replace_extension("").string();
    }
    return specifier;
}

std::string ModuleLoader::resolve_cache_key(const std::string& specifier) const {
    std::string resolved_path;
    try {
        resolved_path = resolve_specifier(specifier);
    } catch (...) {
        if (specifier.rfind("@", 0) != 0) {
            throw;
        }
    }
    return resolved_path.empty() ? specifier : module_cache_key(specifier, resolved_path);
}

std::string ModuleLoader::resolve_specifier(const std::string& specifier) const {
    if (specifier.empty()) {
        throw DryadException("Module specifier cannot be empty");
    }

    if (is_relative_specifier(specifier)) {
        const std::string context = import_context_path();
        if (context.empty()) {
            throw DryadException("Relative import requires module context: " + specifier);
        }
        const auto base = std::filesystem::path(context).parent_path();
        const auto resolved =
            std::filesystem::weakly_canonical(base / specifier).string();
        return resolve_with_extensions(resolved);
    }

    return resolve_module_path(specifier);
}

std::string ModuleLoader::resolve_module_path(const std::string& specifier) const {
    if (specifier.empty()) {
        throw DryadException("Module specifier cannot be empty");
    }

    if (specifier.rfind("@std/", 0) == 0) {
        return resolve_with_extensions(stdlib_root_ + "/" + specifier);
    }

    if (specifier.rfind("@", 0) == 0) {
        for (const auto& root : library_paths_) {
            const std::string candidate = root + "/" + specifier;
            if (has_source_extension(candidate)) {
                if (file_exists(candidate)) {
                    return candidate;
                }
            } else if (file_exists(candidate + ".dyd") || file_exists(candidate + ".dryad")) {
                return resolve_with_extensions(candidate);
            }
        }
    }

    if (has_source_extension(specifier)) {
        return resolve_with_extensions(specifier);
    }

    return resolve_with_extensions(specifier);
}

std::string ModuleLoader::resolve_library_path(const std::string& specifier) const {
    if (specifier.empty()) {
        throw DryadException("Module specifier cannot be empty");
    }

    if (is_relative_specifier(specifier)) {
        const std::string source_path = resolve_specifier(specifier);
        const std::string stem =
            std::filesystem::path(source_path).replace_extension("").string();
        return stem + ".dyl";
    }

    if (specifier.rfind("@std/", 0) == 0) {
        return stdlib_root_ + "/" + specifier + ".dyl";
    }

    if (specifier.rfind("@", 0) == 0) {
        for (const auto& root : library_paths_) {
            const std::string candidate = root + "/" + specifier + ".dyl";
            if (file_exists(candidate)) {
                return candidate;
            }
        }
    }

    if (ends_with(specifier, ".dyl") && file_exists(specifier)) {
        return specifier;
    }

    return specifier + ".dyl";
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

ModuleMetadata collect_module_metadata(const Program* program) {
    ModuleMetadata metadata;
    if (!program) {
        return metadata;
    }

    for (const auto& stmt : program->statements) {
        switch (stmt->type) {
            case ASTNodeType::FunctionDeclaration: {
                auto* fn = static_cast<FunctionDeclaration*>(stmt.get());
                if (fn->is_internal) {
                    metadata.internal_symbols.insert(fn->name);
                }
                if (fn->is_exported) {
                    metadata.exports.insert(fn->name);
                    metadata.has_explicit_exports = true;
                }
                break;
            }
            case ASTNodeType::ClassDeclaration: {
                auto* klass = static_cast<ClassDeclaration*>(stmt.get());
                if (klass->is_internal) {
                    metadata.internal_symbols.insert(klass->name);
                }
                if (klass->is_exported) {
                    metadata.exports.insert(klass->name);
                    metadata.has_explicit_exports = true;
                }
                break;
            }
            case ASTNodeType::VariableDeclaration: {
                auto* var = static_cast<VariableDeclaration*>(stmt.get());
                if (var->is_exported) {
                    metadata.exports.insert(var->name);
                    metadata.has_explicit_exports = true;
                }
                break;
            }
            case ASTNodeType::ExportDeclaration: {
                auto* export_stmt = static_cast<ExportFromDeclaration*>(stmt.get());
                for (const auto& name : export_stmt->names) {
                    metadata.exports.insert(name);
                }
                metadata.has_explicit_exports = true;
                break;
            }
            default:
                break;
        }
    }

    return metadata;
}

void ModuleLoader::validate_import(const std::string& module_path,
                                   const std::string& name,
                                   const ModuleMetadata& metadata) {
    if (metadata.internal_symbols.count(name) != 0) {
        throw DryadException("Cannot import internal member '" + name + "' from '" + module_path + "'");
    }

    if (metadata.has_explicit_exports && metadata.exports.count(name) == 0) {
        throw DryadException("Module '" + module_path + "' does not export '" + name + "'");
    }
}

const ModuleMetadata* ModuleLoader::module_metadata(const std::string& specifier) const {
    auto it = metadata_.find(specifier);
    if (it == metadata_.end()) {
        return nullptr;
    }
    return &it->second;
}

std::shared_ptr<Environment> ModuleLoader::load_module(const std::string& specifier,
                                                       Interpreter* interpreter) {
    std::string resolved_path;
    try {
        resolved_path = resolve_specifier(specifier);
    } catch (...) {
        if (specifier.rfind("@", 0) != 0) {
            throw;
        }
    }

    const std::string cache_key = resolve_cache_key(specifier);

    auto cached = modules_.find(cache_key);
    if (cached != modules_.end()) {
        return cached->second;
    }

    if (loading_.count(cache_key) != 0) {
        throw DryadException("Circular module import detected: " + specifier);
    }

    const std::string library_path = resolve_library_path(specifier);
    if (!prefer_source_modules_ && file_exists(library_path)) {
        const bool source_missing = resolved_path.empty() || !file_exists(resolved_path);
        if (source_missing || library_file_is_fresh(library_path, resolved_path)) {
            loading_.insert(cache_key);

            CompiledLibrary library;
            if (!load_compiled_library(library_path, library)) {
                loading_.erase(cache_key);
                throw DryadException("Failed to load library: " + library_path);
            }

            auto materialization = materialize_library(library, interpreter);
            metadata_[cache_key] = materialization.library->metadata;
            libraries_[cache_key] = materialization.library;
            materializations_[cache_key] =
                std::make_shared<LibraryMaterialization>(std::move(materialization));
            modules_[cache_key] = materializations_[cache_key]->environment;
            if (!resolved_path.empty()) {
                module_source_paths_[cache_key] = resolved_path;
            }
            loading_.erase(cache_key);
            return modules_[cache_key];
        }
    }

    if (resolved_path.empty()) {
        resolved_path = resolve_specifier(specifier);
    }

    const std::string source = read_file(resolved_path);

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw DryadException("Parser error in module " + specifier + ": " + parser.error_message());
    }

    const ModuleMetadata metadata = collect_module_metadata(program.get());

    loading_.insert(cache_key);
    import_context_stack_.push_back(resolved_path);

    auto module_env = std::make_shared<Environment>(interpreter->runtime_env());
    auto previous = interpreter->current_env();
    interpreter->set_current_env(module_env);

    try {
        for (auto& stmt : program->statements) {
            interpreter->execute_statement(stmt.get());
        }
    } catch (...) {
        interpreter->set_current_env(previous);
        import_context_stack_.pop_back();
        loading_.erase(cache_key);
        throw;
    }

    interpreter->set_current_env(previous);
    import_context_stack_.pop_back();
    loading_.erase(cache_key);

    programs_[cache_key] = std::move(program);
    metadata_[cache_key] = metadata;
    modules_[cache_key] = module_env;
    module_source_paths_[cache_key] = resolved_path;
    return module_env;
}

void ModuleLoader::execute_import(ImportDeclaration* stmt, Interpreter* interpreter) {
    auto module_env = load_module(stmt->module_path, interpreter);

    const std::string cache_key = resolve_cache_key(stmt->module_path);
    const ModuleMetadata* metadata = module_metadata(cache_key);
    if (!metadata) {
        throw DryadException("Missing module metadata for '" + stmt->module_path + "'");
    }

    for (const auto& name : stmt->names) {
        validate_import(stmt->module_path, name, *metadata);

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

void ModuleLoader::execute_export_from(ExportFromDeclaration* stmt, Interpreter* interpreter) {
    auto source_env = load_module(stmt->module_path, interpreter);

    const std::string cache_key = resolve_cache_key(stmt->module_path);
    const ModuleMetadata* metadata = module_metadata(cache_key);
    if (!metadata) {
        throw DryadException("Missing module metadata for '" + stmt->module_path + "'");
    }

    for (const auto& name : stmt->names) {
        validate_import(stmt->module_path, name, *metadata);

        if (!source_env->has(name)) {
            throw DryadException("Module '" + stmt->module_path + "' does not export '" + name + "'");
        }

        interpreter->current_env()->define(name, source_env->get(name));
    }
}

} // namespace dryad
