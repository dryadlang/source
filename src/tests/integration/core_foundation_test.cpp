#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(CoreFoundation, BufferVfsAndExceptionsViaImports) {
    std::string source = R"(
        import { Buffer_New, Buffer_Write, Buffer_Read, Buffer_Dispose } from "@std/buffers/buffer";
        import { VFS, MemoryBackend, VFS_writeFile, VFS_readFile } from "@std/io/vfs";
        import { ArgumentNullException } from "@std/core/exceptions";

        let buf = Buffer_New(4);
        Buffer_Write(buf, 0, 65);
        let byteOk = Buffer_Read(buf, 0) == 65;
        Buffer_Dispose(buf);

        let vfs = VFS(MemoryBackend());
        VFS_writeFile(vfs, "/core.txt", "foundation");
        let ioOk = VFS_readFile(vfs, "/core.txt") == "foundation";

        let err = ArgumentNullException("x");
        let errOk = err != null;

        let ok = byteOk && ioOk && errOk;
    )";

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.execute(program.get());

    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}
