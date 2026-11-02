#include <gtest/gtest.h>
#include "dryad/runtime/intrinsics.h"
#include <atomic>
#include <chrono>
#include <thread>
#include <sys/socket.h>
#include <unistd.h>

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
    int64_t handle = buf.as_integer();
    
    // Write data
    intr.buffer_set(handle, 0, 42);
    intr.buffer_set(handle, 9, 99);
    
    // Resize to 20 bytes
    Value resized = intr.realloc(handle, 20);
    int64_t new_handle = resized.as_integer();
    
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
    int64_t handle = buf.as_integer();
    
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
    int64_t handle = buf.as_integer();
    
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
    int64_t handle = buf.as_integer();
    
    Value first_free = intr.free_bytes(handle);
    EXPECT_TRUE(first_free.is_null());
    
    // Second free should fail gracefully (return null)
    Value second_free = intr.free_bytes(handle);
    EXPECT_TRUE(second_free.is_null());
}

TEST(MemoryIntrinsicsTest, MemmoveWithOffsets) {
    Intrinsics intr;
    Value src = intr.alloc_bytes(8);
    Value dst = intr.alloc_bytes(8);
    for (int i = 0; i < 4; ++i) {
        intr.buffer_set(src.as_integer(), i, 10 + i);
    }

    Value moved = intr.memmove(dst.as_integer(), 2, src.as_integer(), 1, 3);
    EXPECT_TRUE(moved.is_null());
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 1).as_integer(), 0);
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 2).as_integer(), 11);
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 3).as_integer(), 12);
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 4).as_integer(), 13);

    intr.free_bytes(src.as_integer());
    intr.free_bytes(dst.as_integer());
}

TEST(MemoryIntrinsicsTest, MemmoveOverlapForward) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(6);
    for (int i = 0; i < 5; ++i) {
        intr.buffer_set(buf.as_integer(), i, i + 1);
    }

    Value moved = intr.memmove(buf.as_integer(), 1, buf.as_integer(), 0, 4);
    EXPECT_TRUE(moved.is_null());
    EXPECT_EQ(intr.buffer_get(buf.as_integer(), 0).as_integer(), 1);
    EXPECT_EQ(intr.buffer_get(buf.as_integer(), 1).as_integer(), 1);
    EXPECT_EQ(intr.buffer_get(buf.as_integer(), 2).as_integer(), 2);
    EXPECT_EQ(intr.buffer_get(buf.as_integer(), 3).as_integer(), 3);
    EXPECT_EQ(intr.buffer_get(buf.as_integer(), 4).as_integer(), 4);

    intr.free_bytes(buf.as_integer());
}

TEST(MemoryIntrinsicsTest, MemmoveRejectsOverflowingOffsets) {
    Intrinsics intr;
    Value src = intr.alloc_bytes(4);
    Value dst = intr.alloc_bytes(4);
    const int64_t huge = 9223372036854775807LL;

    EXPECT_TRUE(intr.memmove(dst.as_integer(), huge, src.as_integer(), 0, 1).is_null());
    EXPECT_TRUE(intr.memmove(dst.as_integer(), 0, src.as_integer(), huge, 1).is_null());
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 0).as_integer(), 0);

    intr.free_bytes(src.as_integer());
    intr.free_bytes(dst.as_integer());
}

TEST(MemoryIntrinsicsTest, MemmoveRejectsOutOfBounds) {
    Intrinsics intr;
    Value src = intr.alloc_bytes(4);
    Value dst = intr.alloc_bytes(4);

    EXPECT_TRUE(intr.memmove(dst.as_integer(), 2, src.as_integer(), 0, 3).is_null());
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 2).as_integer(), 0);
    EXPECT_TRUE(intr.memmove(dst.as_integer(), -1, src.as_integer(), 0, 1).is_null());
    EXPECT_TRUE(intr.memmove(dst.as_integer(), 0, src.as_integer(), 0, -1).is_null());
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 2).as_integer(), 0);

    intr.free_bytes(src.as_integer());
    intr.free_bytes(dst.as_integer());
}

TEST(MemoryIntrinsicsTest, MemmoveZeroCountIsNoOp) {
    Intrinsics intr;
    Value src = intr.alloc_bytes(4);
    Value dst = intr.alloc_bytes(4);
    intr.buffer_set(dst.as_integer(), 0, 77);

    EXPECT_TRUE(intr.memmove(dst.as_integer(), 0, src.as_integer(), 0, 0).is_null());
    EXPECT_EQ(intr.buffer_get(dst.as_integer(), 0).as_integer(), 77);

    intr.free_bytes(src.as_integer());
    intr.free_bytes(dst.as_integer());
}

TEST(MemoryIntrinsicsTest, MemmoveRejectsFreedHandle) {
    Intrinsics intr;
    Value src = intr.alloc_bytes(4);
    Value dst = intr.alloc_bytes(4);
    intr.buffer_set(src.as_integer(), 0, 5);
    intr.free_bytes(dst.as_integer());

    EXPECT_TRUE(intr.memmove(dst.as_integer(), 0, src.as_integer(), 0, 1).is_null());
    EXPECT_EQ(intr.buffer_get(src.as_integer(), 0).as_integer(), 5);
    EXPECT_TRUE(intr.memmove(0, 0, src.as_integer(), 0, 1).is_null());
    EXPECT_EQ(intr.buffer_get(src.as_integer(), 0).as_integer(), 5);

    intr.free_bytes(src.as_integer());
}

TEST(MemoryIntrinsicsTest, BlockingReadDoesNotHoldAllocationLock) {
    int fds[2];
    ASSERT_EQ(::socketpair(AF_UNIX, SOCK_STREAM, 0, fds), 0);

    Value rx = MemoryManager::alloc_bytes(8);
    Value tx = MemoryManager::alloc_bytes(8);
    for (int i = 0; i < 4; ++i) {
        MemoryManager::buffer_set(tx.as_integer(), i, i + 1);
    }

    std::atomic<bool> read_done{false};
    std::thread reader([&]() {
        MemoryManager::read_fd_to_buffer(fds[0], rx.as_integer(), 0, 4);
        read_done = true;
    });

    // Let the reader take the allocation lock and block inside read().
    std::this_thread::sleep_for(std::chrono::milliseconds(100));
    EXPECT_FALSE(read_done.load());

    std::atomic<bool> write_done{false};
    std::thread writer([&]() {
        MemoryManager::write_fd_from_buffer(fds[1], tx.as_integer(), 0, 4);
        write_done = true;
    });

    for (int i = 0; i < 200 && !write_done.load(); ++i) {
        std::this_thread::sleep_for(std::chrono::milliseconds(10));
    }
    EXPECT_TRUE(write_done.load());

    // Unblock the reader whether or not the assertion held.
    ::shutdown(fds[1], SHUT_RDWR);
    reader.join();
    writer.join();
    EXPECT_EQ(MemoryManager::buffer_get(rx.as_integer(), 0).as_integer(), 1);

    MemoryManager::free_bytes(rx.as_integer());
    MemoryManager::free_bytes(tx.as_integer());
    ::close(fds[0]);
    ::close(fds[1]);
}
