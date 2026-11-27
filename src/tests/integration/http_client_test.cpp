#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <arpa/inet.h>
#include <atomic>
#include <cstring>
#include <netinet/in.h>
#include <sys/socket.h>
#include <chrono>
#include <thread>
#include <functional>
#include <string>
#include <unistd.h>

using namespace dryad;

namespace {

void close_fd(int fd) {
    if (fd >= 0) {
        ::close(fd);
    }
}

Value run_source(const std::string& source, const std::string& variable = "ok") {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_network_allowed(true);
    interpreter.execute(program.get());
    return interpreter.global_env()->get(variable);
}

// One-shot loopback HTTP server: accepts one connection, reads the request,
// then hands the client fd to `respond`. The port is chosen by the kernel.
class LoopbackServer {
public:
    explicit LoopbackServer(std::function<void(int)> respond) {
        thread_ = std::thread([this, respond]() {
            server_fd_ = ::socket(AF_INET, SOCK_STREAM, 0);
            if (server_fd_ < 0) {
                ready_ = true;
                return;
            }
            int yes = 1;
            ::setsockopt(server_fd_, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(yes));
            sockaddr_in addr{};
            addr.sin_family = AF_INET;
            addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
            addr.sin_port = 0;
            socklen_t len = sizeof(addr);
            if (::bind(server_fd_, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) < 0 ||
                ::getsockname(server_fd_, reinterpret_cast<sockaddr*>(&addr), &len) < 0 ||
                ::listen(server_fd_, 4) < 0) {
                ready_ = true;
                return;
            }
            port_ = ntohs(addr.sin_port);
            ready_ = true;
            int client_fd = ::accept(server_fd_, nullptr, nullptr);
            if (client_fd < 0) {
                return;
            }
            char request[1024] = {};
            ::recv(client_fd, request, sizeof(request), 0);
            respond(client_fd);
            close_fd(client_fd);
        });
        while (!ready_) {
            std::this_thread::yield();
        }
    }

    ~LoopbackServer() {
        ::shutdown(server_fd_, SHUT_RDWR);
        thread_.join();
        close_fd(server_fd_);
    }

    int port() const { return port_.load(); }

private:
    std::thread thread_;
    std::atomic<int> port_{0};
    std::atomic<bool> ready_{false};
    int server_fd_ = -1;
};

void send_all(int fd, const std::string& data) {
    size_t sent = 0;
    while (sent < data.size()) {
        // MSG_NOSIGNAL: a client that hangs up early fails the test instead of
        // killing the test binary with SIGPIPE.
        ssize_t n = ::send(fd, data.data() + sent, data.size() - sent, MSG_NOSIGNAL);
        if (n <= 0) {
            return;
        }
        sent += static_cast<size_t>(n);
    }
}

} // namespace

TEST(HttpClientIntegration, GetOverHttpClient) {
    std::atomic<int> port{0};
    std::atomic<bool> ready{false};
    int server_fd = -1;

    std::thread server([&]() {
        server_fd = ::socket(AF_INET, SOCK_STREAM, 0);
        if (server_fd < 0) {
            return;
        }

        int yes = 1;
        ::setsockopt(server_fd, SOL_SOCKET, SO_REUSEADDR, &yes, sizeof(yes));

        sockaddr_in addr{};
        addr.sin_family = AF_INET;
        addr.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
        addr.sin_port = 0;
        if (::bind(server_fd, reinterpret_cast<sockaddr*>(&addr), sizeof(addr)) < 0) {
            return;
        }

        socklen_t len = sizeof(addr);
        ::getsockname(server_fd, reinterpret_cast<sockaddr*>(&addr), &len);
        port = ntohs(addr.sin_port);

        if (::listen(server_fd, 4) < 0) {
            return;
        }
        ready = true;

        int client_fd = ::accept(server_fd, nullptr, nullptr);
        if (client_fd < 0) {
            return;
        }

        char buffer[256] = {};
        ::recv(client_fd, buffer, sizeof(buffer), 0);
        const char* response = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";
        ::send(client_fd, response, std::strlen(response), 0);
        close_fd(client_fd);
    });

    while (!ready) {
        std::this_thread::yield();
    }
    ASSERT_GT(port.load(), 0);
    std::this_thread::sleep_for(std::chrono::milliseconds(50));

    const std::string source =
        "import { HttpClient_Create } from \"@std/net/http/http_client\";\n"
        "let client = HttpClient_Create();\n"
        "let response = client.get(\"http://127.0.0.1:" + std::to_string(port.load()) + "/\");\n"
        "let ok = false;\n"
        "if (__runtime_value_kind(response) == \"instance\") {\n"
        "    if (response.statusCode == 200) {\n"
        "        ok = response.body == \"OK\";\n"
        "    }\n"
        "}\n";

    EXPECT_TRUE(run_source(source).is_truthy());

    ::shutdown(server_fd, SHUT_RDWR);
    server.join();
    close_fd(server_fd);
}

TEST(HttpClientIntegration, ReadsResponsesLargerThanOneRead) {
    // 20 KB body in 4 KB pieces with pauses: the client sees several partial
    // reads and must keep reading until the server closes the connection.
    const std::string body(20000, 'x');
    LoopbackServer server([&body](int fd) {
        send_all(fd, "HTTP/1.1 200 OK\r\nContent-Length: " + std::to_string(body.size()) +
                         "\r\nConnection: close\r\n\r\n");
        for (size_t at = 0; at < body.size(); at += 4096) {
            send_all(fd, body.substr(at, 4096));
            std::this_thread::sleep_for(std::chrono::milliseconds(5));
        }
    });
    ASSERT_GT(server.port(), 0);

    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "import { Text_Length } from \"@std/text\";\n"
        "let response = HttpClient_Create().get(\"http://127.0.0.1:" + std::to_string(server.port()) + "/big\");\n"
        "let ok = __runtime_value_kind(response) == \"instance\" && response.statusCode == 200 &&\n"
        "         Text_Length(response.body)[0] == 20000;\n";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(HttpClientIntegration, EmptyResponseIsAnError) {
    // The server closes without answering: an error string, not a throw.
    LoopbackServer server([](int) {});
    ASSERT_GT(server.port(), 0);
    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "let ok = HttpClient_Create().get(\"http://127.0.0.1:" + std::to_string(server.port()) + "/\") ==\n"
        "         \"HTTP response read failed\";\n";
    EXPECT_TRUE(run_source(source).is_truthy());
}
