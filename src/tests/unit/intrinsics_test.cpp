#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"
#include <fcntl.h>
#include <unistd.h>
#include <fstream>

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
    }
    
    void TearDown() override {
        ::unlink("/tmp/dryad_test.txt");
    }
};

TEST_F(IntrinsicsTest, FileOpenClose) {
    auto& registry = IntrinsicsRegistry::instance();
    
    Value fd = registry.call("syscall.open", {
        Value("/tmp/dryad_test.txt"),
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
        Value("/tmp/dryad_test.txt"),
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
        Value("/tmp/dryad_test.txt"),
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
    
    std::ofstream("/tmp/dryad_test.txt") << "test";
    
    Value result = registry.call("syscall.unlink", {
        Value("/tmp/dryad_test.txt")
    });
    
    EXPECT_EQ(result.as_integer(), 0);
    
    EXPECT_FALSE(std::ifstream("/tmp/dryad_test.txt").good());
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
