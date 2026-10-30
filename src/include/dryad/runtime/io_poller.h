#ifndef DRYAD_RUNTIME_IO_POLLER_H
#define DRYAD_RUNTIME_IO_POLLER_H

#include "dryad/runtime/value.h"
#include <cstdint>
#include <unordered_map>

namespace dryad {

class Interpreter;

class IoPoller {
public:
    IoPoller();
    ~IoPoller();

    void register_fd(int64_t fd, uint32_t events, Value callback);
    void unregister_fd(int64_t fd);
    bool poll_once(Interpreter* interpreter, int timeout_ms);
    bool is_available() const { return available_; }

private:
    bool available_ = false;
#if defined(__linux__)
    int epfd_ = -1;
#endif
    std::unordered_map<int64_t, Value> callbacks_;
};

} // namespace dryad

#endif // DRYAD_RUNTIME_IO_POLLER_H
