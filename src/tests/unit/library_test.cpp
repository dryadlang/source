#include <gtest/gtest.h>
#include "dryad/aot/aot_compiler.h"
#include "dryad/common/utils.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/library_module.h"
#include <filesystem>
#include <string>

using namespace dryad;

namespace {

std::string library_fixture_root() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/libs";
}

void run_with_library_path(const std::string& code) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});

    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error()) << parser.error_message();

    interpreter.execute(program.get());
}

} // namespace

TEST(LibraryTest, ParserRecognizesExportDeclarations) {
    Lexer lexer(R"(
        export function add(a, b) { return a + b; }
        export class Point { constructor(x, y) { this.x = x; this.y = y; } }
        export let VERSION = 1;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();

    ASSERT_FALSE(parser.has_error());
    ASSERT_EQ(program->statements.size(), 3u);

    auto* fn = dynamic_cast<FunctionDeclaration*>(program->statements[0].get());
    ASSERT_NE(fn, nullptr);
    EXPECT_TRUE(fn->is_exported);

    auto* klass = dynamic_cast<ClassDeclaration*>(program->statements[1].get());
    ASSERT_NE(klass, nullptr);
    EXPECT_TRUE(klass->is_exported);

    auto* var = dynamic_cast<VariableDeclaration*>(program->statements[2].get());
    ASSERT_NE(var, nullptr);
    EXPECT_TRUE(var->is_exported);
}

TEST(LibraryTest, RejectsExportInternalCombination) {
    Lexer lexer("export internal function secret() { return 1; }");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();

    EXPECT_TRUE(parser.has_error());
    EXPECT_NE(parser.error_message().find("export"), std::string::npos);
}

TEST(LibraryTest, ImportsExportedClassFromDydLibrary) {
    run_with_library_path(R"(
        import { Vec2 } from "@test/geom";
        let v = new Vec2(3, 4);
        let ok = v.x == 3 && v.y == 4 && v.sum() == 7;
    )");

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});

    Lexer lexer(R"(
        import { Vec2 } from "@test/geom";
        let v = new Vec2(3, 4);
        let ok = v.x == 3 && v.y == 4 && v.sum() == 7;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    interpreter.execute(program.get());
    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}

TEST(LibraryTest, ImportsExportedFunctionAndConstant) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});

    Lexer lexer(R"(
        import { makeVec2, ORIGIN_X } from "@test/geom";
        let v = makeVec2(1, 2);
        let ok = ORIGIN_X == 0 && v.x == 1;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    interpreter.execute(program.get());
    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}

TEST(LibraryTest, RejectsNonExportedSymbol) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});

    Lexer lexer(R"(import { _hiddenHelper } from "@test/geom";)");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    EXPECT_THROW(interpreter.execute(program.get()), DryadException);
}

TEST(LibraryTest, RejectsPrivateVariableExport) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});

    Lexer lexer(R"(import { _privateCounter } from "@test/geom";)");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    EXPECT_THROW(interpreter.execute(program.get()), DryadException);
}

TEST(LibraryTest, ModuleIsolationHidesHostGlobals) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});
    interpreter.global_env()->define("hostSecret", Value(static_cast<int64_t>(42)));

    Lexer lexer(R"(
        import { Vec2 } from "@test/geom";
        let ok = Vec2 != null;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    interpreter.execute(program.get());
    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}

TEST(LibraryTest, ImportsFromCompiledDylWithoutSource) {
    const std::string source_path = library_fixture_root() + "/@test/geom.dyd";
    const std::string dyl_only_root = library_fixture_root() + "/@test/dyl_only";
    const std::string dyl_path = dyl_only_root + "/@test/geom.dyl";
    std::filesystem::remove_all(dyl_only_root);
    std::filesystem::create_directories(dyl_only_root + "/@test");

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    auto result = compile_to_dyl(source_path, dyl_path, options);
    ASSERT_TRUE(result.success) << result.message;

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({dyl_only_root});

    Lexer lexer(R"(
        import { Vec2, makeVec2, ORIGIN_X } from "@test/geom";
        let v = new Vec2(3, 4);
        let ok = v.x == 3 && v.y == 4 && v.sum() == 7;
        let v2 = makeVec2(1, 2);
        let ok2 = ORIGIN_X == 0 && v2.x == 1;
        let all_ok = ok && ok2;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    interpreter.execute(program.get());
    EXPECT_TRUE(interpreter.global_env()->get("all_ok").is_truthy());

    std::filesystem::remove_all(dyl_only_root);
}

TEST(LibraryTest, StdlibStillWorksWithoutExplicitExports) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);

    Lexer lexer(R"(
        import { VFS } from "@std/io/vfs";
        let ok = VFS != null;
    )");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    ASSERT_FALSE(parser.has_error());

    interpreter.execute(program.get());
    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());
}
