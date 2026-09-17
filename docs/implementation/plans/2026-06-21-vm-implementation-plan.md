# VM Dispatcher Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Implement a stack-based bytecode virtual machine for the Dryad runtime with specialized INTRINSIC_SYSCALL opcode for zero-overhead C++ syscall invocation.

**Architecture:** Stack-based VM with switch-statement opcode dispatcher executing 100+ opcodes across 10 categories (stack, arithmetic, logical, bitwise, comparison, control flow, variables, functions, objects, classes). INTRINSIC_SYSCALL opcode calls C++ intrinsic functions by registry ID with proper argument marshaling and error handling.

**Tech Stack:** C++17, CMake, Google Test, POSIX syscalls via IntrinsicsRegistry

---

## Implementation Phases

### Phase 1: Foundation (INTRINSIC_SYSCALL Only)
- Create VM core class and opcode definitions
- Implement INTRINSIC_SYSCALL dispatcher only
- Create tests verifying zero-overhead syscall invocation
- **Estimated: 3 tasks, ~2 hours**

### Phase 2: Stack & Basic Opcodes
- Implement stack operations (PUSH, POP, DUP, SWAP)
- Implement arithmetic opcodes (ADD, SUB, MUL, DIV, MOD)
- Create comprehensive unit tests for each opcode
- **Estimated: 4 tasks, ~3 hours**

### Phase 3: Complex Opcodes
- Implement control flow, variables, functions opcodes
- Implement object/array operations
- Add integration tests
- **Estimated: 6 tasks, ~4 hours**

### Phase 4: Bytecode Compiler & Integration
- Create bytecode compiler (AST → bytecode)
- Integrate with existing interpreter
- End-to-end testing
- **Estimated: 4 tasks, ~3 hours**

**Total: 17 tasks, ~12 hours**

---

## Phase 1: Foundation

### Task 1.1: Create Opcode Definitions

**Files:**
- Create: `include/dryad/runtime/opcode.h`
- Create: `include/dryad/runtime/vm.h`
- Reference: Design document at `docs/plans/2026-06-21-vm-dispatcher-design.md`

**Step 1: Write opcode enum header**

Create `include/dryad/runtime/opcode.h`:

```cpp
#ifndef DRYAD_RUNTIME_OPCODE_H
#define DRYAD_RUNTIME_OPCODE_H

#include <cstdint>

namespace dryad {

/**
 * Opcode definitions for Dryad bytecode VM
 * Total: 100+ opcodes across 10 categories
 */
enum class Opcode : uint8_t {
    // Stack operations (6 opcodes)
    PUSH_NULL = 0x00,
    PUSH_TRUE = 0x01,
    PUSH_FALSE = 0x02,
    PUSH_INT = 0x03,
    PUSH_FLOAT = 0x04,
    PUSH_STRING = 0x05,
    POP = 0x06,
    DUP = 0x07,
    SWAP = 0x08,
    
    // Arithmetic operations (5 opcodes)
    ADD = 0x10,
    SUB = 0x11,
    MUL = 0x12,
    DIV = 0x13,
    MOD = 0x14,
    
    // Logical operations (3 opcodes)
    AND = 0x20,
    OR = 0x21,
    NOT = 0x22,
    
    // Bitwise operations (6 opcodes)
    BIT_AND = 0x30,
    BIT_OR = 0x31,
    BIT_XOR = 0x32,
    BIT_NOT = 0x33,
    SHIFT_LEFT = 0x34,
    SHIFT_RIGHT = 0x35,
    
    // Comparison operations (6 opcodes)
    EQ = 0x40,
    NEQ = 0x41,
    LT = 0x42,
    LTE = 0x43,
    GT = 0x44,
    GTE = 0x45,
    
    // Control flow (8 opcodes)
    JUMP = 0x50,
    JUMP_IF_FALSE = 0x51,
    JUMP_IF_TRUE = 0x52,
    CALL = 0x53,
    RET = 0x54,
    RET_VALUE = 0x55,
    BREAK = 0x56,
    CONTINUE = 0x57,
    
    // Variable operations (4 opcodes)
    LOAD_VAR = 0x60,
    STORE_VAR = 0x61,
    LOAD_GLOBAL = 0x62,
    STORE_GLOBAL = 0x63,
    
    // Function operations (3 opcodes)
    MAKE_FUNCTION = 0x70,
    CALL_FUNCTION = 0x71,
    MAKE_CLOSURE = 0x72,
    
    // Object/Array operations (7 opcodes)
    NEW_OBJECT = 0x80,
    OBJ_SET = 0x81,
    OBJ_GET = 0x82,
    NEW_ARRAY = 0x83,
    ARRAY_PUSH = 0x84,
    ARRAY_GET = 0x85,
    ARRAY_SET = 0x86,
    
    // Class operations (5 opcodes)
    NEW_CLASS = 0x90,
    NEW_INSTANCE = 0x91,
    METHOD_CALL = 0x92,
    GET_PROPERTY = 0x93,
    SET_PROPERTY = 0x94,
    
    // INTRINSIC_SYSCALL - THE CRITICAL ONE
    INTRINSIC_SYSCALL = 0xFF,
};

} // namespace dryad

#endif // DRYAD_RUNTIME_OPCODE_H
```

