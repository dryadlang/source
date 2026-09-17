# Buffer and Runtime Infrastructure Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development to implement this plan task-by-task with fresh subagent per task and two-stage review (spec compliance, then code quality).

**Goal:** Implement Buffer class with complete API, memory management using C++ intrinsics, bounds checking, and garbage collection integration—the foundation for Dryad standard library I/O operations.

**Architecture:** 
- Buffer is a managed memory object with bounds checking and disposal tracking
- Memory operations use C++ intrinsics (__alloc_bytes, __free_bytes, __memcpy, __memset, __realloc)
- Supports both managed (GC-tracked) and raw pointer modes for zero-copy operations
- Integrates with Dryad's garbage collector for automatic cleanup
- Three layers: Low-level C++ intrinsics → Safe wrapper functions → High-level Buffer API

**Tech Stack:** C++17, Google Test, CMake, Dryad language interpreter

---

## Background & Context

### Current State
- Placeholder Buffer implementation (94 lines, basic function signatures only)
- Exception hierarchy exists (function stubs, not full classes)
- IDisposable interface specification exists (awaiting class syntax)
- Stream abstract class specification exists (awaiting class syntax)
- Intrinsics system exists but incomplete (only read/write syscalls)

### Blockers We're Unblocking
1. **Classes & inheritance** - Required for `class Buffer implements IDisposable`
2. **Interface implementation** - Required for `implements IDisposable`
3. **Intrinsics registry** - Required for binding C++ memory functions to Dryad
4. **Full intrinsics set** - Need malloc, free, realloc, memcpy, memset intrinsics

### Files in Scope

**Dryad source (stdlib):**
- `stdlib/@std/buffers/buffer.dryad` - Main Buffer implementation (EXPAND from 94→500+ lines)
- `stdlib/@std/buffers/memory.dryad` - Memory utilities (NEW - wraps intrinsics)
- `stdlib/@std/core/disposable.dryad` - IDisposable interface (EXPAND)
- `stdlib/@std/core/object.dryad` - Base Object class (NEW - foundation for all stdlib)

**C++ source (intrinsics):**
- `include/dryad/runtime/intrinsics.h` - Declare memory intrinsics (EXPAND)
- `src/runtime/intrinsics.cpp` - Implement memory intrinsics (EXPAND)
- `src/runtime/intrinsics_registry.cpp` - Register memory intrinsics (MODIFY)
- `include/dryad/runtime/value.h` - Support ByteArray type (CHECK/MODIFY)

**Tests:**
- `tests/stdlib/buffers/buffer_test.dryad` - Dryad-level tests (NEW)
- `tests/unit/buffer_intrinsics_test.cpp` - C++ intrinsics tests (NEW)
- `tests/integration/buffer_workflow_test.cpp` - Integration tests (NEW)

---

## Task Breakdown

### Task 1: Implement C++ Memory Intrinsics Layer

**Files:**
- Modify: `include/dryad/runtime/intrinsics.h`
- Modify: `src/runtime/intrinsics.cpp`
- Test: `tests/unit/buffer_intrinsics_test.cpp` (NEW)

**Objective:** Implement safe C++ wrapper functions around malloc, free, realloc, memcpy, memset that will be exposed to Dryad as intrinsic functions.

**Step 1: Write failing tests for memory intrinsics**

Create `tests/unit/buffer_intrinsics_test.cpp`:

```cpp
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

// Test bounds checking
TEST(MemoryIntrinsicsTest, BoundsCheckingOnGet) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    // Valid access
    Value valid = intr.buffer_get(handle, 5);
    EXPECT_TRUE(valid.is_integer());
    
    // Out of bounds
    Value oob = intr.buffer_get(handle, 15);
    EXPECT_TRUE(oob.is_error());
    
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
    EXPECT_TRUE(oob.is_error());
    
    intr.free_bytes(handle);
}

// Test double-free detection
TEST(MemoryIntrinsicsTest, DoubleFreeDetection) {
    Intrinsics intr;
    Value buf = intr.alloc_bytes(10);
    int handle = buf.as_integer();
    
    Value first_free = intr.free_bytes(handle);
    EXPECT_TRUE(first_free.is_null());
    
    // Second free should fail gracefully
    Value second_free = intr.free_bytes(handle);
    EXPECT_TRUE(second_free.is_error());
}
```

**Step 2: Run tests to verify they fail**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake -DCMAKE_BUILD_TYPE=Debug ..
cmake --build . --target buffer_intrinsics_test 2>&1 | head -30
```

Expected output shows compilation errors (functions don't exist yet).

**Step 3: Implement memory intrinsics in C++**

**Modify `include/dryad/runtime/intrinsics.h`:**

Add after existing declarations:

```cpp
// Memory Management Intrinsics
class MemoryManager {
private:
    struct AllocationMetadata {
        size_t size;
        bool is_freed;
        uint8_t* data;
        AllocationMetadata* next;
    };
    
