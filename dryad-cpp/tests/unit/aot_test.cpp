#include <gtest/gtest.h>
#include <cstdio>
#include <fstream>
#include <sys/wait.h>
#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/analyze_pass.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/bytecode_module.h"
#include "dryad/aot/native_linker.h"
#include "dryad/compiler/program_entry.h"
#include "dryad/runtime/native_module.h"
#include "dryad/runtime/loop_analyzer.h"
#include "dryad/runtime/vm.h"
#include "dryad/aot/jit_compiler.h"

using namespace dryad;

TEST(AotTest, CompileToDryadc) {
    const std::string source = R"(
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

    const std::string input = "/tmp/dryad_aot_input.dryad";
    const std::string output = "/tmp/dryad_aot_output.dryadc";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_dryadc(input, output, options);
    ASSERT_TRUE(result.success);

    CompiledProgram loaded;
    ASSERT_TRUE(load_compiled_program(output, loaded));
    EXPECT_EQ(loaded.functions.size(), 2u);
    EXPECT_EQ(loaded.functions[0].metadata.name, "compute");
    EXPECT_EQ(loaded.functions[1].metadata.name, kScriptEntryFunctionName);

    std::remove(input.c_str());
    std::remove(output.c_str());
}

TEST(AotTest, CompileToLlvmIr) {
    const std::string source = R"(
        @compile("bytecode")
        function add_floats() {
            let a = 1.5;
            let b = 2.5;
            return a + b;
        }
        add_floats();
    )";

    const std::string input = "/tmp/dryad_aot_llvm_input.dryad";
    const std::string output = "/tmp/dryad_aot_llvm_output.ll";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_llvm_ir(input, output, options);
    ASSERT_TRUE(result.success);

    std::ifstream ll(output);
    std::string content((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(content.find("define i8* @add_floats"), std::string::npos);
    EXPECT_NE(content.find("fadd double"), std::string::npos);

    std::remove(input.c_str());
    std::remove(output.c_str());
}

TEST(AotTest, AnalyzeReportsSlowPathForGenericOps) {
    const std::string source = R"(
        function mixed(flag) {
            let value = 0;
            if (flag) {
                value = "text";
            }
            return value;
        }
        mixed(true);
        mixed(false);
    )";

    const std::string input = "/tmp/dryad_analyze_input.dryad";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    AnalyzeReport report = run_analyze_pass(input, options);
    EXPECT_GT(report.slow_path_sites, 0u);
    EXPECT_FALSE(report.format().empty());

    std::remove(input.c_str());
}

TEST(AotTest, MicroIrLowersComparisonOpcodes) {
    const std::string source = R"(
        let score = 85;
        let ok = score >= 90;
        let low = score <= 70;
        let eq = score == 85;
        ok;
        low;
        eq;
    )";

    const std::string input = "/tmp/dryad_micro_ir_cmp.dryad";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    CompiledProgram program = compile_file_to_bytecode(input, options);
    IrModule ir = bytecode_to_micro_ir(program);
    EXPECT_EQ(ir.functions.size(), 1u);

    int cmp_ge = 0;
    int cmp_le = 0;
    int cmp_eq = 0;
    for (const auto& inst : ir.functions[0].instructions) {
        if (inst.op == IrOpcode::CmpGeI32) {
            ++cmp_ge;
        }
        if (inst.op == IrOpcode::CmpLeI32) {
            ++cmp_le;
        }
        if (inst.op == IrOpcode::CmpEqI32) {
            ++cmp_eq;
        }
    }
    EXPECT_GE(cmp_ge, 1);
    EXPECT_GE(cmp_le, 1);
    EXPECT_GE(cmp_eq, 1);

    std::remove(input.c_str());
}

