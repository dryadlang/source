#include "dryad/runtime/promise_runtime.h"
#include "dryad/runtime/class.h"
#include "dryad/runtime/event_loop.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/runtime/function.h"
#include "dryad/common/utils.h"
#include <chrono>
#include <thread>

namespace dryad {

namespace {

Value read_field(const Value& promise, const std::string& key) {
    if (promise.is_instance()) {
        return promise.as_instance()->get(key);
    }
    if (promise.is_object()) {
        return promise.object_get(key);
    }
    throw DryadException("expected a Promise instance");
}

std::string read_state_string(const Value& field) {
    if (field.is_string()) {
        return field.as_string();
    }
    return "";
}

} // namespace

void PromiseRuntime::ensure_loaded(Interpreter* interpreter) {
    auto module_env = interpreter->module_loader().load_module("@std/async/promise", interpreter);
    if (!module_env->has("Promise_resolve")) {
        throw DryadException("Promise module missing Promise_resolve export");
    }
}

Value PromiseRuntime::call_export(Interpreter* interpreter, const std::string& name,
                                  const std::vector<Value>& args) {
    ensure_loaded(interpreter);
    auto module_env = interpreter->module_loader().load_module("@std/async/promise", interpreter);
    Value fn = module_env->get(name);
    if (!fn.is_function()) {
        throw DryadException("Promise export is not a function: " + name);
    }
    return fn.as_function()->call(interpreter, args);
}

Value PromiseRuntime::resolve(Interpreter* interpreter, const Value& value) {
    return call_export(interpreter, "Promise_resolve", {value});
}

Value PromiseRuntime::reject(Interpreter* interpreter, const Value& error) {
    return call_export(interpreter, "Promise_reject", {error});
}

bool PromiseRuntime::is_promise(const Value& value) {
    if (!value.is_instance() && !value.is_object()) {
        return false;
    }
    try {
        Value state_field = read_field(value, "_state");
        return state_field.is_string();
    } catch (...) {
        return false;
    }
}

std::string PromiseRuntime::state(const Value& promise) {
    return read_state_string(read_field(promise, "_state"));
}

Value PromiseRuntime::value(const Value& promise) {
    return read_field(promise, "_value");
}

Value PromiseRuntime::error(const Value& promise) {
    return read_field(promise, "_error");
}

Value PromiseRuntime::await_value(Interpreter* interpreter, const Value& promise) {
    Value current = promise;
    if (!is_promise(current)) {
        return PromiseRuntime::resolve(interpreter, current);
    }

    int idle_spins = 0;
    while (state(current) == "pending") {
        if (EventLoop::instance().run_turn(interpreter)) {
            idle_spins = 0;
            continue;
        }
        ++idle_spins;
        if (idle_spins > 10000) {
            throw DryadException("await: promise remained pending with no runnable tasks");
        }
        std::this_thread::sleep_for(std::chrono::milliseconds(1));
    }

    if (state(current) == "rejected") {
        Value err = error(current);
        if (err.is_string()) {
            throw DryadException(err.as_string());
        }
        throw DryadException("await: promise rejected");
    }

    return value(current);
}

} // namespace dryad
