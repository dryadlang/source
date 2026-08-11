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

std::string stdlib_source() {
    static const std::string exceptions = read_file(
        std::string(DRYAD_STDLIB_DIR) + "/@std/core/exceptions.dryad");
    static const std::string buffer = read_file(
        std::string(DRYAD_STDLIB_DIR) + "/@std/buffers/buffer.dryad");
    return exceptions + "\n" + buffer + "\n";
}

Value run_dryad(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }
    Interpreter interpreter;
    interpreter.execute(program.get());
    return interpreter.global_env()->get("ok");
}

} // namespace

TEST(BufferWorkflow, FileRoundtrip) {
    std::string source = stdlib_source() + R"(
        let text = "Olá, mundo! Buffer file roundtrip.";
        let buf = Buffer_FromString(text);
        // MSYS2 native flags: O_WRONLY(1) | O_CREAT(256) | O_TRUNC(512) = 769.
        // Linux uses O_CREAT=64 (=> 577); the intrinsic contract is native flags.
        let fd = __open("dryad_buffer_roundtrip.txt", 769);
        __write(fd, Buffer_AsString(buf));
        __close(fd);
        Buffer_Dispose(buf);

        let fd_read = __open("dryad_buffer_roundtrip.txt", 0);
        let content = __read(fd_read, 1024);
        __close(fd_read);
        __unlink("dryad_buffer_roundtrip.txt");

        let buf2 = Buffer_FromString(content);
        let ok = Buffer_AsString(buf2) == text;
        Buffer_Dispose(buf2);
    )";
    EXPECT_TRUE(run_dryad(source).is_truthy());
}

TEST(BufferWorkflow, ByteManipulation) {
    std::string source = stdlib_source() + R"(
        let buf = Buffer_FromString("HELLO");
        Buffer_Write(buf, 0, 104);
        let ok = Buffer_AsString(buf) == "hELLO";
        Buffer_Dispose(buf);
    )";
    EXPECT_TRUE(run_dryad(source).is_truthy());
}

TEST(BufferWorkflow, SliceExtraction) {
    std::string source = stdlib_source() + R"(
        let buf = Buffer_FromString("name=Alice&age=30");
        let value = Buffer_Slice(buf, 5, 10);
        let rest = Buffer_Slice(buf, 11, 17);
        let ok1 = Buffer_AsString(value) == "Alice";
        let ok2 = Buffer_AsString(rest) == "age=30";
        Buffer_Dispose(value);
        Buffer_Dispose(rest);
        Buffer_Dispose(buf);
        let ok = ok1 && ok2;
    )";
    EXPECT_TRUE(run_dryad(source).is_truthy());
}

TEST(BufferWorkflow, ExampleScriptRuns) {
    std::string source = stdlib_source() + read_file(
        std::string(DRYAD_EXAMPLE_DIR) + "/buffer_usage.dryad");

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    // The example performs assertions into `ok` (stdout capture is unreliable
    // on MSYS2/UCRT64 consoles, see pre-existing NativePrintFunction failure).
    Interpreter interp;
    interp.execute(program.get());

    EXPECT_TRUE(interp.global_env()->get("ok").is_truthy());
}
