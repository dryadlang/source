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

Value run_dryad(const std::string& source) {
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
    return interpreter.global_env()->get("ok");
}

} // namespace

TEST(BufferWorkflow, FileRoundtrip) {
    std::string source = R"(
        import { Buffer_FromString, Buffer_AsString, Buffer_Dispose } from "@std/buffers/buffer";

        let text = "Olá, mundo! Buffer file roundtrip.";
        let buf = Buffer_FromString(text);
        let fd = __open_write("dryad_buffer_roundtrip.txt");
        __write(fd, Buffer_AsString(buf));
        __close(fd);
        Buffer_Dispose(buf);

        let fd_read = __open_read("dryad_buffer_roundtrip.txt");
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
    std::string source = R"(
        import { Buffer_FromString, Buffer_AsString, Buffer_Write, Buffer_Dispose } from "@std/buffers/buffer";

        let buf = Buffer_FromString("HELLO");
        Buffer_Write(buf, 0, 104);
        let ok = Buffer_AsString(buf) == "hELLO";
        Buffer_Dispose(buf);
    )";
    EXPECT_TRUE(run_dryad(source).is_truthy());
}

TEST(BufferWorkflow, SliceExtraction) {
    std::string source = R"(
        import { Buffer_FromString, Buffer_AsString, Buffer_Slice, Buffer_Dispose } from "@std/buffers/buffer";

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
    std::string source = read_file(std::string(DRYAD_EXAMPLE_DIR) + "/02-buffers/buffer_usage.dryad");

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    Interpreter interp;
    interp.set_stdlib_root(DRYAD_STDLIB_DIR);
    interp.execute(program.get());

    EXPECT_TRUE(interp.global_env()->get("ok").is_truthy());
}