**Step 2: Write VM header**

Create `include/dryad/runtime/vm.h`:

```cpp
#ifndef DRYAD_RUNTIME_VM_H
#define DRYAD_RUNTIME_VM_H

#include "dryad/runtime/value.h"
#include "dryad/runtime/opcode.h"
#include <vector>
#include <memory>
#include <cstdint>

namespace dryad {

class Environment;

struct CallFrame {
    size_t return_pc;                       // Return address in bytecode
    std::shared_ptr<Environment> env;       // Local environment
    size_t stack_base;                      // Stack position when called
};

class VM {
public:
    VM();
    ~VM();
    
    // Load and execute bytecode
    void load_bytecode(const std::vector<uint8_t>& bytecode);
    void execute();
    
    // Stack access (for testing)
    Value get_stack_top() const;
    void set_stack_top(const Value& v);
    size_t stack_size() const { return stack_.size(); }
    void clear_stack() { stack_.clear(); }
    
    // Bytecode info
    size_t bytecode_size() const { return bytecode_.size(); }
    size_t program_counter() const { return pc_; }
    
private:
    std::vector<Value> stack_;              // Main value stack
    std::vector<uint8_t> bytecode_;         // Compiled bytecode
    size_t pc_;                             // Program counter
    std::vector<CallFrame> frames_;         // Call stack
    std::shared_ptr<Environment> env_;      // Global environment
    
    // Dispatcher
    void execute_impl();
    void dispatch_opcode(Opcode op);
    void handle_intrinsic_syscall();
    
    // Helpers
    void validate_stack(size_t required, const std::string& context);
    inline int64_t read_int64_at(size_t offset);
    inline double read_float64_at(size_t offset);
};

} // namespace dryad

#endif // DRYAD_RUNTIME_VM_H
```

**Step 3: Create initial VM implementation stub**

Create `src/runtime/vm.cpp` (stub):

```cpp
#include "dryad/runtime/vm.h"
#include "dryad/runtime/environment.h"
#include "dryad/common/utils.h"

namespace dryad {

VM::VM() : pc_(0) {
    env_ = std::make_shared<Environment>();
}

VM::~VM() = default;

void VM::load_bytecode(const std::vector<uint8_t>& bytecode) {
    bytecode_ = bytecode;
    pc_ = 0;
    stack_.clear();
}

void VM::execute() {
    execute_impl();
}

void VM::execute_impl() {
    while (pc_ < bytecode_.size()) {
        uint8_t opcode_byte = bytecode_[pc_];
        Opcode op = static_cast<Opcode>(opcode_byte);
        dispatch_opcode(op);
    }
}

void VM::dispatch_opcode(Opcode op) {
    switch (op) {
        case Opcode::INTRINSIC_SYSCALL:
            handle_intrinsic_syscall();
            break;
        
        default:
            throw DryadException("Unimplemented opcode: " + 
                std::to_string(static_cast<int>(op)));
    }
}

void VM::handle_intrinsic_syscall() {
    // Will implement in Task 1.2
}

Value VM::get_stack_top() const {
    if (stack_.empty()) {
        throw DryadException("Stack is empty");
    }
    return stack_.back();
}

void VM::set_stack_top(const Value& v) {
    if (stack_.empty()) {
        stack_.push_back(v);
    } else {
        stack_.back() = v;
    }
}

void VM::validate_stack(size_t required, const std::string& context) {
    if (stack_.size() < required) {
        throw DryadException("Stack underflow in " + context + 
            ": need " + std::to_string(required) + 
            ", have " + std::to_string(stack_.size()));
    }
}

inline int64_t VM::read_int64_at(size_t offset) {
    if (offset + 8 > bytecode_.size()) {
        throw DryadException("Bytecode bounds violation reading i64");
    }
    int64_t value = 0;
    for (int i = 0; i < 8; i++) {
        value |= (static_cast<int64_t>(bytecode_[offset + i]) << (i * 8));
    }
    return value;
}

inline double VM::read_float64_at(size_t offset) {
    int64_t bits = read_int64_at(offset);
    return *reinterpret_cast<double*>(&bits);
}

} // namespace dryad
```

**Step 4: Update CMakeLists.txt**

Modify `dryad-cpp/CMakeLists.txt` to add the new files:

Find the section with runtime source files and add:
```cmake
# Add to RUNTIME_SOURCES
"src/runtime/vm.cpp"
```

Find the section with include directories (should already have `include/dryad/runtime`) and verify it's listed.

**Step 5: Commit**

```bash
cd /home/pedro/repo/source/dryad-cpp
git add include/dryad/runtime/opcode.h
git add include/dryad/runtime/vm.h
git add src/runtime/vm.cpp
git commit -m "feat(vm): add opcode definitions and VM stub"
```

---

### Task 1.2: Update IntrinsicsRegistry for VM

