#ifndef DRYAD_COMPILER_INTERPRETER_H
#define DRYAD_COMPILER_INTERPRETER_H

#include "dryad/compiler/ast.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/environment.h"
#include <memory>
#include <functional>

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

class Interpreter {
public:
    Interpreter();
    ~Interpreter();
    
    void execute(Program* program);
    Value evaluate(Expression* expr);
    void execute_statement(Statement* stmt);
    void execute_block(const std::vector<std::unique_ptr<Statement>>& statements,
                      std::shared_ptr<Environment> env);
    
    void define_native(const std::string& name, 
                      std::function<Value(const std::vector<Value>&)> func);
    
    std::shared_ptr<Environment> global_env() { return global_; }
    std::shared_ptr<Environment> current_env() { return current_; }
    void set_current_env(std::shared_ptr<Environment> env) { current_ = env; }
    
private:
    std::shared_ptr<Environment> global_;
    std::shared_ptr<Environment> current_;
    
    std::unique_ptr<ExpressionEvaluator> evaluator_;
    std::unique_ptr<StatementExecutor> executor_;
    
    void setup_native_functions();
    void setup_intrinsic_functions();
};

} // namespace dryad

#endif // DRYAD_COMPILER_INTERPRETER_H