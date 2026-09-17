#include <gtest/gtest.h>
#include <cstdio>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/runtime/vm.h"
#include "dryad/runtime/opcode.h"
#include "dryad/runtime/intrinsics_registry.h"

using namespace dryad;

namespace {

std::unique_ptr<Program> parse_program(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    return parser.parse();
}

void run_interpreter(const std::string& source, Interpreter& interp) {
    auto program = parse_program(source);
    interp.execute(program.get());
}

bool code_contains_opcode(const BytecodeModule& module, Opcode op) {
    const uint8_t target = static_cast<uint8_t>(op);
    for (uint8_t byte : module.code) {
        if (byte == target) {
            return true;
        }
    }
    return false;
}

CompiledFunction compile_named_function(const std::string& source,
                                        const std::string& function_name,
                                        const FunctionProfile* profile) {
    auto program = parse_program(source);
    FunctionDeclaration* fn = find_function_declaration(program.get(), function_name);
    if (!fn) {
        throw std::runtime_error("function not found: " + function_name);
    }
    BytecodeCompiler compiler;
    return compiler.compile_function(fn, profile);
}

} // namespace

TEST(BytecodeCompilerTest, EmitsSpecializedAddI32FromProfile) {
    const std::string source = R"(
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
    run_interpreter(source, interp);
    const FunctionProfile* profile = interp.type_profiler().get_function_profile("compute");
    ASSERT_NE(profile, nullptr);

    auto compiled = compile_named_function(source, "compute", profile);
    EXPECT_EQ(compiled.metadata.name, "compute");
    EXPECT_GE(compiled.metadata.slots.size(), 2u);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::ADD_I32));
    EXPECT_FALSE(code_contains_opcode(compiled.module, Opcode::ADD_F64));
}

TEST(BytecodeCompilerTest, VmExecutionMatchesInterpreterResult) {
    const std::string source = R"(
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
    run_interpreter(source, interp);
    const FunctionProfile* profile = interp.type_profiler().get_function_profile("compute");

    auto compiled = compile_named_function(source, "compute", profile);
    VM vm;
    vm.load(compiled.module);
    vm.execute();

    EXPECT_TRUE(vm.halted());
    EXPECT_EQ(vm.get_stack_top().as_integer(), 3);
}

TEST(BytecodeCompilerTest, StaticAnnotationEmitsAddF64) {
    const std::string source = R"(
        function add_floats() {
            let a: number = 1.5;
            let b: number = 2.5;
            return a + b;
        }
        add_floats();
    )";

    Interpreter interp;
    run_interpreter(source, interp);
    const FunctionProfile* profile = interp.type_profiler().get_function_profile("add_floats");

    auto compiled = compile_named_function(source, "add_floats", profile);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::ADD_F64));

    VM vm;
    vm.load(compiled.module);
    vm.execute();
    EXPECT_DOUBLE_EQ(vm.get_stack_top().as_float(), 4.0);
}

TEST(BytecodeCompilerTest, GetElementF64WithAnnotatedArrayParam) {
    const std::string source = R"(
        function get_at(arr: number, i: i32) {
            return arr[i];
        }
    )";

    auto compiled = compile_named_function(source, "get_at", nullptr);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::GET_ELEMENT_F64));

    Value arr = Value::create_array();
    arr.array_push(Value(2.5));
    arr.array_push(Value(4.5));

    VM vm;
    vm.load(compiled.module);
    vm.set_slot(0, arr);
    vm.set_slot(1, Value(int64_t{1}));
    vm.execute();

    EXPECT_DOUBLE_EQ(vm.get_stack_top().as_float(), 4.5);
}

TEST(BytecodeCompilerTest, MetadataHeaderPopulated) {
    const std::string source = R"(
        function compute() {
            let sum = 0;
            return sum;
        }
        compute();
    )";

    Interpreter interp;
    run_interpreter(source, interp);
    auto compiled = compile_named_function(
        source, "compute", interp.type_profiler().get_function_profile("compute"));

    EXPECT_EQ(compiled.metadata.name, "compute");
    EXPECT_GE(compiled.metadata.opcode_count, 1u);
    EXPECT_FALSE(compiled.metadata.slots.empty());

    bool found_sum = false;
    for (const auto& slot : compiled.metadata.slots) {
        if (slot.name == "sum") {
            found_sum = true;
            EXPECT_EQ(slot.type, ObservedType::I32);
            EXPECT_EQ(slot.confidence, TypeConfidence::OBSERVED);
        }
    }
    EXPECT_TRUE(found_sum);
}

