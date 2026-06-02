# Dryad C++ Reimplementation — Task Checklist

**Status**: 🟢 Active Development  
**Progress**: 0/320 tasks (0%)  
**Current Phase**: Phase 0 — Foundation

---

## 📊 Phase Progress Overview

| Phase | Tasks | Complete | % Done | Status |
|-------|-------|----------|--------|--------|
| Phase 0: Foundation | 7 | 0 | 0% | 🔴 Not Started |
| Phase 1: Lexer & Parser | 15 | 0 | 0% | 🔴 Not Started |
| Phase 2: Interpreter | 25 | 0 | 0% | 🔴 Not Started |
| Phase 3: Intrinsics | 50 | 0 | 0% | 🔴 Not Started |
| Phase 4: Core Stdlib | 30 | 0 | 0% | 🔴 Not Started |
| Phase 5: Async I/O | 20 | 0 | 0% | 🔴 Not Started |
| Phase 6: HTTP Stdlib | 25 | 0 | 0% | 🔴 Not Started |
| Phase 7: Bytecode VM | 30 | 0 | 0% | 🔴 Not Started |
| Phase 8: Garbage Collector | 25 | 0 | 0% | 🔴 Not Started |
| Phase 9: FFI | 15 | 0 | 0% | 🔴 Not Started |
| Phase 10: Tooling | 20 | 0 | 0% | 🔴 Not Started |
| **TOTAL** | **262** | **0** | **0%** | 🔴 |

---

## Phase 0: Foundation (Week 1)

**Goal**: Project skeleton, build system, basic infrastructure  
**Status**: 🔴 Not Started (0/7)

### Repository Setup
- [ ] 0.1.1 Initialize git repository with clean structure
- [ ] 0.1.2 Create `.gitignore` with C++ build artifacts
- [ ] 0.1.3 Add `LICENSE` file (MIT)
- [ ] 0.1.4 Create initial `README.md`
- [ ] 0.1.5 Set up branch protection rules (`main`, `develop`)

### Build System
- [ ] 0.2.1 Create root `CMakeLists.txt` with C++20 standard
- [ ] 0.2.2 Configure compiler flags (`-Wall -Wextra -Werror`)
- [ ] 0.2.3 Set up Debug and Release build types
- [ ] 0.2.4 Add AddressSanitizer option
- [ ] 0.2.5 Configure subdirectories (`src`, `tests`)
- [ ] 0.2.6 Verify builds on Linux
- [ ] 0.2.7 Verify builds on macOS

### CI/CD
- [ ] 0.3.1 Create `.github/workflows/ci.yml`
- [ ] 0.3.2 Configure Ubuntu build job
- [ ] 0.3.3 Configure macOS build job
- [ ] 0.3.4 Add test execution step
- [ ] 0.3.5 Add code coverage reporting (codecov)
- [ ] 0.3.6 Verify CI passes on empty project

### Testing Framework
- [ ] 0.4.1 Install Google Test dependency
- [ ] 0.4.2 Create `tests/CMakeLists.txt`
- [ ] 0.4.3 Write sample test to verify framework
- [ ] 0.4.4 Verify tests run via `ctest`

### Core Types
- [ ] 0.5.1 Define `Value` type (tagged union variant)
- [ ] 0.5.2 Implement `Value` constructors
- [ ] 0.5.3 Implement type checking methods (`is_null()`, etc.)
- [ ] 0.5.4 Implement conversion methods (`as_number()`, etc.)
- [ ] 0.5.5 Write `value_test.cpp` with full coverage
- [ ] 0.5.6 Define `Object` base class
- [ ] 0.5.7 Implement reference counting (temporary, until GC)

### Error Handling
- [ ] 0.6.1 Define `Error` class with category codes
- [ ] 0.6.2 Implement `SourceLocation` struct
- [ ] 0.6.3 Create error message formatting
- [ ] 0.6.4 Write error handling tests

### Documentation
- [ ] 0.7.1 Create `docs/` directory structure
- [ ] 0.7.2 Link to `dryad_theory/` specification
- [ ] 0.7.3 Write architecture overview document

**Phase 0 Deliverable**: ✅ Empty project compiles, tests run

---