**Files:**
- Modify: `include/dryad/runtime/intrinsics_registry.h`
- Modify: `src/runtime/intrinsics_registry.cpp`
- Reference: `src/runtime/vm.cpp` (will use new methods)

**Step 1: Add VM-specific methods to registry header**

Modify `include/dryad/runtime/intrinsics_registry.h`:

Find the `IntrinsicsRegistry` class definition and add:

```cpp
    // New: Call intrinsic by ID (for VM dispatcher)
    Value call_by_id(uint16_t id, const std::vector<Value>& args);
    
    // New: Get ID for a registered intrinsic name
    uint16_t get_id(const std::string& name) const;
    
    // New: List all intrinsic IDs with names (for debugging)
    std::vector<std::pair<uint16_t, std::string>> list_all_intrinsics() const;

private:
    // New: Mapping for VM-based lookups
    std::unordered_map<uint16_t, IntrinsicFunction> functions_by_id_;
    std::unordered_map<std::string, uint16_t> name_to_id_;
    uint16_t next_id_ = 0;  // Auto-incrementing ID assignment
```

**Step 2: Implement new methods in registry**

Modify `src/runtime/intrinsics_registry.cpp`:

Add these new method implementations at the end of the `IntrinsicsRegistry` class:

```cpp
Value IntrinsicsRegistry::call_by_id(uint16_t id, const std::vector<Value>& args) {
    auto it = functions_by_id_.find(id);
    if (it == functions_by_id_.end()) {
        throw DryadException("Unknown intrinsic ID: " + std::to_string(id));
    }
    return it->second(args);
}

uint16_t IntrinsicsRegistry::get_id(const std::string& name) const {
    auto it = name_to_id_.find(name);
    if (it == name_to_id_.end()) {
        throw DryadException("Intrinsic not registered: " + name);
    }
    return it->second;
}

std::vector<std::pair<uint16_t, std::string>> IntrinsicsRegistry::list_all_intrinsics() const {
    std::vector<std::pair<uint16_t, std::string>> result;
    for (const auto& pair : name_to_id_) {
        result.push_back({pair.second, pair.first});
    }
    // Sort by ID
    std::sort(result.begin(), result.end());
    return result;
}
```

**Step 3: Update register_intrinsic to assign IDs**

Modify `src/runtime/intrinsics_registry.cpp`:

Replace the `register_intrinsic` method:

```cpp
void IntrinsicsRegistry::register_intrinsic(const std::string& name, IntrinsicFunction func) {
    intrinsics_[name] = func;
    
    // Also register for VM dispatcher (by ID)
    functions_by_id_[next_id_] = func;
    name_to_id_[name] = next_id_;
    next_id_++;
}
```

**Step 4: Verify existing code still compiles**

Run CMake and build:

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | head -50
```

Expected: No compilation errors (may have linking issues if VM not fully integrated yet)

**Step 5: Commit**

```bash
git add include/dryad/runtime/intrinsics_registry.h
git add src/runtime/intrinsics_registry.cpp
git commit -m "feat(runtime): add ID-based intrinsic lookup for VM"
```

---

### Task 1.3: Implement INTRINSIC_SYSCALL Dispatcher

**Files:**
- Modify: `src/runtime/vm.cpp` (implement `handle_intrinsic_syscall`)
- Reference: `include/dryad/runtime/intrinsics_registry.h`

**Step 1: Implement INTRINSIC_SYSCALL handler**

Modify `src/runtime/vm.cpp`:

Replace the `handle_intrinsic_syscall()` stub with:

```cpp
void VM::handle_intrinsic_syscall() {
    // Bytecode format:
    // [1 byte: opcode = 0xFF]
    // [2 bytes: u16 intrinsic_id (big-endian)]
    // [1 byte: u8 argc]
    
    if (pc_ + 4 > bytecode_.size()) {
        throw DryadException("Bytecode bounds violation in INTRINSIC_SYSCALL");
    }
    
    // Read intrinsic ID (big-endian u16)
    uint16_t intrinsic_id = 
        (static_cast<uint16_t>(bytecode_[pc_ + 1]) << 8) |
        (static_cast<uint16_t>(bytecode_[pc_ + 2]));
    
    // Read argument count
    uint8_t argc = bytecode_[pc_ + 3];
    
    // Extract arguments from stack (in reverse order)
    if (stack_.size() < argc) {
        validate_stack(argc, "INTRINSIC_SYSCALL argument extraction");
    }
    
    std::vector<Value> args(argc);
    for (int i = argc - 1; i >= 0; --i) {
        args[i] = stack_.back();
        stack_.pop_back();
    }
    
    // Call intrinsic via registry
    try {
        Value result = IntrinsicsRegistry::instance().call_by_id(
            intrinsic_id, args);
        
        // Push result onto stack
        stack_.push_back(result);
        
    } catch (const DryadException& e) {
        throw DryadException("Intrinsic syscall error (ID=" + 
            std::to_string(intrinsic_id) + "): " + e.what());
    }
    
    // Advance program counter past opcode + args
    pc_ += 4;
}
```

**Step 2: Verify compilation**

Build and check for errors:

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | head -50
```

