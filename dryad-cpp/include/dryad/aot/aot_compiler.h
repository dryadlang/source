#ifndef DRYAD_AOT_AOT_COMPILER_H
#define DRYAD_AOT_AOT_COMPILER_H

#include "dryad/compiler/compile_pipeline.h"
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

struct AotCompileResult {
    bool success = false;
    std::string output_path;
    std::string message;
    std::vector<uint8_t> binary;
};

AotCompileResult compile_to_dryadc(const std::string& input_path,
                                   const std::string& output_path,
                                   const CompileOptions& options);

AotCompileResult compile_to_llvm_ir(const std::string& input_path,
                                    const std::string& output_path,
                                    const CompileOptions& options);

AotCompileResult compile_to_native_module(const std::string& input_path,
                                          const std::string& output_path,
                                          const CompileOptions& options,
                                          const std::string& aot_runtime_source,
                                          const std::string& include_dir);

AotCompileResult compile_to_native_executable(const std::string& input_path,
                                              const std::string& output_path,
                                              const CompileOptions& options,
                                              const std::string& aot_runtime_source,
                                              const std::string& include_dir);

std::string resolve_entry_function(const CompiledProgram& program,
                                   const CompileOptions& options);

} // namespace dryad

#endif // DRYAD_AOT_AOT_COMPILER_H
