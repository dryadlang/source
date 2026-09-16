#include "dryad/runtime/bytecode_builder.h"
#include <cstring>

namespace dryad {

uint16_t BytecodeBuilder::add_constant(const Value& value) {
    constants_.push_back(value);
    return static_cast<uint16_t>(constants_.size() - 1);
}

void BytecodeBuilder::emit(Opcode op) {
    code_.push_back(static_cast<uint8_t>(op));
}

void BytecodeBuilder::write_u16(uint16_t value) {
    code_.push_back(static_cast<uint8_t>(value & 0xFF));
    code_.push_back(static_cast<uint8_t>((value >> 8) & 0xFF));
}

void BytecodeBuilder::write_i64(int64_t value) {
    for (int i = 0; i < 8; ++i) {
        code_.push_back(static_cast<uint8_t>((value >> (i * 8)) & 0xFF));
    }
}

void BytecodeBuilder::emit_u16(Opcode op, uint16_t operand) {
    emit(op);
    write_u16(operand);
}

void BytecodeBuilder::emit_jump(Opcode op, size_t target_pc) {
    emit_u16(op, static_cast<uint16_t>(target_pc));
}

void BytecodeBuilder::emit_push_int(int64_t value) {
    emit(Opcode::PUSH_INT);
    write_i64(value);
}

void BytecodeBuilder::emit_push_float(double value) {
    emit(Opcode::PUSH_FLOAT);
    int64_t bits = 0;
    std::memcpy(&bits, &value, sizeof(double));
    write_i64(bits);
}

void BytecodeBuilder::emit_push_const(uint16_t index) {
    emit_u16(Opcode::PUSH_CONST, index);
}

void BytecodeBuilder::emit_load_fast(uint16_t slot) {
    emit_u16(Opcode::LOAD_FAST, slot);
}

void BytecodeBuilder::emit_store_fast(uint16_t slot) {
    emit_u16(Opcode::STORE_FAST, slot);
}

void BytecodeBuilder::emit_intrinsic_syscall(uint16_t intrinsic_id, uint8_t argc) {
    emit(Opcode::INTRINSIC_SYSCALL);
    code_.push_back(static_cast<uint8_t>((intrinsic_id >> 8) & 0xFF));
    code_.push_back(static_cast<uint8_t>(intrinsic_id & 0xFF));
    code_.push_back(argc);
}

void BytecodeBuilder::emit_halt() {
    emit(Opcode::HALT);
}

size_t BytecodeBuilder::emit_jump_placeholder(Opcode op) {
    emit(op);
    const size_t operand_offset = code_.size();
    write_u16(0);
    return operand_offset;
}

void BytecodeBuilder::patch_u16(size_t operand_offset, uint16_t value) {
    code_[operand_offset] = static_cast<uint8_t>(value & 0xFF);
    code_[operand_offset + 1] = static_cast<uint8_t>((value >> 8) & 0xFF);
}

BytecodeModule BytecodeBuilder::build() const {
    return BytecodeModule{code_, constants_};
}

} // namespace dryad
