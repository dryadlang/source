#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <fcntl.h>
#include <unistd.h>
#include <fstream>
#include <atomic>
#include <cstdint>
#include <limits>
#include <string>
#include <thread>
#include <vector>

using namespace dryad;

namespace {

// Executes Dryad source and returns the value of the given global variable.
Value run_dryad(const std::string& source, const std::string& var = "ok") {
    Lexer lexer(source);
    auto tokens = lexer.tokenize();
    Parser parser(std::move(tokens));
    auto program = parser.parse();
    
    Interpreter interp;
    interp.execute(program.get());
    
    return interp.global_env()->get(var);
}

}

class IntrinsicsTest : public ::testing::Test {
protected:
    void SetUp() override {
        auto& registry = IntrinsicsRegistry::instance();
        registry.register_all();
        // ctest runs each case in its own process, in parallel: a shared path
        // let one case's TearDown unlink another's file mid-test.
        path_ = std::string("/tmp/dryad_test_") +
                ::testing::UnitTest::GetInstance()->current_test_info()->name() + "_" +
                std::to_string(::getpid()) + ".txt";
    }
    
    void TearDown() override {
        ::unlink(path_.c_str());
    }

    std::string path_;
};

TEST_F(IntrinsicsTest, FileOpenClose) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value fd = registry.call("syscall.open", {
        Value(path_),
        Value(static_cast<int64_t>(O_CREAT | O_WRONLY | O_TRUNC))
    });
    
    EXPECT_TRUE(fd.is_integer());
    EXPECT_GE(fd.as_integer(), 0);
    
    Value close_result = registry.call("syscall.close", {fd});
    EXPECT_EQ(close_result.as_integer(), 0);
}

TEST_F(IntrinsicsTest, FileWriteRead) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value fd_write = registry.call("syscall.open", {
        Value(path_),
        Value(static_cast<int64_t>(O_CREAT | O_WRONLY | O_TRUNC))
    });
    
    std::string test_data = "Hello, Dryad!";
    Value write_result = registry.call("syscall.write", {
        fd_write,
        Value(test_data)
    });
    
    EXPECT_EQ(write_result.as_integer(), static_cast<int64_t>(test_data.size()));
    
    registry.call("syscall.close", {fd_write});
    
    Value fd_read = registry.call("syscall.open", {
        Value(path_),
        Value(static_cast<int64_t>(O_RDONLY))
    });
    
    Value read_result = registry.call("syscall.read", {
        fd_read,
        Value(static_cast<int64_t>(1024))
    });
    
    EXPECT_TRUE(read_result.is_string());
    EXPECT_EQ(read_result.as_string(), test_data);
    
    registry.call("syscall.close", {fd_read});
}

TEST_F(IntrinsicsTest, FileUnlink) {
    auto& registry = IntrinsicsRegistry::instance();
    
    std::ofstream(path_) << "test";
    
    Value result = registry.call("syscall.unlink", {
        Value(path_)
    });
    
    EXPECT_EQ(result.as_integer(), 0);
    
    EXPECT_FALSE(std::ifstream(path_).good());
}

TEST_F(IntrinsicsTest, TimeIntrinsic) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value time1 = registry.call("syscall.time", {});
    EXPECT_TRUE(time1.is_integer());
    EXPECT_GT(time1.as_integer(), 0);
    
    Value time2 = registry.call("syscall.time", {});
    EXPECT_GE(time2.as_integer(), time1.as_integer());
}

TEST_F(IntrinsicsTest, ClockGettimeIntrinsic) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value time1 = registry.call("syscall.clock_gettime", {});
    EXPECT_TRUE(time1.is_float());
    EXPECT_GT(time1.as_float(), 0.0);
    
    Value time2 = registry.call("syscall.clock_gettime", {});
    EXPECT_GE(time2.as_float(), time1.as_float());
}

TEST_F(IntrinsicsTest, MemoryMallocFree) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value ptr = registry.call("syscall.malloc", {Value(static_cast<int64_t>(1024))});
    EXPECT_TRUE(ptr.is_integer());
    EXPECT_NE(ptr.as_integer(), 0);
    
    registry.call("syscall.free", {ptr});
}

