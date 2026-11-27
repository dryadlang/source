#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <string>

using namespace dryad;

TEST(AsyncWorkflowTest, AsyncFunctionReturnsPromise) {
    const std::string source = R"(
        import { Promise_Resolve, Promise_Flush } from "@std/async/promise";

        async function compute() {
            let a = await Promise_Resolve(20);
            let b = await Promise_Resolve(22);
            return a + b;
        }

        let result = compute();
        Promise_Flush();
        let ok = result._state == "fulfilled" && result._value == 42;
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error()) << parser.error_message();

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.execute(program.get());

    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}