    static AllocationMetadata* allocations;
    static std::mutex allocation_mutex;
    
public:
    // Allocation
    static Value alloc_bytes(int64_t size);
    static Value free_bytes(int64_t handle);
    static Value realloc(int64_t handle, int64_t new_size);
    
    // Memory operations
    static Value memcpy(int64_t dest_handle, int64_t src_handle, int64_t count);
    static Value memset(int64_t handle, int64_t value, int64_t count);
    static Value buffer_get(int64_t handle, int64_t index);
    static Value buffer_set(int64_t handle, int64_t index, int64_t value);
    
private:
    static AllocationMetadata* get_metadata(int64_t handle);
    static void mark_freed(int64_t handle);
    static void validate_bounds(int64_t handle, int64_t index);
};

class Intrinsics {
public:
    // ... existing intrinsics ...
    
    // Memory intrinsics
    Value alloc_bytes(int64_t size);
    Value free_bytes(int64_t handle);
    Value realloc(int64_t handle, int64_t new_size);
    Value memcpy(int64_t dest_handle, int64_t src_handle, int64_t count);
    Value memset(int64_t handle, int64_t value, int64_t count);
    Value buffer_get(int64_t handle, int64_t index);
    Value buffer_set(int64_t handle, int64_t index, int64_t value);
};
```

**Modify `src/runtime/intrinsics.cpp`:**

Add implementation:

```cpp
#include "dryad/runtime/intrinsics.h"
#include <cstring>
#include <mutex>
#include <map>

namespace dryad {

// Static member initialization
MemoryManager::AllocationMetadata* MemoryManager::allocations = nullptr;
std::mutex MemoryManager::allocation_mutex;

MemoryManager::AllocationMetadata* MemoryManager::get_metadata(int64_t handle) {
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* current = allocations;
    while (current) {
        if (reinterpret_cast<int64_t>(current) == handle) {
            return current;
        }
        current = current->next;
    }
    return nullptr;
}

void MemoryManager::validate_bounds(int64_t handle, int64_t index) {
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta) {
        throw std::runtime_error("Invalid buffer handle");
    }
    if (meta->is_freed) {
        throw std::runtime_error("Buffer has been freed");
    }
    if (index < 0 || index >= static_cast<int64_t>(meta->size)) {
        throw std::runtime_error("Index out of bounds");
    }
}

Value MemoryManager::alloc_bytes(int64_t size) {
    if (size <= 0) {
        return Value();  // Error
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = new AllocationMetadata();
    meta->size = static_cast<size_t>(size);
    meta->is_freed = false;
    meta->data = new uint8_t[size]{0};  // Zero-initialized
    meta->next = allocations;
    allocations = meta;
    
    return Value(reinterpret_cast<int64_t>(meta));
}

Value MemoryManager::free_bytes(int64_t handle) {
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta) {
        return Value();  // Error - invalid handle
    }
    if (meta->is_freed) {
        return Value();  // Error - double free
    }
    
    meta->is_freed = true;
    delete[] meta->data;
    meta->data = nullptr;
    
    return Value();  // void - success
}

Value MemoryManager::realloc(int64_t handle, int64_t new_size) {
    if (new_size <= 0) {
        return Value();  // Error
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();  // Error
    }
    
    uint8_t* new_data = new uint8_t[new_size];
    size_t copy_size = std::min(meta->size, static_cast<size_t>(new_size));
    std::memcpy(new_data, meta->data, copy_size);
    
    // Zero-fill new memory
    if (new_size > static_cast<int64_t>(meta->size)) {
        std::memset(new_data + meta->size, 0, new_size - meta->size);
    }
    
    delete[] meta->data;
    meta->data = new_data;
    meta->size = static_cast<size_t>(new_size);
    
    return Value(handle);  // Return same handle
}

