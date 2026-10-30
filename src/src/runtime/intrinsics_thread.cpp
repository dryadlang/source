#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/thread_manager.h"
#include "dryad/common/utils.h"

namespace dryad {

namespace {

std::vector<Value> extract_arguments(const std::vector<Value>& args, size_t index) {
    std::vector<Value> arguments;
    if (args.size() <= index) {
        return arguments;
    }
    if (!args[index].is_array()) {
        throw DryadException("thread arguments must be an array");
    }
    const size_t count = args[index].array_length();
    for (size_t i = 0; i < count; ++i) {
        arguments.push_back(args[index].array_get(i));
    }
    return arguments;
}

} // namespace

void IntrinsicsRegistry::register_thread_intrinsics() {
    register_intrinsic("syscall.thread_spawn", [](const std::vector<Value>& args) -> Value {
        if (args.empty() || !args[0].is_function()) {
            throw DryadException("syscall.thread_spawn requires a function");
        }
        const auto arguments = extract_arguments(args, 1);
        return Value(ThreadManager::spawn(args[0], arguments));
    });

    register_intrinsic("syscall.thread_join", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.thread_join requires a job id");
        }
        return ThreadManager::join(args[0].as_integer());
    });

    register_intrinsic("syscall.thread_pool_create", [](const std::vector<Value>& args) -> Value {
        int64_t workers = 1;
        if (!args.empty()) {
            workers = args[0].as_integer();
        }
        if (workers < 1) {
            workers = 1;
        }
        return Value(ThreadManager::create_pool(static_cast<size_t>(workers)));
    });

    register_intrinsic("syscall.thread_pool_destroy", [](const std::vector<Value>& args) -> Value {
        if (args.empty()) {
            throw DryadException("syscall.thread_pool_destroy requires a pool handle");
        }
        ThreadManager::destroy_pool(args[0].as_integer());
        return Value();
    });

    register_intrinsic("syscall.thread_pool_submit", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2 || !args[1].is_function()) {
            throw DryadException("syscall.thread_pool_submit requires pool handle and function");
        }
        const auto arguments = extract_arguments(args, 2);
        return Value(ThreadManager::pool_submit(args[0].as_integer(), args[1], arguments));
    });

    register_intrinsic("syscall.thread_pool_wait", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.thread_pool_wait requires pool handle and job id");
        }
        return ThreadManager::pool_wait(args[0].as_integer(), args[1].as_integer());
    });

    register_intrinsic("syscall.thread_hardware_concurrency", [](const std::vector<Value>&) -> Value {
        return Value(ThreadManager::hardware_concurrency());
    });

    register_intrinsic("syscall.thread_spawn_async", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 3 || !args[0].is_function() || !args[1].is_function() || !args[2].is_function()) {
            throw DryadException("syscall.thread_spawn_async requires function, resolve, reject");
        }
        const auto arguments = extract_arguments(args, 3);
        ThreadManager::spawn_async(args[0], arguments, args[1], args[2]);
        return Value();
    });

    register_intrinsic("syscall.thread_sleep_ms", [](const std::vector<Value>& args) -> Value {
        int64_t ms = 0;
        if (!args.empty()) {
            ms = args[0].as_integer();
        }
        ThreadManager::sleep_ms(ms);
        return Value();
    });
}

} // namespace dryad
