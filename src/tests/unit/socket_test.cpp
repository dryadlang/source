#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <fstream>
#include <sstream>
#include <stdexcept>

using namespace dryad;

namespace {

std::string read_file(const std::string& path) {
    std::ifstream file(path);
    if (!file) {
        throw std::runtime_error("Cannot open file: " + path);
    }
    std::ostringstream buffer;
    buffer << file.rdbuf();
    return buffer.str();
}

Value run_socket_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/net/socket_test.dryad");

    std::string source = tests + "\nlet __ok = " + test_fn + "();";

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
    return interpreter.global_env()->get("__ok");
}

} // namespace

TEST(SocketTest, CreateAndClose) {
    EXPECT_TRUE(run_socket_test("test_socket_create_and_close").is_truthy());
}

TEST(SocketTest, ConnectInvalidPort) {
    EXPECT_TRUE(run_socket_test("test_socket_connect_invalid_port").is_truthy());
}

TEST(SocketTest, IsSocketHelper) {
    EXPECT_TRUE(run_socket_test("test_socket_is_socket_helper").is_truthy());
}

TEST(SocketTest, SendWithoutConnect) {
    EXPECT_TRUE(run_socket_test("test_socket_send_without_connect").is_truthy());
}