Expected: Should compile successfully

**Step 3: Commit**

```bash
git add src/runtime/vm.cpp
git commit -m "feat(vm): implement INTRINSIC_SYSCALL dispatcher"
```

---

### Task 1.4: Write INTRINSIC_SYSCALL Tests

**Files:**
- Create: `tests/unit/vm_intrinsic_syscall_test.cpp`
- Reference: Existing test files in `tests/unit/`

**Step 1: Create test file**

Create `tests/unit/vm_intrinsic_syscall_test.cpp`:

```cpp
#include <gtest/gtest.h>
#include "dryad/runtime/vm.h"
#include "dryad/runtime/intrinsics_registry.h"
#include "dryad/common/utils.h"

namespace dryad {

class VMIntrinsicSyscallTest : public ::testing::Test {
protected:
    VM vm;
    IntrinsicsRegistry& registry = IntrinsicsRegistry::instance();
    
    void SetUp() override {
        vm.clear_stack();
    }
    
    // Helper: Build bytecode for INTRINSIC_SYSCALL(id, argc)
    std::vector<uint8_t> make_intrinsic_bytecode(
        uint16_t intrinsic_id, uint8_t argc) {
        std::vector<uint8_t> bc;
        bc.push_back(static_cast<uint8_t>(Opcode::INTRINSIC_SYSCALL));
        bc.push_back(static_cast<uint8_t>(intrinsic_id >> 8));      // High byte
        bc.push_back(static_cast<uint8_t>(intrinsic_id & 0xFF));    // Low byte
        bc.push_back(argc);
        return bc;
    }
};

// Test 1: INTRINSIC_SYSCALL with zero arguments
TEST_F(VMIntrinsicSyscallTest, ZeroArguments) {
    // Test with syscall.time (0 arguments)
    uint16_t time_id = registry.get_id("syscall.time");
    
    auto bc = make_intrinsic_bytecode(time_id, 0);
    vm.load_bytecode(bc);
    
    // Execute
    vm.execute();
    
    // Should have one value on stack (the time result)
    ASSERT_EQ(vm.stack_size(), 1);
    Value result = vm.get_stack_top();
    ASSERT_TRUE(result.is_integer());
    ASSERT_GT(result.as_integer(), 0);
}

// Test 2: INTRINSIC_SYSCALL with arguments
TEST_F(VMIntrinsicSyscallTest, WithArguments) {
    // Test syscall.open with path and flags
    // This requires PUSH_INT and PUSH_STRING opcodes (to be implemented in Phase 2)
    // For now, manually push values
    
    vm.stack_.push_back(Value(std::string("/tmp/test.txt")));
    vm.stack_.push_back(Value(static_cast<int64_t>(0)));  // O_RDONLY
    
    uint16_t open_id = registry.get_id("syscall.open");
    auto bc = make_intrinsic_bytecode(open_id, 2);
    vm.load_bytecode(bc);
    
    // Execute
    vm.execute();
    
    // Should have fd or error on stack
    ASSERT_EQ(vm.stack_size(), 1);
    Value result = vm.get_stack_top();
    ASSERT_TRUE(result.is_integer());
}

// Test 3: Stack underflow error handling
TEST_F(VMIntrinsicSyscallTest, StackUnderflow) {
    // Try to call with 2 arguments but stack is empty
    uint16_t read_id = registry.get_id("syscall.read");
    auto bc = make_intrinsic_bytecode(read_id, 2);
    vm.load_bytecode(bc);
    
    // Execute should throw
    EXPECT_THROW(vm.execute(), DryadException);
}

// Test 4: Unknown intrinsic ID
TEST_F(VMIntrinsicSyscallTest, UnknownIntrinsicID) {
    // Use invalid ID 9999
    auto bc = make_intrinsic_bytecode(9999, 0);
    vm.load_bytecode(bc);
    
    // Execute should throw
    EXPECT_THROW(vm.execute(), DryadException);
}

// Test 5: Multiple INTRINSIC_SYSCALL in sequence
TEST_F(VMIntrinsicSyscallTest, SequentialCalls) {
    // Call syscall.time twice
    uint16_t time_id = registry.get_id("syscall.time");
    
    std::vector<uint8_t> bc;
    // First call
    auto call1 = make_intrinsic_bytecode(time_id, 0);
    bc.insert(bc.end(), call1.begin(), call1.end());
    // Second call
    auto call2 = make_intrinsic_bytecode(time_id, 0);
    bc.insert(bc.end(), call2.begin(), call2.end());
    
    vm.load_bytecode(bc);
    vm.execute();
    
    // Should have two results on stack
    ASSERT_EQ(vm.stack_size(), 2);
}

} // namespace dryad
```

**Step 2: Add test to CMakeLists.txt**

Modify `dryad-cpp/CMakeLists.txt`:

Find the section with test files and add:

```cmake
add_executable(vm_intrinsic_syscall_test tests/unit/vm_intrinsic_syscall_test.cpp)
target_link_libraries(vm_intrinsic_syscall_test dryad gtest gtest_main)
add_test(NAME VMIntrinsicSyscallTest COMMAND vm_intrinsic_syscall_test)
```

