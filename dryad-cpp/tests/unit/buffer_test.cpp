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

// Loads exceptions.dryad, buffer.dryad and buffer_test.dryad as one source unit
// (the parser has no import support), then runs the named test_* function.
Value run_buffer_test(const std::string& test_fn) {
    static const std::string exceptions = read_file(
        std::string(DRYAD_STDLIB_DIR) + "/@std/core/exceptions.dryad");
    static const std::string buffer = read_file(
        std::string(DRYAD_STDLIB_DIR) + "/@std/buffers/buffer.dryad");
    static const std::string tests = read_file(
        std::string(DRYAD_TEST_DIR) + "/stdlib/buffers/buffer_test.dryad");
    
    std::string source = exceptions + "\n" + buffer + "\n" + tests +
        "\nlet __ok = " + test_fn + "();";
    
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    return interpreter.global_env()->get("__ok");
}

} // namespace

TEST(BufferTest, NewAndBasicProperties) {
    EXPECT_TRUE(run_buffer_test("test_buffer_new_creation").is_truthy());
}

TEST(BufferTest, NewRejectsNegativeSize) {
    EXPECT_TRUE(run_buffer_test("test_buffer_new_negative_size").is_truthy());
}

TEST(BufferTest, NewZeroSize) {
    EXPECT_TRUE(run_buffer_test("test_buffer_new_zero_size").is_truthy());
}

TEST(BufferTest, ReadWriteRoundTrip) {
    EXPECT_TRUE(run_buffer_test("test_buffer_read_write").is_truthy());
}

TEST(BufferTest, ReadOutOfBounds) {
    EXPECT_TRUE(run_buffer_test("test_buffer_read_out_of_bounds").is_truthy());
}

TEST(BufferTest, WriteOutOfBounds) {
    EXPECT_TRUE(run_buffer_test("test_buffer_write_out_of_bounds").is_truthy());
}

TEST(BufferTest, WriteNegativeIndex) {
    EXPECT_TRUE(run_buffer_test("test_buffer_write_negative_index").is_truthy());
}

TEST(BufferTest, WriteRejectsValueOver255) {
    EXPECT_TRUE(run_buffer_test("test_buffer_write_value_range").is_truthy());
}

TEST(BufferTest, Fill) {
    EXPECT_TRUE(run_buffer_test("test_buffer_fill").is_truthy());
}

TEST(BufferTest, SliceView) {
    EXPECT_TRUE(run_buffer_test("test_buffer_slice_view").is_truthy());
}

TEST(BufferTest, SliceZeroCopy) {
    EXPECT_TRUE(run_buffer_test("test_buffer_slice_zero_copy").is_truthy());
}

TEST(BufferTest, SliceRejectsBadRange) {
    EXPECT_TRUE(run_buffer_test("test_buffer_slice_bad_range").is_truthy());
}

TEST(BufferTest, ResizeGrow) {
    EXPECT_TRUE(run_buffer_test("test_buffer_resize_grow").is_truthy());
}

TEST(BufferTest, ResizeShrink) {
    EXPECT_TRUE(run_buffer_test("test_buffer_resize_shrink").is_truthy());
}

TEST(BufferTest, ResizeRejectsSlice) {
    EXPECT_TRUE(run_buffer_test("test_buffer_resize_slice_rejected").is_truthy());
}

TEST(BufferTest, CopyTo) {
    EXPECT_TRUE(run_buffer_test("test_buffer_copy_to").is_truthy());
}

TEST(BufferTest, CopyToOffset) {
    EXPECT_TRUE(run_buffer_test("test_buffer_copy_to_offset").is_truthy());
}

TEST(BufferTest, CopyToRejectsBadRange) {
    EXPECT_TRUE(run_buffer_test("test_buffer_copy_to_bad_range").is_truthy());
}

TEST(BufferTest, UseAfterDispose) {
    EXPECT_TRUE(run_buffer_test("test_buffer_use_after_dispose").is_truthy());
}

TEST(BufferTest, DisposeIdempotent) {
    EXPECT_TRUE(run_buffer_test("test_buffer_dispose_idempotent").is_truthy());
}

TEST(BufferTest, FromPtrNonOwning) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_ptr_non_owning").is_truthy());
}

TEST(BufferTest, FromStringAscii) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_string_ascii").is_truthy());
}

TEST(BufferTest, FromStringUtf8) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_string_utf8").is_truthy());
}

TEST(BufferTest, FromStringMultibyte) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_string_multibyte").is_truthy());
}

TEST(BufferTest, AsStringAscii) {
    EXPECT_TRUE(run_buffer_test("test_buffer_as_string_ascii").is_truthy());
}

TEST(BufferTest, StringRoundtrip) {
    EXPECT_TRUE(run_buffer_test("test_buffer_string_roundtrip").is_truthy());
}

TEST(BufferTest, FromStringEmpty) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_string_empty").is_truthy());
}

TEST(BufferTest, AsStringEmpty) {
    EXPECT_TRUE(run_buffer_test("test_buffer_as_string_empty").is_truthy());
}

TEST(BufferTest, FromStringNull) {
    EXPECT_TRUE(run_buffer_test("test_buffer_from_string_null").is_truthy());
}

TEST(BufferTest, AsStringDisposed) {
    EXPECT_TRUE(run_buffer_test("test_buffer_as_string_disposed").is_truthy());
}

TEST(BufferTest, StringViaSlice) {
    EXPECT_TRUE(run_buffer_test("test_buffer_string_via_slice").is_truthy());
}
