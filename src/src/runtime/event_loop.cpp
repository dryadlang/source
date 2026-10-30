#include "dryad/runtime/event_loop.h"
#include "dryad/runtime/function.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

namespace dryad {

EventLoop& EventLoop::instance() {
    static EventLoop loop;
    return loop;
}

void EventLoop::schedule_microtask(Value callback) {
    if (!callback.is_function()) {
        throw DryadException("schedule_microtask requires a function");
    }
    std::lock_guard<std::mutex> lock(mutex_);
    microtasks_.push_back(std::move(callback));
}

void EventLoop::schedule_thread_completion(const ThreadCompletion& completion) {
    std::lock_guard<std::mutex> lock(mutex_);
    thread_completions_.push_back(completion);
}

void EventLoop::drain_thread_completions(Interpreter* interpreter) {
    std::deque<ThreadCompletion> pending;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        pending.swap(thread_completions_);
    }

    for (const auto& completion : pending) {
        if (!completion.resolve.is_function() || !completion.reject.is_function()) {
            continue;
        }
        if (completion.success) {
            completion.resolve.as_function()->call(interpreter, {completion.result});
        } else {
            completion.reject.as_function()->call(
                interpreter, {Value(completion.error.empty() ? "thread failed" : completion.error)});
        }
    }
}

bool EventLoop::pump_once(Interpreter* interpreter) {
    drain_thread_completions(interpreter);

    Value callback;
    {
        std::lock_guard<std::mutex> lock(mutex_);
        if (microtasks_.empty()) {
            return false;
        }
        callback = std::move(microtasks_.front());
        microtasks_.pop_front();
    }

    callback.as_function()->call(interpreter, {});
    return true;
}

bool EventLoop::poll_io_once(Interpreter* interpreter, int timeout_ms) {
    return io_poller_.poll_once(interpreter, timeout_ms);
}

bool EventLoop::run_turn(Interpreter* interpreter) {
    if (pump_once(interpreter)) {
        return true;
    }
    return poll_io_once(interpreter, 0);
}

size_t EventLoop::pump_all(Interpreter* interpreter) {
    size_t count = 0;
    while (run_turn(interpreter)) {
        ++count;
    }
    return count;
}

size_t EventLoop::pending_count() const {
    std::lock_guard<std::mutex> lock(mutex_);
    return microtasks_.size() + thread_completions_.size();
}

void EventLoop::clear() {
    std::lock_guard<std::mutex> lock(mutex_);
    microtasks_.clear();
    thread_completions_.clear();
}

} // namespace dryad