**Step 3: Run tests**

Build and run tests:

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | tail -30
ctest --output-on-failure -R VMIntrinsicSyscallTest 2>&1
```

Expected: Tests should pass (except Stack underflow and Unknown ID tests which should throw as expected)

**Step 4: Commit**

```bash
git add tests/unit/vm_intrinsic_syscall_test.cpp
git add CMakeLists.txt
git commit -m "test(vm): add comprehensive INTRINSIC_SYSCALL tests"
```

---

### Task 1.5: Benchmark INTRINSIC_SYSCALL Performance

**Files:**
- Create: `benchmarks/vm_intrinsic_benchmark.cpp`

**Step 1: Create benchmark**

Create `benchmarks/vm_intrinsic_benchmark.cpp`:

```cpp
#include <gtest/gtest.h>
#include "dryad/runtime/vm.h"
#include "dryad/runtime/intrinsics_registry.h"
#include <chrono>
#include <iostream>

namespace dryad {

class VMIntrinsicBenchmark : public ::testing::Test {
protected:
    VM vm;
    IntrinsicsRegistry& registry = IntrinsicsRegistry::instance();
};

// Benchmark: 1000 INTRINSIC_SYSCALL executions
TEST_F(VMIntrinsicBenchmark, INTRINSIC_SYSCALL_Throughput) {
    uint16_t time_id = registry.get_id("syscall.time");
    
    // Create bytecode with 1000 INTRINSIC_SYSCALL(time, 0) calls
    std::vector<uint8_t> bc;
    for (int i = 0; i < 1000; i++) {
        bc.push_back(static_cast<uint8_t>(Opcode::INTRINSIC_SYSCALL));
        bc.push_back(static_cast<uint8_t>(time_id >> 8));
        bc.push_back(static_cast<uint8_t>(time_id & 0xFF));
        bc.push_back(0);  // argc = 0
    }
    
    vm.load_bytecode(bc);
    
    // Benchmark
    auto start = std::chrono::high_resolution_clock::now();
    vm.execute();
    auto end = std::chrono::high_resolution_clock::now();
    
    auto duration_us = std::chrono::duration_cast<std::chrono::microseconds>(
        end - start).count();
    
    double avg_ns_per_call = (duration_us * 1000.0) / 1000.0;
    
    std::cout << "\nIntrinsic syscall benchmark:\n";
    std::cout << "  Total time: " << duration_us << " us\n";
    std::cout << "  Average per call: " << avg_ns_per_call << " ns\n";
    std::cout << "  Throughput: " << (1000.0 / (duration_us / 1e6)) << " calls/sec\n";
    
    // Stack should have 1000 results
    ASSERT_EQ(vm.stack_size(), 1000);
}

} // namespace dryad
```

**Step 2: Add benchmark to CMakeLists.txt**

Modify `dryad-cpp/CMakeLists.txt`:

Add to benchmarks section:

```cmake
add_executable(vm_intrinsic_benchmark benchmarks/vm_intrinsic_benchmark.cpp)
target_link_libraries(vm_intrinsic_benchmark dryad gtest gtest_main)
```

**Step 3: Run benchmark**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | tail -10
./vm_intrinsic_benchmark --gtest_filter="*Throughput*"
```

Expected: Output showing throughput (~50-100 ns per opcode dispatch)

**Step 4: Commit**

```bash
git add benchmarks/vm_intrinsic_benchmark.cpp
git add CMakeLists.txt
git commit -m "perf(vm): add INTRINSIC_SYSCALL throughput benchmark"
```

---

## Phase 2: Stack & Basic Opcodes

### Task 2.1: Implement Stack Operations (PUSH, POP, DUP, SWAP)

**Files:**
- Modify: `src/runtime/vm.cpp`
- Modify: `include/dryad/runtime/vm.h` (add helpers)

**Step 1: Add stack operation opcodes to dispatcher**

Modify `src/runtime/vm.cpp`, in `dispatch_opcode()` switch statement:

```cpp
        case Opcode::PUSH_NULL:
            stack_.push_back(Value());
            pc_++;
            break;
        
        case Opcode::PUSH_TRUE:
            stack_.push_back(Value(true));
            pc_++;
            break;
        
        case Opcode::PUSH_FALSE:
            stack_.push_back(Value(false));
            pc_++;
            break;
        
        case Opcode::PUSH_INT: {
            if (pc_ + 9 > bytecode_.size()) {
                throw DryadException("Bytecode bounds violation reading PUSH_INT");
            }
            int64_t value = read_int64_at(pc_ + 1);
            stack_.push_back(Value(value));
            pc_ += 9;
            break;
        }
        
        case Opcode::PUSH_FLOAT: {
            if (pc_ + 9 > bytecode_.size()) {
                throw DryadException("Bytecode bounds violation reading PUSH_FLOAT");
            }
            double value = read_float64_at(pc_ + 1);
            stack_.push_back(Value(value));
            pc_ += 9;
            break;
        }
        
        case Opcode::POP:
            validate_stack(1, "POP");
            stack_.pop_back();
            pc_++;
            break;
        
        case Opcode::DUP:
            validate_stack(1, "DUP");
            stack_.push_back(stack_.back());
            pc_++;
            break;
        
        case Opcode::SWAP:
            validate_stack(2, "SWAP");
            std::swap(stack_[stack_.size() - 1], stack_[stack_.size() - 2]);
            pc_++;
            break;
```