TEST_F(IntrinsicsTest, UnknownIntrinsic) {
    auto& registry = IntrinsicsRegistry::instance();
    
    EXPECT_THROW({
        registry.call("syscall.nonexistent", {});
    }, DryadException);
}

TEST_F(IntrinsicsTest, HasIntrinsic) {
    auto& registry = IntrinsicsRegistry::instance();
    
    EXPECT_TRUE(registry.has("syscall.open"));
    EXPECT_TRUE(registry.has("syscall.read"));
    EXPECT_TRUE(registry.has("syscall.write"));
    EXPECT_TRUE(registry.has("syscall.close"));
    EXPECT_TRUE(registry.has("syscall.time"));
    EXPECT_FALSE(registry.has("syscall.nonexistent"));
}

TEST_F(IntrinsicsTest, MemoryIntrinsicsRegistered) {
    Interpreter interpreter;
    auto env = interpreter.global_env();
    
    EXPECT_TRUE(env->has("__alloc_bytes"));
    EXPECT_TRUE(env->has("__free_bytes"));
    EXPECT_TRUE(env->has("__realloc"));
    EXPECT_TRUE(env->has("__memcpy"));
    EXPECT_TRUE(env->has("__memset"));
    EXPECT_TRUE(env->has("__buffer_get"));
    EXPECT_TRUE(env->has("__buffer_set"));
    
    EXPECT_TRUE(env->has("__runtime_create_object"));
    EXPECT_TRUE(env->has("__runtime_object_set"));
    EXPECT_TRUE(env->has("__runtime_object_get"));
    
    EXPECT_TRUE(env->get("__alloc_bytes").is_function());
    EXPECT_TRUE(env->get("__runtime_create_object").is_function());
}

TEST_F(IntrinsicsTest, AllocBytesCallable) {
    std::string code = R"(
        let handle = __alloc_bytes(1024);
        let result = handle != null;
    )";
    
    auto result = run_dryad(code, "result");
    EXPECT_TRUE(result.is_truthy());
}

TEST_F(IntrinsicsTest, MemoryRoundTrip) {
    std::string code = R"(
        let handle = __alloc_bytes(10);
        __buffer_set(handle, 0, 42);
        __buffer_set(handle, 1, 99);
        
        let val0 = __buffer_get(handle, 0);
        let val1 = __buffer_get(handle, 1);
        
        __free_bytes(handle);
        
        let success = (val0 == 42) && (val1 == 99);
    )";
    
    auto success = run_dryad(code, "success");
    EXPECT_TRUE(success.is_truthy());
}

TEST_F(IntrinsicsTest, MemoryCopyAndRealloc) {
    std::string code = R"(
        let a = __alloc_bytes(4);
        __memset(a, 65, 4);
        let b = __alloc_bytes(4);
        __memcpy(b, a, 4);
        __free_bytes(a);
        
        let bytesOk = (__buffer_get(b, 0) == 65) && (__buffer_get(b, 3) == 65);
        
        let r = __realloc(b, 8);
        __buffer_set(r, 7, 200);
        let growOk = (__buffer_get(r, 7) == 200) && (r != null);
        
        __free_bytes(r);
        let success = bytesOk && growOk;
    )";
    
    auto success = run_dryad(code, "success");
    EXPECT_TRUE(success.is_truthy());
}

TEST_F(IntrinsicsTest, RuntimeObjectIntrinsics) {
    std::string code = R"(
        let obj = __runtime_create_object();
        __runtime_object_set(obj, "key", 7);
        let val = __runtime_object_get(obj, "key");
        let success = (val == 7) && (obj != null);
    )";
    
    auto success = run_dryad(code, "success");
    EXPECT_TRUE(success.is_truthy());
}

