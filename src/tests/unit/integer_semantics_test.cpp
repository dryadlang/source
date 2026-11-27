// Integer and unary-minus semantics, checked in both execution tiers. Each
// script prints its results; the interpreter and the bytecode VM must print
// the same thing. Scripts are top-level only, so in bytecode mode they run
// as __dryad_main inside the VM (no interpreter fallback).

#include <gtest/gtest.h>
#include "dryad/common/int_math.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include <cstdint>
#include <limits>
#include <string>

using namespace dryad;

namespace {

std::string run(const std::string& source, ExecutionMode mode) {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    if (parser.has_error()) {
        return "parse error: " + parser.error_message();
    }
    Interpreter interpreter;
    if (mode == ExecutionMode::Bytecode) {
        interpreter.set_default_execution_mode(ExecutionMode::Bytecode);
    }
    testing::internal::CaptureStdout();
    try {
        interpreter.execute(program.get());
    } catch (const std::exception& ex) {
        return testing::internal::GetCapturedStdout() + "error: " + ex.what() + "\n";
    }
    return testing::internal::GetCapturedStdout();
}

void expect_both(const std::string& source, const std::string& expected) {
    EXPECT_EQ(run(source, ExecutionMode::Interpreter), expected) << "interpreter\n" << source;
    EXPECT_EQ(run(source, ExecutionMode::Bytecode), expected) << "bytecode VM\n" << source;
}

constexpr int64_t kMax = std::numeric_limits<int64_t>::max();
constexpr int64_t kMin = std::numeric_limits<int64_t>::min();

} // namespace

TEST(IntMathTest, WrappingHelpers) {
    EXPECT_EQ(wrapping_add(kMax, 1), kMin);
    EXPECT_EQ(wrapping_sub(kMin, 1), kMax);
    EXPECT_EQ(wrapping_mul(kMax, 2), -2);
    EXPECT_EQ(wrapping_neg(kMin), kMin);
    EXPECT_EQ(wrapping_neg(5), -5);
    EXPECT_EQ(wrapping_div(kMin, -1), kMin);
    EXPECT_EQ(wrapping_div(-7, 2), -3);
    EXPECT_EQ(wrapping_rem(kMin, -1), 0);
    EXPECT_EQ(wrapping_rem(-7, 2), -1);
}

TEST(IntegerSemanticsTest, UnaryMinusNegates) {
    // The VM compiled `-x` as `x - 0`, so every negation was a no-op.
    expect_both("print(-5);\n"
                "let a = 5;\n"
                "print(-a);\n"
                "print(-a - 1);\n"
                "print(- -a);\n"
                "let f = 2.5;\n"
                "print(-f);\n"
                "print(10 + -a);\n",
                "-5\n-5\n-6\n5\n-2.5\n5\n");
}

TEST(IntegerSemanticsTest, UnaryMinusInsideFunctionsAndLoops) {
    expect_both("function neg(x) {\n"
                "    return -x;\n"
                "}\n"
                "let total = 0;\n"
                "for (let i = 0; i < 4; i = i + 1) {\n"
                "    total = total + neg(i);\n"
                "}\n"
                "print(total);\n"
                "print(neg(-3));\n",
                "-6\n3\n");
}

TEST(IntegerSemanticsTest, UnaryMinusRejectsNonNumbers) {
    // Same message in both tiers (the VM used to report a SUB error).
    for (const char* source : {"let s = \"x\";\nprint(-s);\n", "print(-true);\n", "print(-null);\n"}) {
        expect_both(source, "error: Unary minus requires numeric operand\n");
    }
}

TEST(IntegerSemanticsTest, NegatingZeroFloatGivesNegativeZero) {
    // 0 - 0.0 is +0.0; -(0.0) is -0.0. The VM matches the interpreter.
    expect_both("let z = 0.0;\nprint(-z);\nprint(1.0 / -z);\n", "-0\n-inf\n");
}

TEST(IntegerSemanticsTest, OverflowWrapsInBothTiers) {
    expect_both("let max = 9223372036854775807;\n"
                "let min = -9223372036854775807 - 1;\n"
                "print(min);\n"
                "print(max + 1);\n"
                "print(min - 1);\n"
                "print(max * 2);\n"
                "print(-min);\n",
                "-9223372036854775808\n-9223372036854775808\n9223372036854775807\n-2\n"
                "-9223372036854775808\n");
}

TEST(IntegerSemanticsTest, MinDividedByMinusOneDoesNotTrap) {
    // INT64_MIN / -1 and % -1 raised SIGFPE and killed the process.
    expect_both("let min = -9223372036854775807 - 1;\n"
                "print(min / -1);\n"
                "print(min % -1);\n"
                "print(-7 / 2);\n"
                "print(-7 % 2);\n",
                "-9223372036854775808\n0\n-3\n-1\n");
}

TEST(IntegerSemanticsTest, IntegerLiteralOutOfRangeIsAParseError) {
    Lexer lexer("let x = 1;\nlet y = 9223372036854775808;\n");
    Parser parser(lexer.tokenize());
    parser.parse();
    ASSERT_TRUE(parser.has_error());
    EXPECT_NE(parser.error_message().find("Integer literal out of range: 9223372036854775808"),
              std::string::npos)
        << parser.error_message();
    ASSERT_FALSE(parser.diagnostics().empty());
    EXPECT_EQ(parser.diagnostics().front().location.line, 2);
}
