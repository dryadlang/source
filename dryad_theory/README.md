# Dryad Language — Theoretical Foundation Documentation

This directory contains the complete theoretical specification and architectural design documents for the Dryad programming language.

---

## 📁 Directory Structure

```
dryad_theory/
├── PDF Documents (Compiled Specifications)
│   ├── dryad_theoretical_foundation_v2.pdf ⭐ [RECOMMENDED - 34 pages, 418KB]
│   └── dryad_theoretical_foundation.pdf    [LEGACY - 45 pages, 343KB]
│
├── LaTeX Source Files
│   ├── dryad_theoretical_foundation_v2.tex ⭐ [CURRENT - Professional formatting]
│   └── dryad_theoretical_foundation.tex    [LEGACY - Original version]
│
├── Architecture Design Documents
│   ├── intrinsics_architecture_summary.md  [Executive summary of runtime]
│   ├── self_hosted_native_system.md        [Self-hosting stdlib design]
│   └── native_functions_alternatives.md    [8 proposals for native bindings]
│
├── Revision History
│   ├── V2_REVISION_SUMMARY.md             [Complete v2.0 changelog]
│   └── CORRECTIONS_APPLIED.md             [Initial corrections (v1.1)]
│
├── Assets
│   ├── dryadlogo.svg                      [Official logo - SVG format]
│   └── dryadlogo.png                      [Official logo - PNG format]
│
└── README.md                              [This file]
```

---

## 📖 Document Guide

### For First-Time Readers

**Start here:** `dryad_theoretical_foundation_v2.pdf`

This is the **canonical specification** of the Dryad language (Version 2.0, May 2026). It covers:

- ✅ Complete lexical and syntactic specification
- ✅ C++ micro-kernel runtime architecture
- ✅ Self-hosting standard library design (100% Dryad)
- ✅ Minimal intrinsics system (~50 syscalls)
- ✅ Gradual typing roadmap
- ✅ Modern ES6-style module system
- ✅ Concurrency primitives (threads, async/await, actors)
- ✅ FFI and external interface
- ✅ Compilation pipeline (interpreter/bytecode/AOT)

**Format**: IEEE/ACM academic paper style with professional formatting, syntax-highlighted code examples, architecture diagrams, and formal mathematical notation.

---

### For Implementors (C++ Runtime Developers)

**Essential reading order:**

1. **`dryad_theoretical_foundation_v2.pdf`** — Complete specification
   - Section 5: Runtime Architecture (pages 15-18)
   - Section 6: Self-Hosting Stdlib (pages 19-21)
   - Appendix B: Syscall Reference (page 32)

2. **`intrinsics_architecture_summary.md`** — Quick reference for runtime implementation
   - Runtime size: ~500 lines of C++
   - Complete syscall catalog with signatures
   - Performance metrics (5-10x speedup vs bindings)

3. **`self_hosted_native_system.md`** — Detailed stdlib implementation guide
   - VFS architecture with pluggable backends
   - HTTP client/server in pure Dryad
   - Event loop and async I/O
   - Buffer memory management

---

### For Language Designers

**Comparing architectural approaches:**

1. **`native_functions_alternatives.md`** — 8 proposals for replacing native bindings
   - Comparative analysis
   - Implementation effort estimates
   - Performance trade-offs

