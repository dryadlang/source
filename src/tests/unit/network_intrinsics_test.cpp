#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/intrinsics.h"
#include "dryad/common/utils.h"
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>
#include <cstring>
#include <thread>
#include <atomic>

using namespace dryad;

namespace {

class NetworkIntrinsicsTest : public ::testing::Test {
protected:
    void SetUp() override {
        IntrinsicsRegistry::instance().register_all();
        IntrinsicsRegistry::instance().set_network_allowed(true);
    }
};

Value call(const std::string& name, const std::vector<Value>& args) {
    return IntrinsicsRegistry::instance().call(name, args);
}

int64_t as_int(const Value& value) {
    return value.as_integer();
}

Value iv(int64_t n) {
    return Value(n);
}

void close_fd(int64_t fd) {
    if (fd >= 0) {
        ::close(static_cast<int>(fd));
    }
}

} // namespace

TEST_F(NetworkIntrinsicsTest, NetworkDeniedByDefault) {
    IntrinsicsRegistry::instance().set_network_allowed(false);
    EXPECT_THROW(call("syscall.socket", {iv(2), iv(1), iv(0)}), DryadException);
    IntrinsicsRegistry::instance().set_network_allowed(true);
}

TEST_F(NetworkIntrinsicsTest, SocketIntrinsicRegistered) {
    EXPECT_TRUE(IntrinsicsRegistry::instance().has("syscall.socket"));
    EXPECT_TRUE(IntrinsicsRegistry::instance().has("syscall.bind"));
    EXPECT_TRUE(IntrinsicsRegistry::instance().has("syscall.send_buffer"));
    EXPECT_TRUE(IntrinsicsRegistry::instance().has("syscall.getsockname"));
}

TEST_F(NetworkIntrinsicsTest, BindHostPortAndEphemeralPort) {
    int64_t fd = as_int(call("syscall.socket", {iv(AF_INET), iv(SOCK_STREAM), iv(0)}));
    ASSERT_GE(fd, 0);

    EXPECT_EQ(as_int(call("syscall.set_reuseaddr", {iv(fd), Value(true)})), 0);
    EXPECT_EQ(as_int(call("syscall.bind", {iv(fd), Value("127.0.0.1"), iv(0)})), 0);

    int64_t port = as_int(call("syscall.getsockname", {iv(fd)}));
    EXPECT_GT(port, 0);

    close_fd(fd);
}

TEST_F(NetworkIntrinsicsTest, TcpLoopbackSendRecvBuffer) {
    std::atomic<bool> server_ready{false};
    std::atomic<int64_t> bound_port{0};
    int64_t server_fd = -1;

    std::thread server([&]() {
        server_fd = as_int(call("syscall.socket", {iv(AF_INET), iv(SOCK_STREAM), iv(0)}));
        ASSERT_GE(server_fd, 0);
        call("syscall.set_reuseaddr", {iv(server_fd), Value(true)});
        ASSERT_EQ(as_int(call("syscall.bind", {iv(server_fd), Value("127.0.0.1"), iv(0)})), 0);

        bound_port = as_int(call("syscall.getsockname", {iv(server_fd)}));
        ASSERT_GT(bound_port.load(), 0);
        ASSERT_EQ(as_int(call("syscall.listen", {iv(server_fd), iv(4)})), 0);
        server_ready = true;

        int64_t client_fd = as_int(call("syscall.accept", {iv(server_fd)}));
        ASSERT_GE(client_fd, 0);

        int64_t handle = as_int(MemoryManager::alloc_bytes(32));
        ASSERT_GE(handle, 0);
        int64_t nbytes = as_int(call("syscall.recv_buffer", {iv(client_fd), iv(handle), iv(0), iv(32)}));
        EXPECT_GT(nbytes, 0);
        call("syscall.send_buffer", {iv(client_fd), iv(handle), iv(0), iv(nbytes)});
        MemoryManager::free_bytes(handle);
        close_fd(client_fd);
    });

    while (!server_ready) {
        std::this_thread::yield();
    }

    int64_t client_fd = as_int(call("syscall.socket", {iv(AF_INET), iv(SOCK_STREAM), iv(0)}));
    ASSERT_GE(client_fd, 0);
    EXPECT_EQ(as_int(call("syscall.connect", {iv(client_fd), Value("127.0.0.1"), iv(bound_port.load())})), 0);

    EXPECT_EQ(as_int(call("syscall.send", {iv(client_fd), Value(std::string("ping"))})), 4);

    int64_t handle = as_int(MemoryManager::alloc_bytes(32));
    ASSERT_GE(handle, 0);
    int64_t nbytes = as_int(call("syscall.recv_buffer", {iv(client_fd), iv(handle), iv(0), iv(32)}));
    EXPECT_EQ(nbytes, 4);

    std::string response = MemoryManager::bytes_to_string(handle, 0, nbytes).as_string();
    EXPECT_EQ(response, "ping");
    MemoryManager::free_bytes(handle);

    close_fd(client_fd);
    server.join();
    close_fd(server_fd);
}
