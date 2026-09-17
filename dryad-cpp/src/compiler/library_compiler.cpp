#include "dryad/compiler/library_compiler.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/class.h"
#include "dryad/runtime/environment.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/module_loader.h"
#include "dryad/common/utils.h"

namespace dryad {

namespace {

std::string mangle_class_method(const std::string& class_name,
                                const std::string& method_name,
                                bool is_constructor) {
    if (is_constructor) {
        return "$" + class_name + "$constructor";
    }
    return "$" + class_name + "$" + method_name;
}

std::string derive_package_name(const std::string& path) {
    const auto at = path.find('@');
    if (at == std::string::npos) {
        const auto slash = path.find_last_of("/\\");
        const auto dot = path.find_last_of('.');
        if (slash == std::string::npos) {
            return dot == std::string::npos ? path : path.substr(0, dot);
        }
        return dot == std::string::npos ? path.substr(slash + 1)
                                        : path.substr(slash + 1, dot - slash - 1);
    }
    const auto dot = path.find_last_of('.');
    return dot == std::string::npos ? path.substr(at) : path.substr(at, dot - at);
}

uint32_t append_compiled_function(CompiledProgram& program, CompiledFunction&& function) {
    const uint32_t index = static_cast<uint32_t>(program.functions.size());
    program.functions.push_back(std::move(function));
    return index;
}

bool library_exports_function(const CompiledLibrary& library, const std::string& name) {
    for (const auto& entry : library.functions) {
        if (entry.name == name) {
            return true;
        }
    }
    return false;
}

bool library_exports_class(const CompiledLibrary& library, const std::string& name) {
    for (const auto& entry : library.classes) {
        if (entry.name == name) {
            return true;
        }
    }
    return false;
}

bool library_exports_value(const CompiledLibrary& library, const std::string& name) {
    for (const auto& entry : library.values) {
        if (entry.name == name) {
            return true;
        }
    }
    return false;
}

void export_function_from_env(const std::string& name,
                              const std::shared_ptr<Environment>& module_env,
                              BytecodeCompiler& compiler,
                              CompiledLibrary& library) {
    if (!module_env->has(name)) {
        return;
    }
    Value fn_value = module_env->get(name);
    if (!fn_value.is_function()) {
        return;
    }
    auto dryad_fn = std::dynamic_pointer_cast<DryadFunction>(fn_value.as_function());
    if (!dryad_fn || !dryad_fn->declaration()) {
        return;
    }

    LibraryFunctionExport export_entry;
    export_entry.name = name;
    auto compiled_fn = compiler.compile_function(dryad_fn->declaration(), nullptr);
    compiled_fn.metadata.name = name;
    export_entry.function_index = append_compiled_function(library.program, std::move(compiled_fn));
    library.functions.push_back(std::move(export_entry));
}

void export_class_from_env(const std::string& name,
                           const std::shared_ptr<Environment>& module_env,
                           BytecodeCompiler& compiler,
                           CompiledLibrary& library) {
    if (!module_env->has(name)) {
        return;
    }
    Value class_value = module_env->get(name);
    if (!class_value.is_class()) {
        return;
    }
    auto klass = class_value.as_class();

    LibraryClassExport class_export;
    class_export.name = name;

    if (klass->constructor) {
        auto ctor_fn = std::dynamic_pointer_cast<DryadFunction>(klass->constructor);
        if (ctor_fn && ctor_fn->declaration()) {
            auto compiled_ctor =
                compiler.compile_instance_method(ctor_fn->declaration(), nullptr);
            compiled_ctor.metadata.name = mangle_class_method(name, "constructor", true);
            class_export.constructor_index =
                append_compiled_function(library.program, std::move(compiled_ctor));
        }
    }

    for (const auto& method_entry : klass->methods) {
        auto method_fn = std::dynamic_pointer_cast<DryadFunction>(method_entry.second);
        if (!method_fn || !method_fn->declaration()) {
            continue;
        }
        auto compiled_method =
            compiler.compile_instance_method(method_fn->declaration(), nullptr);
        compiled_method.metadata.name =
            mangle_class_method(name, method_entry.first, false);
        const uint32_t index =
            append_compiled_function(library.program, std::move(compiled_method));
        class_export.methods.push_back(LibraryClassMethod{method_entry.first, index});
    }

    library.classes.push_back(std::move(class_export));
}

void export_value_from_env(const std::string& name,
                           const std::shared_ptr<Environment>& module_env,
                           CompiledLibrary& library) {
    if (!module_env->has(name)) {
        return;
    }
    Value value = module_env->get(name);
    if (value.is_function() || value.is_class()) {
        return;
    }
    library.values.push_back({name, value});
}

} // namespace

CompiledLibrary compile_library_file(const std::string& path, const CompileOptions& options) {
    auto program = parse_file(path);
    ModuleMetadata metadata = collect_module_metadata(program.get());

    Interpreter interpreter;
    if (!options.stdlib_root.empty()) {
        interpreter.set_stdlib_root(options.stdlib_root);
    }
    if (!options.library_paths.empty()) {
        interpreter.set_library_paths(options.library_paths);
    }
    interpreter.module_loader().set_prefer_source_modules(true);
    interpreter.set_network_allowed(options.allow_network);

    auto module_env = std::make_shared<Environment>(interpreter.runtime_env());
    auto previous = interpreter.current_env();
    interpreter.set_current_env(module_env);
    interpreter.module_loader().push_import_context(path);

    try {
        for (auto& stmt : program->statements) {
            interpreter.execute_statement(stmt.get());
        }
    } catch (...) {
        interpreter.set_current_env(previous);
        interpreter.module_loader().pop_import_context();
        throw;
    }
    interpreter.set_current_env(previous);
    interpreter.module_loader().pop_import_context();

    CompiledLibrary library;
    library.package_name = derive_package_name(path);
    library.package_version = "1.0.0";
    library.source_path = path;
    library.metadata = metadata;

    BytecodeCompiler compiler;
    compiler.set_interpreter(&interpreter);

    for (const auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::FunctionDeclaration) {
            continue;
        }
        auto* fn = static_cast<FunctionDeclaration*>(stmt.get());
        if (!fn->is_exported || fn->is_internal) {
            continue;
        }
        if (!module_env->has(fn->name)) {
            continue;
        }
        Value fn_value = module_env->get(fn->name);
        if (!fn_value.is_function()) {
            continue;
        }
        auto dryad_fn = std::dynamic_pointer_cast<DryadFunction>(fn_value.as_function());
        if (!dryad_fn || !dryad_fn->declaration()) {
            continue;
        }

        LibraryFunctionExport export_entry;
        export_entry.name = fn->name;
        auto compiled_fn = compiler.compile_function(dryad_fn->declaration(), nullptr);
        compiled_fn.metadata.name = fn->name;
        export_entry.function_index = append_compiled_function(library.program, std::move(compiled_fn));
        library.functions.push_back(std::move(export_entry));
    }

