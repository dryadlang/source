#include <gtest/gtest.h>
#include "dryad/compiler/validator.h"
#include <cstdio>
#include <fstream>

using namespace dryad;

TEST(ValidatorTest, AcceptsValidProgram) {
    const std::string source = R"(
class Point {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}

let p = new Point(1, 2);
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(ValidatorTest, MessageIsEmptyWhenThereAreNoDiagnosticsAtAll) {
    auto result = ProgramValidator::validate_source("let x = 1;\nx = x + 1;");
    EXPECT_TRUE(result.ok);
    EXPECT_EQ(result.message(), "");
}

TEST(ValidatorTest, RejectsInvalidSyntax) {
    const std::string source = "let x = ;";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_FALSE(result.message().empty());
}

TEST(ValidatorTest, AcceptsAbstractClassSyntax) {
    const std::string source = R"(
abstract class Stream {
    abstract close();

    dispose() {
        this.close();
    }
}
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(ValidatorTest, RejectsUndefinedSymbol) {
    const std::string source = R"(
function main() {
    return missing_variable;
}
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("missing_variable"), std::string::npos);
}

TEST(ValidatorTest, AcceptsPropertyAndInterfaceSyntax) {
    const std::string source = R"(
interface IDisposable {
    dispose(): void;
}

abstract class Stream implements IDisposable {
    abstract readonly canRead: boolean;
    abstract close();
}
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(ValidatorTest, AcceptsSuperSyntax) {
    const std::string source = R"(
class Parent {
    constructor(name) {
        this.name = name;
    }

    greet() {
        return this.name;
    }
}

class Child extends Parent {
    constructor(name) {
        super(name);
    }

    greet() {
        return super.greet();
    }
}
    )";

    auto result = ProgramValidator::validate_source(source);
    EXPECT_TRUE(result.ok);
}

TEST(ValidatorTest, ErrorsReturnsOneMessagePerDiagnostic) {
    const std::string source = "let x = ;";

    auto result = ProgramValidator::validate_source(source);
    auto errors = result.errors();
    EXPECT_FALSE(errors.empty());
    EXPECT_EQ(errors.size(), result.diagnostics.size());
}

TEST(ValidatorTest, ValidateFileAcceptsAnExistingValidFile) {
    const std::string path = "dryad_validator_test_valid.dryad";
    {
        std::ofstream file(path);
        file << "let x = 1;";
    }

    auto result = ProgramValidator::validate_file(path);
    std::remove(path.c_str());

    EXPECT_TRUE(result.ok);
}

TEST(ValidatorTest, ValidateFileReportsMissingFile) {
    auto result = ProgramValidator::validate_file("dryad_validator_test_missing_file.dryad");

    EXPECT_FALSE(result.ok);
    EXPECT_NE(result.message().find("Cannot open file"), std::string::npos);
}
