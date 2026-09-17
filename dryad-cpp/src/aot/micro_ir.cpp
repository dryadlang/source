#include "dryad/aot/micro_ir.h"
#include "dryad/runtime/opcode.h"
#include "dryad/common/utils.h"
#include <cstring>
#include <map>
#include <set>

namespace dryad {

namespace {

size_t opcode_length(Opcode op) {
    switch (op) {
        case Opcode::PUSH_INT:
        case Opcode::PUSH_FLOAT:
            return 9;
        case Opcode::PUSH_CONST:
        case Opcode::LOAD_FAST:
        case Opcode::STORE_FAST:
        case Opcode::LOAD_GLOBAL:
        case Opcode::JUMP:
        case Opcode::JUMP_IF_FALSE:
        case Opcode::JUMP_IF_TRUE:
        case Opcode::GET_PROPERTY:
        case Opcode::SET_PROPERTY:
            return 3;
        case Opcode::CALL:
        case Opcode::METHOD_CALL:
        case Opcode::NEW_INSTANCE:
        case Opcode::CALL_GLOBAL:
        case Opcode::INTRINSIC_SYSCALL:
            return 4;
        default:
            return 1;
    }
}

void find_block_leaders(const std::vector<uint8_t>& code, std::set<size_t>& leaders) {
    leaders.insert(0);
    size_t pc = 0;
    while (pc < code.size()) {
        Opcode op = static_cast<Opcode>(code[pc]);
        const size_t len = opcode_length(op);
        if (op == Opcode::JUMP || op == Opcode::JUMP_IF_FALSE || op == Opcode::JUMP_IF_TRUE) {
            const uint16_t target = static_cast<uint16_t>(code[pc + 1]) |
                                    (static_cast<uint16_t>(code[pc + 2]) << 8);
            leaders.insert(target);
            leaders.insert(pc + len);
        }
        pc += len;
    }
}

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
        block_entry_stacks_.clear();
        block_entry_type_stacks_.clear();
        block_stack_known_.clear();
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
        std::set<size_t> leader_set;
        find_block_leaders(code, leader_set);

        std::vector<size_t> leaders(leader_set.begin(), leader_set.end());
        std::map<size_t, uint32_t> pc_to_block;
        for (size_t i = 0; i < leaders.size(); ++i) {
            pc_to_block[leaders[i]] = static_cast<uint32_t>(i);
        }

        ir.blocks.resize(leaders.size());
        for (size_t i = 0; i < leaders.size(); ++i) {
            ir.blocks[i].id = static_cast<uint32_t>(i);
            ir.blocks[i].bytecode_pc = static_cast<uint32_t>(leaders[i]);
        }
        ir.entry_block = 0;

        for (size_t block_index = 0; block_index < leaders.size(); ++block_index) {
            emit_block(static_cast<uint32_t>(block_index), leaders, pc_to_block, ir);
        }

        for (const auto& block : ir.blocks) {
            for (const auto& instruction : block.instructions) {
                if (instruction.op == IrOpcode::StoreFast &&
                    instruction.slot < ir.slot_types.size()) {
                    ir.slot_types[instruction.slot] = instruction.result_type;
                }
            }
        }

        ir.string_literals = string_literals_;
        flatten_ir_function(ir);
        return ir;
    }

private:
    const CompiledFunction& function_;
    std::vector<int> stack_;
    std::vector<IrType> stack_types_;
    std::vector<std::string> string_literals_;
    std::vector<std::vector<int>> block_entry_stacks_;
    std::vector<std::vector<IrType>> block_entry_type_stacks_;
    std::vector<bool> block_stack_known_;
    int next_reg_ = 0;

    void record_block_entry_stack(uint32_t block_index) {
        if (block_index >= block_entry_stacks_.size()) {
            block_entry_stacks_.resize(block_index + 1);
            block_entry_type_stacks_.resize(block_index + 1);
            block_stack_known_.resize(block_index + 1);
        }
        block_entry_stacks_[block_index] = stack_;
        block_entry_type_stacks_[block_index] = stack_types_;
        block_stack_known_[block_index] = true;
    }

    uint16_t intern_string(const std::string& text) {
        for (size_t i = 0; i < string_literals_.size(); ++i) {
            if (string_literals_[i] == text) {
                return static_cast<uint16_t>(i);
            }
        }
        string_literals_.push_back(text);
        return static_cast<uint16_t>(string_literals_.size() - 1);
    }

