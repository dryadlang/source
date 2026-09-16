#include <gtest/gtest.h>
#include <cstdio>
#include <fstream>
#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/analyze_pass.h"
#include "dryad/aot/llvm_backend.h"
#include "dryad/aot/micro_ir.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/runtime/bytecode_module.h"

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
    EXPECT_EQ(loaded.functions.size(), 1u);
    EXPECT_EQ(loaded.functions[0].metadata.name, "compute");

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
    EXPECT_EQ(ir.functions.size(), 1u);
    EXPECT_FALSE(ir.functions[0].instructions.empty());

    std::remove(input.c_str());
}
