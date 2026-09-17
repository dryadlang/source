#ifndef DRYAD_COMPILER_EXECUTION_ENGINE_H
#define DRYAD_COMPILER_EXECUTION_ENGINE_H

#include "dryad/compiler/ast.h"
#include "dryad/compiler/execution_mode.h"
#include "dryad/runtime/bytecode_cache.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/native_module.h"
#include "dryad/runtime/value.h"
#include <memory>
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

class Interpreter;
class DryadFunction;
class ClassInstance;
class VmJitDelegate;
class VmRuntimeDelegate;

class ExecutionEngine {
public:
    explicit ExecutionEngine(Interpreter* interpreter);
    ~ExecutionEngine();

    void set_default_mode(ExecutionMode mode);
    ExecutionMode default_mode() const { return default_mode_; }

    void set_source_path(const std::string& path);
    void push_scope_mode(ExecutionMode mode);
    void pop_scope_mode();

    ExecutionMode effective_mode(FunctionDeclaration* decl) const;
    void execute_program(Program* program);

    Value invoke_function(FunctionDeclaration* decl,
                          DryadFunction* function,
                          const std::vector<Value>& args,
                          const std::shared_ptr<ClassInstance>& receiver = nullptr);

    void invalidate_bytecode_cache();

    void set_jit_delegate(VmJitDelegate* delegate);
    void enable_jit(bool enabled) { jit_enabled_ = enabled; }

private:
    Interpreter* interpreter_;
    ExecutionMode default_mode_;
    std::vector<ExecutionMode> scope_stack_;
    Program* active_program_ = nullptr;
    std::string source_path_;
    BytecodeModuleCache dryadc_cache_;
    NativeModule native_module_;
    std::unordered_map<Program*, CompiledProgram> program_cache_;
    VmJitDelegate* jit_delegate_ = nullptr;
    bool jit_enabled_ = false;
    std::unique_ptr<VmRuntimeDelegate> runtime_adapter_;

    ExecutionMode current_scope_mode() const;
    CompiledProgram& compiled_program_for(Program* program);
    const CompiledProgram& resolve_bytecode_program();
    Value invoke_interpreter(DryadFunction* function,
                             const std::vector<Value>& args,
                             const std::shared_ptr<ClassInstance>& receiver);
    Value invoke_bytecode(FunctionDeclaration* decl, const std::vector<Value>& args);
    Value invoke_bytecode_program(const CompiledProgram& program,
                                  FunctionDeclaration* decl,
                                  const std::vector<Value>& args);
    Value invoke_aot(FunctionDeclaration* decl, const std::vector<Value>& args);

    bool has_bytecode_function(const CompiledProgram& program, const std::string& name) const;
    uint16_t find_bytecode_function_index(const CompiledProgram& program,
                                          const std::string& name) const;
};

} // namespace dryad

#endif // DRYAD_COMPILER_EXECUTION_ENGINE_H