    for (const auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::ClassDeclaration) {
            continue;
        }
        auto* class_decl = static_cast<ClassDeclaration*>(stmt.get());
        if (!class_decl->is_exported || class_decl->is_internal) {
            continue;
        }
        if (!module_env->has(class_decl->name)) {
            continue;
        }
        Value class_value = module_env->get(class_decl->name);
        if (!class_value.is_class()) {
            continue;
        }
        auto klass = class_value.as_class();

        LibraryClassExport class_export;
        class_export.name = class_decl->name;

        if (klass->constructor) {
            auto ctor_fn = std::dynamic_pointer_cast<DryadFunction>(klass->constructor);
            if (ctor_fn && ctor_fn->declaration()) {
                auto compiled_ctor =
                    compiler.compile_instance_method(ctor_fn->declaration(), nullptr);
                compiled_ctor.metadata.name =
                    mangle_class_method(class_decl->name, "constructor", true);
                class_export.constructor_index =
                    append_compiled_function(library.program, std::move(compiled_ctor));
            }
        }

        for (const auto& method_entry : klass->methods) {
            auto method_fn = std::dynamic_pointer_cast<DryadFunction>(method_entry.second);
            if (!method_fn || !method_fn->declaration()) {
                continue;
            }
            auto compiled_method =
                compiler.compile_instance_method(method_fn->declaration(), nullptr);
            compiled_method.metadata.name =
                mangle_class_method(class_decl->name, method_entry.first, false);
            const uint32_t index =
                append_compiled_function(library.program, std::move(compiled_method));
            class_export.methods.push_back(
                LibraryClassMethod{method_entry.first, index});
        }

        library.classes.push_back(std::move(class_export));
    }

    for (const auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::VariableDeclaration) {
            continue;
        }
        auto* var = static_cast<VariableDeclaration*>(stmt.get());
        if (!var->is_exported) {
            continue;
        }
        if (!module_env->has(var->name)) {
            throw DryadException("Exported value is not defined: " + var->name);
        }
        Value value = module_env->get(var->name);
        if (value.is_function() || value.is_class()) {
            continue;
        }
        library.values.push_back({var->name, value});
    }

    for (const auto& export_name : metadata.exports) {
        if (library_exports_function(library, export_name) ||
            library_exports_class(library, export_name) ||
            library_exports_value(library, export_name)) {
            continue;
        }

        if (!module_env->has(export_name)) {
            continue;
        }

        Value value = module_env->get(export_name);
        if (value.is_function()) {
            export_function_from_env(export_name, module_env, compiler, library);
        } else if (value.is_class()) {
            export_class_from_env(export_name, module_env, compiler, library);
        } else {
            export_value_from_env(export_name, module_env, library);
        }
    }

    return library;
}

} // namespace dryad
