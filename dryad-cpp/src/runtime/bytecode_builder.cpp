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

void BytecodeBuilder::emit_load_global(uint16_t name_const) {
    emit_u16(Opcode::LOAD_GLOBAL, name_const);
}

void BytecodeBuilder::emit_make_function(uint16_t function_index) {
    emit_u16(Opcode::MAKE_FUNCTION, function_index);
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

void BytecodeBuilder::emit_call(uint16_t function_index, uint8_t argc) {
    emit(Opcode::CALL);
    write_u16(function_index);
    code_.push_back(argc);
}

void BytecodeBuilder::emit_ret() {
    emit(Opcode::RET);
}

void BytecodeBuilder::emit_new_array() {
    emit(Opcode::NEW_ARRAY);
}

void BytecodeBuilder::emit_array_push() {
    emit(Opcode::ARRAY_PUSH);
}

void BytecodeBuilder::emit_set_element() {
    emit(Opcode::SET_ELEMENT);
}

void BytecodeBuilder::emit_get_property(uint16_t name_const) {
    emit_u16(Opcode::GET_PROPERTY, name_const);
}

void BytecodeBuilder::emit_set_property(uint16_t name_const) {
    emit_u16(Opcode::SET_PROPERTY, name_const);
}

void BytecodeBuilder::emit_method_call(uint16_t name_const, uint8_t argc) {
    emit(Opcode::METHOD_CALL);
    write_u16(name_const);
    code_.push_back(argc);
}

void BytecodeBuilder::emit_new_instance(uint16_t class_name_const, uint8_t argc) {
    emit(Opcode::NEW_INSTANCE);
    write_u16(class_name_const);
    code_.push_back(argc);
}

void BytecodeBuilder::emit_call_global(uint16_t name_const, uint8_t argc) {
    emit(Opcode::CALL_GLOBAL);
    write_u16(name_const);
    code_.push_back(argc);
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