## Phase 1: Lexer & Parser (Week 2)

**Goal**: Complete tokenization and AST generation  
**Status**: 🔴 Not Started (0/15)

### Token System
- [ ] 1.1.1 Define `TokenType` enum with all types
- [ ] 1.1.2 Implement `Token` class with location
- [ ] 1.1.3 Create `SourceLocation` tracking
- [ ] 1.1.4 Write token tests

### Lexer Implementation
- [ ] 1.2.1 Create `Lexer` class skeleton
- [ ] 1.2.2 Implement whitespace skipping
- [ ] 1.2.3 Implement comment skipping (`//` and `/* */`)
- [ ] 1.2.4 Implement identifier lexing
- [ ] 1.2.5 Implement keyword recognition
- [ ] 1.2.6 Implement number lexing (decimal, hex, binary, octal)
- [ ] 1.2.7 Implement string lexing (double/single quotes)
- [ ] 1.2.8 Implement template string lexing
- [ ] 1.2.9 Implement operator lexing
- [ ] 1.2.10 Implement symbol lexing
- [ ] 1.2.11 Add error reporting for invalid tokens
- [ ] 1.2.12 Write comprehensive lexer tests (>90% coverage)

### Parser Implementation
- [ ] 1.3.1 Define AST node base classes (`Expr`, `Stmt`)
- [ ] 1.3.2 Implement expression nodes (literals, binary, unary, etc.)
- [ ] 1.3.3 Implement statement nodes (var decl, func decl, etc.)
- [ ] 1.3.4 Create `Parser` class with recursive descent
- [ ] 1.3.5 Implement expression parsing with precedence
- [ ] 1.3.6 Implement statement parsing
- [ ] 1.3.7 Implement declaration parsing
- [ ] 1.3.8 Add error recovery mechanism
- [ ] 1.3.9 Add error synchronization
- [ ] 1.3.10 Write comprehensive parser tests
- [ ] 1.3.11 Test all language constructs parse correctly
- [ ] 1.3.12 Test error reporting and recovery

### AST Utilities
- [ ] 1.4.1 Implement AST visitor pattern
- [ ] 1.4.2 Create AST printer for debugging
- [ ] 1.4.3 Implement AST validation pass

**Phase 1 Deliverable**: ✅ Can parse all valid Dryad syntax into AST

---

## Phase 2: Tree-Walking Interpreter (Week 3-4)

**Goal**: Execute simple programs via AST interpretation  
**Status**: 🔴 Not Started (0/25)

### Environment & Scope
- [ ] 2.1.1 Implement `Environment` class for scope management
- [ ] 2.1.2 Add variable storage (map-based)
- [ ] 2.1.3 Implement scope chaining (parent scopes)
- [ ] 2.1.4 Add `let` variable declaration
- [ ] 2.1.5 Add `const` variable declaration
- [ ] 2.1.6 Implement variable lookup
- [ ] 2.1.7 Implement variable assignment
- [ ] 2.1.8 Write environment tests

### Expression Evaluation
- [ ] 2.2.1 Implement literal evaluation (number, string, bool, null)
- [ ] 2.2.2 Implement variable reference evaluation
- [ ] 2.2.3 Implement binary operators (+, -, *, /, %)
- [ ] 2.2.4 Implement comparison operators (==, !=, <, >, <=, >=)
- [ ] 2.2.5 Implement logical operators (&&, ||, !)
- [ ] 2.2.6 Implement unary operators (-, !, +)
- [ ] 2.2.7 Implement assignment expression
- [ ] 2.2.8 Write expression evaluation tests

### Control Flow
- [ ] 2.3.1 Implement `if` statement
- [ ] 2.3.2 Implement `if-else` statement
- [ ] 2.3.3 Implement `while` loop
- [ ] 2.3.4 Implement `for` loop
- [ ] 2.3.5 Implement `break` statement
- [ ] 2.3.6 Implement `continue` statement
- [ ] 2.3.7 Write control flow tests

### Functions
- [ ] 2.4.1 Implement function declaration
- [ ] 2.4.2 Implement function call
- [ ] 2.4.3 Implement `return` statement
- [ ] 2.4.4 Implement parameter binding
- [ ] 2.4.5 Implement closure capture
- [ ] 2.4.6 Implement arrow functions
- [ ] 2.4.7 Write function tests (including recursion)

