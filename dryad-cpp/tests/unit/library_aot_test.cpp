#include <gtest/gtest.h>
#include <fstream>
#include "dryad/aot/aot_compiler.h"
#include "dryad/aot/library_linker.h"
#include "dryad/aot/native_linker.h"

using namespace dryad;

namespace {

std::string library_fixture_root() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/libs";
}

} // namespace

TEST(LibraryAotTest, CompilesDylLibraryToLlvmIr) {
    const std::string source_path = library_fixture_root() + "/@test/math.dyd";
    const std::string dyl_path = "/tmp/dryad_math_lib.dyl";
    const std::string llvm_path = "/tmp/dryad_math_lib.ll";

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(source_path, dyl_path, options).success);

    auto result = compile_library_to_llvm_ir(dyl_path, llvm_path);
    ASSERT_TRUE(result.success) << result.message;

    std::ifstream ll(llvm_path);
    std::string content((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(content.find("@dryad_lib_test_math_add"), std::string::npos);
    EXPECT_NE(content.find("@dryad_lib_test_math_multiply"), std::string::npos);

    std::remove(dyl_path.c_str());
    std::remove(llvm_path.c_str());
}

TEST(LibraryAotTest, LinksExecutableAgainstDylLibrary) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source_path = library_fixture_root() + "/@test/math.dyd";
    const std::string dyl_path = "/tmp/dryad_math_link.dyl";
    const std::string app_path = "/tmp/dryad_math_app.dyd";
    const std::string output = "/tmp/dryad_math_app_exec";

    CompileOptions lib_options;
    lib_options.stdlib_root = DRYAD_STDLIB_DIR;
    lib_options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(source_path, dyl_path, lib_options).success);

    {
        std::ofstream app(app_path);
        app << R"(
            import { add } from "@test/math";

            function entrypoint() {
                return add(2, 3);
            }
            entrypoint();
        )";
    }

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = true;
    options.entry_function = "entrypoint";
    options.library_paths = {library_fixture_root()};
    options.link_libraries = {dyl_path};

    auto result = compile_to_native_executable(
        app_path, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;

    std::ifstream ll(output + ".ll");
    std::string llvm((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(llvm.find("@dryad_lib_test_math_add"), std::string::npos);
    EXPECT_NE(llvm.find("call i8* @dryad_lib_test_math_add"), std::string::npos);
    EXPECT_TRUE(is_native_binary_image(result.binary));

    std::remove(dyl_path.c_str());
    std::remove(app_path.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(LibraryAotTest, LinksExecutableAgainstDylClassLibrary) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source_path = library_fixture_root() + "/@test/geom.dyd";
    const std::string dyl_path = "/tmp/dryad_geom_link.dyl";
    const std::string app_path = "/tmp/dryad_geom_app.dyd";
    const std::string output = "/tmp/dryad_geom_app_exec";

    CompileOptions lib_options;
    lib_options.stdlib_root = DRYAD_STDLIB_DIR;
    lib_options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(source_path, dyl_path, lib_options).success);

    {
        std::ofstream app(app_path);
        app << R"(
            import { Vec2 } from "@test/geom";

            function entrypoint() {
                let v = new Vec2(3, 4);
                return v.sum();
            }
            entrypoint();
        )";
    }

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = true;
    options.entry_function = "entrypoint";
    options.library_paths = {library_fixture_root()};
    options.link_libraries = {dyl_path};

    auto result = compile_to_native_executable(
        app_path, output, options, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(result.success) << result.message;

    std::ifstream ll(output + ".ll");
    std::string llvm((std::istreambuf_iterator<char>(ll)), std::istreambuf_iterator<char>());
    EXPECT_NE(llvm.find("@dryad_lib_test_geom_Vec2_sum"), std::string::npos);
    EXPECT_NE(llvm.find("@dryad_lib_test_geom_Vec2_constructor"), std::string::npos);
    EXPECT_NE(llvm.find("call i8* @dryad_lib_test_geom_Vec2_sum"), std::string::npos);
    EXPECT_TRUE(is_native_binary_image(result.binary));

    std::remove(dyl_path.c_str());
    std::remove(app_path.c_str());
    std::remove(output.c_str());
    std::remove(std::string(output + ".ll").c_str());
}

TEST(LibraryAotTest, EmitsNativeCompanionForDylLibrary) {
    if (!clang_available()) {
        GTEST_SKIP() << "clang not available";
    }

    const std::string source_path = library_fixture_root() + "/@test/math.dyd";
    const std::string dyl_path = "/tmp/dryad_math_hybrid.dyl";
    const std::string so_path = library_native_companion_path(dyl_path);

    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    ASSERT_TRUE(compile_to_dyl(source_path, dyl_path, options).success);

    auto native_result = compile_library_to_native(
        dyl_path, so_path, DRYAD_AOT_RUNTIME_SRC, DRYAD_INCLUDE_DIR);
    ASSERT_TRUE(native_result.success) << native_result.message;
    EXPECT_TRUE(is_native_binary_image(native_result.binary));

    std::remove(dyl_path.c_str());
    std::remove(so_path.c_str());
    std::remove(std::string(so_path + ".ll").c_str());
}
