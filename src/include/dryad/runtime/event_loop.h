#ifndef DRYAD_RUNTIME_EVENT_LOOP_H
#define DRYAD_RUNTIME_EVENT_LOOP_H

#include "dryad/runtime/io_poller.h"
#include "dryad/runtime/value.h"
#include <cstddef>
#include <deque>
#include <functional>
#include <mutex>

namespace dryad {

class Interpreter;

struct ThreadCompletion {
    Value resolve;
    Value reject;
    Value result;
    std::string error;
    bool success = false;
};

class EventLoop {
public:
    static EventLoop& instance();

    void schedule_microtask(Value callback);
    void schedule_thread_completion(const ThreadCompletion& completion);

    bool pump_once(Interpreter* interpreter);
    bool poll_io_once(Interpreter* interpreter, int timeout_ms = 0);
    bool run_turn(Interpreter* interpreter);
    size_t pump_all(Interpreter* interpreter);
    size_t pending_count() const;

    IoPoller& io_poller() { return io_poller_; }

    void clear();

private:
    EventLoop() = default;

    void drain_thread_completions(Interpreter* interpreter);

    mutable std::mutex mutex_;
    std::deque<Value> microtasks_;
    std::deque<ThreadCompletion> thread_completions_;
    IoPoller io_poller_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_EVENT_LOOP_H
