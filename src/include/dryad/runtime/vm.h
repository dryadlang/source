#ifndef DRYAD_RUNTIME_VM_H
#define DRYAD_RUNTIME_VM_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/loop_analyzer.h"
#include "dryad/runtime/opcode.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/vm_jit_delegate.h"
#include "dryad/runtime/vm_runtime_delegate.h"
#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

struct VmCallFrame {
    size_t return_pc = 0;
    std::vector<Value> slots;
    size_t function_index = 0;
};

class VM {
public:
    VM();

    void load(const BytecodeModule& module);
    void load_program(const CompiledProgram& program, size_t entry_index);
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

    void set_jit_delegate(VmJitDelegate* delegate);
    void set_runtime_delegate(VmRuntimeDelegate* delegate);
    void enable_jit(bool enabled) { jit_enabled_ = enabled; }
    bool jit_enabled() const { return jit_enabled_; }

private:
    std::vector<Value> stack_;
    std::vector<uint8_t> bytecode_;
    std::vector<Value> constants_;
    std::vector<Value> slots_;
    const CompiledProgram* program_ = nullptr;
    size_t current_function_index_ = 0;
    std::vector<VmCallFrame> frames_;
    size_t pc_;
    bool halted_;
    bool jit_enabled_ = false;
    VmJitDelegate* jit_delegate_ = nullptr;
    VmRuntimeDelegate* runtime_delegate_ = nullptr;
    std::vector<LoopInfo> loops_;

    void execute_impl();
    bool try_run_jit_body();
    void handle_back_edge(uint16_t back_edge_pc, uint16_t target_pc);
    void dispatch_opcode(Opcode op);
    void handle_intrinsic_syscall();
    void activate_function(size_t function_index);
    void enter_function(size_t function_index, uint8_t argc);
    void return_from_function();

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
