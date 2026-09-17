#include "dryad/compiler/execution_engine.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/program_entry.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/class.h"
#include "dryad/runtime/vm.h"
#include "dryad/runtime/vm_jit_delegate.h"
#include "dryad/runtime/vm_runtime_delegate.h"
#include "dryad/common/utils.h"

namespace dryad {

namespace {

class ExecutionVmRuntime final : public VmRuntimeDelegate {
public:
    explicit ExecutionVmRuntime(Interpreter* interpreter) : interpreter_(interpreter) {}

    Value get_global(const std::string& name) override {
        return interpreter_->global_env()->get(name);
    }

    Value get_property(const Value& object, const std::string& name) override {
        if (object.is_instance()) {
            auto instance = object.as_instance();
            auto getter = instance->klass->find_getter(name);
            if (getter) {
                auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(getter);
                if (dryad_func) {
                    return dryad_func->call_with_receiver(interpreter_, {}, instance);
                }
            }
            return instance->get(name);
        }
        if (object.is_object()) {
            return object.object_get(name);
        }
        throw DryadException("GET_PROPERTY requires object or instance");
    }

    void set_property(const Value& object, const std::string& name, const Value& value) override {
        if (object.is_instance()) {
            auto instance = object.as_instance();
            auto setter = instance->klass->find_setter(name);
            if (setter) {
                auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(setter);
                if (dryad_func) {
                    dryad_func->call_with_receiver(interpreter_, {value}, instance);
                    return;
                }
            }
            instance->set(name, value);
            return;
        }
        if (object.is_object()) {
            const_cast<Value&>(object).object_set(name, value);
            return;
        }
        throw DryadException("SET_PROPERTY requires object or instance");
    }

    Value invoke_method(const Value& receiver,
                        const std::string& method,
                        const std::vector<Value>& args) override {
        if (receiver.is_object()) {
            Value method_val = receiver.object_get(method);
            if (!method_val.is_function()) {
                throw DryadException("Cannot call non-function value: " + method);
            }
            auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(method_val.as_function());
            if (dryad_func) {
                return dryad_func->call(interpreter_, args);
            }
            return method_val.as_function()->call(interpreter_, args);
        }
        if (!receiver.is_instance()) {
            throw DryadException("METHOD_CALL requires object or instance receiver");
        }
        auto instance = receiver.as_instance();
        Value method_val = instance->get(method);
        if (!method_val.is_function()) {
            throw DryadException("Cannot call non-function value: " + method);
        }
        auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(method_val.as_function());
        if (dryad_func) {
            return dryad_func->call_with_receiver(interpreter_, args, instance);
        }
        return method_val.as_function()->call(interpreter_, args);
    }

    Value new_instance(const Value& class_value, const std::vector<Value>& args) override {
        if (!class_value.is_class()) {
            throw DryadException("NEW_INSTANCE requires class value");
        }
        return Value(class_value.as_class()->instantiate(args, interpreter_));
    }