2. **`dryad_theoretical_foundation_v2.pdf`** — Adopted solution (Proposal #4: Minimal Intrinsics)
   - Section 5: Intrinsics system design
   - Section 4.5: Gradual typing for optimization

---

## 🎯 Version History

### Version 2.0 (May 27, 2026) ⭐ CURRENT

**Major Rewrite:**
- Complete document restructure for academic publication quality
- C++ micro-kernel architecture specification
- Self-hosting standard library formalization
- Modern ES6-style module system (deprecates `#<module>`)
- Gradual typing roadmap for AOT optimization
- Professional IEEE/ACM-style formatting with:
  - Logo integration
  - Syntax-highlighted code examples
  - TikZ architecture diagrams
  - Formal mathematical notation
  - Color-coded sections (Dryad green theme)
  - Bibliography with 10 academic references

**Statistics:**
- 34 pages (more compact than v1)
- 418KB PDF size
- 40+ code examples with syntax highlighting
- 2 TikZ diagrams
- 8 tables
- 10 equations
- 35 formal definitions
- 15 axioms/properties

**Key Additions:**
- Section 5: Runtime Architecture (C++ micro-kernel)
- Section 6: Self-Hosting Standard Library
- Section 8: FFI and External Interface
- Section 4.3: Native types (ptr<T>, Buffer)
- Section 4.5: Gradual typing and strict mode
- Appendix A: BNF Grammar (partial)
- Appendix B: Syscall Reference (partial)

---

### Version 1.1 (May 27, 2026)

**Initial Corrections:**
- Fixed UTF-8 encoding issues (T1 font encoding + lmodern package)
- Added formal lexical disambiguation rules for `#` character
- Added "Strict Type Mode" section for future AOT optimization
- Corrected Portuguese character rendering (acentos, cedilhas)

**Document:** `dryad_theoretical_foundation.pdf` (343KB, 45 pages)

---

### Version 1.0 (Original)

**Initial theoretical specification:**
- Rust-based runtime architecture (now deprecated)
- Basic lexical, syntactic, and semantic definitions
- Legacy `#<module>` native directives
- Type annotations as documentation only
- Basic error categorization

---

## 🏗️ Architectural Evolution

### Previous Architecture (Rust) — DEPRECATED

```
┌─────────────────────────────────────┐
│  Dryad Application Code             │
├─────────────────────────────────────┤
│  Native Modules em C++/Rust         │
│  - io.read_file() wrapper manual    │
│  - http.get() wrapper manual        │
│  - crypto.sha256() wrapper manual   │
│  ... centenas de wrappers ...       │
└─────────────────────────────────────┘
```

**Problems:**
- ❌ Manual binding maintenance (C++ + Rust + Dryad)
- ❌ Cannot implement in pure Dryad
- ❌ Difficult to test (requires real I/O)
- ❌ Large runtime (~500KB)

---

### New Architecture (C++ Intrinsics) — CURRENT

```
┌─────────────────────────────────────┐
│  Standard Library (100% Dryad!)     │
│  - io.dryad                          │
│  - http.dryad                        │
│  - crypto.dryad                      │
└─────────────────────────────────────┘
         ↓ uses only
┌─────────────────────────────────────┐
│  ~50 Syscall Intrinsics (C++)       │
│  - __sys_open, __sys_read, ...      │
│  - __sys_socket, __sys_connect, ... │
│  - __epoll_wait, __kqueue, ...      │
└─────────────────────────────────────┘
```

**Benefits:**
- ✅ Self-hosting: Dryad implements Dryad
- ✅ Single codebase maintenance
- ✅ Testable with mock backends (MemoryBackend)
- ✅ Minimal runtime (~50KB — 10x smaller)
- ✅ 5-10x faster I/O (zero wrapper overhead)

---

## 🔑 Key Concepts Explained

### Intrinsics System

**What are intrinsics?**
Special functions marked with `@intrinsic("syscall.name")` that compile to direct syscall invocations with zero overhead.

**Example:**
```dryad
// Declaration in Dryad
@intrinsic("syscall.read")
extern function __sys_read(fd: i32, buf: ptr<u8>, len: usize): isize;

// Usage in stdlib
let bytesRead = __sys_read(fd, buffer.ptr, size);  // Direct syscall!
```

**Compiler behavior:**
1. Recognizes `@intrinsic` decorator
2. Generates special `INTRINSIC_SYSCALL <id>` opcode
3. VM dispatcher executes C++ syscall directly
4. Zero function call overhead (like inline assembly)

**Performance:**
- Traditional wrapper: ~20-50 instructions + heap allocations
- Intrinsic: ~3-5 instructions, zero allocations
- **Speedup: 5-10x faster**

---

### Virtual File System (VFS)

**What is VFS?**
Abstraction layer allowing pluggable filesystem backends.

**Interface:**
```dryad
export interface FileSystemBackend {
    open(path: string, mode: string): FileHandle;
    read(handle: FileHandle, size: number): Buffer;
    write(handle: FileHandle, data: Buffer): number;
    close(handle: FileHandle): void;
}
```

**Built-in Backends:**

1. **NativeBackend** — Real filesystem via syscall intrinsics
   ```dryad
   let fs = new FileSystem(new NativeBackend());
   fs.writeFile("/data/config.json", json);  // Writes to disk
   ```

2. **MemoryBackend** — In-memory filesystem (100% Dryad, zero syscalls!)
   ```dryad
   let mockfs = new FileSystem(new MemoryBackend());
   mockfs.writeFile("/virtual/test.txt", "hello");  // In memory only
   assert(mockfs.readFile("/virtual/test.txt") == "hello");
   ```

3. **HttpBackend** — Remote filesystem over HTTP
   ```dryad
   let webfs = new FileSystem(new HttpBackend("https://cdn.example.com"));
   let content = webfs.readFile("/assets/logo.png");
   ```

**Testability Benefit:**
```dryad
// Production code
function processData(fs: FileSystem) {
    let data = fs.readFile("/data/input.csv");
    // ... processing ...
    fs.writeFile("/data/output.json", result);
}

// Unit test (no disk I/O!)
let mockfs = new FileSystem(new MemoryBackend());
mockfs.writeFile("/data/input.csv", "test,data,here");
processData(mockfs);
assert(mockfs.readFile("/data/output.json") == expectedOutput);
```

---

### Gradual Typing

**Current: Dynamic Mode (default)**
```dryad
let x: number = 42;  // Annotation for documentation
function add(a: number, b: number): number {
    return a + b;  // Runtime type checks on every call
}
```

**Future: Strict Mode (opt-in)**
```dryad
"use strict types";

function add(a: number, b: number): number {
    return a + b;  // Compiler GUARANTEES types, no runtime checks!
}

// Compiles to efficient machine code:
// addsd xmm0, xmm1   ; Direct SSE floating-point add
// ret
```

**Performance Impact:**
- Dynamic mode: 100 ns per call (type checks + boxing)
- Strict mode: 5 ns per call (direct CPU instruction)
- **Speedup: 20x faster**

**Interoperability:**
```dryad
"use strict types";

@strict
function strictAdd(a: number, b: number): number {
    return a + b;
}

@dynamic
function flexibleProcess(x: any): any {
    return strictAdd(x as number, 10);  // Type check at boundary
}
```

---

## 🧭 Navigation Guide

### By Use Case

| **I want to...** | **Read this...** |
|------------------|------------------|
| Understand Dryad language design | `dryad_theoretical_foundation_v2.pdf` (full spec) |
| Implement C++ runtime | Sections 5-6 of v2.pdf + `intrinsics_architecture_summary.md` |
| Write Dryad standard library | `self_hosted_native_system.md` + Section 6 of v2.pdf |
| Understand module system | Section 3 of v2.pdf (Modern Module System) |
| Learn about concurrency | Section 9 of v2.pdf (Concurrency and Parallelism) |
| See FFI examples | Section 8 of v2.pdf (FFI and External Interface) |
| Compare architecture approaches | `native_functions_alternatives.md` |
| Review what changed in v2 | `V2_REVISION_SUMMARY.md` |

---

### By Section (v2.0 PDF)

| **Section** | **Page** | **Topic** |
|-------------|----------|-----------|
| 1 | 3 | Introduction and Language Philosophy |
| 2 | 6 | Lexical Structure |
| 3 | 9 | The Modern Module System |
| 4 | 12 | Type System and Gradual Typing |
| 5 | 15 | Runtime Architecture: C++ Micro-Kernel |
| 6 | 19 | Self-Hosting Standard Library |
| 7 | 22 | FFI and External Function Interface |
| 8 | 24 | Concurrency and Parallelism |
| 9 | 27 | Compilation Pipeline and Execution Modes |
| 10 | 30 | Error Handling and Diagnostics |
| 11 | 31 | Future Roadmap |
| 12 | 32 | Conclusion |
| Appendix A | 33 | Complete BNF Grammar (partial) |
| Appendix B | 34 | Syscall Reference (partial) |

---

## 📊 Quick Reference Tables

### Syscall Categories (~50 total)

| **Category** | **Count** | **Examples** |
|--------------|-----------|--------------|
| File I/O | 8 | `open, read, write, close, lseek, stat, unlink, mkdir` |
| Network | 8 | `socket, connect, bind, listen, accept, send, recv, shutdown` |
| Memory | 5 | `malloc, free, realloc, memcpy, memset` |
| Async I/O | 6 | `epoll_create, epoll_ctl, epoll_wait, kqueue, kevent, select` |
| Process/Thread | 6 | `fork, exec, wait, pthread_create, pthread_join, pthread_detach` |
| Time | 4 | `gettimeofday, clock_gettime, sleep, nanosleep` |
| Environment | 5 | `getenv, setenv, getcwd, chdir, getpid` |
| Atomic Ops | 5 | `atomic_load, atomic_store, atomic_compare_exchange, atomic_fetch_add, memory_fence` |

---

### Execution Modes

| **Mode** | **Startup** | **Runtime Speed** | **Memory Usage** | **Use Case** |
|----------|-------------|-------------------|------------------|--------------|
| Interpreter | Fast | Slow (baseline) | High | Development, debugging |
| Bytecode VM | Medium | 5-10x faster | Medium | Production scripts |
| AOT Native | Slow | 20-50x faster | Low | High-performance services |
| AOT + Strict Types | Slow | 100x+ faster | Low | Performance-critical code |

---

### Error Code Categories

| **Category** | **Range** | **Examples** |
|--------------|-----------|--------------|
| Lexical | 1000-1999 | Unexpected character, unterminated string |
| Parser | 2000-2999 | Unexpected token, invalid syntax |
| Runtime | 3000-3999 | Undefined variable, division by zero |
| Type | 4000-4999 | Type mismatch, invalid conversion |
| I/O | 5000-5999 | File not found, permission denied |
| Module | 6000-6999 | Unknown module, circular import |
| Syntax | 7000-7999 | Structural syntax errors |
| Warning | 8000-8999 | Unused variable, deprecated function |
| System | 9000-9999 | Out of memory, stack overflow |

---

## 🚀 Implementation Roadmap

### Phase 1: C++ Runtime (1-2 weeks)
- [ ] Implement ~50 syscall intrinsics
- [ ] VM dispatcher with `INTRINSIC_SYSCALL` opcode
- [ ] `@intrinsic` decorator support in parser
- [ ] Basic memory management (shared_ptr)

### Phase 2: Core Stdlib (2-3 weeks)
- [ ] `@std/runtime/intrinsics` — Low-level syscall declarations
- [ ] `@std/buffer` — Safe Buffer class
- [ ] `@std/io` — File I/O with VFS
- [ ] `@std/net` — TCP/UDP sockets
- [ ] `@std/async` — Event loop

### Phase 3: High-Level Stdlib (2-3 weeks)
- [ ] `@std/http` — HTTP client/server
- [ ] `@std/json` — JSON parser/serializer
- [ ] `@std/crypto` — Cryptographic primitives
- [ ] `@std/encoding` — Base64, UTF-8, etc.
- [ ] `@std/time` — Date/time operations

### Phase 4: Advanced Features (1-2 months)
- [ ] Generational garbage collector
- [ ] Bytecode VM with JIT
- [ ] Strict type mode for AOT
- [ ] Actor model for concurrency
- [ ] LSP server for editor integration

---

## 🔗 External Resources

### Official Documentation
- **GitHub Repository**: https://github.com/dryad-lang/source
- **Language Specification**: This directory (`dryad_theory/`)
- **Issue Tracker**: https://github.com/dryad-lang/source/issues

### Related Papers & Inspirations
- **Go syscall package**: https://pkg.go.dev/syscall
- **Zig std.os**: https://ziglang.org/documentation/master/std/#std.os
- **Rust core::intrinsics**: https://doc.rust-lang.org/core/intrinsics/
- **Lua 5.0 Implementation**: Ierusalimschy et al., 2005
- **Gradual Typing**: Siek & Taha, 2006

---

## 📝 Contributing

### Reporting Issues

**Found an error or ambiguity in the specification?**

1. Check if already reported: [GitHub Issues](https://github.com/dryad-lang/source/issues)
2. Create new issue with:
   - Document version (v2.0)
   - Section number and page
   - Description of error/ambiguity
   - Suggested correction

### Proposing Improvements

**Have an idea for improving the language design?**

1. Read existing proposals in `native_functions_alternatives.md`
2. Create new proposal document following template:
   ```markdown
   # Proposal: [Title]
   
   ## Problem Statement
   ## Proposed Solution
   ## Implementation Complexity
   ## Performance Impact
   ## Comparison with Alternatives
   ```
3. Submit pull request to `dryad_theory/proposals/`

---

## 📄 License

All documents in this directory are licensed under **CC BY 4.0** (Creative Commons Attribution 4.0 International).

**You are free to:**
- Share — copy and redistribute the material in any medium or format
- Adapt — remix, transform, and build upon the material for any purpose

**Under the following terms:**
- Attribution — You must give appropriate credit, provide a link to the license, and indicate if changes were made

See: https://creativecommons.org/licenses/by/4.0/

---

## 📞 Contact

**Dryad Development Team**
- Email: [to be added]
- Discord: [to be added]
- GitHub: https://github.com/dryad-lang

---

## ✨ Acknowledgments

This specification was developed with inspiration from:

- **Go** — Minimal syscall-based runtime
- **Zig** — Explicit memory management and compile-time evaluation
- **Rust** — Type safety and zero-cost abstractions
- **TypeScript** — Gradual typing and developer experience
- **Lua** — Clean language design and efficient VM
- **Erlang** — Actor model and fault tolerance

Special thanks to the open-source community for pioneering modern language design patterns.

---

**Last Updated**: May 27, 2026  
**Document Version**: 2.0  
**Specification Status**: Living Document — Subject to refinement during C++ implementation