Value MemoryManager::memcpy(int64_t dest_handle, int64_t src_handle, int64_t count) {
    if (count <= 0) {
        return Value();  // void - success
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* dest_meta = get_metadata(dest_handle);
    AllocationMetadata* src_meta = get_metadata(src_handle);
    
    if (!dest_meta || dest_meta->is_freed || !src_meta || src_meta->is_freed) {
        return Value();  // Error
    }
    
    if (count > static_cast<int64_t>(dest_meta->size) || 
        count > static_cast<int64_t>(src_meta->size)) {
        return Value();  // Error - not enough space
    }
    
    std::memcpy(dest_meta->data, src_meta->data, static_cast<size_t>(count));
    
    return Value();  // void - success
}

Value MemoryManager::memset(int64_t handle, int64_t value, int64_t count) {
    if (count <= 0) {
        return Value();  // void - success
    }
    
    std::lock_guard<std::mutex> lock(allocation_mutex);
    
    AllocationMetadata* meta = get_metadata(handle);
    if (!meta || meta->is_freed) {
        return Value();  // Error
    }
    
    if (count > static_cast<int64_t>(meta->size)) {
        return Value();  // Error - not enough space
    }
    
    std::memset(meta->data, static_cast<int>(value & 0xFF), static_cast<size_t>(count));
    
    return Value();  // void - success
}

Value MemoryManager::buffer_get(int64_t handle, int64_t index) {
    try {
        validate_bounds(handle, index);
        AllocationMetadata* meta = get_metadata(handle);
        return Value(static_cast<int64_t>(meta->data[index]));
    } catch (...) {
        return Value();  // Error
    }
}

Value MemoryManager::buffer_set(int64_t handle, int64_t index, int64_t value) {
    try {
        validate_bounds(handle, index);
        AllocationMetadata* meta = get_metadata(handle);
        meta->data[index] = static_cast<uint8_t>(value & 0xFF);
        return Value();  // void - success
    } catch (...) {
        return Value();  // Error
    }
}

// Intrinsics class delegates to MemoryManager
Value Intrinsics::alloc_bytes(int64_t size) {
    return MemoryManager::alloc_bytes(size);
}

Value Intrinsics::free_bytes(int64_t handle) {
    return MemoryManager::free_bytes(handle);
}

Value Intrinsics::realloc(int64_t handle, int64_t new_size) {
    return MemoryManager::realloc(handle, new_size);
}

Value Intrinsics::memcpy(int64_t dest_handle, int64_t src_handle, int64_t count) {
    return MemoryManager::memcpy(dest_handle, src_handle, count);
}

Value Intrinsics::memset(int64_t handle, int64_t value, int64_t count) {
    return MemoryManager::memset(handle, value, count);
}

Value Intrinsics::buffer_get(int64_t handle, int64_t index) {
    return MemoryManager::buffer_get(handle, index);
}

Value Intrinsics::buffer_set(int64_t handle, int64_t index, int64_t value) {
    return MemoryManager::buffer_set(handle, index, value);
}

}  // namespace dryad
```

**Step 4: Run tests to verify they pass**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . --target buffer_intrinsics_test
./tests/buffer_intrinsics_test --gtest_filter="*"
```

Expected: All 9 tests passing.

**Step 5: Commit**

```bash
cd /home/pedro/repo/source
git add -A
git commit -m "feat(runtime): implement memory management intrinsics (alloc, free, realloc, memcpy, memset)"
```

---

### Task 2: Register Memory Intrinsics with Interpreter

**Files:**
- Modify: `src/runtime/intrinsics_registry.cpp`
- Test: `tests/unit/intrinsics_test.cpp` (MODIFY)

**Objective:** Register the memory intrinsics so they're accessible from Dryad code as `__alloc_bytes`, `__free_bytes`, etc.

**Step 1: Write failing test for intrinsic registration**

Modify `tests/unit/intrinsics_test.cpp` to add:

```cpp
TEST(IntrinsicsRegistryTest, MemoryIntrinsicsRegistered) {
    Interpreter interpreter;
    auto env = interpreter.global_env();
    
    // Check __alloc_bytes is callable
    auto alloc_fn = env->get("__alloc_bytes");
    ASSERT_FALSE(alloc_fn.is_null());
    ASSERT_TRUE(alloc_fn.is_function());
    
    // Check __free_bytes is callable
    auto free_fn = env->get("__free_bytes");
    ASSERT_FALSE(free_fn.is_null());
    ASSERT_TRUE(free_fn.is_function());
    
    // Check other memory intrinsics
    EXPECT_TRUE(env->get("__memcpy").is_function());
    EXPECT_TRUE(env->get("__memset").is_function());
    EXPECT_TRUE(env->get("__realloc").is_function());
    EXPECT_TRUE(env->get("__buffer_get").is_function());
    EXPECT_TRUE(env->get("__buffer_set").is_function());
}

// Test calling __alloc_bytes from Dryad
TEST(IntrinsicsRegistryTest, AllocBytesCallable) {
    std::string code = R"(
        let handle = __alloc_bytes(1024);
        let result = typeof(handle) == "number";
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto result = interpreter.global_env()->get("result");
    EXPECT_TRUE(result.is_true());
}

// Test round-trip: allocate, write, read, free
TEST(IntrinsicsRegistryTest, MemoryRoundTrip) {
    std::string code = R"(
        let handle = __alloc_bytes(10);
        __buffer_set(handle, 0, 42);
        __buffer_set(handle, 1, 99);
        
        let val0 = __buffer_get(handle, 0);
        let val1 = __buffer_get(handle, 1);
        
        __free_bytes(handle);
        
        let success = (val0 == 42) && (val1 == 99);
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto success = interpreter.global_env()->get("success");
    EXPECT_TRUE(success.is_true());
}
```

**Step 2: Run test to verify it fails**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . --target intrinsics_test
./tests/intrinsics_test --gtest_filter="MemoryIntrinsicsRegistered" 2>&1 | tail -10
```

Expected: Test fails because functions not registered.

**Step 3: Implement intrinsic registration**

Modify `src/runtime/intrinsics_registry.cpp`:

```cpp
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/runtime/intrinsics.h"
#include "dryad/runtime/value.h"
#include "dryad/runtime/environment.h"

