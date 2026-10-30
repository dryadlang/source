#include <gtest/gtest.h>
#include "dryad/common/utils.h"
#include "dryad/compiler/compile_pipeline.h"
#include "dryad/compiler/import_cycle_detector.h"
#include "dryad/runtime/module_loader.h"

using namespace dryad;

namespace {

std::string cycle_fixture_root() {
    return std::string(DRYAD_TEST_DIR) + "/fixtures/libs";
}

} // namespace

TEST(ImportCycleTest, DetectsCircularImports) {
    ModuleLoader loader(DRYAD_STDLIB_DIR);
    loader.set_library_paths({cycle_fixture_root()});

    const std::string entry = cycle_fixture_root() + "/@cycle/a.dyd";
    loader.push_import_context(entry);
    const auto report = detect_import_cycles(entry, loader);
    loader.pop_import_context();

    EXPECT_TRUE(report.has_cycle);
    EXPECT_GE(report.cycle.size(), 2u);
}

TEST(ImportCycleTest, CompilePipelineRejectsCircularImports) {
    CompileOptions options;
    options.stdlib_root = DRYAD_STDLIB_DIR;
    options.run_profiler = false;
    options.library_paths = {cycle_fixture_root()};

    EXPECT_THROW(compile_file_to_bytecode(cycle_fixture_root() + "/@cycle/a.dyd", options),
                 DryadException);
}
