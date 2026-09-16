#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/intrinsics.h"
#include "dryad/common/utils.h"
#ifdef _WIN32
#include <winsock2.h>
#include <ws2tcpip.h>
#else
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#endif
#include <cerrno>
#include <cstring>
#include <netdb.h>

namespace dryad {

#ifdef _WIN32
namespace {
struct WinsockInitializer {
    WinsockInitializer() {
        WSADATA wsa_data;
        WSAStartup(MAKEWORD(2, 2), &wsa_data);
    }
};
}  // namespace
#endif

namespace {

bool parse_ipv4_bind_address(const std::string& host, in_addr* out) {
    if (host.empty() || host == "0.0.0.0" || host == "*") {
        out->s_addr = INADDR_ANY;
        return true;
    }
    return ::inet_pton(AF_INET, host.c_str(), out) > 0;
}

void require_network() {
    IntrinsicsRegistry::instance().require_network_access();
}

}  // namespace

void IntrinsicsRegistry::register_network_intrinsics() {
#ifdef _WIN32
    static WinsockInitializer winsock_init;
#endif

    register_intrinsic("syscall.socket", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 3) {
            throw DryadException("syscall.socket requires 3 arguments");
        }

        int64_t domain = args[0].as_integer();
        int64_t type = args[1].as_integer();
        int64_t protocol = args[2].as_integer();

        int fd = ::socket(static_cast<int>(domain), static_cast<int>(type), static_cast<int>(protocol));
        return Value(static_cast<int64_t>(fd));
    });

    register_intrinsic("syscall.connect", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 3) {
            throw DryadException("syscall.connect requires 3 arguments");
        }

        int64_t fd = args[0].as_integer();
        const std::string& host = args[1].as_string();
        int64_t port = args[2].as_integer();

        struct sockaddr_in addr;
        std::memset(&addr, 0, sizeof(addr));
        addr.sin_family = AF_INET;
        addr.sin_port = htons(static_cast<uint16_t>(port));

        if (::inet_pton(AF_INET, host.c_str(), &addr.sin_addr) <= 0) {
            return Value(static_cast<int64_t>(-1));
        }

        int result = ::connect(static_cast<int>(fd),
                               reinterpret_cast<struct sockaddr*>(&addr),
                               sizeof(addr));
        return Value(static_cast<int64_t>(result));
    });

    register_intrinsic("syscall.bind", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 3) {
            throw DryadException("syscall.bind requires 3 arguments (fd, host, port)");
        }

        int64_t fd = args[0].as_integer();
        const std::string& host = args[1].as_string();
        int64_t port = args[2].as_integer();

        struct sockaddr_in addr;
        std::memset(&addr, 0, sizeof(addr));
        addr.sin_family = AF_INET;
        addr.sin_port = htons(static_cast<uint16_t>(port));

        if (!parse_ipv4_bind_address(host, &addr.sin_addr)) {
            return Value(static_cast<int64_t>(-1));
        }

        int result = ::bind(static_cast<int>(fd),
                            reinterpret_cast<struct sockaddr*>(&addr),
                            sizeof(addr));
        return Value(static_cast<int64_t>(result));
    });

    register_intrinsic("syscall.listen", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 2) {
            throw DryadException("syscall.listen requires 2 arguments");
        }

        int64_t fd = args[0].as_integer();
        int64_t backlog = args[1].as_integer();

        int result = ::listen(static_cast<int>(fd), static_cast<int>(backlog));
        return Value(static_cast<int64_t>(result));
    });

    register_intrinsic("syscall.accept", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.empty()) {
            throw DryadException("syscall.accept requires 1 argument");
        }

        int64_t fd = args[0].as_integer();

        struct sockaddr_in addr;
        socklen_t addrlen = sizeof(addr);

        int client_fd = ::accept(static_cast<int>(fd),
                                 reinterpret_cast<struct sockaddr*>(&addr),
                                 &addrlen);
        return Value(static_cast<int64_t>(client_fd));
    });

    register_intrinsic("syscall.send", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 2) {
            throw DryadException("syscall.send requires 2 arguments");
        }

        int64_t fd = args[0].as_integer();
        const std::string& data = args[1].as_string();

        int n = ::send(static_cast<int>(fd), data.c_str(), static_cast<int>(data.size()), 0);
        return Value(static_cast<int64_t>(n));
    });

    register_intrinsic("syscall.recv", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 2) {
            throw DryadException("syscall.recv requires 2 arguments");
        }

        int64_t fd = args[0].as_integer();
        int64_t size = args[1].as_integer();

        std::string buffer(static_cast<size_t>(size), '\0');
        int n = ::recv(static_cast<int>(fd), &buffer[0], static_cast<int>(size), 0);

        if (n < 0) {
            return Value(static_cast<int64_t>(n));
        }

        buffer.resize(static_cast<size_t>(n));
        return Value(buffer);
    });

    register_intrinsic("syscall.send_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.send_buffer requires 4 integer arguments");
        }
        return MemoryManager::write_fd_from_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());
    });

    register_intrinsic("syscall.recv_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.recv_buffer requires 4 integer arguments");
        }
        return MemoryManager::read_fd_to_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());
    });

    register_intrinsic("syscall.getsockname", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.getsockname requires 1 integer argument");
        }

        struct sockaddr_in addr;
        socklen_t addrlen = sizeof(addr);
        if (::getsockname(static_cast<int>(args[0].as_integer()),
                          reinterpret_cast<struct sockaddr*>(&addr),
                          &addrlen) < 0) {
            return Value(static_cast<int64_t>(-1));
        }

        return Value(static_cast<int64_t>(ntohs(addr.sin_port)));
    });

    register_intrinsic("syscall.set_reuseaddr", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 2 || !args[0].is_integer() || !args[1].is_boolean()) {
            throw DryadException("syscall.set_reuseaddr requires (fd, enabled)");
        }

        int enabled = args[1].as_boolean() ? 1 : 0;
        int result = ::setsockopt(static_cast<int>(args[0].as_integer()),
                                  SOL_SOCKET,
                                  SO_REUSEADDR,
                                  reinterpret_cast<const char*>(&enabled),
                                  sizeof(enabled));
        return Value(static_cast<int64_t>(result));
    });

    register_intrinsic("syscall.get_errno", [](const std::vector<Value>& args) -> Value {
        (void)args;
        return Value(static_cast<int64_t>(errno));
    });

    register_intrinsic("syscall.sendto_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 6 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer() || !args[4].is_string() ||
            !args[5].is_integer()) {
            throw DryadException("syscall.sendto_buffer requires (fd, handle, offset, count, host, port)");
        }
        return MemoryManager::sendto_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(),
            args[3].as_integer(), args[4].as_string(), args[5].as_integer());
    });

    register_intrinsic("syscall.recvfrom_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.recvfrom_buffer requires 4 integer arguments");
        }
        return MemoryManager::recvfrom_buffer(
            args[0].as_integer(), args[1].as_integer(), args[2].as_integer(), args[3].as_integer());
    });

    register_intrinsic("syscall.resolve_host", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.empty() || !args[0].is_string()) {
            throw DryadException("syscall.resolve_host requires 1 string argument");
        }

        const std::string& host = args[0].as_string();
        if (host.empty()) {
            return Value();
        }

        struct in_addr addr;
        if (::inet_pton(AF_INET, host.c_str(), &addr) > 0) {
            return Value(host);
        }

        struct addrinfo hints {};
        struct addrinfo* result = nullptr;
        hints.ai_family = AF_INET;
        hints.ai_socktype = SOCK_STREAM;

        int rc = ::getaddrinfo(host.c_str(), nullptr, &hints, &result);
        if (rc != 0 || result == nullptr) {
            return Value();
        }

        char ip[INET_ADDRSTRLEN] = {};
        auto* sin = reinterpret_cast<struct sockaddr_in*>(result->ai_addr);
        ::inet_ntop(AF_INET, &sin->sin_addr, ip, sizeof(ip));
        ::freeaddrinfo(result);
        return Value(std::string(ip));
    });

    register_intrinsic("syscall.shutdown", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 2) {
            throw DryadException("syscall.shutdown requires 2 arguments");
        }

        int64_t fd = args[0].as_integer();
        int64_t how = args[1].as_integer();

        int result = ::shutdown(static_cast<int>(fd), static_cast<int>(how));
        return Value(static_cast<int64_t>(result));
    });
}

} // namespace dryad
