#include <gtest/gtest.h>
#include "dryad/common/utils.h"
#include "dryad/compiler/bytecode_compiler.h"
#include "dryad/compiler/closure_analysis.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/program_entry.h"
#include <string>

using namespace dryad;

namespace {

std::unique_ptr<Program> parse(const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    return parser.parse();
}

bool compiles_to_bytecode(const std::string& source, const std::string& function_name) {
    Interpreter interp;
    auto program = parse(source);
    BytecodeCompiler compiler;
    compiler.set_interpreter(&interp);
    const CompiledProgram compiled = compiler.compile_program(program.get(), interp.type_profiler());
    for (const auto& function : compiled.functions) {
        if (function.metadata.name == function_name) {
            return true;
        }
    }
    return false;
}

Value run_and_get(const std::string& source, const std::string& name) {
    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());
    return interp.global_env()->get(name);
}

std::string run_vm_script(const std::string& source) {
    Interpreter interp;
    interp.set_default_execution_mode(ExecutionMode::Bytecode);
    auto program = parse(source);
    testing::internal::CaptureStdout();
    interp.execute(program.get());
    return testing::internal::GetCapturedStdout();
}

FunctionExpression* first_lambda(Program* program, const std::string& variable) {
    for (auto& stmt : program->statements) {
        if (stmt->type != ASTNodeType::VariableDeclaration) {
            continue;
        }
        auto* decl = static_cast<VariableDeclaration*>(stmt.get());
        if (decl->name == variable && decl->initializer &&
            decl->initializer->type == ASTNodeType::FunctionExpression) {
            return static_cast<FunctionExpression*>(decl->initializer.get());
        }
    }
    return nullptr;
}

} // namespace

TEST(ClosureAnalysisTest, FreeNamesExcludeParamsAndLocals) {
    auto program = parse(R"(
        let f = function(a) {
            let b = a + c;
            d = b;
            return function(e) { return e + g + a; };
        };
    )");
    FunctionExpression* f = first_lambda(program.get(), "f");
    ASSERT_NE(f, nullptr);
    const auto free = free_names(f->parameters, block_statements_of(f->body.get()));
    EXPECT_EQ(free, (std::vector<std::string>{"c", "d", "g"}));
}

TEST(ClosureAnalysisTest, CapturedNamesAreLocalsUsedByInnerLambdas) {
    auto program = parse(R"(
        let f = function(a, unused) {
            let b = 1;
            let c = 2;
            if (true) {
                let d = 3;
            }
            return function() { return a + b + d + outside; };
        };
    )");
    FunctionExpression* f = first_lambda(program.get(), "f");
    ASSERT_NE(f, nullptr);
    const auto captured = captured_names(f->parameters, block_statements_of(f->body.get()));
    EXPECT_EQ(captured, (std::set<std::string>{"a", "b", "d"}));
}

TEST(BytecodeClosureTest, NamedFunctionReturnsCapturingClosure) {
    const std::string source = R"(
        @compile("bytecode")
        function make_adder(n) {
            return function(x) {
                return x + n;
            };
        }
        let add5 = make_adder(5);
        let r = add5(2);
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "make_adder"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 7);
}

TEST(BytecodeClosureTest, CapturedVariableIsSharedAndMutable) {
    const std::string source = R"(
        @compile("bytecode")
        function make_counter() {
            let c = 0;
            return function() {
                c = c + 1;
                return c;
            };
        }
        let a = make_counter();
        let b = make_counter();
        a();
        a();
        b();
        let r = a() * 10 + b();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "make_counter"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 32);
}

TEST(BytecodeClosureTest, OuterSeesWritesMadeByClosure) {
    const std::string source = R"(
        @compile("bytecode")
        function run() {
            let total = 1;
            let bump = function(by) {
                total = total + by;
                return total;
            };
            bump(2);
            bump(3);
            return total;
        }
        let r = run();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "run"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 6);
}

TEST(BytecodeClosureTest, LoopBodyDeclarationsGetFreshBindings) {
    const std::string source = R"(
        @compile("bytecode")
        function make_all() {
            let fns = [null, null, null];
            let i = 0;
            while (i < 3) {
                let v = i * 10;
                fns[i] = function() { return v; };
                i = i + 1;
            }
            return fns;
        }
        let fns = make_all();
        let first = fns[0];
        let last = fns[2];
        let r = first() + last();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "make_all"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 20);
}

