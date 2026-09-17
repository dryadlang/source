#ifndef DRYAD_RUNTIME_VM_JIT_DELEGATE_H
#define DRYAD_RUNTIME_VM_JIT_DELEGATE_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/loop_analyzer.h"
#include "dryad/runtime/value.h"
#include <cstdint>
#include <vector>

namespace dryad {

class VmJitDelegate {
public:
    virtual ~VmJitDelegate() = default;

    virtual void bind_function(const CompiledFunction* function) = 0;
    virtual void prepare_loops(const std::vector<LoopInfo>& loops) = 0;
    virtual void on_back_edge(size_t loop_index) = 0;
    virtual bool should_compile(size_t loop_index) const = 0;
    virtual bool try_compile_loop(size_t loop_index) = 0;
    virtual bool try_run_body(size_t loop_index, std::vector<Value>& slots) = 0;
    virtual size_t loop_index_for_body(uint16_t body_start_pc) const = 0;
    virtual size_t loop_index_for_back_edge(uint16_t back_edge_pc) const = 0;
    virtual bool is_body_ready(size_t loop_index) const = 0;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VM_JIT_DELEGATE_H
