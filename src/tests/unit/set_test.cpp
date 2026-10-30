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

Value run_set_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/collections/set_test.dryad");

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

TEST(SetTest, AddAndHas) {
    EXPECT_TRUE(run_set_test("test_set_add_and_has").is_truthy());
}

TEST(SetTest, IgnoresDuplicates) {
    EXPECT_TRUE(run_set_test("test_set_ignores_duplicates").is_truthy());
}

TEST(SetTest, Delete) {
    EXPECT_TRUE(run_set_test("test_set_delete").is_truthy());
}

TEST(SetTest, Clear) {
    EXPECT_TRUE(run_set_test("test_set_clear").is_truthy());
}

TEST(SetTest, ForEach) {
    EXPECT_TRUE(run_set_test("test_set_for_each").is_truthy());
}
