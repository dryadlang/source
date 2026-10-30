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

Value run_map_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/collections/map_test.dryad");

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

TEST(MapTest, PutGetHas) {
    EXPECT_TRUE(run_map_test("test_map_put_get_has").is_truthy());
}

TEST(MapTest, UpdateExisting) {
    EXPECT_TRUE(run_map_test("test_map_update_existing").is_truthy());
}

TEST(MapTest, Delete) {
    EXPECT_TRUE(run_map_test("test_map_delete").is_truthy());
}

TEST(MapTest, GetMissingReturnsNull) {
    EXPECT_TRUE(run_map_test("test_map_get_missing_returns_null").is_truthy());
}

TEST(MapTest, ForEach) {
    EXPECT_TRUE(run_map_test("test_map_for_each").is_truthy());
}

TEST(MapTest, KeysAndValues) {
    EXPECT_TRUE(run_map_test("test_map_keys_and_values").is_truthy());
}

TEST(MapTest, NumericKeys) {
    EXPECT_TRUE(run_map_test("test_map_numeric_keys").is_truthy());
}