### Built-in Functions
- [ ] 2.5.1 Implement `print()` function
- [ ] 2.5.2 Implement `typeof()` function
- [ ] 2.5.3 Implement `assert()` function
- [ ] 2.5.4 Write built-in function tests

### Integration Tests
- [ ] 2.6.1 Test Fibonacci program
- [ ] 2.6.2 Test factorial program
- [ ] 2.6.3 Test closure examples
- [ ] 2.6.4 Test scope shadowing
- [ ] 2.6.5 Test error handling

**Phase 2 Deliverable**: ✅ Can run "Hello World" and Fibonacci

---

## Phase 3: Intrinsics Layer (Week 5)

**Goal**: Implement ~50 syscall intrinsics  
**Status**: 🔴 Not Started (0/50)

### Intrinsics Infrastructure
- [ ] 3.0.1 Define `SyscallID` enum with all 50 syscalls
- [ ] 3.0.2 Create intrinsics dispatcher in VM
- [ ] 3.0.3 Add `@intrinsic` decorator to parser
- [ ] 3.0.4 Generate `INTRINSIC_SYSCALL` opcode
- [ ] 3.0.5 Implement intrinsic call mechanism
- [ ] 3.0.6 Add error handling for syscall failures

### File I/O Syscalls (8)
- [ ] 3.1.1 Implement `OPEN` syscall
- [ ] 3.1.2 Implement `READ` syscall
- [ ] 3.1.3 Implement `WRITE` syscall
- [ ] 3.1.4 Implement `CLOSE` syscall
- [ ] 3.1.5 Implement `LSEEK` syscall
- [ ] 3.1.6 Implement `STAT` syscall
- [ ] 3.1.7 Implement `UNLINK` syscall
- [ ] 3.1.8 Implement `MKDIR` syscall
- [ ] 3.1.9 Write File I/O tests

### Network Syscalls (8)
- [ ] 3.2.1 Implement `SOCKET` syscall
- [ ] 3.2.2 Implement `CONNECT` syscall
- [ ] 3.2.3 Implement `BIND` syscall
- [ ] 3.2.4 Implement `LISTEN` syscall
- [ ] 3.2.5 Implement `ACCEPT` syscall
- [ ] 3.2.6 Implement `SEND` syscall
- [ ] 3.2.7 Implement `RECV` syscall
- [ ] 3.2.8 Implement `SHUTDOWN` syscall
- [ ] 3.2.9 Write Network tests

### Memory Syscalls (5)
- [ ] 3.3.1 Implement `MALLOC` syscall
- [ ] 3.3.2 Implement `FREE` syscall
- [ ] 3.3.3 Implement `REALLOC` syscall
- [ ] 3.3.4 Implement `MEMCPY` syscall
- [ ] 3.3.5 Implement `MEMSET` syscall
- [ ] 3.3.6 Write Memory tests

### Async I/O Syscalls (6)
- [ ] 3.4.1 Implement `EPOLL_CREATE` syscall (Linux)
- [ ] 3.4.2 Implement `EPOLL_CTL` syscall (Linux)
- [ ] 3.4.3 Implement `EPOLL_WAIT` syscall (Linux)
- [ ] 3.4.4 Implement `KQUEUE` syscall (macOS/BSD)
- [ ] 3.4.5 Implement `KEVENT` syscall (macOS/BSD)
- [ ] 3.4.6 Implement `SELECT` syscall (fallback)
- [ ] 3.4.7 Write Async I/O tests

### Process/Thread Syscalls (6)
- [ ] 3.5.1 Implement `FORK` syscall
- [ ] 3.5.2 Implement `EXEC` syscall
- [ ] 3.5.3 Implement `WAIT` syscall
- [ ] 3.5.4 Implement `PTHREAD_CREATE` syscall
- [ ] 3.5.5 Implement `PTHREAD_JOIN` syscall
- [ ] 3.5.6 Implement `PTHREAD_DETACH` syscall
- [ ] 3.5.7 Write Process/Thread tests

