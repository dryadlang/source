#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <stdexcept>

using namespace dryad;

namespace {

Value run_workflow(const std::string& source) {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
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

TEST(FileIOWorkflow, CompleteFileWorkflow) {
    const std::string source = R"(
        import { File_open, File_exists, File_delete, File_readAllText, File_writeAllText, FileMode } from "@std/io/file";
        import { Path } from "@std/io/path";
        import { Buffer_New, Buffer_Dispose } from "@std/buffers/buffer";

        let path = Path.join("dryad_workflow", "nested.txt");
        File_writeAllText(path, "Test content");
        let exists = File_exists(path);
        let content = File_readAllText(path);
        File_delete(path);
        let removed = !File_exists(path);

        File_writeAllText("dryad_src.txt", "Source");
        let src = File_open("dryad_src.txt", FileMode.Read);
        let dst = File_open("dryad_dst.txt", FileMode.Write);
        let buf = Buffer_New(1024);
        let n = src.read(buf, 0, 1024);
        dst.write(buf, 0, n);
        Buffer_Dispose(buf);
        src.close();
        dst.close();

        let copied = File_readAllText("dryad_dst.txt");
        File_delete("dryad_src.txt");
        File_delete("dryad_dst.txt");

        let ok = exists && content == "Test content" && removed && copied == "Source";
    )";

    EXPECT_TRUE(run_workflow(source).is_truthy());
}
