#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics.h"

using namespace dryad;

// Test allocation
TEST(MemoryIntrinsicsTest, AllocBytes) {
    Intrinsics intr;
    Value result = intr.alloc_bytes(1024);
    
    // Should return a buffer handle (integer)
    ASSERT_TRUE(result.is_integer());
    EXPECT_GT(result.as_integer(), 0);
}

// Test deallocation
TEST(MemoryIntrinsicsTest, FreeBytes) {
    Intrinsics intr;
    Value allocated = intr.alloc_bytes(512);
    ASSERT_TRUE(allocated.is_integer());
    
    Value freed = intr.free_bytes(allocated.as_integer());
    EXPECT_TRUE(freed.is_null()); // free returns void
}

// Test memcpy
TEST(MemoryIntrinsicsTest, Memcpy) {
    Intrinsics intr;
    
    Value src = intr.alloc_bytes(10);
    Value dst = intr.alloc_bytes(10);
    
    // Write test data to src
    for (int i = 0; i < 10; i++) {
        intr.buffer_set(src.as_integer(), i, i * 10);
    }
    
    // Copy from src to dst
    Value copied = intr.memcpy(dst.as_integer(), src.as_integer(), 10);
    EXPECT_TRUE(copied.is_null());
    
    // Verify dst has same data
    Value val = intr.buffer_get(dst.as_integer(), 5);
    EXPECT_EQ(val.as_integer(), 50);
    
    intr.free_bytes(src.as_integer());
    intr.free_bytes(dst.as_integer());
}

// Test memset
TEST(MemoryIntrinsicsTest, Memset) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(8);
    
    Value result = intr.memset(buf.as_integer(), 0xAA, 8);
    EXPECT_TRUE(result.is_null());
    
    // Verify all bytes are 0xAA
    for (int i = 0; i < 8; i++) {
        Value val = intr.buffer_get(buf.as_integer(), i);
        EXPECT_EQ(val.as_integer(), 0xAA);
    }
    
    intr.free_bytes(buf.as_integer());
}

// Test realloc
TEST(MemoryIntrinsicsTest, Realloc) {
    Intrinsics intr;
    
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    // Write data
    intr.buffer_set(handle, 0, 42);
    intr.buffer_set(handle, 9, 99);
    
    // Resize to 20 bytes
    Value resized = intr.realloc(handle, 20);
    int new_handle = resized.as_integer();
    
    // Old data preserved
    EXPECT_EQ(intr.buffer_get(new_handle, 0).as_integer(), 42);
    EXPECT_EQ(intr.buffer_get(new_handle, 9).as_integer(), 99);
    
    // New bytes are zero-initialized
    EXPECT_EQ(intr.buffer_get(new_handle, 10).as_integer(), 0);
    
    intr.free_bytes(new_handle);
}

// Test bounds checking on get
TEST(MemoryIntrinsicsTest, BoundsCheckingOnGet) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    // Valid access
    Value valid = intr.buffer_get(handle, 5);
    EXPECT_TRUE(valid.is_integer());
    
    // Out of bounds
    Value oob = intr.buffer_get(handle, 15);
    EXPECT_TRUE(oob.is_null());  // Error returns null
    
    intr.free_bytes(handle);
}

// Test bounds checking on set
TEST(MemoryIntrinsicsTest, BoundsCheckingOnSet) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    // Valid write
    Value valid = intr.buffer_set(handle, 5, 42);
    EXPECT_TRUE(valid.is_null());
    
    // Out of bounds
    Value oob = intr.buffer_set(handle, 15, 42);
    EXPECT_TRUE(oob.is_null());  // Error returns null
    
    intr.free_bytes(handle);
}

// Test double-free detection
TEST(MemoryIntrinsicsTest, DoubleFreeDetection) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    Value first_free = intr.free_bytes(handle);
    EXPECT_TRUE(first_free.is_null());
    
    // Second free should fail gracefully (return null)
    Value second_free = intr.free_bytes(handle);
    EXPECT_TRUE(second_free.is_null());
}
