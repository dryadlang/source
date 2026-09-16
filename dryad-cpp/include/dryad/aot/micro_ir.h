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
    LoadFast,
    StoreFast,
    AddI32,
    AddF64,
    SubI32,
    SubF64,
    MulI32,
    MulF64,
    DivF64,
    CmpLtI32,
    GenericAdd,
    GenericGetElement,
    IntrinsicSlow,
    Return
};

struct IrValue {
    int id = -1;
    IrType type = IrType::Variant;
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
    uint16_t slot = 0;
    uint16_t intrinsic_id = 0;
    uint8_t intrinsic_argc = 0;
};

struct IrFunction {
    std::string name;
    FunctionMetadata metadata;
    std::vector<IrInstruction> instructions;
    std::vector<IrType> slot_types;
};

struct IrModule {
    std::vector<IrFunction> functions;
};

IrModule bytecode_to_micro_ir(const CompiledProgram& program);
IrType observed_to_ir_type(ObservedType type, TypeConfidence confidence);

} // namespace dryad

#endif // DRYAD_AOT_MICRO_IR_H