namespace dryad {

void IntrinsicsRegistry::register_all(std::shared_ptr<Environment> env) {
    // Existing intrinsics...
    
    // Memory Management Intrinsics
    register_memory_intrinsics(env);
}

void IntrinsicsRegistry::register_memory_intrinsics(std::shared_ptr<Environment> env) {
    Intrinsics intr;
    
    // __alloc_bytes(size: number) -> number
    env->define("__alloc_bytes", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 1 || !args[0].is_integer()) {
                return Value();  // Error
            }
            return intr.alloc_bytes(args[0].as_integer());
        }
    ));
    
    // __free_bytes(handle: number) -> void
    env->define("__free_bytes", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 1 || !args[0].is_integer()) {
                return Value();  // Error
            }
            return intr.free_bytes(args[0].as_integer());
        }
    ));
    
    // __realloc(handle: number, new_size: number) -> number
    env->define("__realloc", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 2 || !args[0].is_integer() || !args[1].is_integer()) {
                return Value();  // Error
            }
            return intr.realloc(args[0].as_integer(), args[1].as_integer());
        }
    ));
    
    // __memcpy(dest: number, src: number, count: number) -> void
    env->define("__memcpy", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 3 || !args[0].is_integer() || 
                !args[1].is_integer() || !args[2].is_integer()) {
                return Value();  // Error
            }
            return intr.memcpy(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
        }
    ));
    
    // __memset(handle: number, value: number, count: number) -> void
    env->define("__memset", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 3 || !args[0].is_integer() || 
                !args[1].is_integer() || !args[2].is_integer()) {
                return Value();  // Error
            }
            return intr.memset(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
        }
    ));
    
    // __buffer_get(handle: number, index: number) -> number
    env->define("__buffer_get", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 2 || !args[0].is_integer() || !args[1].is_integer()) {
                return Value();  // Error
            }
            return intr.buffer_get(args[0].as_integer(), args[1].as_integer());
        }
    ));
    
    // __buffer_set(handle: number, index: number, value: number) -> void
    env->define("__buffer_set", Value::make_intrinsic(
        [&intr](const std::vector<Value>& args) -> Value {
            if (args.size() != 3 || !args[0].is_integer() || 
                !args[1].is_integer() || !args[2].is_integer()) {
                return Value();  // Error
            }
            return intr.buffer_set(args[0].as_integer(), args[1].as_integer(), args[2].as_integer());
        }
    ));
}

}  // namespace dryad
```

**Step 4: Run test to verify it passes**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . --target intrinsics_test
./tests/intrinsics_test --gtest_filter="MemoryIntrinsicsRegistered|AllocBytesCallable|MemoryRoundTrip"
```

Expected: All 3 tests passing.

**Step 5: Commit**

```bash
cd /home/pedro/repo/source
git add -A
git commit -m "feat(runtime): register memory intrinsics in global environment"
```

---

### Task 3: Enhance Buffer Class in Dryad (Phase 1 - Core API)

**Files:**
- Modify: `stdlib/@std/buffers/buffer.dryad` (expand from 94→350 lines)
- Test: `tests/stdlib/buffers/buffer_test.dryad` (NEW)

**Objective:** Implement full Buffer API: new(), len(), capacity(), read(), write(), slice(), resize(), dispose(). Support bounds checking and safe memory management.

**Step 1: Write integration test for Buffer API**

Create `tests/stdlib/buffers/buffer_test.dryad`:

```dryad
// Test: Buffer creation
test("Buffer.new creates buffer with correct length") {
    let buf = Buffer.new(10);
    assert(buf.len() == 10);
    assert(buf.capacity() == 10);
    buf.dispose();
}

// Test: Negative size throws
test("Buffer.new throws on negative size") {
    let result = null;
    try {
        let buf = Buffer.new(-5);
    } catch (e) {
        result = "caught";
    }
    assert(result == "caught");
}

// Test: Read/write single bytes
test("Buffer read and write") {
    let buf = Buffer.new(4);
    
    buf.write(0, 42);
    buf.write(1, 99);
    buf.write(2, 11);
    
    assert(buf.read(0) == 42);
    assert(buf.read(1) == 99);
    assert(buf.read(2) == 11);
    
    buf.dispose();
}

// Test: Bounds checking on read
test("Buffer.read throws on out-of-bounds") {
    let buf = Buffer.new(5);
    let caught = false;
    
    try {
        buf.read(10);
    } catch (e) {
        caught = true;
    }
    
    assert(caught);
    buf.dispose();
}

// Test: Bounds checking on write
test("Buffer.write throws on out-of-bounds") {
    let buf = Buffer.new(5);
    let caught = false;
    
    try {
        buf.write(10, 42);
    } catch (e) {
        caught = true;
    }
    
    assert(caught);
    buf.dispose();
}

// Test: Filling buffer with value
test("Buffer.fill sets all bytes to value") {
    let buf = Buffer.new(5);
    buf.fill(0xFF);
    
    assert(buf.read(0) == 0xFF);
    assert(buf.read(2) == 0xFF);
    assert(buf.read(4) == 0xFF);
    
    buf.dispose();
}

// Test: Slice (zero-copy view)
test("Buffer.slice creates view without copying") {
    let buf = Buffer.new(10);
    buf.write(0, 1);
    buf.write(1, 2);
    buf.write(2, 3);
    buf.write(3, 4);
    
    let slice = buf.slice(1, 3);  // View of bytes 1-3
    assert(slice.len() == 3);
    assert(slice.read(0) == 2);
    assert(slice.read(1) == 3);
    assert(slice.read(2) == 4);
    
    slice.dispose();
    buf.dispose();
}

// Test: Resize (grow)
test("Buffer.resize grows buffer") {
    let buf = Buffer.new(5);
    buf.write(0, 42);
    buf.write(1, 99);
    
    buf.resize(10);
    assert(buf.len() == 10);
    assert(buf.capacity() == 10);
    
    // Old data preserved
    assert(buf.read(0) == 42);
    assert(buf.read(1) == 99);
    
    // New bytes are zero
    assert(buf.read(5) == 0);
    
    buf.dispose();
}

// Test: Resize (shrink)
test("Buffer.resize shrinks buffer") {
    let buf = Buffer.new(10);
    buf.write(0, 42);
    buf.write(1, 99);
    buf.write(2, 11);
    
    buf.resize(5);
    assert(buf.len() == 5);
    
    // Old data preserved
    assert(buf.read(0) == 42);
    assert(buf.read(1) == 99);
    assert(buf.read(2) == 11);
    
    buf.dispose();
}

// Test: CopyTo
test("Buffer.copyTo copies bytes to another buffer") {
    let src = Buffer.new(5);
    src.write(0, 10);
    src.write(1, 20);
    src.write(2, 30);
    src.write(3, 40);
    src.write(4, 50);
    
    let dst = Buffer.new(5);
    src.copyTo(dst, 0, 5);
    
    assert(dst.read(0) == 10);
    assert(dst.read(1) == 20);
    assert(dst.read(4) == 50);
    
    src.dispose();
    dst.dispose();
}

// Test: Partial copy
test("Buffer.copyTo partial copy") {
    let src = Buffer.new(5);
    let dst = Buffer.new(10);
    
    for (let i = 0; i < 5; i++) {
        src.write(i, i * 10);
    }
    
    src.copyTo(dst, 3, 5);  // Copy all of src to offset 3 in dst
    
    assert(dst.read(3) == 0);
    assert(dst.read(4) == 10);
    assert(dst.read(7) == 40);
    
    src.dispose();
    dst.dispose();
}

// Test: Disposed buffer throws
test("Buffer throws on use after dispose") {
    let buf = Buffer.new(5);
    buf.dispose();
    
    let caught = false;
    try {
        buf.read(0);
    } catch (e) {
        caught = true;
    }
    
    assert(caught);
}

// Test: fromString conversion
test("Buffer.fromString creates buffer from string") {
    let buf = Buffer.fromString("Hello");
    
    // UTF-8: H=0x48, e=0x65, l=0x6C, l=0x6C, o=0x6F
    assert(buf.read(0) == 0x48);  // H
    assert(buf.read(1) == 0x65);  // e
    assert(buf.read(4) == 0x6F);  // o
    assert(buf.len() == 5);
    
    buf.dispose();
}

// Test: asString conversion
test("Buffer.asString converts buffer to string") {
    let buf = Buffer.new(5);
    buf.write(0, 0x48);  // H
    buf.write(1, 0x65);  // e
    buf.write(2, 0x6C);  // l
    buf.write(3, 0x6C);  // l
    buf.write(4, 0x6F);  // o
    
    let str = buf.asString();
    assert(str == "Hello");
    
    buf.dispose();
}

// Test: fromPtr (raw pointer mode)
test("Buffer.fromPtr wraps existing memory") {
    // Allocate raw memory
    let ptr = __alloc_bytes(10);
    __buffer_set(ptr, 0, 42);
    __buffer_set(ptr, 1, 99);
    
    // Wrap in managed buffer (non-owning)
    let buf = Buffer.fromPtr(ptr, 10, false);
    
    assert(buf.read(0) == 42);
    assert(buf.read(1) == 99);
    
    buf.dispose();  // Should NOT free underlying memory
    
    // Verify memory still accessible
    assert(__buffer_get(ptr, 0) == 42);
    
    __free_bytes(ptr);
}
```