### Time Syscalls (4)
- [ ] 3.6.1 Implement `GETTIMEOFDAY` syscall
- [ ] 3.6.2 Implement `CLOCK_GETTIME` syscall
- [ ] 3.6.3 Implement `SLEEP` syscall
- [ ] 3.6.4 Implement `NANOSLEEP` syscall
- [ ] 3.6.5 Write Time tests

### Environment Syscalls (5)
- [ ] 3.7.1 Implement `GETENV` syscall
- [ ] 3.7.2 Implement `SETENV` syscall
- [ ] 3.7.3 Implement `GETCWD` syscall
- [ ] 3.7.4 Implement `CHDIR` syscall
- [ ] 3.7.5 Implement `GETPID` syscall
- [ ] 3.7.6 Write Environment tests

### Atomic Operation Syscalls (5)
- [ ] 3.8.1 Implement `ATOMIC_LOAD` syscall
- [ ] 3.8.2 Implement `ATOMIC_STORE` syscall
- [ ] 3.8.3 Implement `ATOMIC_CAS` syscall
- [ ] 3.8.4 Implement `ATOMIC_FETCH_ADD` syscall
- [ ] 3.8.5 Implement `MEMORY_FENCE` syscall
- [ ] 3.8.6 Write Atomic Operation tests

### Signal Syscalls (3)
- [ ] 3.9.1 Implement `SIGNAL` syscall
- [ ] 3.9.2 Implement `KILL` syscall
- [ ] 3.9.3 Implement `SIGACTION` syscall
- [ ] 3.9.4 Write Signal tests

**Phase 3 Deliverable**: ✅ All 50 syscalls functional and tested

---

## Phase 4: Core Stdlib in Dryad (Week 6-7)

**Goal**: Implement foundational stdlib modules in pure Dryad  
**Status**: 🔴 Not Started (0/30)

### Intrinsics Declarations
- [ ] 4.1.1 Create `@std/runtime/intrinsics.dryad`
- [ ] 4.1.2 Declare all File I/O intrinsics
- [ ] 4.1.3 Declare all Network intrinsics
- [ ] 4.1.4 Declare all Memory intrinsics
- [ ] 4.1.5 Declare all Async I/O intrinsics
- [ ] 4.1.6 Declare all other intrinsics

### Buffer Implementation
- [ ] 4.2.1 Implement `Buffer` class in Dryad
- [ ] 4.2.2 Add `Buffer.allocate()` method
- [ ] 4.2.3 Add `Buffer.fromString()` method
- [ ] 4.2.4 Add `Buffer.toString()` method
- [ ] 4.2.5 Add `Buffer.slice()` method
- [ ] 4.2.6 Add `Buffer.get()` / `Buffer.set()` with bounds checking
- [ ] 4.2.7 Write Buffer tests

### I/O Module
- [ ] 4.3.1 Create `@std/io.dryad`
- [ ] 4.3.2 Implement `File` class using intrinsics
- [ ] 4.3.3 Implement `readFile()` function
- [ ] 4.3.4 Implement `writeFile()` function
- [ ] 4.3.5 Implement `appendFile()` function
- [ ] 4.3.6 Implement `exists()` function
- [ ] 4.3.7 Implement `mkdir()` function
- [ ] 4.3.8 Implement `listDir()` function
- [ ] 4.3.9 Write I/O module tests

### Network Module
- [ ] 4.4.1 Create `@std/net.dryad`
- [ ] 4.4.2 Implement `Socket` class
- [ ] 4.4.3 Implement TCP client
- [ ] 4.4.4 Implement TCP server
- [ ] 4.4.5 Implement UDP socket
- [ ] 4.4.6 Write Network module tests

### VFS Implementation
- [ ] 4.5.1 Create `@std/vfs.dryad`
- [ ] 4.5.2 Define `FileSystemBackend` interface
- [ ] 4.5.3 Implement `NativeBackend` using intrinsics
- [ ] 4.5.4 Implement `MemoryBackend` (pure Dryad, no syscalls)
- [ ] 4.5.5 Implement `FileSystem` abstraction
- [ ] 4.5.6 Write VFS tests with both backends

**Phase 4 Deliverable**: ✅ Can read/write files and create sockets from Dryad

