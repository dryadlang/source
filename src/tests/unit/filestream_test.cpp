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

Value run_filestream_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/filestream_test.dryad");

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

TEST(FileStreamTest, FileModeValues) {
    EXPECT_TRUE(run_filestream_test("test_file_mode_values").is_truthy());
}

TEST(FileStreamTest, OpenReadProperties) {
    EXPECT_TRUE(run_filestream_test("test_file_open_read_properties").is_truthy());
}

TEST(FileStreamTest, ReadWriteRoundtrip) {
    EXPECT_TRUE(run_filestream_test("test_filestream_read_write").is_truthy());
}

TEST(FileStreamTest, SeekAndFlush) {
    EXPECT_TRUE(run_filestream_test("test_filestream_seek_and_flush").is_truthy());
}

TEST(FileStreamTest, CloseDisposes) {
    EXPECT_TRUE(run_filestream_test("test_filestream_close_disposes").is_truthy());
}

TEST(FileStreamTest, LengthProperty) {
    EXPECT_TRUE(run_filestream_test("test_filestream_length").is_truthy());
}

TEST(FileStreamTest, ExistsAndDelete) {
    EXPECT_TRUE(run_filestream_test("test_file_exists_and_delete").is_truthy());
}

TEST(FileStreamTest, ReadWriteAllText) {
    EXPECT_TRUE(run_filestream_test("test_file_read_write_all_text").is_truthy());
}

TEST(FileStreamTest, OpenMissingFile) {
    EXPECT_TRUE(run_filestream_test("test_file_open_missing").is_truthy());
}

TEST(FileStreamTest, WriteToReadOnlyStreamFails) {
    EXPECT_TRUE(run_filestream_test("test_write_to_readonly_stream_fails").is_truthy());
}

TEST(FileStreamTest, WriteNestedPath) {
    EXPECT_TRUE(run_filestream_test("test_file_write_nested_path").is_truthy());
}

TEST(FileStreamTest, AppendModePosition) {
    EXPECT_TRUE(run_filestream_test("test_file_append_mode_position").is_truthy());
}
