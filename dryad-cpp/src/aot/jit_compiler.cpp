#include "dryad/aot/jit_compiler.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/aot/native_linker.h"
#include <dlfcn.h>
#include <cstdio>

namespace dryad {

namespace {

CompiledFunction make_body_fragment(const CompiledFunction& function, const LoopInfo& loop) {
    CompiledFunction fragment;
    fragment.metadata = function.metadata;
    fragment.module.constants = function.module.constants;
    fragment.module.code.assign(function.module.code.begin() + loop.body_start,
                                function.module.code.begin() + loop.body_end);
    return fragment;
}

} // namespace

void JitCompiler::configure(const std::string& aot_runtime_source,
                            const std::string& include_dir) {
    runtime_source_ = aot_runtime_source;
    include_dir_ = include_dir;
}

void JitCompiler::set_function(const CompiledFunction* function) {
    function_ = function;
}

void JitCompiler::bind_function(const CompiledFunction* function) {
    set_function(function);
}

void JitCompiler::prepare_loops(const std::vector<LoopInfo>& loops) {
    loops_.clear();
    for (const auto& loop : loops) {
        JitLoopEntry entry;
        entry.loop = loop;
        loops_.push_back(entry);
    }
}

void JitCompiler::on_back_edge(size_t loop_index) {
    if (loop_index < loops_.size()) {
        loops_[loop_index].back_edge_hits++;
    }
}

bool JitCompiler::should_compile(size_t loop_index) const {
    if (loop_index >= loops_.size() || loops_[loop_index].ready) {
        return false;
    }
    return loops_[loop_index].back_edge_hits >= threshold_;
}

bool JitCompiler::try_compile_loop(size_t loop_index) {
    return compile_loop(loop_index);
}

bool JitCompiler::try_run_body(size_t loop_index, std::vector<Value>& slots) {
    if (loop_index >= loops_.size() || !loops_[loop_index].ready ||
        loops_[loop_index].fn == nullptr) {
        return false;
    }

    std::vector<int64_t> buffer(slots.size(), 0);
    for (size_t i = 0; i < slots.size(); ++i) {
        if (slots[i].is_integer()) {
            buffer[i] = slots[i].as_integer();
        }
    }

    loops_[loop_index].fn(buffer.data());

    for (size_t i = 0; i < slots.size(); ++i) {
        if (slots[i].is_integer()) {
            slots[i] = Value(buffer[i]);
        }
    }
    return true;
}

size_t JitCompiler::loop_index_for_body(uint16_t body_start_pc) const {
    for (size_t i = 0; i < loops_.size(); ++i) {
        if (loops_[i].ready && loops_[i].loop.body_start == body_start_pc) {
            return i;
        }
    }
    return loops_.size();
}

size_t JitCompiler::loop_index_for_back_edge(uint16_t back_edge_pc) const {
    for (size_t i = 0; i < loops_.size(); ++i) {
        if (loops_[i].loop.back_edge_pc == back_edge_pc) {
            return i;
        }
    }
    return loops_.size();
}

bool JitCompiler::is_body_ready(size_t loop_index) const {
    return loop_index < loops_.size() && loops_[loop_index].ready;
}

bool JitCompiler::compile_loop(size_t loop_index) {
    if (loop_index >= loops_.size() || !function_ || runtime_source_.empty() ||
        include_dir_.empty()) {
        return false;
    }
    if (!clang_available()) {
        return false;
    }

    JitLoopEntry& entry = loops_[loop_index];
    if (entry.ready) {
        return true;
    }

    const CompiledFunction fragment = make_body_fragment(*function_, entry.loop);
    IrFunction ir;
    try {
        ir = bytecode_function_to_micro_ir(fragment);
    } catch (...) {
        return false;
    }

    const std::string symbol = "dryad_jit_loop_" + std::to_string(compile_counter_++);
    const std::string llvm_path = "/tmp/" + symbol + ".ll";
    const std::string so_path = "/tmp/" + symbol + ".so";

    if (!write_jit_loop_ir_file(ir, symbol, llvm_path)) {
        return false;
    }

    NativeLinkResult link = link_native_shared_object(
        llvm_path, so_path, runtime_source_, include_dir_);
    std::remove(llvm_path.c_str());
    if (!link.success) {
        return false;
    }

    void* handle = dlopen(so_path.c_str(), RTLD_LAZY | RTLD_LOCAL);
    if (!handle) {
        return false;
    }

    void* sym = dlsym(handle, symbol.c_str());
    if (!sym) {
        dlclose(handle);
        return false;
    }

    entry.fn = reinterpret_cast<JitLoopFn>(sym);
    entry.module_handle = handle;
    entry.ready = true;
    return true;
}

} // namespace dryad
