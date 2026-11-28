// Object literals and `x[key]` element access. Every script runs in the
// interpreter and as __dryad_main in the bytecode VM; both must print the
// same thing.

#include <gtest/gtest.h>
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/validator.h"
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

std::string parse_errors(const std::string& source) {
    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    parser.parse();
    return parser.has_error() ? parser.error_message() : "";
}

} // namespace

TEST(ObjectLiteralTest, BuildsNestedObjects) {
    expect_both("let o = { a: 1, \"b c\": \"two\", nested: { list: [1, { deep: true }] }, };\n"
                "print(o.a);\n"
                "print(o[\"b c\"]);\n"
                "print(o.nested.list[1].deep);\n"
                "print(__runtime_value_kind(o));\n"
                "print(__runtime_value_kind({}));\n"
                "print(__runtime_array_length(__runtime_object_keys(o)));\n",
                "1\ntwo\ntrue\nobject\nobject\n3\n");
}

TEST(ObjectLiteralTest, KeywordIntegerAndShorthandKeys) {
    expect_both("let name = \"dryad\";\n"
                "let o = { set: 1, from: 2, class: 3, null: 4, 7: \"seven\", name };\n"
                "print(o[\"set\"]);\n"
                "print(o[\"from\"]);\n"
                "print(o[\"class\"]);\n"
                "print(o[\"null\"]);\n"
                "print(o[\"7\"]);\n"
                "print(o.name);\n",
                "1\n2\n3\n4\nseven\ndryad\n");
}

TEST(ObjectLiteralTest, ValuesEvaluateLeftToRightAndLastDuplicateWins) {
    expect_both("let log = [];\n"
                "function note(x) {\n"
                "    __runtime_array_push(log, x);\n"
                "    return x;\n"
                "}\n"
                "let o = { b: note(1), a: note(2), b: note(3) };\n"
                "print(log[0]);\n"
                "print(log[1]);\n"
                "print(log[2]);\n"
                "print(o.b);\n"
                "print(__runtime_array_length(__runtime_object_keys(o)));\n",
                "1\n2\n3\n3\n2\n");
}

TEST(ObjectLiteralTest, WorksInsideFunctionsAndClosures) {
    expect_both("function point(x, y) {\n"
                "    return { x: x, y: y, sum: x + y };\n"
                "}\n"
                "function counter() {\n"
                "    let n = 0;\n"
                "    return { next: function() { n = n + 1; return n; } };\n"
                "}\n"
                "let p = point(2, 3);\n"
                "print(p.sum);\n"
                "let c = counter();\n"
                "let next = c.next;\n"
                "next();\n"
                "print(next());\n",
                "5\n2\n");
}

TEST(ObjectLiteralTest, StatementBracesAreStillBlocks) {
    expect_both("let x = 1;\n"
                "{\n"
                "    let y = 2;\n"
                "    print(x + y);\n"
                "}\n"
                "if (true) { print(\"block\"); }\n",
                "3\nblock\n");
}

TEST(ObjectLiteralTest, ReportsMalformedLiterals) {
    EXPECT_NE(parse_errors("let o = { a 1 };").find("Expected ':' after object key"), std::string::npos);
    EXPECT_NE(parse_errors("let o = { a: 1 b: 2 };").find("Expected '}' after object literal"), std::string::npos);
    EXPECT_NE(parse_errors("let o = { [1]: 2 };").find("Expected property name in object literal"),
              std::string::npos);
    EXPECT_NE(parse_errors("let o = { \"s\" };").find("Expected ':' after object key"), std::string::npos);
}

TEST(ObjectLiteralTest, CheckAcceptsObjectLiterals) {
    auto result = ProgramValidator::validate_source("let v = 1;\nlet o = { a: v, b: [v] };\nlet w = o.a;\nw = w + 1;\n");
    EXPECT_TRUE(result.ok) << result.message();
    auto undefined = ProgramValidator::validate_source("let o = { a: missing };\no.a = 1;\n");
    EXPECT_FALSE(undefined.ok);
}

TEST(ElementAccessTest, ObjectsAndInstancesByStringKey) {
    // The VM used to reject o["k"] = v ("SET_ELEMENT requires array").
    expect_both("let o = {};\n"
                "o[\"k\"] = 5;\n"
                "print(o[\"k\"]);\n"
                "print(o.k);\n"
                "print(o[\"missing\"]);\n"
                "class Box {\n"
                "    constructor() {\n"
                "        this.v = 1;\n"
                "    }\n"
                "}\n"
                "let b = new Box();\n"
                "b[\"v\"] = 9;\n"
                "print(b[\"v\"]);\n"
                "print(b.v);\n",
                "5\n5\nnull\n9\n9\n");
}

TEST(ElementAccessTest, ArraysByIntegerIndex) {
    expect_both("let a = [1, 2, 3];\n"
                "a[1] = 20;\n"
                "print(a[1]);\n"
                "print(a[2]);\n",
                "20\n3\n");
}

TEST(ElementAccessTest, SameErrorsInBothTiers) {
    expect_both("let o = {};\nprint(o[1]);\n", "error: Object key must be a string\n");
    expect_both("let o = {};\no[1] = 2;\n", "error: Object key must be a string\n");
    expect_both("let a = [1];\nprint(a[\"0\"]);\n", "error: Array index must be an integer\n");
    expect_both("let a = [1];\na[\"0\"] = 2;\n", "error: Array index must be an integer\n");
    expect_both("let a = [1];\nprint(a[5]);\n", "error: Array index out of bounds\n");
    expect_both("let a = [1];\na[0 - 1] = 2;\n", "error: Array index out of bounds\n");
    expect_both("let n = 5;\nprint(n[0]);\n", "error: Cannot access member of non-object/array value\n");
    // Assigning into a non-container used to be silently ignored by the interpreter.
    expect_both("let n = 5;\nn[0] = 1;\nprint(n);\n",
                "error: Cannot assign to member of non-object/array value\n");
}