    Value invoke_global(const std::string& name, const std::vector<Value>& args) override {
        Value fn_val = interpreter_->global_env()->get(name);
        if (!fn_val.is_function()) {
            throw DryadException("CALL_GLOBAL: not a function: " + name);
        }
        auto dryad_func = std::dynamic_pointer_cast<DryadFunction>(fn_val.as_function());
        if (dryad_func) {
            return dryad_func->call(interpreter_, args);
        }
        return fn_val.as_function()->call(interpreter_, args);
    }

private:
    Interpreter* interpreter_;
};

} // namespace

ExecutionEngine::ExecutionEngine(Interpreter* interpreter)
    : interpreter_(interpreter),
      default_mode_(ExecutionMode::Interpreter),
      runtime_adapter_(std::make_unique<ExecutionVmRuntime>(interpreter)) {}

ExecutionEngine::~ExecutionEngine() = default;

void ExecutionEngine::set_default_mode(ExecutionMode mode) {
    default_mode_ = mode;
}

void ExecutionEngine::set_source_path(const std::string& path) {
    source_path_ = path;
    dryadc_cache_.set_source_path(path);
    dryadc_cache_.try_load_companion();
    native_module_.try_load_companion(path);
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

bool ExecutionEngine::has_bytecode_function(const CompiledProgram& program,
                                            const std::string& name) const {
    for (const auto& function : program.functions) {
        if (function.metadata.name == name) {
            return true;
        }
    }
    return false;
}

uint16_t ExecutionEngine::find_bytecode_function_index(const CompiledProgram& program,
                                                     const std::string& name) const {
    for (size_t i = 0; i < program.functions.size(); ++i) {
        if (program.functions[i].metadata.name == name) {
            return static_cast<uint16_t>(i);
        }
    }
    throw DryadException("Function not found in compiled program: " + name);
}

void ExecutionEngine::execute_program(Program* program) {
    if (!program) {
        return;
    }
    active_program_ = program;

    push_scope_mode(ExecutionMode::Interpreter);
    for (auto& stmt : program->statements) {
        if (stmt->type == ASTNodeType::ImportDeclaration ||
            stmt->type == ASTNodeType::FunctionDeclaration ||
            stmt->type == ASTNodeType::ClassDeclaration ||
            stmt->type == ASTNodeType::InterfaceDeclaration ||
            stmt->type == ASTNodeType::ExportDeclaration ||
            stmt->type == ASTNodeType::IntrinsicDeclaration) {
            interpreter_->execute_statement(stmt.get());
        }
    }
    pop_scope_mode();

    if (default_mode_ == ExecutionMode::Bytecode) {
        const CompiledProgram& compiled = compiled_program_for(program);
        if (has_bytecode_function(compiled, kScriptEntryFunctionName)) {
            VM vm;
            vm.load_program(compiled, find_bytecode_function_index(compiled, kScriptEntryFunctionName));
            vm.set_runtime_delegate(runtime_adapter_.get());
            if (jit_delegate_) {
                jit_delegate_->bind_function(
                    &compiled.functions[find_bytecode_function_index(compiled, kScriptEntryFunctionName)]);
                vm.set_jit_delegate(jit_delegate_);
            }
            vm.enable_jit(jit_enabled_ && jit_delegate_ != nullptr);
            vm.execute();
            return;
        }
    }

    for (auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::ImportDeclaration &&
            stmt->type != ASTNodeType::FunctionDeclaration &&
            stmt->type != ASTNodeType::ClassDeclaration &&
            stmt->type != ASTNodeType::InterfaceDeclaration &&
            stmt->type != ASTNodeType::ExportDeclaration &&
            stmt->type != ASTNodeType::IntrinsicDeclaration) {
            interpreter_->execute_statement(stmt.get());
        }
    }
}

void ExecutionEngine::invalidate_bytecode_cache() {
    program_cache_.clear();
    dryadc_cache_.clear();
    native_module_.unload();
}

void ExecutionEngine::set_jit_delegate(VmJitDelegate* delegate) {
    jit_delegate_ = delegate;
}

CompiledProgram& ExecutionEngine::compiled_program_for(Program* program) {
    auto cached = program_cache_.find(program);
    if (cached != program_cache_.end()) {
        return cached->second;
    }

    BytecodeCompiler compiler;
    CompiledProgram compiled = compiler.compile_program(program, interpreter_->type_profiler());
    program_cache_[program] = compiled;
    return program_cache_[program];
}

const CompiledProgram& ExecutionEngine::resolve_bytecode_program() {
    if (dryadc_cache_.is_loaded()) {
        return dryadc_cache_.program();
    }
    if (!active_program_) {
        throw DryadException("Bytecode execution requires an active program context");
    }
    return compiled_program_for(active_program_);
}

Value ExecutionEngine::invoke_interpreter(DryadFunction* function,
                                          const std::vector<Value>& args,
                                          const std::shared_ptr<ClassInstance>& receiver) {
    return function->call_interpreted(interpreter_, args, receiver);
}

Value ExecutionEngine::invoke_bytecode_program(const CompiledProgram& program,
                                               FunctionDeclaration* decl,
                                               const std::vector<Value>& args) {
    const uint16_t function_index = find_function_index(program, decl->name);

    VM vm;
    vm.load_program(program, function_index);
    if (jit_delegate_) {
        jit_delegate_->bind_function(&program.functions[function_index]);
        vm.set_jit_delegate(jit_delegate_);
    }
    vm.enable_jit(jit_enabled_ && jit_delegate_ != nullptr);
    vm.set_runtime_delegate(runtime_adapter_.get());

    for (size_t i = 0; i < args.size(); ++i) {
        vm.set_slot(static_cast<uint16_t>(i), args[i]);
    }

    vm.execute();
    if (vm.stack_size() == 0) {
        return Value();
    }
    return vm.get_stack_top();
}

Value ExecutionEngine::invoke_bytecode(FunctionDeclaration* decl, const std::vector<Value>& args) {
    return invoke_bytecode_program(resolve_bytecode_program(), decl, args);
}

Value ExecutionEngine::invoke_aot(FunctionDeclaration* decl, const std::vector<Value>& args) {
    if (native_module_.is_loaded() && native_module_.has_function(decl->name)) {
        return native_module_.invoke(decl->name, args);
    }

    if (dryadc_cache_.is_loaded()) {
        return invoke_bytecode_program(dryadc_cache_.program(), decl, args);
    }

    if (active_program_) {
        return invoke_bytecode(decl, args);
    }

    throw DryadException(
        "AOT execution requires a companion .dryadc or .so module. "
        "Use: dryad compile input.dryad -o input.dryadc "
        "or dryad compile --aot --link -o input.so input.dryad");
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
        case ExecutionMode::Bytecode: {
            const CompiledProgram& program = active_program_
                                                 ? compiled_program_for(active_program_)
                                                 : resolve_bytecode_program();
            if (receiver || !has_bytecode_function(program, decl->name)) {
                if (!function) {
                    throw DryadException("ExecutionEngine: interpreter fallback requires function object");
                }
                return invoke_interpreter(function, args, receiver);
            }
            return invoke_bytecode_program(program, decl, args);
        }
        case ExecutionMode::AOT:
            if (receiver) {
                if (!function) {
                    throw DryadException("ExecutionEngine: interpreter fallback requires function object");
                }
                return invoke_interpreter(function, args, receiver);
            }
            return invoke_aot(decl, args);
        case ExecutionMode::Inherit:
            break;
    }

    if (!function) {
        throw DryadException("ExecutionEngine: interpreter invocation requires function object");
    }
    return invoke_interpreter(function, args, receiver);
}

} // namespace dryad
