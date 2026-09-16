#include "dryad/compiler/execution_mode.h"
#include "dryad/common/utils.h"

namespace dryad {

std::string execution_mode_name(ExecutionMode mode) {
    switch (mode) {
        case ExecutionMode::Inherit:     return "inherit";
        case ExecutionMode::Interpreter: return "interpreter";
        case ExecutionMode::Bytecode:    return "bytecode";
        case ExecutionMode::AOT:         return "AOT";
    }
    return "inherit";
}

ExecutionMode parse_execution_mode(const std::string& value) {
    if (value == "interpreter") {
        return ExecutionMode::Interpreter;
    }
    if (value == "bytecode") {
        return ExecutionMode::Bytecode;
    }
    if (value == "AOT" || value == "aot") {
        return ExecutionMode::AOT;
    }
    throw DryadException("Unknown @compile mode: " + value);
}

} // namespace dryad
