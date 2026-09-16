#ifndef DRYAD_COMPILER_COMPILE_PIPELINE_H
#define DRYAD_COMPILER_COMPILE_PIPELINE_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/bytecode_module.h"
#include <memory>
#include <string>

namespace dryad {

struct CompileOptions {
    bool run_profiler = true;
    bool allow_network = false;
    std::string stdlib_root;
};

std::unique_ptr<Program> parse_file(const std::string& path);
CompiledProgram compile_file_to_bytecode(const std::string& path, const CompileOptions& options);

} // namespace dryad

#endif // DRYAD_COMPILER_COMPILE_PIPELINE_H
