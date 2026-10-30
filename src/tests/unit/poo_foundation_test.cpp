#include <gtest/gtest.h>
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/validator.h"
#include "dryad/common/utils.h"

using namespace dryad;

namespace {

void run_program(const std::string& code) {
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error(parser.error_message());
    }
    Interpreter interpreter;
    interpreter.execute(program.get());
}

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

TEST(PooFoundationTest, AbstractReadonlyPropertyRequiresImplementation) {
    const std::string code = R"(
abstract class Stream {
    abstract readonly canRead: boolean;
    abstract close();
}

class MemoryStream extends Stream {
    constructor() {
        this.canRead = true;
    }

    readonly canRead: boolean;

    close() {
        this.closed = true;
    }
}

let s = new MemoryStream();
let readable = s.canRead;
    )";

    auto readable = run_and_get(code, "readable");
    ASSERT_TRUE(readable.is_boolean());
    EXPECT_TRUE(readable.as_boolean());
}

TEST(PooFoundationTest, ReadonlyPropertyCannotBeAssignedOutsideConstructor) {
    const std::string code = R"(
class Box {
    constructor(value) {
        this.value = value;
    }

    readonly value: number;

    setValue(v) {
        this.value = v;
    }
}

let b = new Box(1);
b.setValue(2);
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(PooFoundationTest, InterfaceImplementationRequired) {
    const std::string code = R"(
interface IDisposable {
    dispose(): void;
}

class Resource implements IDisposable {
    constructor() {
        this.disposed = false;
    }

    dispose() {
        this.disposed = true;
    }
}

let r = new Resource();
r.dispose();
let ok = r.disposed;
    )";

    auto ok = run_and_get(code, "ok");
    ASSERT_TRUE(ok.is_boolean());
    EXPECT_TRUE(ok.as_boolean());
}

TEST(PooFoundationTest, MissingInterfaceMethodRejected) {
    const std::string code = R"(
interface IDisposable {
    dispose(): void;
}

class Bad implements IDisposable {
    constructor() {}
}
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(PooFoundationTest, CannotInstantiateInterface) {
    const std::string code = R"(
interface IDisposable {
    dispose(): void;
}

let x = new IDisposable();
    )";

    EXPECT_THROW(run_program(code), DryadException);
}

TEST(PooFoundationTest, SemanticValidatorDetectsUndefinedVariable) {
    const std::string source = R"(
let x = unknown_symbol;
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("unknown_symbol"), std::string::npos);
}

TEST(PooFoundationTest, SemanticValidatorAcceptsValidProgram) {
    const std::string source = R"(
function add(a, b) {
    return a + b;
}

let result = add(1, 2);
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(PooFoundationTest, StreamLikeAbstractClassWithImplements) {
    const std::string code = R"(
interface IDisposable {
    dispose(): void;
}

abstract class Stream implements IDisposable {
    abstract readonly canRead: boolean;
    abstract close();

    dispose() {
        this.close();
    }
}

class MemoryStream extends Stream {
    constructor() {
        this.canRead = true;
        this.closed = false;
    }

    readonly canRead: boolean;

    close() {
        this.closed = true;
    }
}

let s = new MemoryStream();
s.dispose();
let ok = s.closed && s.canRead;
    )";

    auto ok = run_and_get(code, "ok");
    ASSERT_TRUE(ok.is_boolean());
    EXPECT_TRUE(ok.as_boolean());
}