TEST(IntrinsicsRegistryThreadSafetyTest, ConcurrentRegistrationIsIdempotent) {
    auto& registry = IntrinsicsRegistry::instance();
    registry.register_all();
    const uint16_t expected = registry.get_id("syscall.memmove");

    // Every Interpreter construction calls register_all(); worker threads do
    // this concurrently. IDs must stay stable and the maps must survive it.
    std::atomic<bool> stable{true};
    std::vector<std::thread> workers;
    for (int t = 0; t < 8; ++t) {
        workers.emplace_back([&registry, expected, &stable]() {
            for (int i = 0; i < 50; ++i) {
                registry.register_all();
                if (registry.get_id("syscall.memmove") != expected) {
                    stable = false;
                }
            }
        });
    }
    for (auto& worker : workers) {
        worker.join();
    }

    EXPECT_TRUE(stable.load());
    EXPECT_EQ(registry.get_id("syscall.memmove"), expected);
}

TEST_F(IntrinsicsTest, FloatToIntTruncatesTowardZero) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_EQ(registry.call("syscall.float_to_int", {Value(3.9)}).as_integer(), 3);
    EXPECT_EQ(registry.call("syscall.float_to_int", {Value(-3.9)}).as_integer(), -3);
    EXPECT_EQ(registry.call("syscall.float_to_int", {Value(0.0)}).as_integer(), 0);
}

TEST_F(IntrinsicsTest, FloatToIntPassesIntegersThroughExactly) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_EQ(registry.call("syscall.float_to_int",
                            {Value(int64_t(9007199254740993LL))}).as_integer(),
              9007199254740993LL);
}

TEST_F(IntrinsicsTest, FloatToIntRejectsOutOfRangeAndNonFinite) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_TRUE(registry.call("syscall.float_to_int",
                              {Value(9223372036854775808.0)}).is_null());
    EXPECT_TRUE(registry.call("syscall.float_to_int",
                              {Value(-9223372036854775808.0 * 2.0)}).is_null());
    EXPECT_TRUE(registry.call("syscall.float_to_int",
                              {Value(std::numeric_limits<double>::infinity())}).is_null());
    EXPECT_TRUE(registry.call("syscall.float_to_int",
                              {Value(std::numeric_limits<double>::quiet_NaN())}).is_null());
}

TEST_F(IntrinsicsTest, IntToFloatConverts) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_DOUBLE_EQ(registry.call("syscall.int_to_float", {Value(int64_t(42))}).as_float(), 42.0);
    EXPECT_DOUBLE_EQ(registry.call("syscall.int_to_float", {Value(int64_t(-7))}).as_float(), -7.0);
    // 2^53 + 1 is not representable as a double; it must round to 2^53.
    EXPECT_DOUBLE_EQ(
        registry.call("syscall.int_to_float", {Value(int64_t(9007199254740993LL))}).as_float(),
        9007199254740992.0);
}

// --- Filesystem intrinsics: argument validation and nul-byte-path rejection ---
// These guard the error paths behind syscall.mkdir/rmdir/lseek/rename/chdir/
// exists/filesize/readdir/open_*/normalize_path that the happy-path stdlib
// tests never exercise (missing args, wrong types, embedded NUL bytes).

TEST_F(IntrinsicsTest, MkdirRmdirRoundTrip) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string dir = path_ + "_dir";
    EXPECT_EQ(registry.call("syscall.mkdir", {Value(dir)}).as_integer(), 0);
    EXPECT_EQ(registry.call("syscall.rmdir", {Value(dir)}).as_integer(), 0);
}

TEST_F(IntrinsicsTest, MkdirRequiresArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.mkdir", {}), DryadException);
}

TEST_F(IntrinsicsTest, MkdirRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.mkdir", {Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, RmdirRequiresArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.rmdir", {}), DryadException);
}

TEST_F(IntrinsicsTest, RmdirRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.rmdir", {Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, LseekRequiresThreeArguments) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.lseek", {Value(int64_t(0)), Value(int64_t(0))}),
                 DryadException);
}

TEST_F(IntrinsicsTest, RenameRequiresTwoArguments) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.rename", {Value(path_)}), DryadException);
}

