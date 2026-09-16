#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/runtime/bytecode_module.h"

namespace dryad {

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
        result.success = true;
        result.message = "Wrote LLVM IR for " + std::to_string(ir_module.functions.size()) +
                         " function(s) to " + output_path;
    } catch (const std::exception& ex) {
        result.message = ex.what();
    }
    return result;
}

} // namespace dryad
