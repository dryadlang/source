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

Value run_queue_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/collections/queue_test.dryad");

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

TEST(QueueTest, CreateAndEmpty) {
    EXPECT_TRUE(run_queue_test("test_queue_create_and_empty").is_truthy());
}

TEST(QueueTest, EnqueueDequeueFifo) {
    EXPECT_TRUE(run_queue_test("test_queue_enqueue_dequeue_fifo").is_truthy());
}

TEST(QueueTest, Peek) {
    EXPECT_TRUE(run_queue_test("test_queue_peek").is_truthy());
}

TEST(QueueTest, DequeueEmptyReturnsNull) {
    EXPECT_TRUE(run_queue_test("test_queue_dequeue_empty_returns_null").is_truthy());
}

TEST(QueueTest, Clear) {
    EXPECT_TRUE(run_queue_test("test_queue_clear").is_truthy());
}

TEST(QueueTest, GrowsWhenFull) {
    EXPECT_TRUE(run_queue_test("test_queue_grows_when_full").is_truthy());
}

TEST(QueueTest, WrapsAroundBuffer) {
    EXPECT_TRUE(run_queue_test("test_queue_wraps_around_buffer").is_truthy());
}

TEST(QueueTest, ForEach) {
    EXPECT_TRUE(run_queue_test("test_queue_for_each").is_truthy());
}
