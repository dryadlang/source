#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/event_loop.h"
#include "dryad/runtime/promise_runtime.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

void IntrinsicsRegistry::register_event_loop_intrinsics() {
    register_intrinsic("runtime.schedule_microtask", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_function()) {
            throw DryadException("runtime.schedule_microtask requires a function");
        }
        EventLoop::instance().schedule_microtask(args[0]);
        return Value();
    });

    register_intrinsic("runtime.event_loop_pending", [](const std::vector<Value>&) -> Value {
        return Value(static_cast<int64_t>(EventLoop::instance().pending_count()));
    });

    register_intrinsic("runtime.event_loop_pump_all", [](const std::vector<Value>&) -> Value {
        Interpreter* interp = Interpreter::active();
        if (interp == nullptr) {
            throw DryadException("event loop pump requires an active interpreter");
        }
        return Value(static_cast<int64_t>(EventLoop::instance().pump_all(interp)));
    });

    register_intrinsic("runtime.event_loop_run_turn", [](const std::vector<Value>&) -> Value {
        Interpreter* interp = Interpreter::active();
        if (interp == nullptr) {
            throw DryadException("event loop run_turn requires an active interpreter");
        }
        return Value(static_cast<int64_t>(EventLoop::instance().run_turn(interp) ? 1 : 0));
    });

    register_intrinsic("runtime.await_promise", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("runtime.await_promise requires a promise");
        }
        Interpreter* interp = Interpreter::active();
        if (interp == nullptr) {
            throw DryadException("await requires an active interpreter");
        }
        return PromiseRuntime::await_value(interp, args[0]);
    });
}

} // namespace dryad
