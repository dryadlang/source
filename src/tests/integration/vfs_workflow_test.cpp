#include <gtest/gtest.h>
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include <stdexcept>

using namespace dryad;

namespace {

Value run_dryad(const std::string& source) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }
    interpreter.execute(program.get());
    return interpreter.global_env()->get("ok");
}

} // namespace

TEST(VFSWorkflow, NativeRoundtrip) {
    const std::string source = R"(
        import {
            VFS,
            NativeBackend,
            VFS_writeFile,
            VFS_readFile,
            VFS_exists,
            VFS_removeFile,
            VFS_dispose
        } from "@std/io/vfs";

        let vfs = VFS(NativeBackend());
        let path = "dryad_vfs_workflow.txt";
        VFS_writeFile(vfs, path, "workflow");
        let content = VFS_readFile(vfs, path);
        let existed = VFS_exists(vfs, path);
        VFS_removeFile(vfs, path);
        let ok = existed && content == "workflow" && !VFS_exists(vfs, path);
        VFS_dispose(vfs);
    )";
    EXPECT_TRUE(run_dryad(source).is_truthy());
}

TEST(VFSWorkflow, ExampleScript) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.run_file(std::string(DRYAD_EXAMPLE_DIR) + "/04-io/file_io_usage.dryad");
    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}
