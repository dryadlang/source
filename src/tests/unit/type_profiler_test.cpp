#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/type_profiler.h"

using namespace dryad;

namespace {

void run_program(const std::string& source, Interpreter& interp) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    interp.execute(program.get());
}

} // namespace

TEST(TypeProfilerTest, ObservesNumericLocals) {
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
    run_program(source, interp);

    const FunctionProfile* profile = interp.type_profiler().get_function_profile("compute");
    ASSERT_NE(profile, nullptr);
    EXPECT_EQ(profile->invocation_count, 1u);

    const auto sum_it = profile->slots.find("sum");
    ASSERT_NE(sum_it, profile->slots.end());
    EXPECT_EQ(sum_it->second.type, ObservedType::I32);
    EXPECT_EQ(sum_it->second.confidence, TypeConfidence::OBSERVED);

    const auto i_it = profile->slots.find("i");
    ASSERT_NE(i_it, profile->slots.end());
    EXPECT_EQ(i_it->second.type, ObservedType::I32);
    EXPECT_EQ(i_it->second.confidence, TypeConfidence::OBSERVED);
}

TEST(TypeProfilerTest, StaticAnnotationOverridesObservation) {
    const std::string source = R"(
        function typed(x: number) {
            let count: i32 = 0;
            count = count + 1;
            return count;
        }
        typed(1);
    )";

    Interpreter interp;
    run_program(source, interp);

    const FunctionProfile* profile = interp.type_profiler().get_function_profile("typed");
    ASSERT_NE(profile, nullptr);

    const auto count_it = profile->slots.find("count");
    ASSERT_NE(count_it, profile->slots.end());
    EXPECT_EQ(count_it->second.type, ObservedType::I32);
    EXPECT_EQ(count_it->second.confidence, TypeConfidence::STATIC);

    const auto x_it = profile->slots.find("x");
    ASSERT_NE(x_it, profile->slots.end());
    EXPECT_EQ(x_it->second.type, ObservedType::F64);
    EXPECT_EQ(x_it->second.confidence, TypeConfidence::STATIC);
}

TEST(TypeProfilerTest, PolymorphicSlotBecomesUnknown) {
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

    Interpreter interp;
    run_program(source, interp);

    const FunctionProfile* profile = interp.type_profiler().get_function_profile("mixed");
    ASSERT_NE(profile, nullptr);

    const auto value_it = profile->slots.find("value");
    ASSERT_NE(value_it, profile->slots.end());
    EXPECT_EQ(value_it->second.type, ObservedType::Any);
    EXPECT_EQ(value_it->second.confidence, TypeConfidence::UNKNOWN);
}

TEST(TypeProfilerTest, RecordsCallSitesAndBranches) {
    const std::string source = R"(
        function helper() {
            return 1;
        }
        function caller() {
            if (true) {
                helper();
            }
            return 0;
        }
        caller();
    )";

    Interpreter interp;
    run_program(source, interp);

    const FunctionProfile* caller = interp.type_profiler().get_function_profile("caller");
    ASSERT_NE(caller, nullptr);
    EXPECT_GE(caller->call_sites.count("helper"), 1u);
    EXPECT_EQ(caller->call_sites.at("helper").call_count, 1u);

    bool found_branch = false;
    for (const auto& entry : caller->branches) {
        if (entry.first.find("if@") != std::string::npos) {
            EXPECT_GE(entry.second.taken, 1u);
            found_branch = true;
        }
    }
    EXPECT_TRUE(found_branch);
}

TEST(TypeProfilerTest, CanBeDisabled) {
    const std::string source = R"(
        function compute() {
            let sum = 0;
            return sum;
        }
        compute();
    )";

    Interpreter interp;
    interp.set_type_profiling_enabled(false);
    run_program(source, interp);

    EXPECT_EQ(interp.type_profiler().profiles().size(), 0u);
}

TEST(TypeProfilerTest, FromValueAndAnnotationHelpers) {
    EXPECT_EQ(TypeProfiler::from_value(Value(int64_t{42})), ObservedType::I32);
    EXPECT_EQ(TypeProfiler::from_value(Value(3.14)), ObservedType::F64);
    EXPECT_EQ(TypeProfiler::from_value(Value("hi")), ObservedType::String);
    EXPECT_EQ(TypeProfiler::from_annotation("i32"), ObservedType::I32);
    EXPECT_EQ(TypeProfiler::from_annotation("number"), ObservedType::F64);
    EXPECT_EQ(TypeProfiler::from_annotation("string"), ObservedType::String);
}
