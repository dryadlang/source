#include <gtest/gtest.h>
#include <fstream>
#include "dryad/aot/aot_compiler.h"
#include "dryad/runtime/bytecode_cache.h"
#include "dryad/runtime/bytecode_module.h"

using namespace dryad;

TEST(BytecodeCacheTest, CompanionPathResolution) {
    EXPECT_EQ(companion_dryadc_path("/tmp/foo.dryad"), "/tmp/foo.dryadc");
    EXPECT_EQ(companion_native_path("/tmp/foo.dryad"), "/tmp/foo.so");
}

TEST(BytecodeCacheTest, LoadCompanionDryadc) {
    const std::string source = R"(
        function compute() {
            let sum = 0;
            let i = 0;
            while (i < 2) {
                sum = sum + i;
                i = i + 1;
            }
            return sum;
        }
        compute();
    )";

    const std::string input = "/tmp/dryad_cache_input.dryad";
    const std::string dryadc = "/tmp/dryad_cache_input.dryadc";
    {
        std::ofstream file(input);
        file << source;
    }

    CompileOptions options;
    options.run_profiler = true;
    ASSERT_TRUE(compile_to_dryadc(input, dryadc, options).success);

    BytecodeModuleCache cache;
    cache.set_source_path(input);
    EXPECT_TRUE(cache.try_load_companion());
    EXPECT_EQ(cache.program().functions.size(), 1u);
    EXPECT_EQ(cache.program().functions[0].metadata.name, "compute");

    std::remove(input.c_str());
    std::remove(dryadc.c_str());
}
