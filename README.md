# Dryad

A modern dynamic programming language with a self-hosting standard library and a minimal C++ runtime.

Dryad keeps the runtime small by exposing a tiny set of intrinsic syscalls (~50) that the standard library — written 100% in Dryad — builds on. No manual bindings, no wrapper layers: the language implements itself.

## Status

Early implementation. The C++ interpreter (Phase 0–3) is functional, the standard library is growing (Buffer plus VFS I/O), and the test suite is **223/223 passing** on Linux. The CLI runs `.dryad` scripts with `import` support (`dryad script.dryad`); REPL is not implemented yet. See [CHANGELOG](CHANGELOG) and the phase reports under `src/` for details.

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

The project is split into two repositories, assembled as modules:

| Path | Repository | Contents |
|------|------------|----------|
| `src/` | source (this repo) | C++ interpreter, standard library, tests |
| `docs/` | [dryadlang/docs](https://github.com/dryadlang/docs) | Documentation, examples, language specification |
| `CHANGELOG` | source | Notable changes per release |

`docs/` is registered as a git submodule, so a full checkout is:

```bash
git clone --recurse-submodules https://gitlab.com/dryadlang/source.git
```

## Building

Requirements: C++17 compiler (MSVC or GCC/Clang with MSYS2), CMake 3.15+.

```bash
cmake -S src -B src/build -DCMAKE_BUILD_TYPE=Release
cmake --build src/build
```

The `dryad` binary is produced at `src/build/bin/dryad`.

## Testing

```bash
cd src/build
ctest --output-on-failure
```

Or run a single test binary directly:

```bash
./bin/unit_tests         # unit tests (lexer, parser, interpreter, intrinsics, buffers, ...)
./bin/integration_tests  # end-to-end workflows (buffer roundtrips, file I/O, example scripts)
```

Tests are built with `-Wall -Wextra -Werror`.

## Example

`docs/examples/02-buffers/buffer_usage.dryad` — UTF-8 string ↔ buffer conversion, slicing, and byte editing:

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

Run examples with the CLI:

```bash
src/build/bin/dryad docs/examples/02-buffers/buffer_usage.dryad
src/build/bin/dryad docs/examples/04-io/file_io_usage.dryad
src/build/bin/dryad docs/examples/04-io/io_examples.dryad

# Run all examples with timing
./docs/examples/run_benchmark.sh
```

Set `DRYAD_STDLIB_DIR` if the stdlib is not at the default install path.

## Documentation

All documentation is consolidated under [`docs/`](docs/README.md). Full inventory: [`docs/SITEMAP.md`](docs/SITEMAP.md).

| Section | Path |
|---------|------|
| Quick start | [docs/getting-started/quick-start.md](docs/getting-started/quick-start.md) |
| Language reference | [docs/language/](docs/language/README.md) |
| Specification (PDF) | [docs/specification/theory/](docs/specification/theory/) |
| Standard library | [docs/stdlib/](docs/stdlib/README.md) |
| User libraries | [docs/libraries/](docs/libraries/README.md) |
| CLI & tooling | [docs/tooling/](docs/tooling/README.md) |
| Implementation | [docs/implementation/](docs/implementation/README.md) |
| Diagrams (PlantUML) | [docs/diagrams/](docs/language/diagrams.md) |
| Examples | [docs/examples/README.md](docs/examples/README.md) |

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). In short: tests come first (TDD), commits are small and focused, and the whole suite must stay green. Documentation contributions go to the [docs module repository](https://github.com/dryadlang/docs).

## License

[MIT](LICENSE) — Copyright (c) 2026 Pedro Kaleb.
