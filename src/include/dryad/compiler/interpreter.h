#ifndef DRYAD_COMPILER_INTERPRETER_H
#define DRYAD_COMPILER_INTERPRETER_H

#include "dryad/compiler/ast.h"
#include "dryad/compiler/execution_mode.h"
#include "dryad/compiler/type_profiler.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/environment.h"
#include "dryad/runtime/module_loader.h"
#include <memory>
#include <functional>
#include <string>
#include <unordered_map>
#include <vector>

namespace dryad {

class BreakException : public std::exception {
public:
    const char* what() const noexcept override { return "break"; }
};

class ContinueException : public std::exception {
public:
    const char* what() const noexcept override { return "continue"; }
};

class ReturnException : public std::exception {
public:
    Value value;
    explicit ReturnException(Value val) : value(std::move(val)) {}
    const char* what() const noexcept override { return "return"; }
};

class ExpressionEvaluator;
class StatementExecutor;
class ExecutionEngine;

class DryadClass;

class Interpreter {
public:
    static constexpr size_t kMaxCallDepth = 256;

    Interpreter();
    ~Interpreter();
    
    void execute(Program* program);
    void run_file(const std::string& path);
    Value evaluate(Expression* expr);
    void execute_statement(Statement* stmt);
    void execute_block(const std::vector<std::unique_ptr<Statement>>& statements,
                      std::shared_ptr<Environment> env);
    
    void define_native(const std::string& name, 
                      std::function<Value(const std::vector<Value>&)> func);
    void set_stdlib_root(const std::string& path);
    void set_library_paths(const std::vector<std::string>& paths);
    void set_network_allowed(bool allowed);
    
    std::shared_ptr<Environment> runtime_env() { return runtime_; }
    std::shared_ptr<Environment> global_env() { return global_; }
    std::shared_ptr<Environment> current_env() { return current_; }
    void set_current_env(std::shared_ptr<Environment> env) { current_ = env; }
    ModuleLoader& module_loader() { return *module_loader_; }

    std::shared_ptr<DryadClass> executing_class() const { return executing_class_; }
    void set_executing_class(std::shared_ptr<DryadClass> klass) { executing_class_ = std::move(klass); }

    size_t call_depth() const { return call_depth_; }
    void increment_call_depth();
    void decrement_call_depth();

    bool in_constructor() const { return in_constructor_; }
    void set_in_constructor(bool value) { in_constructor_ = value; }

    FunctionDeclaration* get_or_create_lambda_code(FunctionExpression* expr);
    FunctionDeclaration* lookup_lambda_code(const FunctionExpression* expr) const;

    static Interpreter* active();
    size_t pump_event_loop();
    Value await_promise(const Value& promise);

    TypeProfiler& type_profiler() { return type_profiler_; }
    const TypeProfiler& type_profiler() const { return type_profiler_; }
    void set_type_profiling_enabled(bool enabled) { type_profiler_.set_enabled(enabled); }

    ExecutionEngine& execution_engine() { return *execution_engine_; }
    const ExecutionEngine& execution_engine() const { return *execution_engine_; }
    void set_default_execution_mode(ExecutionMode mode);
    
private:
    static thread_local Interpreter* active_;
    std::shared_ptr<Environment> runtime_;
    std::shared_ptr<Environment> global_;
    std::shared_ptr<Environment> current_;
    std::string stdlib_root_;
    std::shared_ptr<DryadClass> executing_class_;
    size_t call_depth_;
    bool in_constructor_;
    
    std::unique_ptr<ExpressionEvaluator> evaluator_;
    std::unique_ptr<StatementExecutor> executor_;
    std::unique_ptr<ModuleLoader> module_loader_;
    std::unordered_map<FunctionExpression*, FunctionDeclaration*> lambda_code_cache_;
    std::vector<std::unique_ptr<FunctionDeclaration>> owned_lambda_code_;
    TypeProfiler type_profiler_;
    std::unique_ptr<ExecutionEngine> execution_engine_;
    
    void setup_native_functions();
    void setup_intrinsic_functions();
};

} // namespace dryad

#endif // DRYAD_COMPILER_INTERPRETER_H