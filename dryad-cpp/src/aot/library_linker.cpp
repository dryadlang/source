#include "dryad/aot/library_linker.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/native_linker.h"
#include "dryad/common/utils.h"
#include <cctype>
#include <fstream>

namespace dryad {

namespace {

std::string sanitize_package_symbol(const std::string& package_name) {
    std::string out = package_name;
    if (!out.empty() && out[0] == '@') {
        out = out.substr(1);
    }
    for (char& ch : out) {
        if (!std::isalnum(static_cast<unsigned char>(ch))) {
            ch = '_';
        }
    }
    return out.empty() ? "lib" : out;
}

std::string sanitize_export_symbol(const std::string& name) {
    std::string out = name;
    for (char& ch : out) {
        if (!std::isalnum(static_cast<unsigned char>(ch))) {
            ch = '_';
        }
    }
    return out.empty() ? "export" : out;
}

} // namespace

std::string mangle_library_export(const std::string& package_name, const std::string& export_name) {
    return "dryad_lib_" + sanitize_package_symbol(package_name) + "_" +
           sanitize_export_symbol(export_name);
}

IrModule library_to_ir_module(const CompiledLibrary& library) {
    IrModule module;
    for (const auto& export_entry : library.functions) {
        if (export_entry.function_index >= library.program.functions.size()) {
            continue;
        }
        IrFunction function =
            bytecode_function_to_micro_ir(library.program.functions[export_entry.function_index]);
        function.name = mangle_library_export(library.package_name, export_entry.name);
        module.functions.push_back(std::move(function));
    }
    return module;
}

void merge_ir_modules(IrModule& target, const IrModule& source) {
    target.functions.insert(target.functions.end(),
                            source.functions.begin(),
                            source.functions.end());
}

std::unordered_map<std::string, std::string> build_linked_export_map(const CompiledLibrary& library) {
    std::unordered_map<std::string, std::string> exports;
    for (const auto& export_entry : library.functions) {
        exports[export_entry.name] =
            mangle_library_export(library.package_name, export_entry.name);
    }
    return exports;
}

void merge_linked_export_maps(std::unordered_map<std::string, std::string>& target,
                              const std::unordered_map<std::string, std::string>& source) {
    for (const auto& entry : source) {
        auto existing = target.find(entry.first);
        if (existing != target.end() && existing->second != entry.second) {
            throw DryadException("Duplicate linked export: " + entry.first);
        }
        target[entry.first] = entry.second;
    }
}

std::vector<CompiledLibrary> load_link_libraries(const std::vector<std::string>& paths) {
    std::vector<CompiledLibrary> libraries;
    for (const auto& path : paths) {
        CompiledLibrary library;
        if (!load_compiled_library(path, library)) {
            throw DryadException("Failed to load library for AOT linking: " + path);
        }
        libraries.push_back(std::move(library));
    }
    return libraries;
}

IrModule link_libraries_into_module(const IrModule& app_module,
                                    const std::vector<std::string>& library_paths,
                                    IrEmitOptions& emit_options) {
    IrModule merged = app_module;
    for (const auto& path : library_paths) {
        CompiledLibrary library;
        if (!load_compiled_library(path, library)) {
            throw DryadException("Failed to load library for AOT linking: " + path);
        }
        merge_ir_modules(merged, library_to_ir_module(library));
        merge_linked_export_maps(emit_options.linked_exports, build_linked_export_map(library));
    }
    return merged;
}

AotCompileResult compile_library_to_llvm_ir(const std::string& dyl_path,
                                             const std::string& llvm_output_path) {
    AotCompileResult result;
    result.output_path = llvm_output_path;
    try {
        CompiledLibrary library;
        if (!load_compiled_library(dyl_path, library)) {
            result.message = "Failed to load library: " + dyl_path;
            return result;
        }

        const IrModule module = library_to_ir_module(library);
        if (!write_llvm_ir_file(module, llvm_output_path)) {
            result.message = "Failed to write LLVM IR file: " + llvm_output_path;
            return result;
        }

        result.binary = read_binary_file(llvm_output_path);
        result.success = true;
        result.message = "Wrote LLVM IR for " + std::to_string(module.functions.size()) +
                         " exported function(s) from " + library.package_name + " to " +
                         llvm_output_path;
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }
    return result;
}

} // namespace dryad
