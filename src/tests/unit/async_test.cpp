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

Value run_async_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/async/async_await_test.dryad");

    std::string source = tests + "\nlet __ok = " + test_fn + "();";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.execute(program.get());
    return interpreter.global_env()->get("__ok");
}

} // namespace

TEST(AsyncTest, MicrotaskOrder) {
    EXPECT_TRUE(run_async_test("test_microtask_order").is_truthy());
}

TEST(AsyncTest, AsyncAwait) {
    EXPECT_TRUE(run_async_test("test_async_await").is_truthy());
}

TEST(AsyncTest, ThreadStartAsync) {
    EXPECT_TRUE(run_async_test("test_thread_start_async").is_truthy());
}

TEST(AsyncTest, ThreadStartAsyncPromise) {
    EXPECT_TRUE(run_async_test("test_thread_start_async_promise").is_truthy());
}
