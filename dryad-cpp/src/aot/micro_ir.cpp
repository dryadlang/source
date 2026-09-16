#include "dryad/aot/micro_ir.h"
#include "dryad/runtime/opcode.h"
#include "dryad/common/utils.h"
#include <cstring>

namespace dryad {

namespace {

IrType ir_type_for_slot(const FunctionMetadata& metadata, uint16_t slot) {
    for (const auto& entry : metadata.slots) {
        if (entry.slot_index == slot) {
            return observed_to_ir_type(entry.type, entry.confidence);
        }
    }
    return IrType::Variant;
}

class BytecodeToIr {
public:
    explicit BytecodeToIr(const CompiledFunction& function) : function_(function) {}

    IrFunction build() {
        IrFunction ir;
        ir.name = function_.metadata.name;
        ir.metadata = function_.metadata;
        ir.slot_types.resize(function_.metadata.slots.size(), IrType::Variant);
        for (const auto& slot : function_.metadata.slots) {
            if (slot.slot_index < ir.slot_types.size()) {
                ir.slot_types[slot.slot_index] =
                    observed_to_ir_type(slot.type, slot.confidence);
            }
        }

        const auto& code = function_.module.code;
        size_t pc = 0;
        while (pc < code.size()) {
            Opcode op = static_cast<Opcode>(code[pc]);
            if (op == Opcode::HALT) {
                if (!stack_.empty()) {
                    IrInstruction ret;
                    ret.op = IrOpcode::Return;
                    ret.result = stack_.back();
                    ir.instructions.push_back(ret);
                }
                break;
            }
            emit_opcode(op, pc, ir);
        }
        return ir;
    }

private:
    const CompiledFunction& function_;
    std::vector<int> stack_;
    int next_reg_ = 0;

    int fresh_reg(IrType type) {
        return next_reg_++;
    }

    uint16_t read_u16(size_t offset) const {
        const auto& code = function_.module.code;
        return static_cast<uint16_t>(code[offset]) |
               (static_cast<uint16_t>(code[offset + 1]) << 8);
    }

    int64_t read_i64(size_t offset) const {
        const auto& code = function_.module.code;
        int64_t value = 0;
        for (int i = 0; i < 8; ++i) {
            value |= static_cast<int64_t>(code[offset + i]) << (i * 8);
        }
        return value;
    }

    double read_f64(size_t offset) const {
        int64_t bits = read_i64(offset);
        double value = 0.0;
        std::memcpy(&value, &bits, sizeof(double));
        return value;
    }

    void push(int reg) { stack_.push_back(reg); }
    int pop() {
        int value = stack_.back();
        stack_.pop_back();
        return value;
    }

    void emit_opcode(Opcode op, size_t& pc, IrFunction& ir) {
        IrInstruction inst;
        switch (op) {
            case Opcode::PUSH_INT:
                inst.op = IrOpcode::ConstI32;
                inst.result = fresh_reg(IrType::I32);
                inst.const_i32 = read_i64(pc + 1);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 9;
                break;
            case Opcode::PUSH_FLOAT:
                inst.op = IrOpcode::ConstF64;
                inst.result = fresh_reg(IrType::F64);
                inst.const_f64 = read_f64(pc + 1);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 9;
                break;
            case Opcode::PUSH_TRUE:
            case Opcode::PUSH_FALSE:
                inst.op = IrOpcode::ConstBool;
                inst.result = fresh_reg(IrType::Bool);
                inst.const_bool = op == Opcode::PUSH_TRUE;
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::LOAD_FAST: {
                uint16_t slot = read_u16(pc + 1);
                inst.op = IrOpcode::LoadFast;
                inst.slot = slot;
                inst.result_type = ir_type_for_slot(function_.metadata, slot);
                inst.result = fresh_reg(inst.result_type);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 3;
                break;
            }
            case Opcode::STORE_FAST: {
                uint16_t slot = read_u16(pc + 1);
                inst.op = IrOpcode::StoreFast;
                inst.slot = slot;
                inst.result = pop();
                ir.instructions.push_back(inst);
                pc += 3;
                break;
            }
            case Opcode::ADD_I32:
                inst.op = IrOpcode::AddI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::ADD_F64:
                inst.op = IrOpcode::AddF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::SUB_I32:
                inst.op = IrOpcode::SubI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::SUB_F64:
                inst.op = IrOpcode::SubF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::MUL_I32:
                inst.op = IrOpcode::MulI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::MUL_F64:
                inst.op = IrOpcode::MulF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::DIV_F64:
                inst.op = IrOpcode::DivF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::LT: {
                inst.op = IrOpcode::CmpLtI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            }
            case Opcode::ADD:
                inst.op = IrOpcode::GenericAdd;
                inst.result_type = IrType::Variant;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Variant);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::GET_ELEMENT:
                inst.op = IrOpcode::GenericGetElement;
                inst.result_type = IrType::Variant;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Variant);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 1;
                break;
            case Opcode::INTRINSIC_SYSCALL: {
                uint16_t id = (static_cast<uint16_t>(function_.module.code[pc + 1]) << 8) |
                              static_cast<uint16_t>(function_.module.code[pc + 2]);
                uint8_t argc = function_.module.code[pc + 3];
                for (int i = 0; i < argc; ++i) {
                    pop();
                }
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = id;
                inst.intrinsic_argc = argc;
                inst.result = fresh_reg(IrType::Variant);
                ir.instructions.push_back(inst);
                push(inst.result);
                pc += 4;
                break;
            }
            case Opcode::POP:
                pop();
                pc += 1;
                break;
            case Opcode::DUP:
                push(stack_.back());
                pc += 1;
                break;
            case Opcode::JUMP:
            case Opcode::JUMP_IF_FALSE:
            case Opcode::JUMP_IF_TRUE:
                if (op != Opcode::JUMP) {
                    pop();
                }
                pc = read_u16(pc + 1);
                break;
            default:
                throw DryadException(std::string("micro_ir: unsupported opcode ") +
                                     opcode_name(op));
        }
    }
};

} // namespace

IrType observed_to_ir_type(ObservedType type, TypeConfidence confidence) {
    if (confidence == TypeConfidence::UNKNOWN) {
        return IrType::Variant;
    }
    switch (type) {
        case ObservedType::I32:  return IrType::I32;
        case ObservedType::F64:  return IrType::F64;
        case ObservedType::Bool: return IrType::Bool;
        default:                 return IrType::Variant;
    }
}

IrModule bytecode_to_micro_ir(const CompiledProgram& program) {
    IrModule module;
    for (const auto& function : program.functions) {
        module.functions.push_back(BytecodeToIr(function).build());
    }
    return module;
}

} // namespace dryad
