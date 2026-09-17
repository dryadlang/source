#ifndef DRYAD_RUNTIME_BYTECODE_BUILDER_H
#define DRYAD_RUNTIME_BYTECODE_BUILDER_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/opcode.h"
#include <cstdint>
#include <vector>

namespace dryad {

class BytecodeBuilder {
public:
    uint16_t add_constant(const Value& value);
    void emit(Opcode op);
    void emit_u16(Opcode op, uint16_t operand);
    void emit_jump(Opcode op, size_t target_pc);
    void emit_push_int(int64_t value);
    void emit_push_float(double value);
    void emit_push_const(uint16_t index);
    void emit_load_fast(uint16_t slot);
    void emit_store_fast(uint16_t slot);
    void emit_load_global(uint16_t name_const);
    void emit_intrinsic_syscall(uint16_t intrinsic_id, uint8_t argc);
    void emit_halt();
    void emit_call(uint16_t function_index, uint8_t argc);
    void emit_ret();
    void emit_new_array();
    void emit_array_push();
    void emit_set_element();
    void emit_get_property(uint16_t name_const);
    void emit_set_property(uint16_t name_const);
    void emit_method_call(uint16_t name_const, uint8_t argc);
    void emit_new_instance(uint16_t class_name_const, uint8_t argc);
    void emit_call_global(uint16_t name_const, uint8_t argc);

    size_t current_pc() const { return code_.size(); }
    const std::vector<uint8_t>& code() const { return code_; }
    BytecodeModule build() const;

    size_t emit_jump_placeholder(Opcode op);
    void patch_u16(size_t operand_offset, uint16_t value);

private:
    std::vector<uint8_t> code_;
    std::vector<Value> constants_;

    void write_u16(uint16_t value);
    void write_i64(int64_t value);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_BYTECODE_BUILDER_H
