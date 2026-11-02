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

// Loads text_test.dryad via the import system, then runs the named test_* function.
Value run_text_test(const std::string& test_fn) {
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/text/text_test.dryad");
    
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

TEST(TextTest, LengthIsBytes) { EXPECT_TRUE(run_text_test("text_test_length_is_bytes").is_truthy()); }
TEST(TextTest, CharAtUsesByteIndexes) { EXPECT_TRUE(run_text_test("text_test_char_at_uses_byte_indexes").is_truthy()); }
TEST(TextTest, CodePointAt) { EXPECT_TRUE(run_text_test("text_test_code_point_at").is_truthy()); }
TEST(TextTest, CodePointAtRejectsContinuationByte) { EXPECT_TRUE(run_text_test("text_test_code_point_at_rejects_continuation_byte").is_truthy()); }
TEST(TextTest, FromCodePoint) { EXPECT_TRUE(run_text_test("text_test_from_code_point").is_truthy()); }
TEST(TextTest, SliceAndSubstring) { EXPECT_TRUE(run_text_test("text_test_slice_and_substring").is_truthy()); }
TEST(TextTest, IndexOfAndLastIndexOf) { EXPECT_TRUE(run_text_test("text_test_index_of_and_last_index_of").is_truthy()); }
TEST(TextTest, IncludesAndAffixes) { EXPECT_TRUE(run_text_test("text_test_includes_and_affixes").is_truthy()); }
TEST(TextTest, EqualsAndCompare) { EXPECT_TRUE(run_text_test("text_test_equals_and_compare").is_truthy()); }
TEST(TextTest, CodePointsAndCharCount) { EXPECT_TRUE(run_text_test("text_test_code_points_and_char_count").is_truthy()); }
TEST(TextTest, EmptyStrings) { EXPECT_TRUE(run_text_test("text_test_empty_strings").is_truthy()); }
TEST(TextTest, Utf8Rejections) { EXPECT_TRUE(run_text_test("text_test_utf8_rejections").is_truthy()); }
TEST(TextTest, AsciiCase) { EXPECT_TRUE(run_text_test("text_test_ascii_case").is_truthy()); }
TEST(TextTest, TrimOnlyAsciiWhitespace) { EXPECT_TRUE(run_text_test("text_test_trim_only_ascii_whitespace").is_truthy()); }
TEST(TextTest, RepeatAndPad) { EXPECT_TRUE(run_text_test("text_test_repeat_and_pad").is_truthy()); }
TEST(TextTest, Split) { EXPECT_TRUE(run_text_test("text_test_split").is_truthy()); }
TEST(TextTest, SplitRejectsEmptySeparator) { EXPECT_TRUE(run_text_test("text_test_split_rejects_empty_separator").is_truthy()); }
TEST(TextTest, Join) { EXPECT_TRUE(run_text_test("text_test_join").is_truthy()); }
TEST(TextTest, Replace) { EXPECT_TRUE(run_text_test("text_test_replace").is_truthy()); }
TEST(TextTest, ReplaceDeletesAndJoinsEmptySeparator) { EXPECT_TRUE(run_text_test("text_test_replace_deletes_and_joins_empty_separator").is_truthy()); }
TEST(TextTest, SplitPartContents) { EXPECT_TRUE(run_text_test("text_test_split_part_contents").is_truthy()); }
TEST(TextTest, ParseInt) { EXPECT_TRUE(run_text_test("text_test_parse_int").is_truthy()); }
TEST(TextTest, ParseIntRejectsOverflow) { EXPECT_TRUE(run_text_test("text_test_parse_int_rejects_overflow").is_truthy()); }
TEST(TextTest, FormatInt) { EXPECT_TRUE(run_text_test("text_test_format_int").is_truthy()); }
TEST(TextTest, ParseFloat) { EXPECT_TRUE(run_text_test("text_test_parse_float").is_truthy()); }
TEST(TextTest, FormatFloat) { EXPECT_TRUE(run_text_test("text_test_format_float").is_truthy()); }
TEST(TextTest, NumericEdges) { EXPECT_TRUE(run_text_test("text_test_numeric_edges").is_truthy()); }
TEST(TextTest, FormatFloatNegativeZeroAndPrecision) { EXPECT_TRUE(run_text_test("text_test_format_float_negative_zero_and_precision").is_truthy()); }
