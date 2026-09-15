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

Value run_io_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/error_handling_test.dryad");

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

TEST(IOErrorsTest, MissingFile) {
    EXPECT_TRUE(run_io_test("test_error_on_missing_file").is_truthy());
}

TEST(IOErrorsTest, InvalidPath) {
    EXPECT_TRUE(run_io_test("test_error_on_invalid_path").is_truthy());
}

TEST(IOErrorsTest, DisposedVfs) {
    EXPECT_TRUE(run_io_test("test_error_on_disposed_vfs").is_truthy());
}

TEST(IOErrorsTest, PathSeparators) {
    EXPECT_TRUE(run_io_test("test_cross_platform_path_separators").is_truthy());
}

TEST(IOErrorsTest, NormalizePathHelper) {
    EXPECT_TRUE(run_io_test("test_normalize_path_helper").is_truthy());
}
