#ifndef DRYAD_AOT_AOT_COMPILER_H
#define DRYAD_AOT_AOT_COMPILER_H

#include "dryad/compiler/compile_pipeline.h"
#include <string>

namespace dryad {

struct AotCompileResult {
    bool success = false;
    std::string output_path;
    std::string message;
};

AotCompileResult compile_to_dryadc(const std::string& input_path,
                                   const std::string& output_path,
                                   const CompileOptions& options);

AotCompileResult compile_to_llvm_ir(const std::string& input_path,
                                    const std::string& output_path,
                                    const CompileOptions& options);

} // namespace dryad

#endif // DRYAD_AOT_AOT_COMPILER_H
