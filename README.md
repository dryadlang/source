# Dryad

A modern dynamic programming language with a self-hosting standard library and a minimal C++ runtime.

Dryad keeps the runtime small by exposing a tiny set of intrinsic syscalls (~50) that the standard library — written 100% in Dryad — builds on. No manual bindings, no wrapper layers: the language implements itself.

## Status

Early implementation. The C++ interpreter (Phase 0–3) is functional, the standard library is growing, and the test suite is at **171/178 passing**. The CLI does not execute scripts or a REPL yet — scripts are validated through the test suite. See [CHANGELOG](CHANGELOG) and the phase reports under `dryad-cpp/` for details.

## Architecture

```
Layer 4: Application Code (user Dryad programs)
   ↓
Layer 3: Standard Library (100% Dryad)
   @std/core, @std/io, @std/buffers
   ↓
Layer 2: Intrinsics Layer (~50 syscall primitives)
   file I/O, memory, strings, process, network
   ↓
Layer 1: C++ Micro-Kernel Runtime
   interpreter, GC, module loader, environment
```

- **Self-hosting**: the standard library is written in Dryad and only talks to the runtime through intrinsics.
- **Minimal runtime**: ~50 intrinsic syscalls instead of hundreds of hand-maintained native bindings.
- **Error model**: stdlib errors are values (strings from `@std/core/exceptions`), never exceptions thrown across the language boundary.

## Repository layout

| Path | Contents |
|------|----------|
| `dryad_theory/` | Language specification (PDF + LaTeX) and architecture design documents |
| `dryad-cpp/` | C++ interpreter implementation, standard library, tests, examples |
| `docs/` | Supporting documentation |
| `archived/` | Superseded material |
| `CHANGELOG` | Notable changes per release |

## Building

Requirements: C++17 compiler (MSVC or GCC/Clang with MSYS2), CMake 3.15+.

```bash
mkdir build && cd build
cmake -DCMAKE_BUILD_TYPE=Release ..
cmake --build .
```

The `dryad` binary is produced at `build/bin/dryad`.

## Testing

```bash
cd build
ctest --output-on-failure
```

Or run a single test binary directly:

```bash
./bin/unit_tests         # unit tests (lexer, parser, interpreter, intrinsics, buffers, ...)
./bin/integration_tests  # end-to-end workflows (buffer roundtrips, file I/O, example scripts)
```

Known pre-existing failures (7) are documented in [CHANGELOG](CHANGELOG): class `this`-binding, `/tmp` paths on Windows, and stdout capture on MSYS2 consoles. Tests are built with `-Wall -Wextra -Werror`.

## Example

`dryad-cpp/examples/buffer_usage.dryad` — UTF-8 string ↔ buffer conversion, slicing, and byte editing:

```dryad
let text = "Olá, mundo!";
let buf = Buffer_FromString(text);
let ok = Buffer_AsString(buf) == text;

let slice = Buffer_Slice(buf, 0, 4);
ok = ok && (Buffer_AsString(slice) == "Olá");
Buffer_Dispose(slice);

Buffer_Write(buf, 0, 88);              // 'X'
ok = ok && (Buffer_AsString(buf) == "Xlá, mundo!");

Buffer_Dispose(buf);
```

The CLI does not execute scripts yet; this example is validated by `tests/integration/buffer_workflow_test.cpp`.

## Documentation

- **Specification**: `dryad_theory/dryad_theoretical_foundation_v2.pdf` (canonical, v2.0)
- **Theory index**: `dryad_theory/README.md`
- **Implementation plans**: `dryad-cpp/STDLIB_ARCHITECTURE.md`, `dryad-cpp/STDLIB_IMPLEMENTATION_PLAN.md`
- **Phase reports**: `dryad-cpp/PHASE_0_COMPLETE.md` … `dryad-cpp/PHASE_4_PROGRESS_REPORT.md`

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: tests come first (TDD), commits are small and focused, and the whole suite must stay green.

## License

[MIT](LICENSE) — Copyright (c) 2026 Pedro Kaleb.
