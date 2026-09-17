#include <gtest/gtest.h>
#include "dryad/common/utils.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/execution_engine.h"
#include "dryad/aot/aot_compiler.h"
#include <fstream>

using namespace dryad;

namespace {

std::unique_ptr<Program> parse(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    return parser.parse();
}

FunctionDeclaration* find_fn(Program* program, const std::string& name) {
    return find_function_declaration(program, name);
}

} // namespace

TEST(ExecutionEngineTest, ParsesCompileDecoratorOnFunction) {
    const std::string source = R"(
        @compile("bytecode")
        function hot() {
            let x = 1;
            return x;
        }
    )";
    auto program = parse(source);
    FunctionDeclaration* fn = find_fn(program.get(), "hot");
    ASSERT_NE(fn, nullptr);
    EXPECT_EQ(fn->compile_mode, ExecutionMode::Bytecode);
}

TEST(ExecutionEngineTest, ParsesCompileBlock) {
    const std::string source = R"(
        function mixed() {
            let x = 0;
            @compile("bytecode") {
                x = 1;
            }
            return x;
        }
    )";
    auto program = parse(source);
    FunctionDeclaration* fn = find_fn(program.get(), "mixed");
    ASSERT_NE(fn, nullptr);
    bool found_compile_block = false;
    for (const auto& stmt : fn->body->statements) {
        if (stmt->type == ASTNodeType::CompileBlockStatement) {
            found_compile_block = true;
            EXPECT_EQ(static_cast<CompileBlockStatement*>(stmt.get())->mode,
                      ExecutionMode::Bytecode);
        }
    }
    EXPECT_TRUE(found_compile_block);
}

TEST(ExecutionEngineTest, BytecodeFunctionExecutesViaVm) {
    const std::string source = R"(
        @compile("bytecode")
        function compute() {
            let sum = 0;
            let i = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        compute();
    )";

    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());

    FunctionDeclaration* fn = find_fn(program.get(), "compute");
    ASSERT_NE(fn, nullptr);

    Value result = interp.execution_engine().invoke_function(fn, nullptr, {});
    EXPECT_EQ(result.as_integer(), 3);
}

TEST(ExecutionEngineTest, InterpreterOverrideWithCompileDecorator) {
    const std::string source = R"(
        @compile("interpreter")
        function compute() {
            let sum = 0;
            let i = 0;
            while (i < 2) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        compute();
    )";

    Interpreter interp;
    interp.set_default_execution_mode(ExecutionMode::Bytecode);
    auto program = parse(source);
    interp.execute(program.get());

    FunctionDeclaration* fn = find_fn(program.get(), "compute");
    EXPECT_EQ(interp.execution_engine().effective_mode(fn), ExecutionMode::Interpreter);
}

TEST(ExecutionEngineTest, TransitiveBytecodeFromCompileBlock) {
    const std::string source = R"(
        function helper() {
            let sum = 0;
            let i = 0;
            while (i < 2) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        function driver() {
            @compile("bytecode") {
                return helper();
            }
            return 0;
        }
        driver();
    )";

    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());
}

TEST(ExecutionEngineTest, BytecodeCrossFunctionCall) {
    const std::string source = R"(
        function helper(a, b) {
            return a + b;
        }
        @compile("bytecode")
        function driver() {
            return helper(20, 22);
        }
        driver();
    )";

    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());

    FunctionDeclaration* driver = find_fn(program.get(), "driver");
    ASSERT_NE(driver, nullptr);

    Value result = interp.execution_engine().invoke_function(driver, nullptr, {});
    EXPECT_EQ(result.as_integer(), 42);
}

TEST(ExecutionEngineTest, AotModeThrowsWithoutCompanion) {
    const std::string source = R"(
        @compile("AOT")
        function hot() {
            return 1;
        }
    )";
    auto program = parse(source);
    Interpreter interp;
    FunctionDeclaration* fn = find_fn(program.get(), "hot");
    ASSERT_NE(fn, nullptr);
    EXPECT_THROW(interp.execution_engine().invoke_function(fn, nullptr, {}), DryadException);
}

TEST(ExecutionEngineTest, AotModeUsesDryadcCompanion) {
    const std::string source = R"(
        @compile("AOT")
        function compute() {
            let sum = 0;
            let i = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        compute();
    )";

    const std::string input = "/tmp/dryad_aot_runtime_input.dryad";
    const std::string dryadc = "/tmp/dryad_aot_runtime_input.dryadc";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    ASSERT_TRUE(compile_to_dryadc(input, dryadc, options).success);

    Interpreter interp;
    interp.execution_engine().set_source_path(input);
    auto program = parse(source);
    FunctionDeclaration* fn = find_fn(program.get(), "compute");
    ASSERT_NE(fn, nullptr);

    Value result = interp.execution_engine().invoke_function(fn, nullptr, {});
    EXPECT_EQ(result.as_integer(), 3);

    std::remove(input.c_str());
    std::remove(dryadc.c_str());
}

TEST(ExecutionEngineTest, BytecodeClassMethodCall) {
    const std::string source = R"(
        class Counter {
            constructor() {
                this.value = 0;
            }
            add(n) {
                this.value = this.value + n;
                return this.value;
            }
        }

        @compile("bytecode")
        function run() {
            let c = new Counter();
            return c.add(41);
        }
        run();
    )";

    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());

    FunctionDeclaration* fn = find_fn(program.get(), "run");
    ASSERT_NE(fn, nullptr);

    Value result = interp.execution_engine().invoke_function(fn, nullptr, {});
    EXPECT_EQ(result.as_integer(), 41);
}

TEST(ExecutionEngineTest, BytecodeCallsStdlibBufferViaGlobal) {
    const std::string source = R"(
        import {
            Buffer_FromString,
            Buffer_AsString,
            Buffer_Dispose
        } from "@std/buffers/buffer";

        @compile("bytecode")
        function probe() {
            let buf = Buffer_FromString("hi");
            let ok = Buffer_AsString(buf) == "hi";
            Buffer_Dispose(buf);
            return ok;
        }
        probe();
    )";

    Interpreter interp;
    interp.set_stdlib_root(DRYAD_STDLIB_DIR);
    auto program = parse(source);
    interp.execute(program.get());

    FunctionDeclaration* fn = find_fn(program.get(), "probe");
    ASSERT_NE(fn, nullptr);

    Value result = interp.execution_engine().invoke_function(fn, nullptr, {});
    EXPECT_TRUE(result.is_truthy());
}
