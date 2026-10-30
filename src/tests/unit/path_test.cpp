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

Value run_path_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/io/path_test.dryad");

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

TEST(PathTest, Join) {
    EXPECT_TRUE(run_path_test("test_path_join").is_truthy());
}

TEST(PathTest, Dirname) {
    EXPECT_TRUE(run_path_test("test_path_dirname").is_truthy());
}

TEST(PathTest, Basename) {
    EXPECT_TRUE(run_path_test("test_path_basename").is_truthy());
}

TEST(PathTest, Extension) {
    EXPECT_TRUE(run_path_test("test_path_extension").is_truthy());
}

TEST(PathTest, EnsureParentForNestedFile) {
    EXPECT_TRUE(run_path_test("test_path_ensure_parent_for_nested_file").is_truthy());
}