TEST_F(IntrinsicsTest, RenameRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.rename", {Value(bad_path), Value(path_)}).as_integer(), -1);
    EXPECT_EQ(registry.call("syscall.rename", {Value(path_), Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, RenameMovesFile) {
    auto& registry = IntrinsicsRegistry::instance();
    std::ofstream(path_) << "renamed";
    std::string new_path = path_ + "_renamed";

    EXPECT_EQ(registry.call("syscall.rename", {Value(path_), Value(new_path)}).as_integer(), 0);
    EXPECT_FALSE(std::ifstream(path_).good());
    EXPECT_TRUE(std::ifstream(new_path).good());

    ::unlink(new_path.c_str());
}

TEST_F(IntrinsicsTest, GetcwdReturnsNonEmptyString) {
    auto& registry = IntrinsicsRegistry::instance();
    Value cwd = registry.call("syscall.getcwd", {});
    EXPECT_TRUE(cwd.is_string());
    EXPECT_FALSE(cwd.as_string().empty());
}

TEST_F(IntrinsicsTest, ChdirRequiresArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.chdir", {}), DryadException);
}

TEST_F(IntrinsicsTest, ChdirRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.chdir", {Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, ExistsRequiresStringArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.exists", {}), DryadException);
    EXPECT_THROW(registry.call("syscall.exists", {Value(int64_t(0))}), DryadException);
}

TEST_F(IntrinsicsTest, ExistsRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_FALSE(registry.call("syscall.exists", {Value(bad_path)}).as_boolean());
}

TEST_F(IntrinsicsTest, FilesizeRequiresStringArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.filesize", {}), DryadException);
    EXPECT_THROW(registry.call("syscall.filesize", {Value(int64_t(0))}), DryadException);
}

TEST_F(IntrinsicsTest, FilesizeRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.filesize", {Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, ReaddirRequiresStringArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.readdir", {}), DryadException);
    EXPECT_THROW(registry.call("syscall.readdir", {Value(int64_t(0))}), DryadException);
}

TEST_F(IntrinsicsTest, ReaddirRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    Value result = registry.call("syscall.readdir", {Value(bad_path)});
    EXPECT_TRUE(result.is_array());
    EXPECT_EQ(result.array_length(), 0);
}

TEST_F(IntrinsicsTest, ReaddirReturnsEmptyArrayForMissingDirectory) {
    auto& registry = IntrinsicsRegistry::instance();
    Value result = registry.call("syscall.readdir", {Value(std::string("/tmp/dryad_does_not_exist_xyz"))});
    EXPECT_TRUE(result.is_array());
    EXPECT_EQ(result.array_length(), 0);
}

TEST_F(IntrinsicsTest, OpenVariantsRequireStringArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    for (const char* name : {"syscall.open_read", "syscall.open_write",
                             "syscall.open_append", "syscall.open_readwrite"}) {
        EXPECT_THROW(registry.call(name, {}), DryadException) << name;
        EXPECT_THROW(registry.call(name, {Value(int64_t(0))}), DryadException) << name;
    }
}

TEST_F(IntrinsicsTest, OpenReadRejectsNulByte) {
    auto& registry = IntrinsicsRegistry::instance();
    std::string bad_path = std::string("/tmp/dryad\0nul", 14);
    EXPECT_EQ(registry.call("syscall.open_read", {Value(bad_path)}).as_integer(), -1);
}

TEST_F(IntrinsicsTest, NormalizePathReplacesBackslashes) {
    auto& registry = IntrinsicsRegistry::instance();
    Value result = registry.call("syscall.normalize_path", {Value(std::string("a\\b\\c"))});
    EXPECT_TRUE(result.is_string());
    EXPECT_EQ(result.as_string(), "a/b/c");
}

TEST_F(IntrinsicsTest, NormalizePathRequiresStringArgument) {
    auto& registry = IntrinsicsRegistry::instance();
    EXPECT_THROW(registry.call("syscall.normalize_path", {}), DryadException);
    EXPECT_THROW(registry.call("syscall.normalize_path", {Value(int64_t(0))}), DryadException);
}
