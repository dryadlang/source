#include <gtest/gtest.h>
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/validator.h"
#include "dryad/common/utils.h"

using namespace dryad;

namespace {

Value run_and_get(const std::string& code, const std::string& name) {
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error(parser.error_message());
    }
    Interpreter interpreter;
    interpreter.execute(program.get());
    return interpreter.global_env()->get(name);
}

} // namespace

TEST(LspFoundationTest, ExplicitGetterAndSetter) {
    const std::string code = R"(
class Counter {
    constructor() {
        this._value = 0;
    }

    get value() {
        return this._value;
    }

    set value(next) {
        this._value = next;
    }
}

let c = new Counter();
c.value = 5;
let result = c.value;
    )";

    auto result = run_and_get(code, "result");
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 5);
}

TEST(LspFoundationTest, GetterUsesBackingField) {
    const std::string code = R"(
class Sensor {
    constructor() {
        this._canRead = true;
    }

    get canRead() {
        return this._canRead;
    }
}

let s = new Sensor();
let readable = s.canRead;
    )";

    auto readable = run_and_get(code, "readable");
    ASSERT_TRUE(readable.is_boolean());
    EXPECT_TRUE(readable.as_boolean());
}

TEST(LspFoundationTest, ParserCollectsMultipleSyntaxErrors) {
    const std::string source = R"(
let a = ;
let b = ;
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    parser.parse();

    EXPECT_TRUE(parser.has_error());
    EXPECT_GE(parser.errors().size(), 2u);
}

TEST(LspFoundationTest, SemanticValidatorReportsMultipleErrors) {
    const std::string source = R"(
let a = missing_one;
let b = missing_two;
let c: UnknownType = 1;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_GE(result.errors().size(), 2u);
}

TEST(LspFoundationTest, TypeMismatchDetected) {
    const std::string source = R"(
let count: number = "not a number";
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("Type mismatch"), std::string::npos);
}

TEST(LspFoundationTest, UnknownTypeAnnotationRejected) {
    const std::string source = R"(
let value: NotARealType = 1;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("Unknown type"), std::string::npos);
}

TEST(LspFoundationTest, ValidTypedProgramAccepted) {
    const std::string source = R"(
let count: number = 42;
let label: string = "ok";
let active: boolean = true;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(LspFoundationTest, ParserRecoversInsideClassBody) {
    const std::string source = R"(
class Example {
    get ok() {
        return 1;
    }

    broken method

    get stillParsed() {
        return 2;
    }
}
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();

    EXPECT_TRUE(parser.has_error());
    ASSERT_FALSE(program->statements.empty());
    EXPECT_EQ(program->statements[0]->type, ASTNodeType::ClassDeclaration);
}

TEST(LspFoundationTest, InlineAutoPropertyAccessors) {
    const std::string code = R"(
class Box {
    constructor() {
        this._size = 0;
    }

    size: number {
        get;
        set;
    }
}

let box = new Box();
box.size = 10;
let result = box.size;
    )";

    auto result = run_and_get(code, "result");
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 10);
}

TEST(LspFoundationTest, InlinePropertyAccessorBodies) {
    const std::string code = R"(
class Counter {
    constructor() {
        this._count = 0;
    }

    count: number {
        get {
            return this._count;
        }
        set(next) {
            this._count = next;
        }
    }
}

let counter = new Counter();
counter.count = 7;
let result = counter.count;
    )";

    auto result = run_and_get(code, "result");
    ASSERT_TRUE(result.is_integer());
    EXPECT_EQ(result.as_integer(), 7);
}

TEST(LspFoundationTest, ParserCreatesErrorExpressionNodes) {
    const std::string source = R"(
let broken = ;
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    auto program = parser.parse();

    EXPECT_TRUE(parser.has_error());
    ASSERT_FALSE(program->statements.empty());
    auto* expr_stmt = static_cast<ExpressionStatement*>(program->statements[0].get());
    auto* assignment = static_cast<AssignmentExpression*>(expr_stmt->expression.get());
    EXPECT_EQ(assignment->value->type, ASTNodeType::ErrorExpression);
}

TEST(LspFoundationTest, UnusedVariableWarning) {
    const std::string source = R"(
let unused = 1;
let used = 2;
used = used + 1;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
    bool found_warning = false;
    for (const auto& diagnostic : result.diagnostics) {
        if (diagnostic.severity == DiagnosticSeverity::Warning &&
            diagnostic.message.find("Unused variable 'unused'") != std::string::npos) {
            found_warning = true;
            break;
        }
    }
    EXPECT_TRUE(found_warning);
}

TEST(LspFoundationTest, FunctionCallArgumentCountValidated) {
    const std::string source = R"(
function add(a: number, b: number): number {
    return a + b;
}

let value = add(1);
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("expects between 2 and 2 arguments"), std::string::npos);
}

TEST(LspFoundationTest, BinaryExpressionTypeValidation) {
    const std::string source = R"(
let value = 1 + "text";
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("Invalid operands"), std::string::npos);
}

TEST(LspFoundationTest, DiagnosticIncludesLineAndColumn) {
    const std::string source = R"(
let value = missing_symbol;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("2:"), std::string::npos);
}

TEST(LspFoundationTest, SetterRequiresSingleParameter) {
    const std::string source = R"(
class Bad {
    set value(a, b) {
        this._value = a;
    }
}
    )";

    Lexer lexer(source);
    Parser parser(lexer.tokenize());
    parser.parse();
    EXPECT_TRUE(parser.has_error());
}
