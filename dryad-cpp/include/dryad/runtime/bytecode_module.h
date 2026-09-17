#ifndef DRYAD_RUNTIME_BYTECODE_MODULE_H
#define DRYAD_RUNTIME_BYTECODE_MODULE_H

#include "dryad/compiler/type_profiler.h"
#include "dryad/runtime/value.h"
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

struct BytecodeModule {
    std::vector<uint8_t> code;
    std::vector<Value> constants;
};

struct SlotMetadata {
    std::string name;
    uint16_t slot_index = 0;
    ObservedType type = ObservedType::Any;
    TypeConfidence confidence = TypeConfidence::UNKNOWN;
};

struct FunctionMetadata {
    std::string name;
    std::vector<SlotMetadata> slots;
    uint8_t parameter_count = 0;
    uint16_t stack_depth = 0;
    uint32_t opcode_count = 0;
};

struct CompiledFunction {
    FunctionMetadata metadata;
    BytecodeModule module;
};

struct CompiledProgram {
    std::vector<CompiledFunction> functions;
};

bool save_compiled_program(const CompiledProgram& program, const std::string& path);
bool load_compiled_program(const std::string& path, CompiledProgram& program);
uint16_t find_function_index(const CompiledProgram& program, const std::string& name);

} // namespace dryad

#endif // DRYAD_RUNTIME_BYTECODE_MODULE_H