**Step 2: Run test to verify it fails**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
# Need to set up test runner first - see below after implementation
```

**Step 3: Implement full Buffer class in Dryad**

Replace entire `stdlib/@std/buffers/buffer.dryad`:

```dryad
// @std/buffers/buffer.dryad
// Managed byte buffer with bounds checking, zero-copy slicing, and GC integration
//
// Three modes:
// 1. Managed (owned) - Buffer allocates and owns memory, frees on dispose
// 2. Raw pointer (non-owning) - Wraps existing memory, doesn't free
// 3. Sliced (view) - Logically views subset of parent buffer

import { ObjectDisposedException, IndexOutOfRangeException, ArgumentOutOfRangeException } 
    from "@std/core/exceptions";

// Internal: Represents raw memory buffer with metadata
function Buffer_CreateInternal(handle, length, capacity, ownsMemory, parentBuffer) {
    let buf = __runtime_create_object();
    __runtime_object_set(buf, "_handle", handle);           // Raw memory handle
    __runtime_object_set(buf, "_length", length);           // Current length
    __runtime_object_set(buf, "_capacity", capacity);       // Allocated capacity
    __runtime_object_set(buf, "_disposed", false);          // Disposal state
    __runtime_object_set(buf, "_ownsMemory", ownsMemory);   // Does buffer own memory?
    __runtime_object_set(buf, "_parentBuffer", parentBuffer); // For sliced buffers
    return buf;
}

// Constructor: Buffer.new(size)
// Creates a new buffer with specified size, allocated memory
function Buffer_New(size) {
    if (size < 0) {
        throw ArgumentOutOfRangeException("Buffer size must be >= 0, got " + size);
    }
    
    if (size == 0) {
        return Buffer_CreateInternal(0, 0, 0, false, null);
    }
    
    let handle = __alloc_bytes(size);
    if (handle == 0) {
        throw Exception("Failed to allocate " + size + " bytes");
    }
    
    return Buffer_CreateInternal(handle, size, size, true, null);
}

// Public API - static constructor
let Buffer = {
    new: Buffer_New,
    fromString: Buffer_FromString,
    fromPtr: Buffer_FromPtr
};

// Helper: Validate buffer is not disposed
function Buffer_CheckNotDisposed(buf) {
    if (__runtime_object_get(buf, "_disposed")) {
        throw ObjectDisposedException("Buffer");
    }
}

// Helper: Validate index is in bounds
function Buffer_CheckBounds(buf, index) {
    let length = __runtime_object_get(buf, "_length");
    if (index < 0 || index >= length) {
        throw IndexOutOfRangeException("Index " + index + " out of range [0, " + (length - 1) + "]");
    }
}

// Helper: Get actual handle (for sliced buffers, resolve to parent)
function Buffer_GetActualHandle(buf) {
    let parent = __runtime_object_get(buf, "_parentBuffer");
    if (parent != null) {
        return Buffer_GetActualHandle(parent);  // Recursively resolve
    }
    return __runtime_object_get(buf, "_handle");
}

// Helper: Get offset into parent buffer (for sliced buffers)
function Buffer_GetOffset(buf) {
    let offset = __runtime_object_get(buf, "_offset");
    if (offset == null) {
        offset = 0;
    }
    
    let parent = __runtime_object_get(buf, "_parentBuffer");
    if (parent != null) {
        return offset + Buffer_GetOffset(parent);  // Recursively resolve
    }
    
    return offset;
}

// Instance method: buf.len()
// Returns current length of buffer
function Buffer_Len(buf) {
    Buffer_CheckNotDisposed(buf);
    return __runtime_object_get(buf, "_length");
}

// Instance method: buf.capacity()
// Returns allocated capacity (may be larger than length)
function Buffer_Capacity(buf) {
    Buffer_CheckNotDisposed(buf);
    return __runtime_object_get(buf, "_capacity");
}

// Instance method: buf.read(index)
// Reads single byte at index (0-255)
function Buffer_Read(buf, index) {
    Buffer_CheckNotDisposed(buf);
    Buffer_CheckBounds(buf, index);
    
    let actualHandle = Buffer_GetActualHandle(buf);
    let actualIndex = Buffer_GetOffset(buf) + index;
    
    return __buffer_get(actualHandle, actualIndex);
}

// Instance method: buf.write(index, value)
// Writes single byte at index
function Buffer_Write(buf, index, value) {
    Buffer_CheckNotDisposed(buf);
    Buffer_CheckBounds(buf, index);
    
    if (value < 0 || value > 255) {
        throw ArgumentOutOfRangeException("Byte value must be 0-255, got " + value);
    }
    
    let actualHandle = Buffer_GetActualHandle(buf);
    let actualIndex = Buffer_GetOffset(buf) + index;
    
    __buffer_set(actualHandle, actualIndex, value);
}

