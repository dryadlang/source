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
    std::ifstream f(path);
    if (!f) {
        throw std::runtime_error("Cannot open file: " + path);
    }
    std::ostringstream ss;
    ss << f.rdbuf();
    return ss.str();
}

Value run_handle_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/filehandle_test.dryad");

    std::string source = tests + "\nlet __ok = " + test_fn + "();";

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
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

TEST(FileHandleTest, WriteThenRead) {
    EXPECT_TRUE(run_handle_test("test_filehandle_write_then_read").is_truthy());
}

TEST(FileHandleTest, RejectsReadOnWriteMode) {
    EXPECT_TRUE(run_handle_test("test_filehandle_rejects_read_on_write_mode").is_truthy());
}

TEST(FileHandleTest, ClosedIsNotOpen) {
    EXPECT_TRUE(run_handle_test("test_filehandle_closed_is_not_open").is_truthy());
}

TEST(FileHandleTest, UseAfterClose) {
    EXPECT_TRUE(run_handle_test("test_filehandle_use_after_close").is_truthy());
}

TEST(FileHandleTest, Append) {
    EXPECT_TRUE(run_handle_test("test_filehandle_append").is_truthy());
}

TEST(FileHandleTest, InvalidMode) {
    EXPECT_TRUE(run_handle_test("test_filehandle_invalid_mode").is_truthy());
}
