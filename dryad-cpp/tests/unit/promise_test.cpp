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

Value run_promise_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/async/promise_test.dryad");

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

TEST(PromiseTest, Resolve) {
    EXPECT_TRUE(run_promise_test("test_promise_resolve").is_truthy());
}

TEST(PromiseTest, Reject) {
    EXPECT_TRUE(run_promise_test("test_promise_reject").is_truthy());
}

TEST(PromiseTest, Then) {
    EXPECT_TRUE(run_promise_test("test_promise_then").is_truthy());
}

TEST(PromiseTest, All) {
    EXPECT_TRUE(run_promise_test("test_promise_all").is_truthy());
}

TEST(PromiseTest, Race) {
    EXPECT_TRUE(run_promise_test("test_promise_race").is_truthy());
}

TEST(PromiseTest, DeferredThen) {
    EXPECT_TRUE(run_promise_test("test_promise_deferred_then_resolve").is_truthy());
}

TEST(PromiseTest, AllDeferred) {
    EXPECT_TRUE(run_promise_test("test_promise_all_deferred").is_truthy());
}
