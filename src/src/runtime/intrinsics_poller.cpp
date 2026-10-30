#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/event_loop.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

void IntrinsicsRegistry::register_poller_intrinsics() {
    register_intrinsic("runtime.io_register_fd", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3) {
            throw DryadException("runtime.io_register_fd requires fd, events, callback");
        }
        if (!args[2].is_function()) {
            throw DryadException("runtime.io_register_fd callback must be a function");
        }
        EventLoop::instance().io_poller().register_fd(
            args[0].as_integer(),
            static_cast<uint32_t>(args[1].as_integer()),
            args[2]);
        return Value();
    });

    register_intrinsic("runtime.io_unregister_fd", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("runtime.io_unregister_fd requires fd");
        }
        EventLoop::instance().io_poller().unregister_fd(args[0].as_integer());
        return Value();
    });

    register_intrinsic("runtime.io_poll_once", [](const std::vector<Value>& args) -> Value {
        Interpreter* interp = Interpreter::active();
        if (interp == nullptr) {
            throw DryadException("io_poll_once requires an active interpreter");
        }
        int64_t timeout_ms = 0;
        if (!args.empty()) {
            timeout_ms = args[0].as_integer();
        }
        return Value(static_cast<int64_t>(
            EventLoop::instance().io_poller().poll_once(interp, static_cast<int>(timeout_ms)) ? 1 : 0));
    });
}

} // namespace dryad