**Step 2: Write tests**

Create `tests/unit/vm_stack_operations_test.cpp`:

```cpp
#include <gtest/gtest.h>
#include "dryad/runtime/vm.h"

namespace dryad {

class VMStackOperationsTest : public ::testing::Test {
protected:
    VM vm;
    
    void SetUp() override {
        vm.clear_stack();
    }
    
    // Helper: write int64 to bytecode
    void write_int64(std::vector<uint8_t>& bc, int64_t value) {
        for (int i = 0; i < 8; i++) {
            bc.push_back(static_cast<uint8_t>((value >> (i * 8)) & 0xFF));
        }
    }
};

TEST_F(VMStackOperationsTest, PUSH_NULL) {
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::PUSH_NULL));
    vm.load_bytecode(bc);
    vm.execute();
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_TRUE(vm.get_stack_top().is_null());
}

TEST_F(VMStackOperationsTest, PUSH_TRUE) {
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::PUSH_TRUE));
    vm.load_bytecode(bc);
    vm.execute();
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_TRUE(vm.get_stack_top().as_boolean());
}

TEST_F(VMStackOperationsTest, PUSH_FALSE) {
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::PUSH_FALSE));
    vm.load_bytecode(bc);
    vm.execute();
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_FALSE(vm.get_stack_top().as_boolean());
}

TEST_F(VMStackOperationsTest, PUSH_INT) {
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::PUSH_INT));
    write_int64(bc, 42);
    vm.load_bytecode(bc);
    vm.execute();
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST_F(VMStackOperationsTest, PUSH_FLOAT) {
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::PUSH_FLOAT));
    double value = 3.14;
    int64_t bits = *reinterpret_cast<int64_t*>(&value);
    write_int64(bc, bits);
    vm.load_bytecode(bc);
    vm.execute();
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_DOUBLE_EQ(vm.get_stack_top().as_float(), 3.14);
}

TEST_F(VMStackOperationsTest, POP) {
    vm.stack_.push_back(Value(42));
    vm.stack_.push_back(Value(99));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::POP));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST_F(VMStackOperationsTest, DUP) {
    vm.stack_.push_back(Value(42));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::DUP));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 2);
}

TEST_F(VMStackOperationsTest, SWAP) {
    vm.stack_.push_back(Value(42));
    vm.stack_.push_back(Value(99));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::SWAP));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 2);
    ASSERT_EQ(vm.stack_[0].as_integer(), 99);
    ASSERT_EQ(vm.stack_[1].as_integer(), 42);
}

} // namespace dryad
```

**Step 3: Add test to CMakeLists.txt**

```cmake
add_executable(vm_stack_operations_test tests/unit/vm_stack_operations_test.cpp)
target_link_libraries(vm_stack_operations_test dryad gtest gtest_main)
add_test(NAME VMStackOperationsTest COMMAND vm_stack_operations_test)
```

**Step 4: Run tests**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | tail -20
ctest --output-on-failure -R VMStackOperationsTest 2>&1
```

Expected: All tests pass

**Step 5: Commit**

```bash
git add src/runtime/vm.cpp
git add tests/unit/vm_stack_operations_test.cpp
git add CMakeLists.txt
git commit -m "feat(vm): implement stack operations (PUSH, POP, DUP, SWAP)"
```

---

### Task 2.2: Implement Arithmetic Operations

**Files:**
- Modify: `src/runtime/vm.cpp`
- Create tests

**Step 1: Add arithmetic opcodes**

Modify `src/runtime/vm.cpp`, add to `dispatch_opcode()`:

```cpp
        case Opcode::ADD: {
            validate_stack(2, "ADD");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (left.is_integer() && right.is_integer()) {
                stack_.push_back(Value(left.as_integer() + right.as_integer()));
            } else if (left.is_number() && right.is_number()) {
                stack_.push_back(Value(left.as_float() + right.as_float()));
            } else if (left.is_string() && right.is_string()) {
                stack_.push_back(Value(left.as_string() + right.as_string()));
            } else {
                throw DryadException("Type error: invalid operands for +");
            }
            pc_++;
            break;
        }
        
        case Opcode::SUB: {
            validate_stack(2, "SUB");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for -");
            }
            
            if (left.is_integer() && right.is_integer()) {
                stack_.push_back(Value(left.as_integer() - right.as_integer()));
            } else {
                stack_.push_back(Value(left.as_float() - right.as_float()));
            }
            pc_++;
            break;
        }
        
        case Opcode::MUL: {
            validate_stack(2, "MUL");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for *");
            }
            
            if (left.is_integer() && right.is_integer()) {
                stack_.push_back(Value(left.as_integer() * right.as_integer()));
            } else {
                stack_.push_back(Value(left.as_float() * right.as_float()));
            }
            pc_++;
            break;
        }
        
        case Opcode::DIV: {
            validate_stack(2, "DIV");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for /");
            }
            
            if (left.is_integer() && right.is_integer()) {
                if (right.as_integer() == 0) {
                    throw DryadException("Division by zero");
                }
                stack_.push_back(Value(left.as_integer() / right.as_integer()));
            } else {
                double r = right.as_float();
                if (r == 0.0) {
                    throw DryadException("Division by zero");
                }
                stack_.push_back(Value(left.as_float() / r));
            }
            pc_++;
            break;
        }
        
        case Opcode::MOD: {
            validate_stack(2, "MOD");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_integer() || !right.is_integer()) {
                throw DryadException("Type error: operands must be integers for %");
            }
            if (right.as_integer() == 0) {
                throw DryadException("Division by zero in modulo");
            }
            stack_.push_back(Value(left.as_integer() % right.as_integer()));
            pc_++;
            break;
        }