---

## Phase 5: Async I/O & Event Loop (Week 8)

**Goal**: Non-blocking I/O with async/await  
**Status**: 🔴 Not Started (0/20)

### Coroutine Support
- [ ] 5.1.1 Add coroutine state to VM
- [ ] 5.1.2 Implement coroutine suspension
- [ ] 5.1.3 Implement coroutine resumption
- [ ] 5.1.4 Implement yield mechanism
- [ ] 5.1.5 Write coroutine tests

### Async/Await Compiler Transformation
- [ ] 5.2.1 Detect `async` functions in parser
- [ ] 5.2.2 Transform `await` expressions to yield points
- [ ] 5.2.3 Generate coroutine setup code
- [ ] 5.2.4 Generate coroutine cleanup code
- [ ] 5.2.5 Write async/await transformation tests

### Event Loop
- [ ] 5.3.1 Create `@std/async/event_loop.dryad`
- [ ] 5.3.2 Implement event loop using epoll (Linux)
- [ ] 5.3.3 Implement event loop using kqueue (macOS)
- [ ] 5.3.4 Implement fallback using select
- [ ] 5.3.5 Add task registration
- [ ] 5.3.6 Add task deregistration
- [ ] 5.3.7 Implement event loop run method
- [ ] 5.3.8 Write event loop tests

### Promise Implementation
- [ ] 5.4.1 Create `@std/async/promise.dryad`
- [ ] 5.4.2 Implement Promise constructor
- [ ] 5.4.3 Implement `then()` method
- [ ] 5.4.4 Implement `catch()` method
- [ ] 5.4.5 Implement `Promise.all()`
- [ ] 5.4.6 Implement `Promise.race()`
- [ ] 5.4.7 Write Promise tests

### Integration Tests
- [ ] 5.5.1 Test concurrent file I/O
- [ ] 5.5.2 Test concurrent network I/O
- [ ] 5.5.3 Test async function composition

**Phase 5 Deliverable**: ✅ Can handle concurrent I/O operations

---

## Phase 6: HTTP & High-Level Stdlib (Week 9-10)

**Goal**: Complete stdlib with HTTP, JSON, crypto  
**Status**: 🔴 Not Started (0/25)

### HTTP Module
- [ ] 6.1.1 Create `@std/http.dryad`
- [ ] 6.1.2 Implement `HttpClient` class
- [ ] 6.1.3 Implement HTTP request builder
- [ ] 6.1.4 Implement HTTP response parser
- [ ] 6.1.5 Implement `get()` method
- [ ] 6.1.6 Implement `post()` method
- [ ] 6.1.7 Implement `put()` / `delete()` methods
- [ ] 6.1.8 Implement `HttpServer` class
- [ ] 6.1.9 Implement route registration
- [ ] 6.1.10 Implement request handling
- [ ] 6.1.11 Write HTTP tests

### JSON Module
- [ ] 6.2.1 Create `@std/json.dryad`
- [ ] 6.2.2 Implement JSON tokenizer
- [ ] 6.2.3 Implement JSON parser
- [ ] 6.2.4 Implement `parse()` function
- [ ] 6.2.5 Implement JSON serializer
- [ ] 6.2.6 Implement `stringify()` function
- [ ] 6.2.7 Write JSON tests

### Crypto Module
- [ ] 6.3.1 Create `@std/crypto.dryad`
- [ ] 6.3.2 Implement SHA-256 hashing
- [ ] 6.3.3 Implement MD5 hashing
- [ ] 6.3.4 Implement HMAC
- [ ] 6.3.5 Implement base64 encoding/decoding
- [ ] 6.3.6 Write Crypto tests

### Encoding Module
- [ ] 6.4.1 Create `@std/encoding.dryad`
- [ ] 6.4.2 Implement Base64 encoder
- [ ] 6.4.3 Implement Base64 decoder
- [ ] 6.4.4 Implement UTF-8 utilities
- [ ] 6.4.5 Write Encoding tests

### Time Module
- [ ] 6.5.1 Create `@std/time.dryad`
- [ ] 6.5.2 Implement `Date` class
- [ ] 6.5.3 Implement date parsing
- [ ] 6.5.4 Implement date formatting
- [ ] 6.5.5 Implement time arithmetic
- [ ] 6.5.6 Write Time tests

