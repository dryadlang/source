# Dryad Programming Language - C++ Implementation

Modern dynamic programming language with self-hosting standard library and minimal runtime.

## Features

- **Clean Syntax**: Python-like readability with modern ES6-style modules
- **Self-Hosting**: 100% of standard library written in Dryad
- **Minimal Runtime**: ~50 intrinsic syscalls instead of manual bindings
- **Fast I/O**: 5-10x faster than traditional binding approaches
- **Gradual Typing**: Optional type annotations for AOT optimization
- **Async-First**: Event loop and async I/O in pure Dryad

## Quick Start

```bash
# Build
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build .

# Run REPL
./bin/dryad

# Run a script
./bin/dryad script.dry

# Run tests
ctest --output-on-failure
```

## Architecture

```
Layer 4: Application Code (User Dryad Programs)
   ↓
Layer 3: Standard Library (100% Dryad)
   @std/io, @std/http, @std/crypto, @std/net
   ↓
Layer 2: Intrinsics Layer (~50 Syscall Primitives)
   File I/O, Network, Memory, Async I/O
   ↓
Layer 1: C++ Micro-Kernel Runtime
   VM Core, GC, Module Loader, Bytecode, JIT/AOT
```

## Documentation

- **Theory**: See `/home/pedro/repo/source/dryad_theory/dryad_theoretical_foundation_v2.pdf`
- **Implementation Plan**: See `/home/pedro/repo/source/dryad_theory/REWORK_OVERVIEW.md`
- **Quick Start**: See `/home/pedro/repo/source/dryad_theory/QUICK_START.md`

## Development

- **Language**: C++17 or later
- **Build System**: CMake 3.15+
- **Testing**: Google Test
- **Coverage**: >90% target
- **Discipline**: TDD enforced (tests before implementation)

## Project Status

🚧 **Phase 0: Foundation (Week 1)** - In Progress

See task tracking in `/home/pedro/repo/source/dryad_theory/TASK_CHECKLIST.md`

## License

MIT License - See LICENSE file
