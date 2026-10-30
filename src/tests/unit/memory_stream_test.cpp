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

Value run_memory_stream_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/memory_stream_test.dryad");

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

TEST(MemoryStreamTest, ReadWriteRoundtrip) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_read_write").is_truthy());
}

TEST(MemoryStreamTest, FromString) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_from_string").is_truthy());
}

TEST(MemoryStreamTest, Seek) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_seek").is_truthy());
}

TEST(MemoryStreamTest, ToString) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_to_string").is_truthy());
}

TEST(MemoryStreamTest, GrowsOnWrite) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_grows_on_write").is_truthy());
}

TEST(MemoryStreamTest, ReadonlyRejectsWrite) {
    EXPECT_TRUE(run_memory_stream_test("test_memory_stream_readonly_rejects_write").is_truthy());
}