TEST(BytecodeCompilerTest, CallExpressionCompilesAndRuns) {
    const std::string source = R"(
        function helper(a, b) {
            return a + b;
        }
        function driver() {
            return helper(20, 22);
        }
        driver();
    )";

    Interpreter interp;
    run_interpreter(source, interp);
    auto program = parse_program(source);
    BytecodeCompiler compiler;
    CompiledProgram compiled = compiler.compile_program(program.get(), interp.type_profiler());
    ASSERT_EQ(compiled.functions.size(), 2u);
    EXPECT_TRUE(code_contains_opcode(compiled.functions[1].module, Opcode::CALL));

    VM vm;
    vm.load_program(compiled, find_function_index(compiled, "driver"));
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST(BytecodeCompilerTest, DryadcRoundTrip) {
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

    Interpreter interp;
    run_interpreter(source, interp);
    auto program = parse_program(source);
    BytecodeCompiler compiler;
    CompiledProgram compiled = compiler.compile_program(
        program.get(), interp.type_profiler());

    const std::string path = "/tmp/dryad_test_compute.dryadc";
    ASSERT_TRUE(save_compiled_program(compiled, path));

    CompiledProgram loaded;
    ASSERT_TRUE(load_compiled_program(path, loaded));
    std::remove(path.c_str());

    EXPECT_EQ(loaded.functions.size(), 1u);
    EXPECT_EQ(loaded.functions[0].metadata.name, "compute");
    EXPECT_EQ(loaded.functions[0].module.code, compiled.functions[0].module.code);

    VM vm;
    vm.load(loaded.functions[0].module);
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 1);
}

TEST(BytecodeCompilerTest, ArrayLiteralAndIndexedAssignment) {
    const std::string source = R"(
        function build() {
            let arr = [10, 20, 30];
            arr[1] = 99;
            return arr[1];
        }
        build();
    )";

    auto compiled = compile_named_function(source, "build", nullptr);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::NEW_ARRAY));
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::ARRAY_PUSH));
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::SET_ELEMENT));

    VM vm;
    vm.load(compiled.module);
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 99);
}

TEST(BytecodeCompilerTest, ForLoopCompilesAndRuns) {
    const std::string source = R"(
        function sum_to(n) {
            let total = 0;
            for (let i = 0; i < n; i = i + 1) {
                total = total + i;
            }
            return total;
        }
        sum_to(4);
    )";

    Interpreter interp;
    run_interpreter(source, interp);
    auto compiled = compile_named_function(
        source, "sum_to", interp.type_profiler().get_function_profile("sum_to"));

    VM vm;
    vm.load(compiled.module);
    vm.set_slot(0, Value(int64_t{4}));
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 6);
}

TEST(BytecodeCompilerTest, LogicalAndOrShortCircuit) {
    const std::string source = R"(
        function test() {
            let a = true && 10;
            let b = false || 20;
            return a + b;
        }
    )";

    auto compiled = compile_named_function(source, "test", nullptr);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::JUMP_IF_FALSE));
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::JUMP_IF_TRUE));

    VM vm;
    vm.load(compiled.module);
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 30);
}

TEST(BytecodeCompilerTest, UnaryNotAndNegation) {
    const std::string source = R"(
        function test() {
            let a = !false;
            let b = -5;
            return a && b == -5;
        }
    )";

    auto compiled = compile_named_function(source, "test", nullptr);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::NOT));

    VM vm;
    vm.load(compiled.module);
    vm.execute();
    EXPECT_TRUE(vm.get_stack_top().is_truthy());
}

TEST(BytecodeCompilerTest, BreakAndContinueInForLoop) {
    const std::string source = R"(
        function test() {
            let sum = 0;
            for (let i = 0; i < 10; i = i + 1) {
                if (i == 3) {
                    continue;
                }
                if (i == 7) {
                    break;
                }
                sum = sum + i;
            }
            return sum;
        }
    )";

    auto compiled = compile_named_function(source, "test", nullptr);
    VM vm;
    vm.load(compiled.module);
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 18);
}

TEST(BytecodeCompilerTest, IntrinsicSyscallEmission) {
    IntrinsicsRegistry::instance().register_all();

    const std::string source = R"(
        function len(arr) {
            return __runtime_array_length(arr);
        }
    )";

    auto compiled = compile_named_function(source, "len", nullptr);
    EXPECT_TRUE(code_contains_opcode(compiled.module, Opcode::INTRINSIC_SYSCALL));

    Value arr = Value::create_array();
    arr.array_push(Value(int64_t{1}));
    arr.array_push(Value(int64_t{2}));

    VM vm;
    vm.load(compiled.module);
    vm.set_slot(0, arr);
    vm.execute();
    EXPECT_EQ(vm.get_stack_top().as_integer(), 2);
}
