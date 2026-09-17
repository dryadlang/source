#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/aot/native_linker.h"
#include "dryad/compiler/program_entry.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/common/utils.h"

namespace dryad {

std::string resolve_entry_function(const CompiledProgram& program,
                                   const CompileOptions& options) {
    if (!options.entry_function.empty()) {
        for (const auto& function : program.functions) {
            if (function.metadata.name == options.entry_function) {
                return options.entry_function;
            }
        }
        throw DryadException("Entry function not found: " + options.entry_function);
    }
    if (program.functions.empty()) {
        throw DryadException("Cannot link native binary: program has no entry function");
    }

    for (const auto& function : program.functions) {
        if (function.metadata.name == kScriptEntryFunctionName) {
            return kScriptEntryFunctionName;
        }
    }
    return program.functions.front().metadata.name;
}

namespace {

AotCompileResult link_compiled_program(const std::string& input_path,
                                       const std::string& output_path,
                                       const CompileOptions& options,
                                       NativeLinkKind kind,
                                       const std::string& aot_runtime_source,
                                       const std::string& include_dir) {
    AotCompileResult result;
    result.output_path = output_path;

    try {
        CompiledProgram program = compile_file_to_bytecode(input_path, options);
        const std::string entry = resolve_entry_function(program, options);
        IrModule ir_module = bytecode_to_micro_ir(program);

        const std::string llvm_path = output_path + ".ll";
        const bool wrote_ir = kind == NativeLinkKind::Executable
                                  ? write_llvm_ir_executable_file(ir_module, llvm_path, entry)
                                  : write_llvm_ir_file(ir_module, llvm_path);
        if (!wrote_ir) {
            result.message = "Failed to write LLVM IR file: " + llvm_path;
            return result;
        }

        NativeLinkResult link_result =
            link_native_binary(kind, llvm_path, output_path, aot_runtime_source, include_dir);
        if (!link_result.success) {
            result.message = link_result.message;
            return result;
        }

        result.success = true;
        result.message = link_result.message;
        result.binary = std::move(link_result.binary);
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }

    return result;
}

} // namespace

AotCompileResult compile_to_dryadc(const std::string& input_path,
                                   const std::string& output_path,
                                   const CompileOptions& options) {
    AotCompileResult result;
    result.output_path = output_path;
    try {
        CompiledProgram program = compile_file_to_bytecode(input_path, options);
        if (!save_compiled_program(program, output_path)) {
            result.message = "Failed to write output file: " + output_path;
            return result;
        }
        result.binary = read_binary_file(output_path);
        result.success = true;
        result.message = "Wrote " + std::to_string(program.functions.size()) +
                         " function(s) to " + output_path;
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }
    return result;
}

AotCompileResult compile_to_llvm_ir(const std::string& input_path,
                                    const std::string& output_path,
                                    const CompileOptions& options) {
    AotCompileResult result;
    result.output_path = output_path;
    try {
        CompiledProgram program = compile_file_to_bytecode(input_path, options);
        IrModule ir_module = bytecode_to_micro_ir(program);
        if (!write_llvm_ir_file(ir_module, output_path)) {
            result.message = "Failed to write LLVM IR file: " + output_path;
            return result;
        }
        result.binary = read_binary_file(output_path);
        result.success = true;
        result.message = "Wrote LLVM IR for " + std::to_string(ir_module.functions.size()) +
                         " function(s) to " + output_path;
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }
    return result;
}

AotCompileResult compile_to_native_module(const std::string& input_path,
                                          const std::string& output_path,
                                          const CompileOptions& options,
                                          const std::string& aot_runtime_source,
                                          const std::string& include_dir) {
    return link_compiled_program(input_path,
                                 output_path,
                                 options,
                                 NativeLinkKind::SharedObject,
                                 aot_runtime_source,
                                 include_dir);
}

AotCompileResult compile_to_native_executable(const std::string& input_path,
                                              const std::string& output_path,
                                              const CompileOptions& options,
                                              const std::string& aot_runtime_source,
                                              const std::string& include_dir) {
    return link_compiled_program(input_path,
                                 output_path,
                                 options,
                                 NativeLinkKind::Executable,
                                 aot_runtime_source,
                                 include_dir);
}

} // namespace dryad
