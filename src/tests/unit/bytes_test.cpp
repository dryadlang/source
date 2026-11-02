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

// Loads bytes_test.dryad via the import system, then runs the named test_* function.
Value run_bytes_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/bytes/bytes_test.dryad");
    
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

TEST(BytesTest, CopyRoundtrip) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_roundtrip").is_truthy()); }
TEST(BytesTest, CopyOffsets) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_offsets").is_truthy()); }
TEST(BytesTest, CopyZeroLengthAtEnd) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_zero_length_at_end").is_truthy()); }
TEST(BytesTest, CopyRejectsBadRange) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_rejects_bad_range").is_truthy()); }
TEST(BytesTest, Fill) { EXPECT_TRUE(run_bytes_test("bytes_test_fill").is_truthy()); }
TEST(BytesTest, CompareEqual) { EXPECT_TRUE(run_bytes_test("bytes_test_compare_equal").is_truthy()); }
TEST(BytesTest, CompareOrders) { EXPECT_TRUE(run_bytes_test("bytes_test_compare_orders").is_truthy()); }
TEST(BytesTest, EqualsLengthMismatch) { EXPECT_TRUE(run_bytes_test("bytes_test_equals_length_mismatch").is_truthy()); }
TEST(BytesTest, ValidateRejectsDisposed) { EXPECT_TRUE(run_bytes_test("bytes_test_validate_rejects_disposed").is_truthy()); }
TEST(BytesTest, CopyFromSlice) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_from_slice").is_truthy()); }
TEST(BytesTest, FillFromSlice) { EXPECT_TRUE(run_bytes_test("bytes_test_fill_from_slice").is_truthy()); }
TEST(BytesTest, CompareFromSlices) { EXPECT_TRUE(run_bytes_test("bytes_test_compare_from_slices").is_truthy()); }
TEST(BytesTest, CopyOverlap) { EXPECT_TRUE(run_bytes_test("bytes_test_copy_overlap").is_truthy()); }
TEST(BytesTest, FillRejectsValue) { EXPECT_TRUE(run_bytes_test("bytes_test_fill_rejects_value").is_truthy()); }
TEST(BytesTest, EqualsTrueAndLengthOrder) { EXPECT_TRUE(run_bytes_test("bytes_test_equals_true_and_length_order").is_truthy()); }
TEST(BytesTest, ValidateRejectsOverflowingRange) { EXPECT_TRUE(run_bytes_test("bytes_test_validate_rejects_overflowing_range").is_truthy()); }
TEST(BytesTest, IndexOf) { EXPECT_TRUE(run_bytes_test("bytes_test_index_of").is_truthy()); }
TEST(BytesTest, Contains) { EXPECT_TRUE(run_bytes_test("bytes_test_contains").is_truthy()); }
TEST(BytesTest, StartsEndsWith) { EXPECT_TRUE(run_bytes_test("bytes_test_starts_ends_with").is_truthy()); }
