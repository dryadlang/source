#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <arpa/inet.h>
#include <atomic>
#include <cstring>
#include <netinet/in.h>
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

TEST(UdpEchoIntegration, DryadUdpClientEcho) {
    std::atomic<int> port{0};
    std::atomic<bool> ready{false};
    int server_fd = -1;

    std::thread server([&]() {
        server_fd = ::socket(AF_INET, SOCK_DGRAM, 0);
        if (server_fd < 0) {
            return;
        }

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
        ready = true;

        char buffer[32] = {};
        sockaddr_in client_addr{};
        socklen_t client_len = sizeof(client_addr);
        ssize_t nbytes = ::recvfrom(server_fd, buffer, sizeof(buffer), 0,
                                    reinterpret_cast<sockaddr*>(&client_addr), &client_len);
        if (nbytes > 0) {
            ::sendto(server_fd, buffer, static_cast<size_t>(nbytes), 0,
                     reinterpret_cast<sockaddr*>(&client_addr), client_len);
        }
    });

    while (!ready) {
        std::this_thread::yield();
    }
    ASSERT_GT(port.load(), 0);

    const std::string source = R"(
        import { UdpClient_create } from "@std/net/udp_client";
        import { Buffer_FromString, Buffer_AsString, Buffer_Dispose } from "@std/buffers/buffer";

        let client = UdpClient_create();
        let ok = false;
        if (client._lastError == null) {
            let buf = Buffer_FromString("echo");
            let sent = client.sendTo("127.0.0.1", )" + std::to_string(port.load()) + R"(, buf, 0, 4);
            let endpoint = client.receiveFrom(buf, 0, 4);
            let text = Buffer_AsString(buf);
            Buffer_Dispose(buf);
            client.close();
            ok = sent == 4 && endpoint != null && endpoint.nbytes == 4 && text == "echo";
        }
    )";

    EXPECT_TRUE(run_source(source).is_truthy());

    server.join();
    close_fd(server_fd);
}
