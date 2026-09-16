#ifndef DRYAD_COMPILER_EXECUTION_ENGINE_H
#define DRYAD_COMPILER_EXECUTION_ENGINE_H

#include "dryad/compiler/ast.h"
#include "dryad/compiler/execution_mode.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/runtime/value.h"
#include <memory>
#include <unordered_map>
#include <vector>

namespace dryad {

class Interpreter;
class DryadFunction;
class ClassInstance;

class ExecutionEngine {
public:
    explicit ExecutionEngine(Interpreter* interpreter);

    void set_default_mode(ExecutionMode mode);
    ExecutionMode default_mode() const { return default_mode_; }

    void push_scope_mode(ExecutionMode mode);
    void pop_scope_mode();

    ExecutionMode effective_mode(FunctionDeclaration* decl) const;
    void execute_program(Program* program);

    Value invoke_function(FunctionDeclaration* decl,
                          DryadFunction* function,
                          const std::vector<Value>& args,
                          const std::shared_ptr<ClassInstance>& receiver = nullptr);

    void invalidate_bytecode_cache();

private:
    Interpreter* interpreter_;
    ExecutionMode default_mode_;
    std::vector<ExecutionMode> scope_stack_;
    std::unordered_map<FunctionDeclaration*, CompiledFunction> bytecode_cache_;

    ExecutionMode current_scope_mode() const;
    const CompiledFunction& compiled_for(FunctionDeclaration* decl);
    Value invoke_interpreter(DryadFunction* function,
                             const std::vector<Value>& args,
                             const std::shared_ptr<ClassInstance>& receiver);
    Value invoke_bytecode(FunctionDeclaration* decl, const std::vector<Value>& args);
};

} // namespace dryad

#endif // DRYAD_COMPILER_EXECUTION_ENGINE_H
