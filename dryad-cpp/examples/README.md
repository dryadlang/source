# Dryad Code Examples

Runnable examples for every major language and stdlib feature. Use `run_benchmark.sh` to compile and test each one with timing.

## Quick start

```bash
cd dryad-cpp
cmake -B build && cmake --build build -j$(nproc)
./examples/run_benchmark.sh
```

Options:

| Flag | Effect |
|------|--------|
| `--verbose` | Show script output on success |
| `--triple` | Run each example in interpreter, VM, and AOT compile modes |
| `--include-net` | Run TCP client (needs echo server on `:9090`) |
| `--include-optional` | Same as `--include-net` for optional examples |

Triple coverage gate (phase 6.15): all examples must pass interpreter, `--vm`, and `compile --aot`. Track progress with `./examples/run_benchmark.sh --triple`.

## Catalog

### 01 — Language

| File | Feature |
|------|---------|
| `basics.dryad` | Literals, arithmetic, comparison, logical |
| `control_flow.dryad` | if/else, while, for-in |
| `functions.dryad` | Functions, closures, recursion |
| `classes.dryad` | Classes, inheritance, methods |
| `compile_decorator.dryad` | `@compile("bytecode")` decorator |

### 02 — Buffers

| File | Feature |
|------|---------|
| `buffer_usage.dryad` | UTF-8 ↔ buffer, slice, byte write |

### 03 — Collections

| File | Feature |
|------|---------|
| `collections_usage.dryad` | List, Map, Set, Queue |

### 04 — I/O

| File | Feature |
|------|---------|
| `io_examples.dryad` | VFS, file API, FileHandle |
| `file_io_usage.dryad` | NativeBackend filesystem |
| `stream_usage.dryad` | MemoryStream, FileStream |
| `paths.dryad` | Path join, dirname, basename |

### 05 — Async

| File | Feature |
|------|---------|
| `promises.dryad` | Promise resolve/reject/then/all/race |
| `async_await.dryad` | async/await, microtask ordering |
| `threading.dryad` | Thread, ThreadPool, parallel |

### 06 — Networking

| File | Feature |
|------|---------|
| `http_parse.dryad` | HTTP URL/response parsing (no network) |
| `tcp_client.dryad` | TCP echo client (`--allow-net`, optional) |

### DryadSquare — Turing benchmark

| File | Feature |
|------|---------|
| `dryadsquare/dryadsquare.dryad` | Mini-compiler + VM (100% Dryad), golden tests |
| `dryadsquare/run_turing_benchmark.sh` | Interpreter + VM + AOT gate |

See [dryadsquare/README.md](dryadsquare/README.md) and [dryadsquare/SPEC.md](dryadsquare/SPEC.md).

### 07 — Tooling (via benchmark script)

| Command | Feature |
|---------|---------|
| `dryad check` | Syntax validation |
| `dryad compile -o .dryadc` | Bytecode compilation |
| `dryad analyze` | Fast/slow path report |
| `dryad compile --aot -o .ll` | LLVM IR generation |

## Run individual examples

```bash
export DRYAD_STDLIB_DIR=dryad-cpp/stdlib
./build/bin/dryad examples/01-language/basics.dryad
./build/bin/dryad examples/05-async/async_await.dryad
./build/bin/dryad --allow-net examples/06-net/tcp_client.dryad
```

## Documentation

Full docs index: [docs/README.md](../../docs/README.md)
