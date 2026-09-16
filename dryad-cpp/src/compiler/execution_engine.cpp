#include "dryad/compiler/execution_engine.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/vm.h"
#include "dryad/common/utils.h"

namespace dryad {

ExecutionEngine::ExecutionEngine(Interpreter* interpreter)
    : interpreter_(interpreter), default_mode_(ExecutionMode::Interpreter) {}

void ExecutionEngine::set_default_mode(ExecutionMode mode) {
    default_mode_ = mode;
}

void ExecutionEngine::push_scope_mode(ExecutionMode mode) {
    scope_stack_.push_back(mode);
}

void ExecutionEngine::pop_scope_mode() {
    if (!scope_stack_.empty()) {
        scope_stack_.pop_back();
    }
}

ExecutionMode ExecutionEngine::current_scope_mode() const {
    if (!scope_stack_.empty()) {
        return scope_stack_.back();
    }
    return default_mode_;
}

ExecutionMode ExecutionEngine::effective_mode(FunctionDeclaration* decl) const {
    if (decl && decl->compile_mode != ExecutionMode::Inherit) {
        return decl->compile_mode;
    }
    return current_scope_mode();
}

void ExecutionEngine::execute_program(Program* program) {
    if (!program) {
        return;
    }
    for (auto& stmt : program->statements) {
        interpreter_->execute_statement(stmt.get());
    }
}

void ExecutionEngine::invalidate_bytecode_cache() {
    bytecode_cache_.clear();
}

const CompiledFunction& ExecutionEngine::compiled_for(FunctionDeclaration* decl) {
    auto cached = bytecode_cache_.find(decl);
    if (cached != bytecode_cache_.end()) {
        return cached->second;
    }

    const FunctionProfile* profile = nullptr;
    for (const auto& entry : interpreter_->type_profiler().profiles()) {
        if (entry.first == decl) {
            profile = &entry.second;
            break;
        }
    }

    BytecodeCompiler compiler;
    CompiledFunction compiled = compiler.compile_function(decl, profile);
    bytecode_cache_[decl] = compiled;
    return bytecode_cache_[decl];
}

Value ExecutionEngine::invoke_interpreter(DryadFunction* function,
                                          const std::vector<Value>& args,
                                          const std::shared_ptr<ClassInstance>& receiver) {
    return function->call_interpreted(interpreter_, args, receiver);
}

Value ExecutionEngine::invoke_bytecode(FunctionDeclaration* decl, const std::vector<Value>& args) {
    const CompiledFunction& compiled = compiled_for(decl);
    VM vm;
    vm.load(compiled.module);

    for (size_t i = 0; i < args.size(); ++i) {
        vm.set_slot(static_cast<uint16_t>(i), args[i]);
    }

    vm.execute();
    if (vm.stack_size() == 0) {
        return Value();
    }
    return vm.get_stack_top();
}

Value ExecutionEngine::invoke_function(FunctionDeclaration* decl,
                                       DryadFunction* function,
                                       const std::vector<Value>& args,
                                       const std::shared_ptr<ClassInstance>& receiver) {
    if (!decl) {
        throw DryadException("ExecutionEngine: invalid function invocation");
    }

    if (decl->is_async) {
        if (!function) {
            throw DryadException("ExecutionEngine: async functions require interpreter invocation");
        }
        return invoke_interpreter(function, args, receiver);
    }

    switch (effective_mode(decl)) {
        case ExecutionMode::Interpreter:
            break;
        case ExecutionMode::Bytecode:
            if (receiver) {
                throw DryadException("Bytecode mode does not support method calls with receiver yet");
            }
            return invoke_bytecode(decl, args);
        case ExecutionMode::AOT:
            throw DryadException(
                "AOT runtime execution requires a precompiled native module. "
                "Use: dryad compile --aot -o output.ll input.dryad");
        case ExecutionMode::Inherit:
            break;
    }

    if (!function) {
        throw DryadException("ExecutionEngine: interpreter invocation requires function object");
    }
    return invoke_interpreter(function, args, receiver);
}

} // namespace dryad
