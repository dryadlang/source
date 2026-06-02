# Dryad C++ Reimplementation — Rework Overview & Roadmap

**Document Type**: Implementation Strategy & Project Plan  
**Version**: 1.0  
**Date**: May 27, 2026  
**Status**: Active Development Plan  
**Target**: Clean C++ reimplementation with intrinsics-based runtime

---

## 🎯 Executive Summary

This document outlines the complete strategy for reimplementing Dryad from scratch in C++, transitioning from the archived Rust codebase to a modern, intrinsics-based architecture. The reimplementation adopts a **minimal runtime + self-hosting stdlib** approach, eliminating the binding maintenance burden that plagued the previous version.

**Key Strategic Shifts:**
- **Language**: Rust → C++ (better LLVM integration, familiar to systems programmers)
- **Runtime**: Monolithic native modules → Micro-kernel with ~50 syscall intrinsics
- **Stdlib**: C++/Rust bindings → 100% pure Dryad implementation
- **Module System**: `#<module>` directives → ES6-style `import/export`
- **Testing**: Real I/O → Pluggable backends (MemoryBackend for tests)

**Critical Success Factors:**
1. ✅ Clean slate — archived old codebase, no legacy debt
2. ✅ Formal specification — v2.0 theoretical foundation complete
3. ✅ Clear architecture — intrinsics system fully designed
4. ✅ Incremental approach — build in testable layers
5. ✅ TDD discipline — tests before implementation, always

---

## 📋 Table of Contents

