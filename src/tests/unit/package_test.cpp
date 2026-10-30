#include <gtest/gtest.h>
#include "dryad/aot/aot_compiler.h"
#include "dryad/compiler/interpreter.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/runtime/package_installer.h"
#include "dryad/runtime/package_manifest.h"
#include <filesystem>
#include <fstream>

using namespace dryad;

namespace {

std::string packages_fixture_root() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/packages";
}

std::string temp_modules_dir() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/packages/sample/dryad_modules_test";
}

void cleanup_modules_dir() {
    std::error_code ec;
    std::filesystem::remove_all(temp_modules_dir(), ec);
}

} // namespace

TEST(PackageTest, ParsesDryadToml) {
    const auto manifest =
        parse_package_manifest(packages_fixture_root() + "/sample/dryad.toml");

    EXPECT_EQ(manifest.name, "@acme/sample");
    EXPECT_EQ(manifest.version, "0.1.0");
    ASSERT_EQ(manifest.dependencies.size(), 1u);
    EXPECT_EQ(manifest.dependencies[0].name, "@test/geom");
    EXPECT_TRUE(manifest.dependencies[0].path.find("@test/geom") != std::string::npos);
}

TEST(PackageTest, InstallCreatesModulesDirectory) {
    cleanup_modules_dir();

    auto result = install_package(packages_fixture_root() + "/sample/dryad.toml",
                                  temp_modules_dir());
    ASSERT_TRUE(result.success) << result.message;

    EXPECT_TRUE(std::filesystem::exists(temp_modules_dir() + "/@test/geom.dyl"));
    EXPECT_TRUE(std::filesystem::exists(temp_modules_dir() + "/@test/geom.dyd") ||
                std::filesystem::exists(temp_modules_dir() + "/@test/geom.dryad"));

    cleanup_modules_dir();
}

TEST(PackageTest, RelativeImportAndReExport) {
    cleanup_modules_dir();
    auto install_result = install_package(packages_fixture_root() + "/sample/dryad.toml",
                                          temp_modules_dir());
    ASSERT_TRUE(install_result.success) << install_result.message;

    const std::string bundle_path =
        packages_fixture_root() + "/sample/src/@acme/bundle.dyd";
    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    options.library_paths = {temp_modules_dir()};
    ASSERT_TRUE(compile_to_dyl(bundle_path,
                               temp_modules_dir() + "/@acme/bundle.dyl",
                               options).success);

    Interpreter interpreter;
    interpreter.set_stdlib_root(DRYAD_STDLIB_DIR);
    interpreter.module_loader().set_library_paths({temp_modules_dir()});

    const std::string app_path = packages_fixture_root() + "/sample/src/app.dyd";
    interpreter.run_file(app_path);

    EXPECT_TRUE(interpreter.global_env()->get("ok").is_truthy());

    cleanup_modules_dir();
}

TEST(PackageTest, ParserRecognizesExportFrom) {
    Lexer lexer(R"(export { Vec2, makeVec2 } from "@test/geom";)");
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();

    ASSERT_FALSE(parser.has_error());
    ASSERT_EQ(program->statements.size(), 1u);

    auto* export_stmt = dynamic_cast<ExportFromDeclaration*>(program->statements[0].get());
    ASSERT_NE(export_stmt, nullptr);
    EXPECT_EQ(export_stmt->names.size(), 2u);
    EXPECT_EQ(export_stmt->module_path, "@test/geom");
}