    int fresh_reg(IrType type) {
        (void)type;
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

    void push(int reg, IrType type) {
        stack_.push_back(reg);
        stack_types_.push_back(type);
    }

    int pop(IrType* type_out = nullptr) {
        if (stack_.empty()) {
            throw DryadException("micro_ir: stack underflow during bytecode lowering");
        }
        int value = stack_.back();
        stack_.pop_back();
        if (type_out && !stack_types_.empty()) {
            *type_out = stack_types_.back();
            stack_types_.pop_back();
        }
        return value;
    }

    void emit_block(uint32_t block_index,
                    const std::vector<size_t>& leaders,
                    const std::map<size_t, uint32_t>& pc_to_block,
                    IrFunction& ir) {
        const auto& code = function_.module.code;
        size_t pc = leaders[block_index];
        const size_t end_pc = (block_index + 1 < leaders.size())
                                  ? leaders[block_index + 1]
                                  : code.size();
        IrBasicBlock& block = ir.blocks[block_index];
        if (block_index == 0) {
            stack_.clear();
            stack_types_.clear();
        } else if (block_index < block_stack_known_.size() &&
                   block_stack_known_[block_index]) {
            stack_ = block_entry_stacks_[block_index];
            stack_types_ = block_entry_type_stacks_[block_index];
        } else {
            stack_.clear();
            stack_types_.clear();
        }

        while (pc < end_pc) {
            Opcode op = static_cast<Opcode>(code[pc]);
            if (op == Opcode::HALT || op == Opcode::RET) {
                if (!stack_.empty()) {
                    IrInstruction ret;
                    ret.op = IrOpcode::Return;
                    ret.result = stack_.back();
                    ret.result_type = stack_types_.back();
                    block.instructions.push_back(ret);
                } else {
                    IrInstruction ret;
                    ret.op = IrOpcode::Return;
                    ret.result = -1;
                    ret.result_type = IrType::Variant;
                    block.instructions.push_back(ret);
                }
                return;
            }
            if (op == Opcode::JUMP || op == Opcode::JUMP_IF_FALSE || op == Opcode::JUMP_IF_TRUE) {
                emit_jump(op, pc, block_index, block, pc_to_block);
                return;
            }
            emit_opcode(op, pc, block);
        }

        if (block_index + 1 < leaders.size()) {
            IrInstruction br;
            br.op = IrOpcode::Br;
            br.branch_target = static_cast<uint32_t>(block_index + 1);
            block.instructions.push_back(br);
            record_block_entry_stack(static_cast<uint32_t>(block_index + 1));
        }
    }

    void emit_jump(Opcode op,
                   size_t pc,
                   uint32_t block_index,
                   IrBasicBlock& block,
                   const std::map<size_t, uint32_t>& pc_to_block) {
        const uint16_t target_pc = read_u16(pc + 1);
        const auto target_it = pc_to_block.find(target_pc);
        if (target_it == pc_to_block.end()) {
            throw DryadException("micro_ir: jump target is not a basic block leader");
        }
        const uint32_t target_block = target_it->second;
        const uint32_t fallthrough_block = block_index + 1;

        if (op == Opcode::JUMP) {
            IrInstruction br;
            br.op = IrOpcode::Br;
            br.branch_target = target_block;
            block.instructions.push_back(br);
            record_block_entry_stack(target_block);
        } else {
            IrType cond_type = IrType::Bool;
            const int cond = pop(&cond_type);
            IrInstruction br;
            br.op = IrOpcode::BrCond;
            br.lhs = cond;
            if (op == Opcode::JUMP_IF_FALSE) {
                br.branch_target = fallthrough_block;
                br.branch_false_target = target_block;
            } else {
                br.branch_target = target_block;
                br.branch_false_target = fallthrough_block;
            }
            block.instructions.push_back(br);
            record_block_entry_stack(fallthrough_block);
            record_block_entry_stack(target_block);
        }
    }

    void emit_opcode(Opcode op, size_t& pc, IrBasicBlock& block) {
        IrInstruction inst;
        switch (op) {
            case Opcode::PUSH_INT:
                inst.op = IrOpcode::ConstI32;
                inst.result = fresh_reg(IrType::I32);
                inst.const_i32 = read_i64(pc + 1);
                block.instructions.push_back(inst);
                push(inst.result, IrType::I32);
                pc += 9;
                break;
            case Opcode::PUSH_FLOAT:
                inst.op = IrOpcode::ConstF64;
                inst.result = fresh_reg(IrType::F64);
                inst.const_f64 = read_f64(pc + 1);
                block.instructions.push_back(inst);
                push(inst.result, IrType::F64);
                pc += 9;
                break;
            case Opcode::PUSH_CONST: {
                const uint16_t const_index = read_u16(pc + 1);
                if (const_index >= function_.module.constants.size()) {
                    throw DryadException("micro_ir: PUSH_CONST index out of bounds");
                }
                const Value& constant = function_.module.constants[const_index];
                if (!constant.is_string()) {
                    throw DryadException("micro_ir: unsupported PUSH_CONST value type");
                }
                inst.op = IrOpcode::ConstString;
                inst.const_string_id = intern_string(constant.as_string());
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 3;
                break;
            }
            case Opcode::PUSH_TRUE:
            case Opcode::PUSH_FALSE:
                inst.op = IrOpcode::ConstBool;
                inst.result = fresh_reg(IrType::Bool);
                inst.const_bool = op == Opcode::PUSH_TRUE;
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::LOAD_FAST: {
                uint16_t slot = read_u16(pc + 1);
                inst.op = IrOpcode::LoadFast;
                inst.slot = slot;
                inst.result_type = ir_type_for_slot(function_.metadata, slot);
                inst.result = fresh_reg(inst.result_type);
                block.instructions.push_back(inst);
                push(inst.result, inst.result_type);
                pc += 3;
                break;
            }
            case Opcode::LOAD_GLOBAL: {
                const uint16_t name_index = read_u16(pc + 1);
                if (name_index < function_.module.constants.size() &&
                    function_.module.constants[name_index].is_string()) {
                    inst.const_string_id =
                        intern_string(function_.module.constants[name_index].as_string());
                }
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id =
                    static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(Opcode::LOAD_GLOBAL));
                inst.intrinsic_argc = 0;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 3;
                break;
            }
            case Opcode::STORE_FAST: {
                uint16_t slot = read_u16(pc + 1);
                IrType value_type = IrType::Variant;
                inst.op = IrOpcode::StoreFast;
                inst.slot = slot;
                inst.result = pop(&value_type);
                inst.result_type = value_type;
                block.instructions.push_back(inst);
                pc += 3;
                break;
            }
            case Opcode::ADD_I32:
                inst.op = IrOpcode::AddI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                block.instructions.push_back(inst);
                push(inst.result, IrType::I32);
                pc += 1;
                break;
            case Opcode::ADD_F64:
                inst.op = IrOpcode::AddF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                block.instructions.push_back(inst);
                push(inst.result, IrType::F64);
                pc += 1;
                break;
            case Opcode::SUB_I32:
                inst.op = IrOpcode::SubI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                block.instructions.push_back(inst);
                push(inst.result, IrType::I32);
                pc += 1;
                break;
            case Opcode::SUB_F64:
                inst.op = IrOpcode::SubF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                block.instructions.push_back(inst);
                push(inst.result, IrType::F64);
                pc += 1;
                break;
            case Opcode::MUL_I32:
                inst.op = IrOpcode::MulI32;
                inst.result_type = IrType::I32;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::I32);
                block.instructions.push_back(inst);
                push(inst.result, IrType::I32);
                pc += 1;
                break;
            case Opcode::SUB:
            case Opcode::MUL:
            case Opcode::DIV:
            case Opcode::MOD: {
                inst.rhs = pop();
                inst.lhs = pop();
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 2;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            }
            case Opcode::DIV_I32:
            case Opcode::NOT:
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = (op == Opcode::NOT) ? 1 : 2;
                if (op == Opcode::NOT) {
                    inst.lhs = pop();
                } else {
                    inst.rhs = pop();
                    inst.lhs = pop();
                }
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            case Opcode::MUL_F64:
                inst.op = IrOpcode::MulF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                block.instructions.push_back(inst);
                push(inst.result, IrType::F64);
                pc += 1;
                break;
            case Opcode::DIV_F64:
                inst.op = IrOpcode::DivF64;
                inst.result_type = IrType::F64;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::F64);
                block.instructions.push_back(inst);
                push(inst.result, IrType::F64);
                pc += 1;
                break;
            case Opcode::EQ:
                inst.op = IrOpcode::CmpEqI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::NEQ:
                inst.op = IrOpcode::CmpNeI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::LT:
                inst.op = IrOpcode::CmpLtI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::LTE:
                inst.op = IrOpcode::CmpLeI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::GT:
                inst.op = IrOpcode::CmpGtI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::GTE:
                inst.op = IrOpcode::CmpGeI32;
                inst.result_type = IrType::Bool;
                inst.rhs = pop();
                inst.lhs = pop();
                inst.result = fresh_reg(IrType::Bool);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Bool);
                pc += 1;
                break;
            case Opcode::ADD: {
                IrType rhs_type = IrType::Variant;
                IrType lhs_type = IrType::Variant;
                inst.rhs = pop(&rhs_type);
                inst.lhs = pop(&lhs_type);
                if (lhs_type == IrType::I32 && rhs_type == IrType::I32) {
                    inst.op = IrOpcode::AddI32;
                    inst.result_type = IrType::I32;
                    inst.result = fresh_reg(IrType::I32);
                    block.instructions.push_back(inst);
                    push(inst.result, IrType::I32);
                } else {
                    inst.op = IrOpcode::GenericAdd;
                    inst.result_type = IrType::Variant;
                    inst.result = fresh_reg(IrType::Variant);
                    block.instructions.push_back(inst);
                    push(inst.result, IrType::Variant);
                }
                pc += 1;
                break;
            }
            case Opcode::NEW_ARRAY: {
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 0;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            }
            case Opcode::ARRAY_PUSH: {
                inst.rhs = pop();
                inst.lhs = pop();
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 2;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            }
            case Opcode::SET_ELEMENT: {
                const int value = pop();
                const int index = pop();
                const int array = pop();
                inst.lhs = array;
                inst.rhs = index;
                inst.slot = static_cast<uint16_t>(value);
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 3;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            }
            case Opcode::GET_PROPERTY: {
                inst.lhs = pop();
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 1;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 3;
                break;
            }
            case Opcode::SET_PROPERTY: {
                inst.rhs = pop();
                inst.lhs = pop();
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 2;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 3;
                break;
            }
            case Opcode::METHOD_CALL:
            case Opcode::NEW_INSTANCE:
            case Opcode::CALL_GLOBAL: {
                uint8_t argc = function_.module.code[pc + 3];
                std::vector<int> args;
                std::vector<IrType> arg_types;
                for (uint8_t i = 0; i < argc; ++i) {
                    IrType arg_type = IrType::Variant;
                    args.push_back(pop(&arg_type));
                    arg_types.push_back(arg_type);
                }
                const uint16_t name_index = read_u16(pc + 1);
                if (name_index < function_.module.constants.size() &&
                    function_.module.constants[name_index].is_string()) {
                    inst.const_string_id =
                        intern_string(function_.module.constants[name_index].as_string());
                }
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = argc;
                if (!args.empty()) {
                    inst.rhs = args.back();
                    inst.result_type = arg_types.back();
                }
                if (args.size() > 1) {
                    inst.lhs = args[args.size() - 2];
                }
                if (op == Opcode::METHOD_CALL) {
                    inst.lhs = pop();
                }
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 4;
                break;
            }
            case Opcode::GET_ELEMENT: {
                inst.op = IrOpcode::GenericGetElement;
                inst.result_type = IrType::Variant;
                inst.rhs = pop();
                inst.lhs = pop();
                const int elem = fresh_reg(IrType::Variant);
                inst.result = elem;
                block.instructions.push_back(inst);
                IrInstruction unbox;
                unbox.op = IrOpcode::UnboxI32;
                unbox.lhs = elem;
                unbox.result = fresh_reg(IrType::I32);
                block.instructions.push_back(unbox);
                push(unbox.result, IrType::I32);
                pc += 1;
                break;
            }
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
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 4;
                break;
            }
            case Opcode::PUSH_NULL:
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | static_cast<uint8_t>(op));
                inst.intrinsic_argc = 0;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 1;
                break;
            case Opcode::POP:
                pop();
                pc += 1;
                break;
            case Opcode::DUP:
                push(stack_.back(), stack_types_.back());
                pc += 1;
                break;
            case Opcode::CALL: {
                uint16_t function_index = read_u16(pc + 1);
                uint8_t argc = function_.module.code[pc + 3];
                for (uint8_t i = 0; i < argc; ++i) {
                    pop();
                }
                inst.op = IrOpcode::IntrinsicSlow;
                inst.intrinsic_id = static_cast<uint16_t>(0xFF00u | function_index);
                inst.intrinsic_argc = argc;
                inst.result = fresh_reg(IrType::Variant);
                block.instructions.push_back(inst);
                push(inst.result, IrType::Variant);
                pc += 4;
                break;
            }
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

void flatten_ir_function(IrFunction& function) {
    function.instructions.clear();
    for (const auto& block : function.blocks) {
        for (const auto& inst : block.instructions) {
            function.instructions.push_back(inst);
        }
    }
}

IrFunction bytecode_function_to_micro_ir(const CompiledFunction& function) {
    return BytecodeToIr(function).build();
}

IrModule bytecode_to_micro_ir(const CompiledProgram& program) {
    IrModule module;
    for (const auto& function : program.functions) {
        module.functions.push_back(bytecode_function_to_micro_ir(function));
    }
    return module;
}

} // namespace dryad