1. [Analysis: What Went Wrong Before](#1-analysis-what-went-wrong-before)
2. [Architecture: The New Foundation](#2-architecture-the-new-foundation)
3. [Project Structure](#3-project-structure)
4. [Implementation Phases](#4-implementation-phases)
5. [Detailed Task Breakdown](#5-detailed-task-breakdown)
6. [Testing Strategy](#6-testing-strategy)
7. [Risk Mitigation](#7-risk-mitigation)
8. [Success Criteria](#8-success-criteria)
9. [Timeline & Milestones](#9-timeline--milestones)
10. [Appendix: Migration from Rust](#10-appendix-migration-from-rust)

---

## 1. Analysis: What Went Wrong Before

### 1.1 Problems with the Rust Implementation

| Problem | Impact | Root Cause |
|---------|--------|------------|
| **Binding Maintenance Hell** | Every stdlib function required C++/Rust wrapper + Dryad declaration | No separation between runtime and language features |
| **Testing Difficulty** | Tests required real filesystem/network access | Hard-coded dependency on OS syscalls |
| **Runtime Bloat** | ~500KB runtime with hundreds of native functions | Monolithic design without clear boundaries |
| **Extension Friction** | Adding new features meant writing C++ → Manual effort discouraged innovation | No self-hosting capability |
| **Inconsistent Patterns** | Some modules used different binding styles | Organic growth without architectural vision |
| **Debugging Pain** | Errors in native code obscure, mixed Rust/C++ stack traces | Too many abstraction layers |

### 1.2 What We're Keeping from the Old Codebase

**Keep (Conceptually):**
- ✅ Core language syntax (already well-designed)
- ✅ AST structure (sound foundation)
- ✅ Parser patterns (recursive descent works well)
- ✅ Error categorization system (good DX)
- ✅ Test suite structure (comprehensive coverage)

**Discard:**
- ❌ Entire native module system
- ❌ Rust runtime implementation
- ❌ Manual binding generation
- ❌ Legacy `#<module>` directive syntax
- ❌ Tightly coupled components

### 1.3 Lessons Learned

**Architectural Lessons:**
1. **Minimize Runtime Footprint**: Fewer syscalls = less maintenance
2. **Self-Hosting is Key**: Language features should be implementable in the language itself
3. **Clear Boundaries**: Runtime (C++) and stdlib (Dryad) must be strictly separated
4. **Testability First**: Design for mocking from day one
5. **Incremental Delivery**: Ship working features early, iterate fast

**Process Lessons:**
1. **Spec Before Code**: Formal specification prevents scope drift
2. **TDD Discipline**: Tests catch regressions in complex refactors
3. **Parallel Development**: Independent components can be built simultaneously
4. **Documentation as Code**: Keep specs and implementation synchronized
5. **Community Feedback**: Early feedback prevents costly rewrites

---

## 2. Architecture: The New Foundation

### 2.1 The Four-Layer Architecture

```
┌──────────────────────────────────────────────────────┐
│  Layer 4: APPLICATION CODE (User Dryad Programs)     │
├──────────────────────────────────────────────────────┤
│  Layer 3: STANDARD LIBRARY (100% Dryad)              │
│  • @std/io       • @std/http      • @std/crypto      │
│  • @std/net      • @std/async     • @std/json        │
│  • @std/buffer   • @std/encoding  • @std/time        │
├──────────────────────────────────────────────────────┤
│  Layer 2: INTRINSICS LAYER (~50 Syscall Primitives)  │
│  • File I/O (8)  • Network (8)    • Memory (5)       │
│  • Async I/O (6) • Process (6)    • Time (4)         │
│  • Env (5)       • Atomic Ops (5) • Signals (3)      │
├──────────────────────────────────────────────────────┤
│  Layer 1: C++ MICRO-KERNEL RUNTIME                   │
│  • VM Core       • GC/Memory Mgmt • Module Loader    │
│  • Bytecode      • JIT/AOT        • Error Handling   │
└──────────────────────────────────────────────────────┘
```

### 2.2 Dependency Rules (Strict Enforcement)

**Layer Interaction Rules:**
- Layer N can **only** depend on Layer N-1
- Layer 1 (C++) **cannot** depend on Layer 3 (Dryad stdlib)
- Layer 2 (Intrinsics) is the **only** communication channel between C++ and Dryad
- Layer 3 (Stdlib) **never** calls C++ directly (only via Layer 2 intrinsics)

**Forbidden:**
```cpp
// ❌ WRONG: Stdlib calling C++ directly
namespace dryad::stdlib {
    std::string readFile(const std::string& path) {
        std::ifstream file(path);  // Direct C++ I/O!
        ...
    }
}
```

**Correct:**
```cpp
// ✅ CORRECT: Runtime exposes syscall intrinsic
namespace dryad::runtime {
    Value intrinsic_sys_read(int fd, uint8_t* buf, size_t len) {
        ssize_t n = ::read(fd, buf, len);  // POSIX syscall
        return Value::from_int(n);
    }
}
```

```dryad
// ✅ CORRECT: Stdlib uses intrinsic
// @std/io.dryad
@intrinsic("syscall.read")
extern function __sys_read(fd: i32, buf: ptr<u8>, len: usize): isize;

export function readFile(path: string): string {
    let fd = __sys_open(path, O_RDONLY);  // Intrinsic call
    let buf = Buffer.allocate(4096);
    let n = __sys_read(fd, buf.ptr, 4096);  // Intrinsic call
    __sys_close(fd);  // Intrinsic call
    return buf.slice(0, n).toString();
}
```

### 2.3 Component Boundaries

```
dryad/
├── runtime/          → Layer 1: C++ Micro-Kernel
│   ├── core/        → VM, memory, execution
│   ├── intrinsics/  → Layer 2: Syscall bindings
│   ├── bytecode/    → Bytecode compiler/interpreter
│   └── gc/          → Garbage collector
│
├── compiler/         → Lexer, Parser, AST
│   ├── lexer/
│   ├── parser/
│   ├── ast/
│   └── codegen/     → Bytecode/LLVM IR generation
│
├── stdlib/           → Layer 3: 100% Dryad
│   ├── runtime/     → @std/runtime/intrinsics (declarations only)
│   ├── core/        → @std/buffer, @std/io, @std/net
│   ├── async/       → @std/async (event loop)
│   └── high_level/  → @std/http, @std/json, @std/crypto
│
└── tools/
    ├── dryad_cli/   → Main executable
    ├── dryad_repl/  → Interactive shell
    └── dryad_fmt/   → Code formatter
```

---

## 3. Project Structure

### 3.1 Directory Layout (Clean Start)

```
dryad-cpp/
├── CMakeLists.txt              # Root build configuration
├── README.md
├── LICENSE
│
├── include/                    # Public headers
│   └── dryad/
│       ├── runtime/
│       ├── compiler/
│       └── common/
│
├── src/                        # Implementation
│   ├── runtime/               # Layer 1: C++ Runtime
│   │   ├── core/
│   │   │   ├── vm.cpp
│   │   │   ├── value.cpp
│   │   │   ├── object.cpp
│   │   │   └── module_loader.cpp
│   │   ├── intrinsics/        # Layer 2: Syscalls
│   │   │   ├── intrinsics.cpp
│   │   │   ├── file_io.cpp
│   │   │   ├── network.cpp
│   │   │   ├── memory.cpp
│   │   │   └── async_io.cpp
│   │   ├── gc/
│   │   │   └── gc.cpp
│   │   └── bytecode/
│   │       ├── bytecode.cpp
│   │       └── opcodes.hpp
│   │
│   ├── compiler/              # Compiler Pipeline
│   │   ├── lexer/
│   │   │   ├── lexer.cpp
│   │   │   └── token.cpp
│   │   ├── parser/
│   │   │   ├── parser.cpp
│   │   │   └── ast.cpp
│   │   └── codegen/
│   │       ├── bytecode_gen.cpp
│   │       └── llvm_gen.cpp
│   │
│   ├── tools/                 # CLI Tools
│   │   ├── cli/
│   │   │   └── main.cpp
│   │   └── repl/
│   │       └── repl.cpp
│   │
│   └── common/                # Shared utilities
│       ├── error.cpp
│       └── utils.cpp
│
├── stdlib/                    # Layer 3: Dryad Stdlib
│   ├── runtime/
│   │   └── intrinsics.dryad   # Syscall declarations
│   ├── buffer.dryad
│   ├── io.dryad
│   ├── net.dryad
│   ├── async/
│   │   ├── event_loop.dryad
│   │   └── promise.dryad
│   ├── http.dryad
│   ├── json.dryad
│   └── vfs/
│       ├── vfs.dryad
│       ├── native_backend.dryad
│       └── memory_backend.dryad
│
├── tests/                     # Test Suite
│   ├── unit/                  # C++ unit tests (Google Test)
│   │   ├── runtime/
│   │   ├── compiler/
│   │   └── intrinsics/
│   ├── integration/           # End-to-end tests
│   └── stdlib/                # Stdlib tests (in Dryad)
│
├── benchmarks/                # Performance tests
├── docs/                      # Documentation
│   └── theory/                # Link to dryad_theory/
├── examples/                  # Example programs
└── scripts/                   # Build/CI scripts
```

### 3.2 Build System (CMake)

**Root CMakeLists.txt:**
```cmake
cmake_minimum_required(VERSION 3.20)
project(Dryad VERSION 2.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)

# Options
option(DRYAD_BUILD_TESTS "Build test suite" ON)
option(DRYAD_BUILD_BENCHMARKS "Build benchmarks" OFF)
option(DRYAD_ENABLE_JIT "Enable JIT compilation" OFF)
option(DRYAD_ENABLE_AOT "Enable AOT compilation (requires LLVM)" OFF)

# Dependencies
find_package(GTest REQUIRED)
if(DRYAD_ENABLE_AOT)
    find_package(LLVM REQUIRED CONFIG)
endif()

# Subdirectories
add_subdirectory(src/runtime)
add_subdirectory(src/compiler)
add_subdirectory(src/tools)

if(DRYAD_BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()

if(DRYAD_BUILD_BENCHMARKS)
    add_subdirectory(benchmarks)
endif()
```

**Incremental Build Targets:**
```bash
# Phase 1: Minimal runtime
cmake -B build -DDRYAD_ENABLE_JIT=OFF -DDRYAD_ENABLE_AOT=OFF
make -C build dryad_runtime

# Phase 2: Add compiler
make -C build dryad_compiler

# Phase 3: Full CLI
make -C build dryad_cli

# Phase 4: JIT (later)
cmake -B build -DDRYAD_ENABLE_JIT=ON
make -C build

# Phase 5: AOT (much later)
cmake -B build -DDRYAD_ENABLE_AOT=ON
make -C build
```

### 3.3 Naming Conventions

**C++ Code:**
```cpp
// Namespaces: lowercase, underscores
namespace dryad::runtime {}
namespace dryad::compiler::ast {}

// Classes: PascalCase
class VirtualMachine {};
class BytecodeCompiler {};

// Functions: camelCase
void executeInstruction();
Value evaluateExpression();

// Constants: UPPER_SNAKE_CASE
const int MAX_STACK_SIZE = 4096;

// Member variables: snake_case with trailing underscore
class Foo {
    int member_var_;
    std::string name_;
};
```

**Dryad Code:**
```dryad
// Functions: camelCase
function readFile(path: string): string {}

// Classes: PascalCase
class HttpClient {}

// Constants: UPPER_SNAKE_CASE
const MAX_RETRIES = 3;

// Private members: leading underscore
class Foo {
    private _internal: number;
    
    _privateMethod() {}
}
```

**Files:**
```
// Headers: .hpp
vm.hpp
bytecode.hpp

// Implementation: .cpp
vm.cpp
bytecode.cpp

// Dryad source: .dryad
io.dryad
http.dryad

// Tests: _test suffix
vm_test.cpp
parser_test.cpp
```

---

## 4. Implementation Phases

### Phase 0: Foundation (Week 1)
**Goal**: Project skeleton, build system, basic infrastructure

**Tasks:**
- [ ] Initialize git repository with clean structure
- [ ] Set up CMake build system
- [ ] Configure CI/CD (GitHub Actions)
- [ ] Set up Google Test framework
- [ ] Create basic error handling system
- [ ] Define `Value` type (tagged union)
- [ ] Implement basic memory allocation

**Deliverable**: Empty project compiles, tests run (even if 0 tests)

---

### Phase 1: Lexer & Parser (Week 2)
**Goal**: Complete tokenization and AST generation

**Tasks:**
- [ ] Implement lexer with all token types
- [ ] Write lexer unit tests (100% coverage)
- [ ] Implement recursive descent parser
- [ ] Generate AST nodes for all language constructs
- [ ] Write parser unit tests
- [ ] Implement error recovery in parser
- [ ] Add source location tracking

**Deliverable**: Can parse all valid Dryad syntax into AST

**Test Examples:**
```dryad
// Should parse successfully
let x = 42;
function add(a: number, b: number): number { return a + b; }
class Foo extends Bar { method() {} }
import { x } from "@std/io";
```

---

### Phase 2: Tree-Walking Interpreter (Week 3-4)
**Goal**: Execute simple programs via AST interpretation

**Tasks:**
- [ ] Implement `Value` type with all primitives
- [ ] Create scope/environment management
- [ ] Implement expression evaluator
- [ ] Implement statement executor
- [ ] Add variable declarations (let/const)
- [ ] Implement functions (declaration + calls)
- [ ] Add control flow (if/while/for)
- [ ] Implement basic operators
- [ ] Write interpreter integration tests

**Deliverable**: Can run "Hello World" and Fibonacci

**Test Program:**
```dryad
function fib(n: number): number {
    if (n <= 1) return n;
    return fib(n - 1) + fib(n - 2);
}

print(fib(10));  // Should output: 55
```

---

### Phase 3: Intrinsics Layer (Week 5)
**Goal**: Implement ~50 syscall intrinsics

**Tasks:**
- [ ] Define `SyscallID` enum (50 syscalls)
- [ ] Implement intrinsics dispatcher in VM
- [ ] Add `@intrinsic` decorator to parser
- [ ] Generate `INTRINSIC_SYSCALL` opcode
- [ ] Implement File I/O syscalls (8)
- [ ] Implement Network syscalls (8)
- [ ] Implement Memory syscalls (5)
- [ ] Implement Async I/O syscalls (6)
- [ ] Implement Process/Thread syscalls (6)
- [ ] Implement Time syscalls (4)
- [ ] Implement Environment syscalls (5)
- [ ] Implement Atomic Operation syscalls (5)
- [ ] Write intrinsics unit tests

**Deliverable**: All 50 syscalls functional and tested

**Test Example:**
```cpp
// C++ test
TEST(Intrinsics, FileIO) {
    VM vm;
    int fd = vm.execute_intrinsic(SyscallID::OPEN, 
                                   "test.txt", O_RDONLY);
    ASSERT_GT(fd, 0);
    
    uint8_t buf[100];
    ssize_t n = vm.execute_intrinsic(SyscallID::READ, 
                                      fd, buf, 100);
    ASSERT_GT(n, 0);
    
    vm.execute_intrinsic(SyscallID::CLOSE, fd);
}
```

---

### Phase 4: Core Stdlib in Dryad (Week 6-7)
**Goal**: Implement foundational stdlib modules in pure Dryad

**Tasks:**
- [ ] Create `@std/runtime/intrinsics.dryad` (declarations)
- [ ] Implement `@std/buffer.dryad` (Buffer class)
- [ ] Implement `@std/io.dryad` (File I/O)
- [ ] Implement `@std/net.dryad` (Sockets)
- [ ] Write stdlib unit tests (in Dryad)
- [ ] Implement VFS interface
- [ ] Implement NativeBackend (using intrinsics)
- [ ] Implement MemoryBackend (pure Dryad, no syscalls)

**Deliverable**: Can read/write files and create sockets from Dryad

**Test Program:**
```dryad
import { readFile, writeFile } from "@std/io";

writeFile("test.txt", "Hello, World!");
let content = readFile("test.txt");
assert(content == "Hello, World!");
```

---

### Phase 5: Async I/O & Event Loop (Week 8)
**Goal**: Non-blocking I/O with async/await

**Tasks:**
- [ ] Implement event loop in pure Dryad
- [ ] Add coroutine support to VM
- [ ] Implement `async`/`await` compiler transformation
- [ ] Add epoll/kqueue intrinsics
- [ ] Implement Promise class in Dryad
- [ ] Write async integration tests

**Deliverable**: Can handle concurrent I/O operations

**Test Program:**
```dryad
import { HttpClient } from "@std/http";

async function fetchMultiple() {
    let client = new HttpClient();
    let [data1, data2] = await Promise.all([
        client.get("http://api.example.com/1"),
        client.get("http://api.example.com/2")
    ]);
    return data1 + data2;
}
```

---

### Phase 6: HTTP & High-Level Stdlib (Week 9-10)
**Goal**: Complete stdlib with HTTP, JSON, crypto

**Tasks:**
- [ ] Implement `@std/http.dryad` (client + server)
- [ ] Implement `@std/json.dryad` (parser + serializer)
- [ ] Implement `@std/crypto.dryad` (hashing, encryption)
- [ ] Implement `@std/encoding.dryad` (base64, UTF-8)
- [ ] Implement `@std/time.dryad` (date/time)
- [ ] Write comprehensive stdlib tests

**Deliverable**: Can build real-world HTTP services in Dryad

**Test Program:**
```dryad
import { HttpServer } from "@std/http";
import { readFile } from "@std/io";
import * as json from "@std/json";

let server = new HttpServer(8080);

server.get("/api/data", async (req, res) => {
    let data = readFile("data.json");
    let obj = json.parse(data);
    res.json(obj);
});

server.listen();
```

---

### Phase 7: Bytecode VM (Week 11-12)
**Goal**: Compile to bytecode for better performance

**Tasks:**
- [ ] Define bytecode opcodes (~50 opcodes)
- [ ] Implement bytecode compiler (AST → bytecode)
- [ ] Implement stack-based VM
- [ ] Add VM execution loop
- [ ] Implement bytecode serialization/deserialization
- [ ] Write bytecode tests
- [ ] Benchmark: interpreter vs bytecode

**Deliverable**: 5-10x speedup over tree-walking interpreter

---

### Phase 8: Garbage Collector (Week 13-14)
**Goal**: Replace reference counting with real GC

**Tasks:**
- [ ] Implement mark-sweep collector
- [ ] Add write barriers
- [ ] Implement generational GC (young + old gen)
- [ ] Add incremental collection
- [ ] Tune GC parameters
- [ ] Write GC stress tests
- [ ] Benchmark memory usage

**Deliverable**: Robust GC handling real-world workloads

---

### Phase 9: FFI & Bindings (Week 15)
**Goal**: Interop with C libraries

**Tasks:**
- [ ] Implement `@ffi` decorator
- [ ] Add C calling convention support
- [ ] Implement `extern "C"` blocks
- [ ] Create `dryad-bindgen` tool (C headers → Dryad)
- [ ] Write FFI tests with libcrypto
- [ ] Document FFI best practices

**Deliverable**: Can call arbitrary C libraries

---

### Phase 10: Tooling & DX (Week 16)
**Goal**: Developer experience improvements

**Tasks:**
- [ ] Implement REPL with history
- [ ] Add `dryad fmt` code formatter
- [ ] Create `dryad check` syntax validator
- [ ] Improve error messages (colors, suggestions)
- [ ] Add `--verbose` flag for debugging
- [ ] Write CLI user guide

**Deliverable**: Pleasant developer experience

---

### Phase 11: JIT Compilation (Week 17-20) [OPTIONAL]
**Goal**: Just-in-time compilation for hot code

**Tasks:**
- [ ] Integrate with LLVM JIT
- [ ] Implement hot path detection
- [ ] Add bytecode → LLVM IR translation
- [ ] Implement tiered compilation
- [ ] Benchmark JIT performance

**Deliverable**: 10-20x speedup on compute-heavy code

---

### Phase 12: AOT Compilation (Week 21-24) [OPTIONAL]
**Goal**: Ahead-of-time native binaries

**Tasks:**
- [ ] Implement full LLVM IR generation
- [ ] Add LLVM optimization passes
- [ ] Generate native executables (ELF/PE/Mach-O)
- [ ] Implement static linking
- [ ] Support cross-compilation
- [ ] Write AOT integration tests

**Deliverable**: Standalone native executables

---

## 5. Detailed Task Breakdown

### 5.1 Phase 0: Foundation — DETAILED

#### Task 0.1: Repository Setup
```bash
# Initialize repository
git init dryad-cpp
cd dryad-cpp
git branch -M main

# Create .gitignore
cat > .gitignore << EOF
build/
*.o
*.a
*.so
*.dylib
CMakeCache.txt
CMakeFiles/
compile_commands.json
.vscode/
.idea/
*.swp
EOF

# Initial commit
git add .
git commit -m "Initial commit: clean C++ reimplementation"
```

#### Task 0.2: CMake Build System
```cmake
# CMakeLists.txt (root)
cmake_minimum_required(VERSION 3.20)
project(Dryad VERSION 2.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Compiler flags
if(CMAKE_CXX_COMPILER_ID MATCHES "GNU|Clang")
    add_compile_options(-Wall -Wextra -Wpedantic -Werror)
endif()

# Build types
if(NOT CMAKE_BUILD_TYPE)
    set(CMAKE_BUILD_TYPE Debug)
endif()

# Options
option(DRYAD_BUILD_TESTS "Build tests" ON)
option(DRYAD_ENABLE_ASAN "Enable AddressSanitizer" OFF)

if(DRYAD_ENABLE_ASAN)
    add_compile_options(-fsanitize=address)
    add_link_options(-fsanitize=address)
endif()

# Subdirectories
add_subdirectory(src)

if(DRYAD_BUILD_TESTS)
    enable_testing()
    add_subdirectory(tests)
endif()
```

#### Task 0.3: Value Type Implementation
```cpp
// include/dryad/runtime/value.hpp
#pragma once
#include <cstdint>
#include <string>
#include <variant>
#include <memory>

namespace dryad::runtime {

enum class ValueType {
    Null,
    Bool,
    Number,
    String,
    Object,
    Array,
    Function
};

class Object;  // Forward declaration

class Value {
public:
    // Constructors
    Value() : data_(nullptr) {}
    explicit Value(bool b) : data_(b) {}
    explicit Value(double n) : data_(n) {}
    explicit Value(const std::string& s) : data_(s) {}
    explicit Value(std::shared_ptr<Object> obj) : data_(obj) {}
    
    // Type checking
    bool is_null() const;
    bool is_bool() const;
    bool is_number() const;
    bool is_string() const;
    bool is_object() const;
    
    // Conversions
    bool as_bool() const;
    double as_number() const;
    std::string as_string() const;
    std::shared_ptr<Object> as_object() const;
    
    // Static factories
    static Value from_null();
    static Value from_bool(bool b);
    static Value from_number(double n);
    static Value from_string(const std::string& s);
    
private:
    using DataType = std::variant<
        std::nullptr_t,
        bool,
        double,
        std::string,
        std::shared_ptr<Object>
    >;
    
    DataType data_;
};

}  // namespace dryad::runtime
```

**Test:**
```cpp
// tests/unit/runtime/value_test.cpp
#include <gtest/gtest.h>
#include "dryad/runtime/value.hpp"

using namespace dryad::runtime;

TEST(Value, Null) {
    Value v = Value::from_null();
    EXPECT_TRUE(v.is_null());
    EXPECT_FALSE(v.is_number());
}

TEST(Value, Number) {
    Value v = Value::from_number(42.5);
    EXPECT_TRUE(v.is_number());
    EXPECT_EQ(v.as_number(), 42.5);
}

TEST(Value, String) {
    Value v = Value::from_string("hello");
    EXPECT_TRUE(v.is_string());
    EXPECT_EQ(v.as_string(), "hello");
}
```

#### Task 0.4: CI/CD Setup (GitHub Actions)
```yaml
# .github/workflows/ci.yml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest]
        build_type: [Debug, Release]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install dependencies (Ubuntu)
      if: matrix.os == 'ubuntu-latest'
      run: |
        sudo apt-get update
        sudo apt-get install -y cmake g++ libgtest-dev
    
    - name: Install dependencies (macOS)
      if: matrix.os == 'macos-latest'
      run: |
        brew install cmake googletest
    
    - name: Configure CMake
      run: |
        cmake -B build \
          -DCMAKE_BUILD_TYPE=${{ matrix.build_type }} \
          -DDRYAD_BUILD_TESTS=ON
    
    - name: Build
      run: cmake --build build
    
    - name: Run tests
      run: cd build && ctest --output-on-failure
```

---

### 5.2 Phase 1: Lexer & Parser — DETAILED

#### Task 1.1: Token Definition
```cpp
// include/dryad/compiler/token.hpp
#pragma once
#include <string>
#include <variant>

namespace dryad::compiler {

enum class TokenType {
    // Literals
    Number,
    String,
    True,
    False,
    Null,
    
    // Identifiers & Keywords
    Identifier,
    Let,
    Const,
    Function,
    Class,
    If,
    Else,
    While,
    For,
    Return,
    Import,
    Export,
    From,
    As,
    Async,
    Await,
    
    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Equal,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    AmpersandAmpersand,
    PipePipe,
    
    // Symbols
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Semicolon,
    Comma,
    Dot,
    Colon,
    Arrow,
    DoubleColon,
    
    // Special
    Eof,
    Error
};

struct SourceLocation {
    std::string filename;
    size_t line;
    size_t column;
};

class Token {
public:
    Token(TokenType type, SourceLocation loc)
        : type_(type), location_(loc) {}
    
    Token(TokenType type, const std::string& lexeme, SourceLocation loc)
        : type_(type), lexeme_(lexeme), location_(loc) {}
    
    TokenType type() const { return type_; }
    const std::string& lexeme() const { return lexeme_; }
    const SourceLocation& location() const { return location_; }
    
private:
    TokenType type_;
    std::string lexeme_;
    SourceLocation location_;
};

}  // namespace dryad::compiler
```

#### Task 1.2: Lexer Implementation
```cpp
// include/dryad/compiler/lexer.hpp
#pragma once
#include "token.hpp"
#include <string>
#include <vector>

namespace dryad::compiler {

class Lexer {
public:
    explicit Lexer(const std::string& source, 
                   const std::string& filename = "<stdin>");
    
    std::vector<Token> tokenize();
    
private:
    Token next_token();
    char peek() const;
    char advance();
    bool match(char expected);
    void skip_whitespace();
    void skip_comment();
    
    Token identifier();
    Token number();
    Token string();
    
    bool is_at_end() const;
    bool is_digit(char c) const;
    bool is_alpha(char c) const;
    
    SourceLocation current_location() const;
    
    std::string source_;
    std::string filename_;
    size_t current_ = 0;
    size_t line_ = 1;
    size_t column_ = 1;
};

}  // namespace dryad::compiler
```

**Test:**
```cpp
// tests/unit/compiler/lexer_test.cpp
#include <gtest/gtest.h>
#include "dryad/compiler/lexer.hpp"

using namespace dryad::compiler;

TEST(Lexer, Numbers) {
    Lexer lexer("42 3.14 0xFF 0b1010");
    auto tokens = lexer.tokenize();
    
    ASSERT_EQ(tokens.size(), 5);  // 4 numbers + EOF
    EXPECT_EQ(tokens[0].type(), TokenType::Number);
    EXPECT_EQ(tokens[0].lexeme(), "42");
    EXPECT_EQ(tokens[1].lexeme(), "3.14");
    EXPECT_EQ(tokens[2].lexeme(), "0xFF");
}

TEST(Lexer, Keywords) {
    Lexer lexer("let const function if else");
    auto tokens = lexer.tokenize();
    
    EXPECT_EQ(tokens[0].type(), TokenType::Let);
    EXPECT_EQ(tokens[1].type(), TokenType::Const);
    EXPECT_EQ(tokens[2].type(), TokenType::Function);
}

TEST(Lexer, Operators) {
    Lexer lexer("+ - * / == != <= >=");
    auto tokens = lexer.tokenize();
    
    EXPECT_EQ(tokens[0].type(), TokenType::Plus);
    EXPECT_EQ(tokens[4].type(), TokenType::EqualEqual);
    EXPECT_EQ(tokens[5].type(), TokenType::BangEqual);
}
```

#### Task 1.3: AST Node Definitions
```cpp
// include/dryad/compiler/ast.hpp
#pragma once
#include "token.hpp"
#include <memory>
#include <vector>

namespace dryad::compiler::ast {

// Forward declarations
class Visitor;

// Base node
class Node {
public:
    virtual ~Node() = default;
    virtual void accept(Visitor& visitor) = 0;
};

// Expressions
class Expr : public Node {};

class NumberLiteral : public Expr {
public:
    explicit NumberLiteral(double value) : value_(value) {}
    void accept(Visitor& visitor) override;
    double value() const { return value_; }
private:
    double value_;
};

class StringLiteral : public Expr {
public:
    explicit StringLiteral(std::string value) 
        : value_(std::move(value)) {}
    void accept(Visitor& visitor) override;
    const std::string& value() const { return value_; }
private:
    std::string value_;
};

class BinaryExpr : public Expr {
public:
    BinaryExpr(std::unique_ptr<Expr> left,
               Token op,
               std::unique_ptr<Expr> right)
        : left_(std::move(left)),
          op_(std::move(op)),
          right_(std::move(right)) {}
    
    void accept(Visitor& visitor) override;
    
    const Expr* left() const { return left_.get(); }
    const Expr* right() const { return right_.get(); }
    const Token& op() const { return op_; }
    
private:
    std::unique_ptr<Expr> left_;
    Token op_;
    std::unique_ptr<Expr> right_;
};

// Statements
class Stmt : public Node {};

class VarDecl : public Stmt {
public:
    VarDecl(Token name, 
            std::unique_ptr<Expr> initializer,
            bool is_const)
        : name_(std::move(name)),
          initializer_(std::move(initializer)),
          is_const_(is_const) {}
    
    void accept(Visitor& visitor) override;
    
    const Token& name() const { return name_; }
    const Expr* initializer() const { return initializer_.get(); }
    bool is_const() const { return is_const_; }
    
private:
    Token name_;
    std::unique_ptr<Expr> initializer_;
    bool is_const_;
};

class Program : public Node {
public:
    void add_statement(std::unique_ptr<Stmt> stmt) {
        statements_.push_back(std::move(stmt));
    }
    
    const std::vector<std::unique_ptr<Stmt>>& statements() const {
        return statements_;
    }
    
    void accept(Visitor& visitor) override;
    
private:
    std::vector<std::unique_ptr<Stmt>> statements_;
};

// Visitor pattern
class Visitor {
public:
    virtual ~Visitor() = default;
    virtual void visit(NumberLiteral& node) = 0;
    virtual void visit(StringLiteral& node) = 0;
    virtual void visit(BinaryExpr& node) = 0;
    virtual void visit(VarDecl& node) = 0;
    virtual void visit(Program& node) = 0;
};

}  // namespace dryad::compiler::ast
```

---

### 5.3 Phase 3: Intrinsics Layer — DETAILED

#### Task 3.1: Syscall ID Enum
```cpp
// include/dryad/runtime/intrinsics.hpp
#pragma once
#include <cstdint>

namespace dryad::runtime {

enum class SyscallID : uint16_t {
    // File I/O (8)
    OPEN = 1,
    READ = 2,
    WRITE = 3,
    CLOSE = 4,
    LSEEK = 5,
    STAT = 6,
    UNLINK = 7,
    MKDIR = 8,
    
    // Network (8)
    SOCKET = 10,
    CONNECT = 11,
    BIND = 12,
    LISTEN = 13,
    ACCEPT = 14,
    SEND = 15,
    RECV = 16,
    SHUTDOWN = 17,
    
    // Memory (5)
    MALLOC = 20,
    FREE = 21,
    REALLOC = 22,
    MEMCPY = 23,
    MEMSET = 24,
    
    // Async I/O (6)
    EPOLL_CREATE = 30,
    EPOLL_CTL = 31,
    EPOLL_WAIT = 32,
    KQUEUE = 33,
    KEVENT = 34,
    SELECT = 35,
    
    // Process/Thread (6)
    FORK = 40,
    EXEC = 41,
    WAIT = 42,
    PTHREAD_CREATE = 43,
    PTHREAD_JOIN = 44,
    PTHREAD_DETACH = 45,
    
    // Time (4)
    GETTIMEOFDAY = 50,
    CLOCK_GETTIME = 51,
    SLEEP = 52,
    NANOSLEEP = 53,
    
    // Environment (5)
    GETENV = 60,
    SETENV = 61,
    GETCWD = 62,
    CHDIR = 63,
    GETPID = 64,
    
    // Atomic Operations (5)
    ATOMIC_LOAD = 70,
    ATOMIC_STORE = 71,
    ATOMIC_CAS = 72,
    ATOMIC_FETCH_ADD = 73,
    MEMORY_FENCE = 74,
    
    // Signals (3)
    SIGNAL = 80,
    KILL = 81,
    SIGACTION = 82
};

const char* syscall_name(SyscallID id);

}  // namespace dryad::runtime
```

#### Task 3.2: Intrinsics Dispatcher
```cpp
// src/runtime/intrinsics/intrinsics.cpp
#include "dryad/runtime/intrinsics.hpp"
#include "dryad/runtime/value.hpp"
#include "dryad/runtime/vm.hpp"
#include <unistd.h>
#include <fcntl.h>
#include <sys/socket.h>

namespace dryad::runtime {

Value VM::execute_intrinsic(SyscallID id, 
                            const std::vector<Value>& args) {
    switch (id) {
        case SyscallID::OPEN: {
            // Args: (path: string, flags: i32)
            auto path = args[0].as_string();
            auto flags = static_cast<int>(args[1].as_number());
            int fd = ::open(path.c_str(), flags);
            return Value::from_number(fd);
        }
        
        case SyscallID::READ: {
            // Args: (fd: i32, buf: ptr<u8>, len: usize)
            auto fd = static_cast<int>(args[0].as_number());
            auto buf = args[1].as_buffer();
            auto len = static_cast<size_t>(args[2].as_number());
            ssize_t n = ::read(fd, buf->data(), len);
            return Value::from_number(n);
        }
        
        case SyscallID::WRITE: {
            // Args: (fd: i32, buf: ptr<u8>, len: usize)
            auto fd = static_cast<int>(args[0].as_number());
            auto buf = args[1].as_buffer();
            auto len = static_cast<size_t>(args[2].as_number());
            ssize_t n = ::write(fd, buf->data(), len);
            return Value::from_number(n);
        }
        
        case SyscallID::CLOSE: {
            // Args: (fd: i32)
            auto fd = static_cast<int>(args[0].as_number());
            ::close(fd);
            return Value::from_null();
        }
        
        case SyscallID::SOCKET: {
            // Args: (domain: i32, type: i32)
            auto domain = static_cast<int>(args[0].as_number());
            auto type = static_cast<int>(args[1].as_number());
            int fd = ::socket(domain, type, 0);
            return Value::from_number(fd);
        }
        
        // ... Implement remaining 45 syscalls ...
        
        default:
            throw std::runtime_error("Unknown syscall ID");
    }
}

}  // namespace dryad::runtime
```

**Test:**
```cpp
// tests/unit/runtime/intrinsics_test.cpp
#include <gtest/gtest.h>
#include "dryad/runtime/vm.hpp"
#include <fcntl.h>

using namespace dryad::runtime;

TEST(Intrinsics, FileIO) {
    VM vm;
    
    // Create test file
    std::ofstream("test_file.txt") << "Hello, World!";
    
    // Test OPEN
    Value fd = vm.execute_intrinsic(SyscallID::OPEN, {
        Value::from_string("test_file.txt"),
        Value::from_number(O_RDONLY)
    });
    ASSERT_GT(fd.as_number(), 0);
    
    // Test READ
    auto buf = std::make_shared<Buffer>(100);
    Value bytes_read = vm.execute_intrinsic(SyscallID::READ, {
        fd,
        Value::from_buffer(buf),
        Value::from_number(100)
    });
    EXPECT_GT(bytes_read.as_number(), 0);
    
    // Test CLOSE
    vm.execute_intrinsic(SyscallID::CLOSE, {fd});
    
    // Cleanup
    std::remove("test_file.txt");
}
```

---

## 6. Testing Strategy

### 6.1 Test Pyramid

```
         /\
        /  \
       / E2E \        10% — End-to-end (real programs)
      /------\
     / Integ  \       20% — Integration (modules together)
    /----------\
   /   Unit     \     70% — Unit tests (isolated components)
  /--------------\
```

### 6.2 Unit Testing (Google Test)

**Coverage Target**: >90% code coverage

**Test Organization:**
```
tests/unit/
├── runtime/
│   ├── value_test.cpp
│   ├── vm_test.cpp
│   ├── gc_test.cpp
│   └── intrinsics_test.cpp
├── compiler/
│   ├── lexer_test.cpp
│   ├── parser_test.cpp
│   └── codegen_test.cpp
└── stdlib/
    └── (Dryad test files)
```

**Example Test:**
```cpp
TEST(Parser, FunctionDeclaration) {
    const char* source = R"(
        function add(a: number, b: number): number {
            return a + b;
        }
    )";
    
    Parser parser(source);
    auto program = parser.parse();
    
    ASSERT_EQ(program->statements().size(), 1);
    
    auto func = dynamic_cast<FunctionDecl*>(
        program->statements()[0].get()
    );
    ASSERT_NE(func, nullptr);
    EXPECT_EQ(func->name().lexeme(), "add");
    EXPECT_EQ(func->parameters().size(), 2);
}
```

### 6.3 Integration Testing

**Stdlib Tests (in Dryad):**
```dryad
// tests/integration/stdlib/io_test.dryad
import { test, assert } from "@std/testing";
import { readFile, writeFile } from "@std/io";

test("File I/O", () => {
    writeFile("/tmp/test.txt", "Hello");
    let content = readFile("/tmp/test.txt");
    assert(content == "Hello");
});

test("MemoryBackend", () => {
    let fs = new FileSystem(new MemoryBackend());
    fs.writeFile("/virtual/test.txt", "World");
    assert(fs.readFile("/virtual/test.txt") == "World");
});
```

### 6.4 End-to-End Testing

**Real Programs:**
```dryad
// examples/http_server.dryad
import { HttpServer } from "@std/http";

let server = new HttpServer(8080);

server.get("/", (req, res) => {
    res.send("Hello, World!");
});

server.listen();
print("Server running on http://localhost:8080");
```

**E2E Test:**
```bash
#!/bin/bash
# tests/e2e/http_server_test.sh

# Start server in background
./dryad run examples/http_server.dryad &
SERVER_PID=$!
sleep 1

# Test request
RESPONSE=$(curl -s http://localhost:8080)
if [ "$RESPONSE" != "Hello, World!" ]; then
    echo "FAIL: Expected 'Hello, World!', got '$RESPONSE'"
    kill $SERVER_PID
    exit 1
fi

echo "PASS"
kill $SERVER_PID
```

### 6.5 Performance Testing

**Benchmarks:**
```cpp
// benchmarks/fib_benchmark.cpp
#include <benchmark/benchmark.h>
#include "dryad/runtime/vm.hpp"

static void BM_Fibonacci(benchmark::State& state) {
    VM vm;
    vm.load_script(R"(
        function fib(n) {
            if (n <= 1) return n;
            return fib(n-1) + fib(n-2);
        }
        fib(20);
    )");
    
    for (auto _ : state) {
        vm.execute();
    }
}
BENCHMARK(BM_Fibonacci);
```

---

## 7. Risk Mitigation

### 7.1 Technical Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **GC performance issues** | High | Medium | Start with simple mark-sweep, profile early, iterate |
| **Intrinsics overhead** | Medium | Low | Benchmark against Go syscalls, optimize hot paths |
| **Memory leaks in C++** | High | Medium | Use smart pointers, AddressSanitizer in CI |
| **Bytecode VM bugs** | High | Low | Comprehensive unit tests, fuzzing |
| **LLVM integration complexity** | Medium | High | Make AOT optional, ship interpreter+bytecode first |
| **Platform-specific syscalls** | Low | High | Abstract platform differences, test on multiple OS |

### 7.2 Schedule Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Scope creep** | High | High | Strict phase boundaries, no features outside roadmap |
| **Underestimated effort** | Medium | High | 25% time buffer per phase, re-assess weekly |
| **Blocked dependencies** | Medium | Low | Parallel workstreams where possible |
| **Key person unavailability** | Low | Low | Document everything, pair programming |

### 7.3 Quality Risks

| Risk | Impact | Probability | Mitigation |
|------|--------|-------------|------------|
| **Regressions** | High | Medium | CI runs all tests on every commit |
| **Poor error messages** | Medium | Medium | Dedicated UX testing, user feedback |
| **Memory safety bugs** | High | Medium | ASAN/MSAN in CI, code review |
| **Incomplete stdlib** | Low | Low | Incremental delivery, MVP first |

---

## 8. Success Criteria

### 8.1 Phase Completion Criteria

**Phase is DONE when:**
- ✅ All tasks completed
- ✅ All tests passing (>90% coverage)
- ✅ Documentation updated
- ✅ Code reviewed
- ✅ CI green on all platforms
- ✅ Performance benchmarks meet targets

**Phase is NOT done if:**
- ❌ Any test failing
- ❌ Coverage <90%
- ❌ Compiler warnings present
- ❌ Memory leaks detected
- ❌ Documentation missing

### 8.2 MVP Success Criteria (Phase 6 Complete)

**Must Have:**
- ✅ Execute all valid Dryad syntax
- ✅ All 50 intrinsics functional
- ✅ Core stdlib (@std/io, @std/net, @std/http)
- ✅ VFS with NativeBackend and MemoryBackend
- ✅ Async/await working
- ✅ Error messages helpful
- ✅ Can build real HTTP server

**Nice to Have:**
- Bytecode VM (Phase 7)
- GC (Phase 8)
- FFI (Phase 9)

### 8.3 Performance Targets

| Metric | Target | Baseline (Rust) |
|--------|--------|-----------------|
| **Interpreter Speed** | 100k ops/sec | N/A (new) |
| **Bytecode Speed** | 5-10x interpreter | N/A (new) |
| **Intrinsic Overhead** | <10ns per call | N/A (new) |
| **Startup Time** | <50ms | ~200ms |
| **Memory Usage** | <50MB for hello world | ~100MB |
| **GC Pause** | <10ms for 100MB heap | N/A (new) |

---

## 9. Timeline & Milestones

### 9.1 Gantt Chart (16 Weeks to MVP)

```
Week │ Phase
─────┼────────────────────────────────────────────────
 1   │ ████ Phase 0: Foundation
 2   │ ████ Phase 1: Lexer & Parser
 3   │ ████ Phase 2: Interpreter (1/2)
 4   │ ████ Phase 2: Interpreter (2/2)
 5   │ ████ Phase 3: Intrinsics
 6   │ ████ Phase 4: Core Stdlib (1/2)
 7   │ ████ Phase 4: Core Stdlib (2/2)
 8   │ ████ Phase 5: Async I/O
 9   │ ████ Phase 6: HTTP Stdlib (1/2)
10   │ ████ Phase 6: HTTP Stdlib (2/2)
─────┼──────────────── MVP COMPLETE ────────────────
11   │ ████ Phase 7: Bytecode VM (1/2)
12   │ ████ Phase 7: Bytecode VM (2/2)
13   │ ████ Phase 8: GC (1/2)
14   │ ████ Phase 8: GC (2/2)
15   │ ████ Phase 9: FFI
16   │ ████ Phase 10: Tooling
─────┼──────────────── v2.0 RELEASE ───────────────
```

### 9.2 Milestones

| Milestone | Week | Deliverable |
|-----------|------|-------------|
| **M0: Foundation** | 1 | Build system + Value type |
| **M1: Parser** | 2 | Can parse all Dryad syntax |
| **M2: Interpreter** | 4 | Can run Fibonacci |
| **M3: Intrinsics** | 5 | All 50 syscalls working |
| **M4: Core Stdlib** | 7 | File I/O and sockets in Dryad |
| **M5: Async** | 8 | Event loop functional |
| **M6: MVP** | 10 | HTTP server demo working |
| **M7: Bytecode** | 12 | 5x speedup achieved |
| **M8: GC** | 14 | Memory stable under load |
| **M9: FFI** | 15 | Can call libcrypto |
| **M10: Release** | 16 | v2.0 tagged and published |

### 9.3 Weekly Review Cadence

**Every Friday:**
1. Review completed tasks
2. Update roadmap if needed
3. Identify blockers
4. Plan next week's sprint

**Every Month:**
1. Stakeholder demo
2. Performance benchmark review
3. Roadmap adjustment
4. Retrospective

---

## 10. Appendix: Migration from Rust

### 10.1 What We're NOT Migrating

**Don't port these:**
- ❌ Entire Rust runtime implementation
- ❌ Native module bindings
- ❌ Reference counting logic
- ❌ Rust-specific abstractions

**Rationale**: Clean reimplementation is faster than gradual migration.

### 10.2 What We ARE Migrating

**Port these (conceptually):**
- ✅ Test cases (logic, not code)
- ✅ Error messages (copy exact text)
- ✅ Parser structure (proven sound)
- ✅ Example programs

**How to port tests:**
```rust
// Rust test (archived)
#[test]
fn test_fib() {
    let result = run_dryad("function fib(n) { ... }");
    assert_eq!(result, 55);
}
```

```cpp
// C++ test (new)
TEST(Interpreter, Fibonacci) {
    VM vm;
    vm.load_script("function fib(n) { ... }");
    Value result = vm.execute();
    EXPECT_EQ(result.as_number(), 55);
}
```

### 10.3 Knowledge Transfer

**Documentation to preserve:**
1. Read `archived/README.md` for historical context
2. Extract error message catalog
3. Document known edge cases
4. Preserve performance benchmark baselines

**What NOT to preserve:**
- Implementation details
- Architectural decisions (superseded by v2.0 spec)
- Rust idioms

---

## 11. Communication & Collaboration

### 11.1 Documentation Standards

**Every new component must have:**
- Header comment explaining purpose
- Function-level documentation (Doxygen style)
- Usage examples
- Test coverage report

**Example:**
```cpp
/**
 * @brief Virtual Machine for executing Dryad bytecode
 * 
 * The VM maintains a stack, heap, and instruction pointer.
 * It executes bytecode instructions sequentially and handles
 * intrinsic calls via the intrinsics dispatcher.
 * 
 * Example usage:
 * @code
 * VM vm;
 * vm.load_bytecode(bytecode);
 * Value result = vm.execute();
 * @endcode
 * 
 * @see BytecodeCompiler for bytecode generation
 * @see Intrinsics for syscall handling
 */
class VM {
    // ...
};
```

### 11.2 Code Review Process

**All code must be reviewed before merge.**

**Review Checklist:**
- [ ] Code follows style guide
- [ ] Tests included and passing
- [ ] Documentation updated
- [ ] No compiler warnings
- [ ] ASAN clean
- [ ] Performance acceptable

**Review Time SLA**: <24 hours for small PRs, <48 hours for large

### 11.3 Branching Strategy

```
main           ←─ Protected, always stable
  ├── develop  ←─ Integration branch
  │   ├── feature/lexer
  │   ├── feature/parser
  │   └── feature/intrinsics
  └── release/v2.0  ←─ Release branch
```

**Rules:**
- Never commit directly to `main`
- Feature branches from `develop`
- PR to `develop` (requires review)
- `develop` → `main` weekly (after all tests pass)

---

## 🎯 Conclusion

This roadmap provides a clear path from archived Rust codebase to production-ready C++ implementation with intrinsics-based architecture. The key to success is:

1. **Strict Phase Boundaries** — Don't start Phase N+1 until Phase N is 100% complete
2. **Test-First Discipline** — Every feature has tests before implementation
3. **Incremental Delivery** — Ship working MVP (Phase 6), iterate from there
4. **Clear Architecture** — Respect the 4-layer model, no cross-layer dependencies
5. **Weekly Reviews** — Catch drift early, adjust course frequently

**Next Steps:**
1. ✅ Review this document with team
2. Create GitHub project board with all tasks
3. Set up development environment
4. Begin Phase 0: Foundation
5. Weekly progress updates

**Let's build Dryad v2.0!** 🚀

---

**Document Version**: 1.0  
**Last Updated**: May 27, 2026  
**Owner**: Dryad Development Team  
**Status**: Active — Implementation Starting