```

**Step 2: Write comprehensive tests**

Create `tests/unit/vm_arithmetic_test.cpp`:

```cpp
#include <gtest/gtest.h>
#include "dryad/runtime/vm.h"

namespace dryad {

class VMArithmeticTest : public ::testing::Test {
protected:
    VM vm;
    
    void SetUp() override {
        vm.clear_stack();
    }
    
    void write_int64(std::vector<uint8_t>& bc, int64_t value) {
        for (int i = 0; i < 8; i++) {
            bc.push_back(static_cast<uint8_t>((value >> (i * 8)) & 0xFF));
        }
    }
};

TEST_F(VMArithmeticTest, IntegerAddition) {
    vm.stack_.push_back(Value(10));
    vm.stack_.push_back(Value(5));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::ADD));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 15);
}

TEST_F(VMArithmeticTest, FloatAddition) {
    vm.stack_.push_back(Value(10.5));
    vm.stack_.push_back(Value(3.2));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::ADD));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_DOUBLE_EQ(vm.get_stack_top().as_float(), 13.7);
}

TEST_F(VMArithmeticTest, StringConcatenation) {
    vm.stack_.push_back(Value("Hello"));
    vm.stack_.push_back(Value(" World"));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::ADD));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_string(), "Hello World");
}

TEST_F(VMArithmeticTest, IntegerSubtraction) {
    vm.stack_.push_back(Value(10));
    vm.stack_.push_back(Value(3));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::SUB));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 7);
}

TEST_F(VMArithmeticTest, IntegerMultiplication) {
    vm.stack_.push_back(Value(6));
    vm.stack_.push_back(Value(7));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::MUL));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 42);
}

TEST_F(VMArithmeticTest, IntegerDivision) {
    vm.stack_.push_back(Value(20));
    vm.stack_.push_back(Value(4));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::DIV));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 5);
}

TEST_F(VMArithmeticTest, IntegerModulo) {
    vm.stack_.push_back(Value(10));
    vm.stack_.push_back(Value(3));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::MOD));
    vm.load_bytecode(bc);
    vm.execute();
    
    ASSERT_EQ(vm.stack_size(), 1);
    ASSERT_EQ(vm.get_stack_top().as_integer(), 1);
}