**Phase 6 Deliverable**: ✅ Can build real-world HTTP services in Dryad

---

## Phase 7: Bytecode VM (Week 11-12)

**Goal**: Compile to bytecode for better performance  
**Status**: 🔴 Not Started (0/30)

### Bytecode Definition
- [ ] 7.1.1 Define all bytecode opcodes (~50 opcodes)
- [ ] 7.1.2 Create opcode documentation
- [ ] 7.1.3 Implement bytecode serialization format
- [ ] 7.1.4 Implement bytecode deserialization

### Bytecode Compiler
- [ ] 7.2.1 Create bytecode compiler class
- [ ] 7.2.2 Implement expression compilation
- [ ] 7.2.3 Implement statement compilation
- [ ] 7.2.4 Implement function compilation
- [ ] 7.2.5 Implement class compilation
- [ ] 7.2.6 Implement constant pool generation
- [ ] 7.2.7 Write bytecode compiler tests

### Stack-Based VM
- [ ] 7.3.1 Implement VM stack
- [ ] 7.3.2 Implement instruction pointer
- [ ] 7.3.3 Implement call frames
- [ ] 7.3.4 Implement VM execution loop
- [ ] 7.3.5 Implement arithmetic opcodes
- [ ] 7.3.6 Implement logical opcodes
- [ ] 7.3.7 Implement control flow opcodes
- [ ] 7.3.8 Implement function call opcodes
- [ ] 7.3.9 Implement intrinsic call opcode
- [ ] 7.3.10 Write VM execution tests

### Optimization
- [ ] 7.4.1 Implement constant folding
- [ ] 7.4.2 Implement dead code elimination
- [ ] 7.4.3 Implement jump threading
- [ ] 7.4.4 Write optimization tests

### Performance Testing
- [ ] 7.5.1 Benchmark interpreter vs bytecode
- [ ] 7.5.2 Profile hot paths
- [ ] 7.5.3 Optimize critical opcodes
- [ ] 7.5.4 Verify 5-10x speedup achieved

**Phase 7 Deliverable**: ✅ 5-10x speedup over tree-walking interpreter

---

## Phase 8: Garbage Collector (Week 13-14)

**Goal**: Replace reference counting with real GC  
**Status**: 🔴 Not Started (0/25)

### Mark-Sweep GC
- [ ] 8.1.1 Implement object graph traversal
- [ ] 8.1.2 Implement mark phase
- [ ] 8.1.3 Implement sweep phase
- [ ] 8.1.4 Implement heap management
- [ ] 8.1.5 Write mark-sweep tests

### Generational GC
- [ ] 8.2.1 Implement young generation
- [ ] 8.2.2 Implement old generation
- [ ] 8.2.3 Implement promotion logic
- [ ] 8.2.4 Implement remembered sets
- [ ] 8.2.5 Write generational GC tests

### Write Barriers
- [ ] 8.3.1 Implement write barrier insertion
- [ ] 8.3.2 Instrument object writes
- [ ] 8.3.3 Write write barrier tests

### Incremental Collection
- [ ] 8.4.1 Implement incremental mark phase
- [ ] 8.4.2 Implement work scheduling
- [ ] 8.4.3 Tune pause times
- [ ] 8.4.4 Write incremental GC tests

### GC Tuning
- [ ] 8.5.1 Add GC statistics collection
- [ ] 8.5.2 Implement heap sizing heuristics
- [ ] 8.5.3 Add GC configuration options
- [ ] 8.5.4 Benchmark memory usage
- [ ] 8.5.5 Stress test GC under load

**Phase 8 Deliverable**: ✅ Robust GC handling real-world workloads

---

## Phase 9: FFI & Bindings (Week 15)

**Goal**: Interop with C libraries  
**Status**: 🔴 Not Started (0/15)

### FFI Infrastructure
- [ ] 9.1.1 Add `@ffi` decorator to parser
- [ ] 9.1.2 Implement C calling convention support
- [ ] 9.1.3 Implement type marshaling (Dryad ↔ C)
- [ ] 9.1.4 Implement `extern "C"` blocks
- [ ] 9.1.5 Write FFI infrastructure tests

