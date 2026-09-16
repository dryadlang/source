#include "dryad/runtime/io_poller.h"
#include "dryad/runtime/function.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"

#if defined(__linux__)
#include <sys/epoll.h>
#include <unistd.h>
#endif

namespace dryad {

IoPoller::IoPoller() {
#if defined(__linux__)
    epfd_ = ::epoll_create1(0);
    available_ = epfd_ >= 0;
#endif
}

IoPoller::~IoPoller() {
#if defined(__linux__)
    if (epfd_ >= 0) {
        ::close(epfd_);
    }
#endif
}

void IoPoller::register_fd(int64_t fd, uint32_t events, Value callback) {
#if defined(__linux__)
    if (!available_) {
        throw DryadException("I/O polling is not available on this platform");
    }

    epoll_event ev{};
    ev.events = events;
    ev.data.fd = static_cast<int>(fd);

    auto existing = callbacks_.find(fd);
    int op = existing == callbacks_.end() ? EPOLL_CTL_ADD : EPOLL_CTL_MOD;
    if (::epoll_ctl(epfd_, op, static_cast<int>(fd), &ev) != 0) {
        throw DryadException("epoll_ctl failed for fd " + std::to_string(fd));
    }
    callbacks_[fd] = std::move(callback);
#else
    (void)fd;
    (void)events;
    (void)callback;
    throw DryadException("I/O polling is only implemented on Linux (epoll)");
#endif
}

void IoPoller::unregister_fd(int64_t fd) {
#if defined(__linux__)
    if (!available_) {
        return;
    }
    if (callbacks_.erase(fd) == 0) {
        return;
    }
    ::epoll_ctl(epfd_, EPOLL_CTL_DEL, static_cast<int>(fd), nullptr);
#endif
}

bool IoPoller::poll_once(Interpreter* interpreter, int timeout_ms) {
#if defined(__linux__)
    if (!available_) {
        return false;
    }

    epoll_event events[8];
    const int ready = ::epoll_wait(epfd_, events, 8, timeout_ms);
    if (ready <= 0) {
        return false;
    }

    for (int i = 0; i < ready; ++i) {
        const int64_t fd = events[i].data.fd;
        auto it = callbacks_.find(fd);
        if (it == callbacks_.end() || !it->second.is_function()) {
            continue;
        }
        it->second.as_function()->call(interpreter, {Value(fd), Value(static_cast<int64_t>(events[i].events))});
    }
    return true;
#else
    (void)interpreter;
    (void)timeout_ms;
    return false;
#endif
}

} // namespace dryad
