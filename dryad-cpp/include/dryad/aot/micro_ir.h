#ifndef DRYAD_AOT_MICRO_IR_H
#define DRYAD_AOT_MICRO_IR_H

#include "dryad/runtime/bytecode_module.h"
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

enum class IrType {
    Void,
    I32,
    F64,
    Bool,
    Ptr,
    Variant
};

enum class IrOpcode {
    ConstI32,
    ConstF64,
    ConstBool,
    ConstString,
    LoadFast,
    StoreFast,
    AddI32,
    AddF64,
    SubI32,
    SubF64,
    MulI32,
    MulF64,
    DivF64,
    CmpEqI32,
    CmpNeI32,
    CmpLtI32,
    CmpLeI32,
    CmpGtI32,
    CmpGeI32,
    GenericAdd,
    GenericGetElement,
    UnboxI32,
    IntrinsicSlow,
    Br,
    BrCond,
    Return
};

struct IrInstruction {
    IrOpcode op = IrOpcode::Return;
    IrType result_type = IrType::Void;
    int result = -1;
    int lhs = -1;
    int rhs = -1;
    int64_t const_i32 = 0;
    double const_f64 = 0.0;
    bool const_bool = false;
    uint16_t const_string_id = 0;
    uint16_t slot = 0;
    uint16_t intrinsic_id = 0;
    uint8_t intrinsic_argc = 0;
    uint32_t branch_target = 0;
    uint32_t branch_false_target = 0;
};

struct IrBasicBlock {
    uint32_t id = 0;
    uint32_t bytecode_pc = 0;
    std::vector<IrInstruction> instructions;
};

struct IrFunction {
    std::string name;
    FunctionMetadata metadata;
    std::vector<IrBasicBlock> blocks;
    std::vector<IrInstruction> instructions;
    std::vector<IrType> slot_types;
    std::vector<std::string> string_literals;
    uint32_t entry_block = 0;
};

struct IrModule {
    std::vector<IrFunction> functions;
};

IrModule bytecode_to_micro_ir(const CompiledProgram& program);
IrFunction bytecode_function_to_micro_ir(const CompiledFunction& function);
IrType observed_to_ir_type(ObservedType type, TypeConfidence confidence);
void flatten_ir_function(IrFunction& function);

} // namespace dryad

#endif // DRYAD_AOT_MICRO_IR_H