### dryad-bindgen Tool
- [ ] 9.2.1 Create `dryad-bindgen` CLI tool
- [ ] 9.2.2 Integrate libclang for C header parsing
- [ ] 9.2.3 Generate Dryad type declarations
- [ ] 9.2.4 Generate `@ffi` function declarations
- [ ] 9.2.5 Handle structs and enums
- [ ] 9.2.6 Write bindgen tests

### FFI Examples
- [ ] 9.3.1 Create libcrypto binding example
- [ ] 9.3.2 Create libcurl binding example
- [ ] 9.3.3 Write FFI integration tests
- [ ] 9.3.4 Document FFI best practices

**Phase 9 Deliverable**: ✅ Can call arbitrary C libraries

---

## Phase 10: Tooling & DX (Week 16)

**Goal**: Developer experience improvements  
**Status**: 🔴 Not Started (0/20)

### REPL
- [ ] 10.1.1 Create `dryad_repl` executable
- [ ] 10.1.2 Implement readline integration
- [ ] 10.1.3 Implement history support
- [ ] 10.1.4 Implement tab completion
- [ ] 10.1.5 Implement multi-line input
- [ ] 10.1.6 Write REPL tests

### Code Formatter
- [ ] 10.2.1 Create `dryad fmt` command
- [ ] 10.2.2 Implement AST pretty-printer
- [ ] 10.2.3 Add formatting rules
- [ ] 10.2.4 Support `--check` flag
- [ ] 10.2.5 Write formatter tests

### Syntax Checker
- [ ] 10.3.1 Create `dryad check` command
- [ ] 10.3.2 Run parser without execution
- [ ] 10.3.3 Report all syntax errors
- [ ] 10.3.4 Add `--json` output mode

### Error Messages
- [ ] 10.4.1 Add ANSI color support
- [ ] 10.4.2 Implement error suggestions
- [ ] 10.4.3 Improve error message clarity
- [ ] 10.4.4 Add error code documentation

### CLI Improvements
- [ ] 10.5.1 Add `--verbose` flag for debugging
- [ ] 10.5.2 Add `--version` flag
- [ ] 10.5.3 Add `--help` documentation
- [ ] 10.5.4 Improve startup time
- [ ] 10.5.5 Write CLI user guide

**Phase 10 Deliverable**: ✅ Pleasant developer experience

---

## 🎯 Milestones

- [ ] **M0: Foundation** (Week 1) — Build system + Value type
- [ ] **M1: Parser** (Week 2) — Can parse all Dryad syntax
- [ ] **M2: Interpreter** (Week 4) — Can run Fibonacci
- [ ] **M3: Intrinsics** (Week 5) — All 50 syscalls working
- [ ] **M4: Core Stdlib** (Week 7) — File I/O and sockets in Dryad
- [ ] **M5: Async** (Week 8) — Event loop functional
- [ ] **M6: MVP** (Week 10) — HTTP server demo working
- [ ] **M7: Bytecode** (Week 12) — 5x speedup achieved
- [ ] **M8: GC** (Week 14) — Memory stable under load
- [ ] **M9: FFI** (Week 15) — Can call libcrypto
- [ ] **M10: Release** (Week 16) — v2.0 tagged and published

---

## 📈 Weekly Progress Tracking

### Week 1 (Foundation)
**Planned**: 7 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 2 (Lexer & Parser)
**Planned**: 15 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 3-4 (Interpreter)
**Planned**: 25 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 5 (Intrinsics)
**Planned**: 50 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 6-7 (Core Stdlib)
**Planned**: 30 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 8 (Async)
**Planned**: 20 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 9-10 (HTTP Stdlib)
**Planned**: 25 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 11-12 (Bytecode)
**Planned**: 30 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 13-14 (GC)
**Planned**: 25 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 15 (FFI)
**Planned**: 15 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

### Week 16 (Tooling)
**Planned**: 20 tasks  
**Completed**: 0 tasks  
**Status**: 🔴 Not Started

---

**Last Updated**: May 27, 2026  
**Next Review**: Weekly on Fridays  
**Owner**: Dryad Development Team
