#include <gtest/gtest.h>
#include "dryad/aot/aot_compiler.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/execution_mode.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/function.h"
#include "dryad/runtime/library_module.h"
#include "dryad/runtime/value.h"
#include <filesystem>
#include <fstream>
#include <stdexcept>

using namespace dryad;

namespace {

std::string library_fixture_root() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/libs";
}

std::string library_usage_source() {
    return R"(
        import { Vec2, makeVec2, ORIGIN_X } from "@test/geom";

        function verify_library() {
            let v = new Vec2(3, 4);
            let ok = v.x == 3 && v.y == 4 && v.sum() == 7;

            let v2 = makeVec2(1, 2);
            let ok2 = ORIGIN_X == 0 && v2.x == 1;

            return ok && ok2;
        }
    )";
}

Value run_library_verify(Interpreter& interpreter, const std::string& source) {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    if (parser.has_error()) {
        throw std::runtime_error("parse error in library triple test fixture");
    }
    interpreter.execute(program.get());
    Value verify_fn = interpreter.global_env()->get("verify_library");
    if (!verify_fn.is_function()) {
        throw std::runtime_error("verify_library not found in global environment");
    }
    return verify_fn.as_function()->call(&interpreter, {});
}

} // namespace

TEST(LibraryTripleTest, InterpreterLoadsSourceLibrary) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});
    EXPECT_TRUE(run_library_verify(interpreter, library_usage_source()).is_truthy());
}

TEST(LibraryTripleTest, VmLoadsSourceLibrary) {
    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({library_fixture_root()});
    interpreter.set_default_execution_mode(ExecutionMode::Bytecode);
    EXPECT_TRUE(run_library_verify(interpreter, library_usage_source()).is_truthy());
}

TEST(LibraryTripleTest, AotCompileWithSourceLibrary) {
    const std::string app_path = "/tmp/dryad_lib_triple_app.dyd";
    {
        std::ofstream app(app_path);
        app << library_usage_source();
        app << "\nfunction entrypoint() { return verify_library(); }\n";
        app << "entrypoint();\n";
    }

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = true;
    options.library_paths = {library_fixture_root()};

    auto result = compile_to_llvm_ir(app_path, "/tmp/dryad_lib_triple_app.ll", options);
    ASSERT_TRUE(result.success) << result.message;

    std::remove(app_path.c_str());
    std::remove("/tmp/dryad_lib_triple_app.ll");
}

TEST(LibraryTripleTest, VmLoadsDylOnlyLibrary) {
    const std::string dyl_only_root = library_fixture_root() + "/@test/dyl_only_triple";
    const std::string dyl_path = dyl_only_root + "/@test/geom.dyl";
    std::filesystem::remove_all(dyl_only_root);
    std::filesystem::create_directories(dyl_only_root + "/@test");

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(library_fixture_root() + "/@test/geom.dyd", dyl_path, options).success);

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({dyl_only_root});
    interpreter.set_default_execution_mode(ExecutionMode::Bytecode);
    EXPECT_TRUE(run_library_verify(interpreter, library_usage_source()).is_truthy());

    std::filesystem::remove_all(dyl_only_root);
}

TEST(LibraryTripleTest, InterpreterLoadsDylOnlyLibrary) {
    const std::string dyl_only_root = library_fixture_root() + "/@test/dyl_only_triple";
    const std::string dyl_path = dyl_only_root + "/@test/geom.dyl";
    std::filesystem::remove_all(dyl_only_root);
    std::filesystem::create_directories(dyl_only_root + "/@test");

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(library_fixture_root() + "/@test/geom.dyd", dyl_path, options).success);

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.set_library_paths({dyl_only_root});
    EXPECT_TRUE(run_library_verify(interpreter, library_usage_source()).is_truthy());

    std::filesystem::remove_all(dyl_only_root);
}