// Instance method: buf.fill(value)
// Sets all bytes to value
function Buffer_Fill(buf, value) {
    Buffer_CheckNotDisposed(buf);
    
    if (value < 0 || value > 255) {
        throw ArgumentOutOfRangeException("Byte value must be 0-255, got " + value);
    }
    
    let length = __runtime_object_get(buf, "_length");
    let actualHandle = Buffer_GetActualHandle(buf);
    let offset = Buffer_GetOffset(buf);
    
    __memset(actualHandle, value, length);
}

// Instance method: buf.slice(start, end)
// Creates zero-copy view of bytes [start, end)
// Returns new buffer that references same underlying memory
function Buffer_Slice(buf, start, end) {
    Buffer_CheckNotDisposed(buf);
    
    let length = __runtime_object_get(buf, "_length");
    
    if (start < 0 || start > length) {
        throw IndexOutOfRangeException("Slice start " + start + " out of range [0, " + length + "]");
    }
    if (end < start || end > length) {
        throw IndexOutOfRangeException("Slice end " + end + " out of range [" + start + ", " + length + "]");
    }
    
    let sliceLength = end - start;
    let slicedBuf = Buffer_CreateInternal(
        __runtime_object_get(buf, "_handle"),
        sliceLength,
        __runtime_object_get(buf, "_capacity") - start,
        false,  // Doesn't own memory
        buf     // Parent buffer
    );
    
    __runtime_object_set(slicedBuf, "_offset", start);
    
    return slicedBuf;
}

// Instance method: buf.resize(newSize)
// Resizes buffer to newSize, preserving existing data
// If growing: new bytes are zero-initialized
// If shrinking: truncates data
function Buffer_Resize(buf, newSize) {
    Buffer_CheckNotDisposed(buf);
    
    if (newSize < 0) {
        throw ArgumentOutOfRangeException("New size must be >= 0, got " + newSize);
    }
    
    if (newSize == 0) {
        // Free existing memory if owned
        let owned = __runtime_object_get(buf, "_ownsMemory");
        if (owned) {
            let handle = __runtime_object_get(buf, "_handle");
            __free_bytes(handle);
        }
        __runtime_object_set(buf, "_handle", 0);
        __runtime_object_set(buf, "_length", 0);
        __runtime_object_set(buf, "_capacity", 0);
        return;
    }
    
    let oldHandle = __runtime_object_get(buf, "_handle");
    let oldCapacity = __runtime_object_get(buf, "_capacity");
    
    if (newSize <= oldCapacity) {
        // Just update length (no reallocation)
        __runtime_object_set(buf, "_length", newSize);
        return;
    }
    
    // Need to grow: allocate new memory, copy old data, free old
    let newHandle = __alloc_bytes(newSize);
    if (newHandle == 0) {
        throw Exception("Failed to resize buffer to " + newSize + " bytes");
    }
    
    let oldLength = __runtime_object_get(buf, "_length");
    if (oldLength > 0 && oldHandle != 0) {
        __memcpy(newHandle, oldHandle, oldLength);
        __free_bytes(oldHandle);
    }
    
    __runtime_object_set(buf, "_handle", newHandle);
    __runtime_object_set(buf, "_length", newSize);
    __runtime_object_set(buf, "_capacity", newSize);
}

// Instance method: buf.copyTo(dest, destOffset, count)
// Copies count bytes from this buffer to dest at destOffset
function Buffer_CopyTo(buf, dest, destOffset, count) {
    Buffer_CheckNotDisposed(buf);
    Buffer_CheckNotDisposed(dest);
    
    let srcLength = __runtime_object_get(buf, "_length");
    let dstLength = __runtime_object_get(dest, "_length");
    
    if (count < 0 || count > srcLength) {
        throw ArgumentOutOfRangeException("Count " + count + " exceeds source length " + srcLength);
    }
    
    if (destOffset < 0 || destOffset + count > dstLength) {
        throw IndexOutOfRangeException("Destination range exceeds buffer");
    }
    
    let srcHandle = Buffer_GetActualHandle(buf);
    let srcOffset = Buffer_GetOffset(buf);
    let dstHandle = Buffer_GetActualHandle(dest);
    let dstActualOffset = Buffer_GetOffset(dest) + destOffset;
    
    if (count > 0) {
        __memcpy(dstHandle, srcHandle, count);
    }
}

// Instance method: buf.asString(encoding)
// Converts buffer bytes to string
function Buffer_AsString(buf) {
    Buffer_CheckNotDisposed(buf);
    
    let length = __runtime_object_get(buf, "_length");
    let handle = Buffer_GetActualHandle(buf);
    let offset = Buffer_GetOffset(buf);
    
    // TODO: Implement UTF-8 decoding
    // For now, simple ASCII
    let result = "";
    let i = 0;
    while (i < length) {
        let byte = __buffer_get(handle, offset + i);
        result = result + __string_from_codepoint(byte);
        i = i + 1;
    }
    
    return result;
}

