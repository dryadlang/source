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

Value run_threading_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/async/threading_test.dryad");

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

TEST(ThreadingTest, StartJoin) {
    EXPECT_TRUE(run_threading_test("test_thread_start_join").is_truthy());
}

TEST(ThreadingTest, WithArgs) {
    EXPECT_TRUE(run_threading_test("test_thread_with_args").is_truthy());
}

TEST(ThreadingTest, Parallel) {
    EXPECT_TRUE(run_threading_test("test_thread_parallel").is_truthy());
}

TEST(ThreadingTest, PoolRun) {
    EXPECT_TRUE(run_threading_test("test_thread_pool_run").is_truthy());
}

TEST(ThreadingTest, PoolMap) {
    EXPECT_TRUE(run_threading_test("test_thread_pool_map").is_truthy());
}

TEST(ThreadingTest, HardwareConcurrency) {
    EXPECT_TRUE(run_threading_test("test_thread_hardware_concurrency").is_truthy());
}

TEST(ThreadingTest, Sleep) {
    EXPECT_TRUE(run_threading_test("test_thread_sleep").is_truthy());
}
