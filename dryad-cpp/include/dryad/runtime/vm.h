#ifndef DRYAD_RUNTIME_VM_H
#define DRYAD_RUNTIME_VM_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/opcode.h"
#include "dryad/runtime/value.h"
#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

class VM {
public:
    VM();

    void load(const BytecodeModule& module);
    void load_bytecode(const std::vector<uint8_t>& bytecode);
    void set_constants(const std::vector<Value>& constants);

    void execute();
    void reset();

    size_t stack_size() const { return stack_.size(); }
    size_t bytecode_size() const { return bytecode_.size(); }
    size_t program_counter() const { return pc_; }
    bool halted() const { return halted_; }

    Value get_stack_top() const;
    const std::vector<Value>& stack() const { return stack_; }
    const std::vector<Value>& slots() const { return slots_; }

    void push_stack(const Value& value);
    void clear_stack() { stack_.clear(); }
    void set_slot(uint16_t index, const Value& value);

private:
    std::vector<Value> stack_;
    std::vector<uint8_t> bytecode_;
    std::vector<Value> constants_;
    std::vector<Value> slots_;
    size_t pc_;
    bool halted_;

    void execute_impl();
    void dispatch_opcode(Opcode op);
    void handle_intrinsic_syscall();

    void validate_stack(size_t required, const char* context);
    uint16_t read_u16_at(size_t offset) const;
    int64_t read_i64_at(size_t offset) const;
    double read_f64_at(size_t offset) const;
    void advance(size_t bytes);

    static double as_f64(const Value& value);
    static int64_t as_i32_slot(const Value& value);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VM_H