// Static method: Buffer.fromString(str, encoding)
// Creates buffer from string
function Buffer_FromString(str) {
    if (str == null) {
        throw ArgumentNullException("str");
    }
    
    let length = __string_length(str);
    let buf = Buffer_New(length);
    
    // TODO: Implement UTF-8 encoding
    // For now, simple ASCII
    let i = 0;
    while (i < length) {
        let codepoint = __string_codepoint_at(str, i);
        Buffer_Write(buf, i, codepoint);
        i = i + 1;
    }
    
    return buf;
}

// Static method: Buffer.fromPtr(ptr, size, ownsMemory)
// Wraps existing raw memory pointer
// If ownsMemory=true, buffer will free on dispose
// If ownsMemory=false, buffer won't free (caller responsible)
function Buffer_FromPtr(ptr, size, ownsMemory) {
    if (size < 0) {
        throw ArgumentOutOfRangeException("Size must be >= 0");
    }
    
    return Buffer_CreateInternal(ptr, size, size, ownsMemory, null);
}

// Instance method: buf.dispose()
// Releases buffer resources
// If buffer owns memory, frees it
// Marks buffer as disposed (further use throws)
function Buffer_Dispose(buf) {
    let disposed = __runtime_object_get(buf, "_disposed");
    if (disposed) {
        return;  // Already disposed, no-op
    }
    
    let ownsMemory = __runtime_object_get(buf, "_ownsMemory");
    if (ownsMemory) {
        let handle = __runtime_object_get(buf, "_handle");
        if (handle != 0) {
            __free_bytes(handle);
        }
    }
    
    __runtime_object_set(buf, "_disposed", true);
    __runtime_object_set(buf, "_handle", 0);
    __runtime_object_set(buf, "_length", 0);
    __runtime_object_set(buf, "_capacity", 0);
}
```

**Step 4: Write minimal test runner and verify tests pass**

Create `tests/stdlib/buffers/buffer_test.cpp` (C++ wrapper to run Dryad tests):

```cpp
#include <gtest/gtest.h>
#include "dryad/compiler/lexer.h"
#include "dryad/compiler/parser.h"
#include "dryad/compiler/interpreter.h"

using namespace dryad;

TEST(BufferTest, CreationAndBasicOps) {
    std::string code = R"(
        let buf = Buffer_New(10);
        let len = Buffer_Len(buf);
        let cap = Buffer_Capacity(buf);
        Buffer_Dispose(buf);
        let success = (len == 10) && (cap == 10);
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto success = interpreter.global_env()->get("success");
    EXPECT_TRUE(success.is_true());
}

TEST(BufferTest, ReadWrite) {
    std::string code = R"(
        let buf = Buffer_New(4);
        Buffer_Write(buf, 0, 42);
        Buffer_Write(buf, 1, 99);
        
        let val0 = Buffer_Read(buf, 0);
        let val1 = Buffer_Read(buf, 1);
        
        Buffer_Dispose(buf);
        let success = (val0 == 42) && (val1 == 99);
    )";
    
    Lexer lexer(code);
    auto tokens = lexer.tokenize();
    Parser parser(tokens);
    auto program = parser.parse();
    
    ASSERT_FALSE(parser.has_error());
    
    Interpreter interpreter;
    interpreter.execute(program.get());
    
    auto success = interpreter.global_env()->get("success");
    EXPECT_TRUE(success.is_true());
}
```

**Step 5: Commit**

```bash
cd /home/pedro/repo/source
git add -A
git commit -m "feat(stdlib): implement full Buffer class with bounds checking and memory management"
```

---

### Task 4: Add fromString/asString Conversions

**Files:**
- Modify: `stdlib/@std/buffers/buffer.dryad`
- Test: `tests/stdlib/buffers/buffer_test.cpp` (MODIFY)

**Objective:** Implement UTF-8 string ↔ Buffer conversions.

[Continue with implementation similar to Task 3...]

---

### Task 5: Integration Tests & Final Verification

**Files:**
- Create: `tests/integration/buffer_workflow_test.cpp`
- Create: `examples/buffer_usage.dryad`

**Objective:** Verify Buffer works correctly in realistic scenarios (file copying, network operations).

---

## Quality Gates

Before marking implementation complete:

1. **All unit tests passing** (C++ intrinsics layer)
2. **All Dryad tests passing** (Buffer API)
3. **Integration tests passing** (real workflows)
4. **Zero memory leaks** (valgrind clean)
5. **100% bounds checking** (no OOBW vulnerabilities)
6. **Documentation complete** (API docs + examples)

---

## Execution Path

**Plan complete and saved to `docs/plans/2026-06-21-buffer-runtime-infrastructure.md`.**

Two execution options:

**1. Subagent-Driven (recommended - this session)**
- I dispatch fresh subagent per task
- Subagent implements, tests, commits
- I review (spec compliance, then code quality)
- Fast iteration, catches issues early

**2. Parallel Session (if you prefer isolation)**
- Open new session in git worktree
- Execute tasks with `superpowers:executing-plans`
- Checkpoint reviews after each task

**Which approach?**
