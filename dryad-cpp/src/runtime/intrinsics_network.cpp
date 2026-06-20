#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cerrno>
#include <cstring>

namespace dryad {

void IntrinsicsRegistry::register_network_intrinsics() {
    register_intrinsic("syscall.socket", [](const std::vector<Value>& args) -> Value {
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
        if (args.size() < 2) {
            throw DryadException("syscall.bind requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        int64_t port = args[1].as_integer();
        
        struct sockaddr_in addr;
        std::memset(&addr, 0, sizeof(addr));
        addr.sin_family = AF_INET;
        addr.sin_addr.s_addr = INADDR_ANY;
        addr.sin_port = htons(static_cast<uint16_t>(port));
        
        int result = ::bind(static_cast<int>(fd),
                          reinterpret_cast<struct sockaddr*>(&addr),
                          sizeof(addr));
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.listen", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.listen requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        int64_t backlog = args[1].as_integer();
        
        int result = ::listen(static_cast<int>(fd), static_cast<int>(backlog));
        return Value(static_cast<int64_t>(result));
    });
    
    register_intrinsic("syscall.accept", [](const std::vector<Value>& args) -> Value {
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
        if (args.size() < 2) {
            throw DryadException("syscall.send requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        const std::string& data = args[1].as_string();
        
        ssize_t n = ::send(static_cast<int>(fd), data.c_str(), data.size(), 0);
        return Value(static_cast<int64_t>(n));
    });
    
    register_intrinsic("syscall.recv", [](const std::vector<Value>& args) -> Value {
        if (args.size() < 2) {
            throw DryadException("syscall.recv requires 2 arguments");
        }
        
        int64_t fd = args[0].as_integer();
        int64_t size = args[1].as_integer();
        
        std::string buffer(size, '\0');
        ssize_t n = ::recv(static_cast<int>(fd), &buffer[0], size, 0);
        
        if (n < 0) {
            return Value(static_cast<int64_t>(n));
        }
        
        buffer.resize(n);
        return Value(buffer);
    });
    
    register_intrinsic("syscall.shutdown", [](const std::vector<Value>& args) -> Value {
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