#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/intrinsics.h"
#include "dryad/common/utils.h"
#include <map>
#include <mutex>

#ifdef DRYAD_HAS_TLS
#include <openssl/ssl.h>
#include <openssl/err.h>
#endif

namespace dryad {

namespace {

#ifdef DRYAD_HAS_TLS

class TlsSessionManager {
public:
    int64_t add(SSL* ssl) {
        std::lock_guard<std::mutex> lock(mutex_);
        const int64_t handle = next_handle_++;
        sessions_[handle] = ssl;
        return handle;
    }

    SSL* get(int64_t handle) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = sessions_.find(handle);
        return it == sessions_.end() ? nullptr : it->second;
    }

    void remove(int64_t handle) {
        std::lock_guard<std::mutex> lock(mutex_);
        auto it = sessions_.find(handle);
        if (it == sessions_.end()) {
            return;
        }
        SSL_shutdown(it->second);
        SSL_free(it->second);
        sessions_.erase(it);
    }

private:
    std::mutex mutex_;
    std::map<int64_t, SSL*> sessions_;
    int64_t next_handle_ = 1;
};

TlsSessionManager& tls_sessions() {
    static TlsSessionManager manager;
    return manager;
}

SSL_CTX* client_ctx() {
    static SSL_CTX* ctx = SSL_CTX_new(TLS_client_method());
    return ctx;
}

void require_tls() {
    if (client_ctx() == nullptr) {
        throw DryadException("TLS initialization failed");
    }
}

#endif

void require_network() {
    IntrinsicsRegistry::instance().require_network_access();
}

} // namespace

void IntrinsicsRegistry::register_tls_intrinsics() {
#ifdef DRYAD_HAS_TLS
    register_intrinsic("syscall.tls_client_handshake", [](const std::vector<Value>& args) -> Value {
        require_network();
        require_tls();
        if (args.size() < 2 || !args[0].is_integer() || !args[1].is_string()) {
            throw DryadException("syscall.tls_client_handshake requires (fd, host)");
        }

        int fd = static_cast<int>(args[0].as_integer());
        const std::string& host = args[1].as_string();

        SSL* ssl = SSL_new(client_ctx());
        if (ssl == nullptr) {
            return Value(static_cast<int64_t>(-1));
        }

        SSL_set_fd(ssl, fd);
        SSL_set_tlsext_host_name(ssl, host.c_str());

        if (SSL_connect(ssl) != 1) {
            SSL_free(ssl);
            return Value(static_cast<int64_t>(-1));
        }

        return Value(tls_sessions().add(ssl));
    });

    register_intrinsic("syscall.tls_send_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.tls_send_buffer requires (tls_handle, handle, offset, count)");
        }

        SSL* ssl = tls_sessions().get(args[0].as_integer());
        if (ssl == nullptr) {
            return Value(static_cast<int64_t>(-1));
        }

        return MemoryManager::io_buffer(
            args[1].as_integer(), args[2].as_integer(), args[3].as_integer(),
            [ssl](uint8_t* data, size_t count) -> int64_t {
                int n = SSL_write(ssl, data, static_cast<int>(count));
                return static_cast<int64_t>(n);
            });
    });

    register_intrinsic("syscall.tls_recv_buffer", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.size() < 4 || !args[0].is_integer() || !args[1].is_integer() ||
            !args[2].is_integer() || !args[3].is_integer()) {
            throw DryadException("syscall.tls_recv_buffer requires (tls_handle, handle, offset, count)");
        }

        SSL* ssl = tls_sessions().get(args[0].as_integer());
        if (ssl == nullptr) {
            return Value(static_cast<int64_t>(-1));
        }

        return MemoryManager::io_buffer(
            args[1].as_integer(), args[2].as_integer(), args[3].as_integer(),
            [ssl](uint8_t* data, size_t count) -> int64_t {
                int n = SSL_read(ssl, data, static_cast<int>(count));
                return static_cast<int64_t>(n);
            });
    });

    register_intrinsic("syscall.tls_close", [](const std::vector<Value>& args) -> Value {
        require_network();
        if (args.empty() || !args[0].is_integer()) {
            throw DryadException("syscall.tls_close requires 1 integer argument");
        }
        tls_sessions().remove(args[0].as_integer());
        return Value(static_cast<int64_t>(0));
    });

    register_intrinsic("syscall.tls_available", [](const std::vector<Value>& args) -> Value {
        (void)args;
        return Value(true);
    });
#else
    register_intrinsic("syscall.tls_available", [](const std::vector<Value>& args) -> Value {
        (void)args;
        return Value(false);
    });
#endif
}

} // namespace dryad
