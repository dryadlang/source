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

Value run_list_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/collections/list_test.dryad");

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

TEST(ListTest, CreateAndCount) {
    EXPECT_TRUE(run_list_test("test_list_create_and_count").is_truthy());
}

TEST(ListTest, AddAndGet) {
    EXPECT_TRUE(run_list_test("test_list_add_and_get").is_truthy());
}

TEST(ListTest, DefaultCapacity) {
    EXPECT_TRUE(run_list_test("test_list_default_capacity").is_truthy());
}

TEST(ListTest, Set) {
    EXPECT_TRUE(run_list_test("test_list_set").is_truthy());
}

TEST(ListTest, Insert) {
    EXPECT_TRUE(run_list_test("test_list_insert").is_truthy());
}

TEST(ListTest, RemoveAndRemoveAt) {
    EXPECT_TRUE(run_list_test("test_list_remove_and_remove_at").is_truthy());
}

TEST(ListTest, Clear) {
    EXPECT_TRUE(run_list_test("test_list_clear").is_truthy());
}

TEST(ListTest, ContainsAndIndexOf) {
    EXPECT_TRUE(run_list_test("test_list_contains_and_index_of").is_truthy());
}

TEST(ListTest, OutOfBounds) {
    EXPECT_TRUE(run_list_test("test_list_out_of_bounds").is_truthy());
}

TEST(ListTest, ForEach) {
    EXPECT_TRUE(run_list_test("test_list_for_each").is_truthy());
}

TEST(ListTest, MapWithLambda) {
    EXPECT_TRUE(run_list_test("test_list_map_with_lambda").is_truthy());
}

TEST(ListTest, FilterWithLambda) {
    EXPECT_TRUE(run_list_test("test_list_filter_with_lambda").is_truthy());
}

TEST(ListTest, ReduceWithLambda) {
    EXPECT_TRUE(run_list_test("test_list_reduce_with_lambda").is_truthy());
}
