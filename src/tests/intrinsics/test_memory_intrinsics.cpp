#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/value.h"

namespace dryad {

class MemoryIntrinsicsTest : public ::testing::Test {
protected:
    void SetUp() override {
        IntrinsicsRegistry::instance().register_all();
    }
};

TEST_F(MemoryIntrinsicsTest, Realloc) {
    // Allocate initial memory
    std::vector<Value> malloc_args;
    malloc_args.push_back(Value(static_cast<int64_t>(100)));
    Value ptr_val = IntrinsicsRegistry::instance().call("syscall.malloc", malloc_args);
    EXPECT_TRUE(ptr_val.is_integer());
    int64_t ptr = ptr_val.as_integer();
    EXPECT_NE(ptr, 0);
    
    // Reallocate to larger size
    std::vector<Value> realloc_args;
    realloc_args.push_back(ptr_val);
    realloc_args.push_back(Value(static_cast<int64_t>(200)));
    Value new_ptr_val = IntrinsicsRegistry::instance().call("syscall.realloc", realloc_args);
    EXPECT_TRUE(new_ptr_val.is_integer());
    
    // Clean up
    std::vector<Value> free_args;
    free_args.push_back(new_ptr_val);
    IntrinsicsRegistry::instance().call("syscall.free", free_args);
}

TEST_F(MemoryIntrinsicsTest, Memcpy) {
    // Create source buffer with data
    std::vector<Value> malloc_src;
    malloc_src.push_back(Value(static_cast<int64_t>(10)));
    Value src_ptr = IntrinsicsRegistry::instance().call("syscall.malloc", malloc_src);
    
    // Create destination buffer
    std::vector<Value> malloc_dst;
    malloc_dst.push_back(Value(static_cast<int64_t>(10)));
    Value dst_ptr = IntrinsicsRegistry::instance().call("syscall.malloc", malloc_dst);
    
    // Copy memory
    std::vector<Value> memcpy_args;
    memcpy_args.push_back(dst_ptr);
    memcpy_args.push_back(src_ptr);
    memcpy_args.push_back(Value(static_cast<int64_t>(10)));
    Value result = IntrinsicsRegistry::instance().call("syscall.memcpy", memcpy_args);
    EXPECT_TRUE(result.is_integer());
    
    // Clean up
    std::vector<Value> free_args1, free_args2;
    free_args1.push_back(src_ptr);
    free_args2.push_back(dst_ptr);
    IntrinsicsRegistry::instance().call("syscall.free", free_args1);
    IntrinsicsRegistry::instance().call("syscall.free", free_args2);
}

TEST_F(MemoryIntrinsicsTest, Memset) {
    // Allocate buffer
    std::vector<Value> malloc_args;
    malloc_args.push_back(Value(static_cast<int64_t>(10)));
    Value ptr_val = IntrinsicsRegistry::instance().call("syscall.malloc", malloc_args);
    
    // Set memory to 0xFF
    std::vector<Value> memset_args;
    memset_args.push_back(ptr_val);
    memset_args.push_back(Value(static_cast<int64_t>(0xFF)));
    memset_args.push_back(Value(static_cast<int64_t>(10)));
    Value result = IntrinsicsRegistry::instance().call("syscall.memset", memset_args);
    EXPECT_TRUE(result.is_integer());
    
    // Clean up
    std::vector<Value> free_args;
    free_args.push_back(ptr_val);
    IntrinsicsRegistry::instance().call("syscall.free", free_args);
}

} // namespace dryad
