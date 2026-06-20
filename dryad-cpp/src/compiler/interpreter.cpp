#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/evaluator.h"
#include "dryad/compiler/statement_executor.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/intrinsics_registry.h"
#include <iostream>

namespace dryad {

Interpreter::Interpreter() {
    global_ = std::make_shared<Environment>();
    current_ = global_;
    
    evaluator_ = std::make_unique<ExpressionEvaluator>(this);
    executor_ = std::make_unique<StatementExecutor>(this);
    
    IntrinsicsRegistry::instance().register_all();
    setup_native_functions();
    setup_intrinsic_functions();
}

Interpreter::~Interpreter() = default;

void Interpreter::setup_native_functions() {
    define_native("print", [](const std::vector<Value>& args) -> Value {
        for (size_t i = 0; i < args.size(); i++) {
            if (i > 0) std::cout << " ";
            std::cout << args[i].to_string();
        }
        std::cout << std::endl;
        return Value();
    });
}

void Interpreter::setup_intrinsic_functions() {
    auto intrinsic_names = std::vector<std::string>{
        "syscall.open", "syscall.read", "syscall.write", "syscall.close",
        "syscall.unlink", "syscall.stat", "syscall.time", "syscall.clock_gettime",
        "syscall.malloc", "syscall.free",
        "syscall.socket", "syscall.connect", "syscall.bind", "syscall.listen",
        "syscall.accept", "syscall.send", "syscall.recv", "syscall.shutdown",
        "syscall.mkdir", "syscall.rmdir", "syscall.lseek", "syscall.rename",
        "syscall.getcwd", "syscall.chdir",
        "syscall.getpid", "syscall.getenv", "syscall.setenv", "syscall.sleep",
        "syscall.exit",
        "runtime.get_stack_trace", "runtime.create_object", "runtime.object_set",
        "runtime.object_get", "runtime.array_length"
    };
    
    for (const auto& name : intrinsic_names) {
        std::string func_name = name;
        
        if (name.find("syscall.") == 0) {
            func_name.replace(0, 8, "__");
        } else if (name.find("runtime.") == 0) {
            func_name = "__" + name.substr(8);
            for (char& c : func_name) {
                if (c == '.') c = '_';
            }
        }
        
        define_native(func_name, [name](const std::vector<Value>& args) -> Value {
            return IntrinsicsRegistry::instance().call(name, args);
        });
    }
}

void Interpreter::define_native(const std::string& name,
                                std::function<Value(const std::vector<Value>&)> func) {
    auto native_fn = std::make_shared<NativeFunction>(std::move(func));
    global_->define(name, Value(native_fn));
}

void Interpreter::execute(Program* program) {
    for (auto& stmt : program->statements) {
        execute_statement(stmt.get());
    }
}

Value Interpreter::evaluate(Expression* expr) {
    return evaluator_->evaluate(expr);
}

void Interpreter::execute_statement(Statement* stmt) {
    executor_->execute(stmt);
}

void Interpreter::execute_block(const std::vector<std::unique_ptr<Statement>>& statements,
                                std::shared_ptr<Environment> env) {
    executor_->execute_block(statements, env);
}

} // namespace dryad