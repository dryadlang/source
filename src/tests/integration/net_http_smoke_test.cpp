#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <arpa/inet.h>
#include <atomic>
#include <cstring>
#include <netinet/in.h>
#include <string>
#include <sys/socket.h>
#include <thread>
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

} // namespace

TEST(NetHttpSmokeIntegration, RawHttpGetOverNetworkStream) {
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
        const char* response = "HTTP/1.0 200 OK\r\nContent-Length: 2\r\n\r\nOK";
        ::send(client_fd, response, std::strlen(response), 0);
        close_fd(client_fd);
    });

    while (!ready) {
        std::this_thread::yield();
    }
    ASSERT_GT(port.load(), 0);

    const std::string source = R"(
        import { TcpClient_connect } from "@std/net/tcp_client";
        import { Buffer_FromString, Buffer_Len, Buffer_New, Buffer_AsString, Buffer_Dispose } from "@std/buffers/buffer";

        let client = TcpClient_connect("127.0.0.1", )" + std::to_string(port.load()) + R"();
        let ok = false;
        if (client._lastError == null && client.connected) {
            let stream = client.getStream();
            let request = "GET / HTTP/1.0\r\nHost: localhost\r\n\r\n";
            let reqBuf = Buffer_FromString(request);
            let reqLen = Buffer_Len(reqBuf);
            let wrote = stream.write(reqBuf, 0, reqLen);
            Buffer_Dispose(reqBuf);

            let respBuf = Buffer_New(256);
            let read = stream.read(respBuf, 0, 256);
            let response = Buffer_AsString(respBuf);
            Buffer_Dispose(respBuf);
            stream.close();

            ok = wrote > 0 && read > 0 && response != null;
        }
    )";

    EXPECT_TRUE(run_source(source).is_truthy());

    server.join();
    close_fd(server_fd);
}
