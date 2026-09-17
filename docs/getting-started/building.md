# Building Dryad

## Requirements

- C++17 compiler (GCC, Clang, or MSVC)
- CMake 3.15+
- Optional: OpenSSL (TLS), libcurl (HTTP client)

## Build

```bash
cd dryad-cpp
cmake -B build -DCMAKE_BUILD_TYPE=Release -DDRYAD_ENABLE_TLS=ON
cmake --build build -j$(nproc)
```

The `dryad` binary is at `build/bin/dryad`.

## CLI

```
dryad [--allow-net] [--vm] <script.dryad>     Run a script
dryad compile <file> [-o out] [--aot] [--link] [--executable] [--entry fn] [--no-profile]

`--link` produces a native `.so`; `--executable` produces a runnable ELF binary.
Both populate `AotCompileResult::binary` in the API. Companion artifacts:
`script.dryadc` (bytecode) and `script.so` (native) auto-load with
`script.dryad` when using `@compile("AOT")`.
dryad analyze <file> [--no-profile]           Fast/slow path report
dryad check <file>                            Validate syntax only
```

## Environment

| Variable | Purpose |
|----------|---------|
| `DRYAD_STDLIB_DIR` | Override stdlib path (default: `dryad-cpp/stdlib`) |
| `DRYAD_ALLOW_NET` | Set to `1` to enable network syscalls |

## Testing

```bash
cd dryad-cpp/build
ctest --output-on-failure
./unit_tests          # 408 unit tests (+1 JIT slow test)
./integration_tests   # end-to-end workflows
```

## Examples & benchmark

```bash
cd dryad-cpp
./examples/run_benchmark.sh
```

See [examples catalog](../examples/README.md) and [dryad-cpp/examples/README.md](../../dryad-cpp/examples/README.md) for the full list.