TEST(AotTest, ControlFlowExampleCompilesToLlvmIr) {
    const std::string input = std::string(DRYAD_EXAMPLE_DIR) + "/01-language/control_flow.dryad";
    const std::string output = "/tmp/dryad_control_flow.ll";

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_llvm_ir(input, output, options);
    ASSERT_TRUE(result.success) << result.message;

    std::ifstream ll(output);
    std::string content((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(content.find("icmp sge i32"), std::string::npos);
    EXPECT_NE(content.find("icmp sle i32"), std::string::npos);

    std::remove(output.c_str());
}

TEST(AotTest, MicroIrFromBytecode) {
    const std::string source = R"(
        function compute() {
            let sum = 0;
            return sum;
        }
        compute();
    )";

    const std::string input = "/tmp/dryad_micro_ir.dryad";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    CompiledProgram program = compile_file_to_bytecode(input, options);
    IrModule ir = bytecode_to_micro_ir(program);
    EXPECT_EQ(ir.functions.size(), 2u);
    EXPECT_FALSE(ir.functions[0].instructions.empty());
    EXPECT_EQ(ir.functions[1].metadata.name, kScriptEntryFunctionName);

    std::remove(input.c_str());
}

TEST(AotTest, NativeLinkProducesSharedObject) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        @compile("bytecode")
        function add_floats() {
            let a = 1.5;
            let b = 2.5;
            return a + b;
        }
        add_floats();
    )";

    const std::string input = "/tmp/dryad_native_input.dryad";
    const std::string output = "/tmp/dryad_native_output.so";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_native_module(
        input, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;
    EXPECT_FALSE(result.binary.empty());

    NativeModule module;
    EXPECT_TRUE(module.try_load_file(output));
    EXPECT_TRUE(module.has_function("add_floats"));

    std::remove(input.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(AotTest, NativeModuleInvokesWithParameters) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        function add(a, b) {
            return a + b;
        }
        add(10, 32);
    )";

    const std::string input = "/tmp/dryad_native_params_input.dryad";
    const std::string output = "/tmp/dryad_native_params_output.so";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_native_module(
        input, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;

    NativeModule module;
    ASSERT_TRUE(module.try_load_file(output));
    Value sum = module.invoke("add", {Value(int64_t{10}), Value(int64_t{32})});
    EXPECT_EQ(sum.as_integer(), 42);

    std::remove(input.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(AotTest, LlvmIrEmitsParameterSignature) {
    const std::string source = R"(
        function add(a, b) {
            return a + b;
        }
        add(1, 2);
    )";

    const std::string input = "/tmp/dryad_native_sig_input.dryad";
    const std::string output = "/tmp/dryad_native_sig_output.ll";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_llvm_ir(input, output, options);
    ASSERT_TRUE(result.success);

    std::ifstream ll(output);
    std::string content((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(content.find("define i8* @add(i8* %arg0, i8* %arg1)"), std::string::npos);
    EXPECT_NE(content.find("@dryad_unbox_i32"), std::string::npos);

    std::remove(input.c_str());
    std::remove(output.c_str());
}

TEST(AotTest, LoopAnalyzerFindsWhileBackEdge) {
    const std::string source = R"(
        function compute() {
            let i = 0;
            while (i < 2) {
                i = i + 1;
            }
            return i;
        }
        compute();
    )";

    const std::string input = "/tmp/dryad_loop_analyzer.dryad";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    CompiledProgram program = compile_file_to_bytecode(input, options);

    const auto loops = analyze_loops(program.functions[0].module.code);
    EXPECT_EQ(loops.size(), 1u);
    EXPECT_LT(loops[0].header_pc, loops[0].back_edge_pc);
    EXPECT_LT(loops[0].body_start, loops[0].body_end);

    std::remove(input.c_str());
}

TEST(AotTest, JitHotLoopAcceleratesIntegerWhile) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        function count() {
            let i = 0;
            while (i < 200) {
                i = i + 1;
            }
            return i;
        }
        count();
    )";

    const std::string input = "/tmp/dryad_jit_input.dryad";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    CompiledProgram program = compile_file_to_bytecode(input, options);

    JitCompiler jit;
    jit.configure(DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    jit.set_threshold(5);
    jit.bind_function(&program.functions[0]);

    VM vm;
    vm.set_jit_delegate(&jit);
    vm.enable_jit(true);
    vm.load_program(program, 0);
    vm.execute();

    EXPECT_EQ(vm.get_stack_top().as_integer(), 200);

    std::remove(input.c_str());
}

TEST(AotTest, NativeExecutableReturnsBinaryAndRuns) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        function entrypoint() {
            let sum = 0;
            let i = 0;
            while (i < 3) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        entrypoint();
    )";

    const std::string input = "/tmp/dryad_native_exec_input.dryad";
    const std::string output = "/tmp/dryad_native_exec_output";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    options.entry_function = "entrypoint";
    auto result = compile_to_native_executable(
        input, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;
    EXPECT_TRUE(is_native_binary_image(result.binary));

    const int exit_code = std::system(("\"" + output + "\"").c_str());
    EXPECT_EQ(WEXITSTATUS(exit_code), 0);

    std::remove(input.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(AotTest, CompileToDryadcReturnsBinaryBytes) {
    const std::string source = R"(
        function compute() {
            return 1;
        }
        compute();
    )";

    const std::string input = "/tmp/dryad_dryadc_binary_input.dryad";
    const std::string output = "/tmp/dryad_dryadc_binary_output.dryadc";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_dryadc(input, output, options);
    ASSERT_TRUE(result.success) << result.message;
    EXPECT_FALSE(result.binary.empty());
    EXPECT_EQ(result.binary[0], 'D');
    EXPECT_EQ(result.binary[1], 'R');
    EXPECT_EQ(result.binary[2], 'Y');
    EXPECT_EQ(result.binary[3], 'A');
    EXPECT_EQ(result.binary[4], 'D');
    EXPECT_EQ(result.binary[5], 'C');

    std::remove(input.c_str());
    std::remove(output.c_str());
}

TEST(AotTest, TopLevelScriptCompilesToExecutable) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        let sum = 0;
        let i = 0;
        while (i < 3) {
            sum = sum + i;
            i = i + 1;
        }
    )";

    const std::string input = "/tmp/dryad_toplevel_exec_input.dryad";
    const std::string output = "/tmp/dryad_toplevel_exec_output";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    auto result = compile_to_native_executable(
        input, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;
    EXPECT_TRUE(is_native_binary_image(result.binary));

    const int exit_code = std::system(("\"" + output + "\"").c_str());
    EXPECT_EQ(WEXITSTATUS(exit_code), 0);

    std::remove(input.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(AotTest, NativeExecutableEntryWithArgvParameters) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source = R"(
        function add(a, b) {
            return a + b;
        }
        add(10, 32);
    )";

    const std::string input = "/tmp/dryad_argv_exec_input.dryad";
    const std::string output = "/tmp/dryad_argv_exec_output";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    options.entry_function = "add";
    auto result = compile_to_native_executable(
        input, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;
    EXPECT_TRUE(is_native_binary_image(result.binary));

    std::ifstream ll(output + ".ll");
    std::string llvm((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(llvm.find("@dryad_argv_box_i32"), std::string::npos);
    EXPECT_NE(llvm.find("define i32 @main(i32 %argc, i8** %argv)"), std::string::npos);

    const int exit_code = std::system(("\"" + output + "\" 10 32").c_str());
    EXPECT_EQ(WEXITSTATUS(exit_code), 0);

    std::remove(input.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(AotTest, ScriptEntryUsesDryadMainSymbol) {
    const std::string source = R"(
        let ok = true;
    )";

    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();

    EXPECT_FALSE(program_has_user_functions(program.get()));
    EXPECT_TRUE(program_has_script_body(program.get()));

    auto entry = synthesize_script_entry_function(program.get());
    ASSERT_NE(entry, nullptr);
    EXPECT_EQ(entry->name, kScriptEntryFunctionName);
    EXPECT_TRUE(program->statements.empty());
}
