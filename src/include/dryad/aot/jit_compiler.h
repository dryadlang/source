#ifndef DRYAD_AOT_JIT_COMPILER_H
#define DRYAD_AOT_JIT_COMPILER_H

#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/loop_analyzer.h"
#include "dryad/runtime/vm_jit_delegate.h"
#include <cstdint>
#include <string>
#include <vector>

namespace dryad {

using JitLoopFn = void (*)(int64_t*);

class JitCompiler : public VmJitDelegate {
public:
    void configure(const std::string& aot_runtime_source, const std::string& include_dir);
    void set_function(const CompiledFunction* function);

    void bind_function(const CompiledFunction* function) override;
    void prepare_loops(const std::vector<LoopInfo>& loops) override;
    void on_back_edge(size_t loop_index) override;
    bool should_compile(size_t loop_index) const override;
    bool try_compile_loop(size_t loop_index) override;
    bool try_run_body(size_t loop_index, std::vector<Value>& slots) override;
    size_t loop_index_for_body(uint16_t body_start_pc) const override;
    size_t loop_index_for_back_edge(uint16_t back_edge_pc) const override;
    bool is_body_ready(size_t loop_index) const override;

    void set_threshold(uint32_t threshold) { threshold_ = threshold; }
    uint32_t threshold() const { return threshold_; }

private:
    struct JitLoopEntry {
        LoopInfo loop;
        uint32_t back_edge_hits = 0;
        JitLoopFn fn = nullptr;
        bool ready = false;
        void* module_handle = nullptr;
    };

    std::string runtime_source_;
    std::string include_dir_;
    uint32_t threshold_ = 1000;
    uint32_t compile_counter_ = 0;
    const CompiledFunction* function_ = nullptr;
    std::vector<JitLoopEntry> loops_;

    bool compile_loop(size_t loop_index);
};

} // namespace dryad

#endif // DRYAD_AOT_JIT_COMPILER_H