TEST_F(VMArithmeticTest, DivisionByZero) {
    vm.stack_.push_back(Value(10));
    vm.stack_.push_back(Value(0));
    
    std::vector<uint8_t> bc;
    bc.push_back(static_cast<uint8_t>(Opcode::DIV));
    vm.load_bytecode(bc);
    
    EXPECT_THROW(vm.execute(), DryadException);
}

} // namespace dryad
```

**Step 3: Update CMakeLists.txt**

```cmake
add_executable(vm_arithmetic_test tests/unit/vm_arithmetic_test.cpp)
target_link_libraries(vm_arithmetic_test dryad gtest gtest_main)
add_test(NAME VMArithmeticTest COMMAND vm_arithmetic_test)
```

**Step 4: Run tests**

```bash
cd /home/pedro/repo/source/dryad-cpp/build
cmake --build . 2>&1 | tail -20
ctest --output-on-failure -R VMArithmeticTest 2>&1
```

**Step 5: Commit**

```bash
git add src/runtime/vm.cpp
git add tests/unit/vm_arithmetic_test.cpp
git add CMakeLists.txt
git commit -m "feat(vm): implement arithmetic operations (ADD, SUB, MUL, DIV, MOD)"
```

---

### Task 2.3: Implement Comparison Operations

Similar structure to Task 2.2 but for EQ, NEQ, LT, LTE, GT, GTE opcodes.

[Details follow same pattern: add to dispatch_opcode(), write tests, update CMakeLists.txt, commit]

**Step 1: Add comparison opcodes to dispatcher**

```cpp
        case Opcode::EQ: {
            validate_stack(2, "EQ");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (left.type() != right.type()) {
                stack_.push_back(Value(false));
            } else if (left.is_null()) {
                stack_.push_back(Value(true));
            } else if (left.is_boolean()) {
                stack_.push_back(Value(left.as_boolean() == right.as_boolean()));
            } else if (left.is_integer()) {
                stack_.push_back(Value(left.as_integer() == right.as_integer()));
            } else if (left.is_float()) {
                stack_.push_back(Value(left.as_float() == right.as_float()));
            } else if (left.is_string()) {
                stack_.push_back(Value(left.as_string() == right.as_string()));
            } else {
                stack_.push_back(Value(false));
            }
            pc_++;
            break;
        }
        
        case Opcode::NEQ: {
            validate_stack(2, "NEQ");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (left.type() != right.type()) {
                stack_.push_back(Value(true));
            } else if (left.is_null()) {
                stack_.push_back(Value(false));
            } else if (left.is_boolean()) {
                stack_.push_back(Value(left.as_boolean() != right.as_boolean()));
            } else if (left.is_integer()) {
                stack_.push_back(Value(left.as_integer() != right.as_integer()));
            } else if (left.is_float()) {
                stack_.push_back(Value(left.as_float() != right.as_float()));
            } else if (left.is_string()) {
                stack_.push_back(Value(left.as_string() != right.as_string()));
            } else {
                stack_.push_back(Value(true));
            }
            pc_++;
            break;
        }
        
        case Opcode::LT: {
            validate_stack(2, "LT");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for <");
            }
            
            bool result = (left.is_integer() && right.is_integer()) ?
                (left.as_integer() < right.as_integer()) :
                (left.as_float() < right.as_float());
            
            stack_.push_back(Value(result));
            pc_++;
            break;
        }
        
        case Opcode::LTE: {
            validate_stack(2, "LTE");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for <=");
            }
            
            bool result = (left.is_integer() && right.is_integer()) ?
                (left.as_integer() <= right.as_integer()) :
                (left.as_float() <= right.as_float());
            
            stack_.push_back(Value(result));
            pc_++;
            break;
        }
        
        case Opcode::GT: {
            validate_stack(2, "GT");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for >");
            }
            
            bool result = (left.is_integer() && right.is_integer()) ?
                (left.as_integer() > right.as_integer()) :
                (left.as_float() > right.as_float());
            
            stack_.push_back(Value(result));
            pc_++;
            break;
        }
        
        case Opcode::GTE: {
            validate_stack(2, "GTE");
            Value right = stack_.back(); stack_.pop_back();
            Value left = stack_.back(); stack_.pop_back();
            
            if (!left.is_number() || !right.is_number()) {
                throw DryadException("Type error: operands must be numeric for >=");
            }
            
            bool result = (left.is_integer() && right.is_integer()) ?
                (left.as_integer() >= right.as_integer()) :
                (left.as_float() >= right.as_float());
            
            stack_.push_back(Value(result));
            pc_++;
            break;
        }
```

**Step 2-5: Tests, CMakeLists, run, commit (following same pattern as Task 2.2)**

---

### Task 2.4: Implement Logical Operations

Add AND, OR, NOT opcodes (similar structure to above tasks).

---

## Phase 3 & 4 Tasks

(Abbreviated for brevity - follow same structure as Phase 2)

**Phase 3 Quick Summary:**
- Task 3.1: Bitwise operations (BIT_AND, BIT_OR, BIT_XOR, BIT_NOT, SHIFT_LEFT, SHIFT_RIGHT)
- Task 3.2: Variable operations (LOAD_VAR, STORE_VAR, LOAD_GLOBAL, STORE_GLOBAL)
- Task 3.3: Control flow (JUMP, JUMP_IF_FALSE, RET, RET_VALUE)
- Task 3.4: Object/Array operations
- Task 3.5: Class operations
- Task 3.6: Integration tests

**Phase 4 Quick Summary:**
- Task 4.1: Bytecode compiler (AST → bytecode)
- Task 4.2: Parser support for @intrinsic decorator
- Task 4.3: Full integration with interpreter
- Task 4.4: End-to-end tests and benchmarks

---

## Testing & Verification Checklist

Before declaring complete:

- [ ] All 100+ opcodes implemented and tested
- [ ] INTRINSIC_SYSCALL working with zero overhead
- [ ] Stack validation prevents all underflow/overflow
- [ ] Comprehensive test suite (>100 tests, all passing)
- [ ] Performance benchmark shows 3-5x speedup vs interpreter
- [ ] Zero regressions in existing tests
- [ ] All code reviewed and documented
- [ ] Design document updated with implementation notes

---

## Performance Targets

| Metric | Target | Measured |
|--------|--------|----------|
| Opcode dispatch overhead | <100 ns | TBD |
| INTRINSIC_SYSCALL overhead | <5% | TBD |
| Bytecode size vs AST | 15-30x compression | TBD |
| VM throughput | 3-5x faster than interpreter | TBD |

---

## Document History

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | 2026-06-21 | Initial implementation plan |

---

**End of Implementation Plan**

Use skill: superpowers:executing-plans to implement this plan task-by-task.
