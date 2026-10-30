#ifndef DRYAD_COMPILER_EXECUTION_MODE_H
#define DRYAD_COMPILER_EXECUTION_MODE_H

#include <string>

namespace dryad {

enum class ExecutionMode {
    Inherit,
    Interpreter,
    Bytecode,
    AOT
};

std::string execution_mode_name(ExecutionMode mode);
ExecutionMode parse_execution_mode(const std::string& value);

} // namespace dryad

#endif // DRYAD_COMPILER_EXECUTION_MODE_H
