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
#include <mutex>
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
            std::string request = read_request(client_fd);
            {
                std::lock_guard<std::mutex> lock(mutex_);
                request_ = request;
            }
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

    // The raw request the server received (head and Content-Length body).
    std::string request() const {
        std::lock_guard<std::mutex> lock(mutex_);
        return request_;
    }

private:
    // Reads the request head, then as many body bytes as Content-Length says.
    static std::string read_request(int fd) {
        std::string data;
        char chunk[1024];
        size_t body_start = std::string::npos;
        size_t wanted = 0;
        while (true) {
            if (body_start != std::string::npos && data.size() >= body_start + wanted) {
                return data;
            }
            ssize_t n = ::recv(fd, chunk, sizeof(chunk), 0);
            if (n <= 0) {
                return data;
            }
            data.append(chunk, static_cast<size_t>(n));
            if (body_start == std::string::npos) {
                size_t head_end = data.find("\r\n\r\n");
                if (head_end == std::string::npos) {
                    continue;
                }
                body_start = head_end + 4;
                size_t length_at = data.find("Content-Length: ");
                if (length_at != std::string::npos && length_at < head_end) {
                    wanted = std::stoul(data.substr(length_at + 16));
                }
            }
        }
    }

    std::thread thread_;
    std::atomic<int> port_{0};
    std::atomic<bool> ready_{false};
    int server_fd_ = -1;
    mutable std::mutex mutex_;
    std::string request_;
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

namespace {

const char* kOkResponse = "HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nOK";

std::string url_for(const LoopbackServer& server, const std::string& path) {
    return "http://127.0.0.1:" + std::to_string(server.port()) + path;
}

} // namespace

TEST(HttpClientIntegration, PostSendsBodyWithByteContentLength) {
    LoopbackServer server([](int fd) { send_all(fd, kOkResponse); });
    ASSERT_GT(server.port(), 0);
    // "olá" is 3 characters but 4 bytes: Content-Length counts bytes.
    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "let response = HttpClient_Create().post(\"" + url_for(server, "/submit") + "\", \"ol\u00e1\");\n"
        "let ok = __runtime_value_kind(response) == \"instance\" && response.statusCode == 200 &&\n"
        "         response.body == \"OK\";\n";
    EXPECT_TRUE(run_source(source).is_truthy());
    const std::string request = server.request();
    EXPECT_EQ(request.rfind("POST /submit HTTP/1.1\r\n", 0), 0u) << request;
    EXPECT_NE(request.find("\r\nContent-Length: 4\r\n"), std::string::npos) << request;
    EXPECT_EQ(request.substr(request.size() - 4), "ol\u00e1") << request;
}

TEST(HttpClientIntegration, SendWritesTheRequestHeaders) {
    LoopbackServer server([](int fd) { send_all(fd, kOkResponse); });
    ASSERT_GT(server.port(), 0);
    const std::string source =
        "import { HttpClient_Create, HttpRequest, HttpHeaders } from \"@std/net\";\n"
        "let headers = new HttpHeaders();\n"
        "headers.set(\"X-Token\", \"abc\");\n"
        "headers.set(\"Accept\", \"text/plain\");\n"
        "let request = new HttpRequest(\"GET\", \"" + url_for(server, "/h") + "\", headers, \"\");\n"
        "let response = HttpClient_Create().send(request);\n"
        "let ok = __runtime_value_kind(response) == \"instance\" && response.statusCode == 200;\n";
    EXPECT_TRUE(run_source(source).is_truthy());
    const std::string request = server.request();
    EXPECT_NE(request.find("\r\nX-Token: abc\r\n"), std::string::npos) << request;
    EXPECT_NE(request.find("\r\nAccept: text/plain\r\n"), std::string::npos) << request;
    // Host and Connection are still written exactly once.
    EXPECT_EQ(request.find("Host: "), request.rfind("Host: ")) << request;
}

TEST(HttpClientIntegration, ResponseHeaderLookupIgnoresCase) {
    LoopbackServer server([](int fd) {
        send_all(fd, "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: 2\r\n"
                     "Connection: close\r\n\r\nOK");
    });
    ASSERT_GT(server.port(), 0);
    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "let response = HttpClient_Create().get(\"" + url_for(server, "/") + "\");\n"
        "let ok = __runtime_value_kind(response) == \"instance\" &&\n"
        "         response.headers.get(\"content-type\") == \"text/plain\" &&\n"
        "         response.headers.get(\"CONTENT-LENGTH\") == \"2\" &&\n"
        "         response.headers.has(\"Content-Type\");\n";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(HttpClientIntegration, DecodesChunkedBodies) {
    LoopbackServer server([](int fd) {
        send_all(fd, "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
                     "5\r\nHello\r\n"
                     "6;ext=1\r\n World\r\n"
                     "b\r\n from Dryad\r\n"
                     "0\r\n\r\n");
    });
    ASSERT_GT(server.port(), 0);
    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "let response = HttpClient_Create().get(\"" + url_for(server, "/chunked") + "\");\n"
        "let ok = __runtime_value_kind(response) == \"instance\" &&\n"
        "         response.body == \"Hello World from Dryad\";\n";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(HttpClientIntegration, GetAsyncAndPostAsyncResolveWithTheResponse) {
    LoopbackServer get_server([](int fd) { send_all(fd, kOkResponse); });
    LoopbackServer post_server([](int fd) { send_all(fd, kOkResponse); });
    ASSERT_GT(get_server.port(), 0);
    ASSERT_GT(post_server.port(), 0);
    const std::string source =
        "import { HttpClient_Create } from \"@std/net\";\n"
        "import { Promise_Await } from \"@std/async\";\n"
        "let client = HttpClient_Create();\n"
        "let got = Promise_Await(client.getAsync(\"" + url_for(get_server, "/a") + "\"));\n"
        "let posted = Promise_Await(client.postAsync(\"" + url_for(post_server, "/b") + "\", \"x=1\"));\n"
        "let ok = got.statusCode == 200 && posted.statusCode == 200 && posted.body == \"OK\";\n";
    EXPECT_TRUE(run_source(source).is_truthy());
    EXPECT_EQ(get_server.request().rfind("GET /a HTTP/1.1\r\n", 0), 0u);
    const std::string posted = post_server.request();
    EXPECT_EQ(posted.rfind("POST /b HTTP/1.1\r\n", 0), 0u) << posted;
    EXPECT_EQ(posted.substr(posted.size() - 3), "x=1") << posted;
}
