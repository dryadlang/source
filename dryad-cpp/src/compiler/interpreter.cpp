#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/execution_engine.h"
#include "dryad/compiler/evaluator.h"
#include "dryad/compiler/statement_executor.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/thread_manager.h"
#include "dryad/runtime/event_loop.h"
#include "dryad/runtime/promise_runtime.h"
#include "dryad/common/utils.h"
#include <fstream>
#include <iostream>
#include <sstream>

namespace dryad {

thread_local Interpreter* Interpreter::active_ = nullptr;

Interpreter* Interpreter::active() {
    return active_;
}

Interpreter::Interpreter()
    : call_depth_(0), in_constructor_(false) {
    global_ = std::make_shared<Environment>();
    current_ = global_;
    
    evaluator_ = std::make_unique<ExpressionEvaluator>(this);
    executor_ = std::make_unique<StatementExecutor>(this);
    execution_engine_ = std::make_unique<ExecutionEngine>(this);
    module_loader_ = std::make_unique<ModuleLoader>(stdlib_root_);
    
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
        "syscall.malloc", "syscall.free", "syscall.realloc", "syscall.memcpy", "syscall.memset",
        "syscall.alloc_bytes", "syscall.free_bytes", "syscall.buffer_get", "syscall.buffer_set",
        "syscall.string_length", "syscall.int_to_string", "syscall.string_to_bytes",
        "syscall.bytes_to_string",
        "syscall.socket", "syscall.connect", "syscall.bind", "syscall.listen",
        "syscall.accept", "syscall.send", "syscall.recv", "syscall.shutdown",
        "syscall.send_buffer", "syscall.recv_buffer", "syscall.sendto_buffer",
        "syscall.recvfrom_buffer", "syscall.resolve_host", "syscall.getsockname",
        "syscall.set_reuseaddr", "syscall.get_errno",
        "syscall.tls_client_handshake", "syscall.tls_send_buffer", "syscall.tls_recv_buffer",
        "syscall.tls_close", "syscall.tls_available",
        "syscall.mkdir", "syscall.rmdir", "syscall.lseek", "syscall.rename",
        "syscall.getcwd", "syscall.chdir",
        "syscall.exists", "syscall.filesize", "syscall.readdir",
        "syscall.open_read", "syscall.open_write", "syscall.open_append", "syscall.open_readwrite",
        "syscall.read_buffer", "syscall.write_buffer", "syscall.fstat", "syscall.fsync",
        "syscall.getpid", "syscall.getenv", "syscall.setenv", "syscall.sleep",
        "syscall.thread_spawn", "syscall.thread_join", "syscall.thread_spawn_async",
        "syscall.set_nonblocking",
        "syscall.thread_pool_create", "syscall.thread_pool_destroy",
        "syscall.thread_pool_submit", "syscall.thread_pool_wait",
        "syscall.thread_hardware_concurrency", "syscall.thread_sleep_ms",
        "syscall.exit",
        "syscall.string_starts_with", "syscall.normalize_path",
        "runtime.get_stack_trace", "runtime.create_object", "runtime.value_kind",
        "runtime.object_set", "runtime.object_get", "runtime.object_keys", "runtime.array_length",
        "runtime.array_push",
        "runtime.schedule_microtask", "runtime.event_loop_pending",
        "runtime.event_loop_pump_all", "runtime.event_loop_run_turn",
        "runtime.await_promise", "runtime.io_register_fd", "runtime.io_unregister_fd",
        "runtime.io_poll_once", "runtime.thread_spawn_async"
    };
    
    for (const auto& name : intrinsic_names) {
        std::string func_name = name;
        
        if (name.find("syscall.") == 0) {
            func_name.replace(0, 8, "__");
        } else if (name.find("runtime.") == 0) {
            func_name = "__runtime_" + name.substr(8);
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

void Interpreter::increment_call_depth() {
    if (++call_depth_ > kMaxCallDepth) {
        throw DryadException("Maximum call stack depth exceeded");
    }
}

void Interpreter::decrement_call_depth() {
    if (call_depth_ > 0) {
        --call_depth_;
    }
}

void Interpreter::set_stdlib_root(const std::string& path) {
    stdlib_root_ = path;
    module_loader_->set_stdlib_root(path);
    ThreadManager::set_stdlib_root(path);
}

void Interpreter::set_network_allowed(bool allowed) {
    IntrinsicsRegistry::instance().set_network_allowed(allowed);
}

void Interpreter::set_default_execution_mode(ExecutionMode mode) {
    execution_engine_->set_default_mode(mode);
}

void Interpreter::run_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw DryadException("Cannot open file: " + path);
    }
    std::ostringstream buffer;
    buffer << file.rdbuf();

    Lexer lexer(buffer.str());
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw DryadException("Parser error: " + parser.error_message());
    }

    execute(program.get());
}

void Interpreter::execute(Program* program) {
    active_ = this;
    for (auto& stmt : program->statements) {
        execute_statement(stmt.get());
    }
    pump_event_loop();
    active_ = nullptr;
}

size_t Interpreter::pump_event_loop() {
    return EventLoop::instance().pump_all(this);
}

Value Interpreter::await_promise(const Value& promise) {
    return PromiseRuntime::await_value(this, promise);
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

FunctionDeclaration* Interpreter::get_or_create_lambda_code(FunctionExpression* expr) {
    auto cached = lambda_code_cache_.find(expr);
    if (cached != lambda_code_cache_.end()) {
        return cached->second;
    }

    auto owned = std::make_unique<FunctionDeclaration>(
        "",
        std::move(expr->parameters),
        std::move(expr->body),
        expr->return_type,
        false,
        false,
        ExecutionMode::Inherit,
        expr->location
    );
    FunctionDeclaration* code = owned.get();
    owned_lambda_code_.push_back(std::move(owned));
    lambda_code_cache_[expr] = code;
    return code;
}

} // namespace dryad