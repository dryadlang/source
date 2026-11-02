#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/common/utils.h"
#include <stdexcept>

using namespace dryad;

namespace {

Value run_source(const std::string& source, const std::string& variable = "result") {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("Parser error: " + parser.error_message());
    }

    Interpreter interpreter;
    interpreter.execute(program.get());
    return interpreter.global_env()->get(variable);
}

} // namespace

TEST(LanguageFeaturesTest, MemberAccessGetSetDelete) {
    const std::string source = R"(
        class Box {
            constructor() {
                this._value = 0;
            }

            get(index) {
                return this._value + index;
            }

            set(index, value) {
                this._value = value;
                return null;
            }

            delete(index) {
                this._value = 0;
                return true;
            }
        }

        let box = new Box();
        box.set(2, 5);
        let read = box.get(3);
        let cleared = box.delete(0);
        let result = read == 8 && cleared && box.get(0) == 0;
    )";

    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, FunctionExpressionAsArgument) {
    const std::string source = R"(
        function apply(fn, value) {
            return fn(value);
        }

        let result = apply(function(x) {
            return x + 1;
        }, 41);
    )";

    auto result = run_source(source);
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 42);
}

TEST(LanguageFeaturesTest, DefaultParameterValues) {
    const std::string source = R"(
        function greet(name, suffix = "!") {
            return name + suffix;
        }

        let with_default = greet("Dryad");
        let with_custom = greet("Dryad", "?");
        let result = with_default == "Dryad!" && with_custom == "Dryad?";
    )";

    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, ConstructorDefaultParameters) {
    const std::string source = R"(
        class Counter {
            constructor(start = 10) {
                this._value = start;
            }

            value() {
                return this._value;
            }
        }

        let default_counter = new Counter();
        let custom_counter = new Counter(3);
        let result = default_counter.value() == 10 && custom_counter.value() == 3;
    )";

    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, CatchFinallyAsMemberAccess) {
    const std::string source = R"(
        class Box {
            constructor(value) {
                this._value = value;
            }

            catch(handler) {
                return handler(this._value);
            }

            finally(handler) {
                handler();
                return this._value;
            }
        }

        let box = new Box(42);
        let from_catch = box.catch(function(v) { return v + 1; });
        let from_finally = box.finally(function() { return null; });
        let result = from_catch == 43 && from_finally == 42;
    )";

    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, ParserAcceptsReservedMemberNamesAfterDot) {
    const std::string source = R"(
        class Holder {
            constructor() {
                this._value = 7;
            }

            get() {
                return this._value;
            }
        }

        let holder = new Holder();
        let result = holder.get();
    )";

    auto result = run_source(source);
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 7);
}

TEST(LanguageFeaturesTest, BitwiseOperations) {
    const std::string source = R"(
        let a = 6 & 3;
        let b = 6 | 3;
        let c = 6 ^ 3;
        let d = ~0;
        let e = 1 << 4;
        let f = -16 >> 2;
        let result = a == 2 && b == 7 && c == 5 && d == -1 && e == 16 && f == -4;
    )";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, BitwisePrecedence) {
    const std::string source = R"(
        let result = (1 | 2 & 3) == 3 && (7 ^ 3 & 1) == 6 && (1 << 2 + 1) == 8;
    )";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, BitwiseRejectsBooleans) {
    // `1 & 3 == 1` parses as `1 & (3 == 1)`; equality yields a boolean, which
    // bitwise operators reject instead of coercing to 0/1.
    EXPECT_THROW(run_source("let result = 1 & 3 == 1;"), DryadException);
}

TEST(LanguageFeaturesTest, ShiftCountsAreMasked) {
    const std::string source = R"(
        let result = (1 << 64) == 1 && (1 << 65) == 2 && (16 >> 64) == 16;
    )";
    EXPECT_TRUE(run_source(source).is_truthy());
}

TEST(LanguageFeaturesTest, BitwiseRejectsFloats) {
    EXPECT_THROW(run_source("let result = 1.5 & 1;"), DryadException);
    EXPECT_THROW(run_source("let result = 1 | 2.5;"), DryadException);
    EXPECT_THROW(run_source("let result = ~1.5;"), DryadException);
}