TEST(BytecodeClosureTest, CapturesPassThroughNestedLambdas) {
    const std::string source = R"(
        @compile("bytecode")
        function outer(a) {
            return function(b) {
                return function(c) {
                    return a * 100 + b * 10 + c;
                };
            };
        }
        let mid = outer(1);
        let inner = mid(2);
        let r = inner(3);
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "outer"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 123);
}

TEST(BytecodeClosureTest, LambdasInSeveralNamedFunctionsKeepDistinctIndices) {
    const std::string source = R"(
        function helper(v) {
            return v + 1000;
        }
        @compile("bytecode")
        function tens() {
            return function(y) { return y * 10; };
        }
        @compile("bytecode")
        function hundreds() {
            return function(y) { return y * 100; };
        }
        let t = tens();
        let h = hundreds();
        let r = t(2) + h(3) + helper(0);
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "tens"));
    EXPECT_TRUE(compiles_to_bytecode(source, "hundreds"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 1320);
}

TEST(BytecodeClosureTest, NamedFunctionCallsParameterFunction) {
    const std::string source = R"(
        function inc(v) {
            return v + 1;
        }
        @compile("bytecode")
        function apply(f, x) {
            return f(x);
        }
        let r = apply(inc, 41);
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "apply"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 42);
}

TEST(BytecodeClosureTest, CallValueEvaluatesArgumentsBeforeCallee) {
    const std::string source = R"(
        let log = "";
        function note(s) {
            log = log + s;
            return s;
        }
        function pick() {
            note("c");
            return function(a, b) { return a + b; };
        }
        @compile("bytecode")
        function run() {
            return pick()(note("a"), note("b"));
        }
        let r = run();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "run"));
    Interpreter interp;
    auto program = parse(source);
    interp.execute(program.get());
    EXPECT_EQ(interp.global_env()->get("r").as_string(), "ab");
    EXPECT_EQ(interp.global_env()->get("log").as_string(), "abc");
}

TEST(BytecodeClosureTest, NamedFunctionAssignsScriptGlobal) {
    const std::string source = R"(
        let counter = 10;
        @compile("bytecode")
        function bump() {
            counter = counter + 1;
            return counter;
        }
        bump();
        let r = bump();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "bump"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 12);
    EXPECT_EQ(run_and_get(source, "counter").as_integer(), 12);
}

TEST(BytecodeClosureTest, PureVmScriptLambdaCapturesScriptVariable) {
    const std::string out = run_vm_script(R"(
        let base = 5;
        let add_base = function(x) {
            return x + base;
        };
        base = 7;
        print(add_base(2));
    )");
    EXPECT_EQ(out, "9\n");
}

TEST(BytecodeClosureTest, PureVmScriptCallsCallResult) {
    const std::string out = run_vm_script(R"(
        let make = function() {
            return function(y) { return y * 3; };
        };
        print(make()(4));
    )");
    EXPECT_EQ(out, "12\n");
}

TEST(BytecodeClosureTest, PureVmScriptUsesBytecodeEntry) {
    const std::string source = R"(
        let base = 5;
        let add_base = function(x) { return x + base; };
        print(add_base(1));
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, kScriptEntryFunctionName));
}

TEST(BytecodeClosureTest, RecursiveLambdaSeesItsOwnBinding) {
    const std::string source = R"(
        @compile("bytecode")
        function run(n) {
            let fact = function(k) {
                if (k < 2) {
                    return 1;
                }
                return k * fact(k - 1);
            };
            return fact(n);
        }
        let r = run(5);
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "run"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 120);
}

TEST(BytecodeClosureTest, PureVmScriptRecursiveLambda) {
    const std::string out = run_vm_script(R"(
        let fib = function(n) {
            if (n < 2) {
                return n;
            }
            return fib(n - 1) + fib(n - 2);
        };
        print(fib(10));
    )");
    EXPECT_EQ(out, "55\n");
}

TEST(BytecodeClosureTest, ForInitializerBindingIsSharedAcrossIterations) {
    const std::string source = R"(
        @compile("bytecode")
        function make_all() {
            let fns = [null, null, null];
            for (let i = 0; i < 3; i = i + 1) {
                fns[i] = function() { return i; };
            }
            return fns;
        }
        let fns = make_all();
        let first = fns[0];
        let r = first();
    )";
    EXPECT_TRUE(compiles_to_bytecode(source, "make_all"));
    EXPECT_EQ(run_and_get(source, "r").as_integer(), 3);
}
