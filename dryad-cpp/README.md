# Dryad — C++ Implementation

Modern dynamic programming language with self-hosting standard library and minimal runtime.

## Quick start

```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release -DDRYAD_ENABLE_TLS=ON
cmake --build build -j$(nproc)

# Run a script
./build/bin/dryad examples/01-language/basics.dryad

# Run all examples (benchmark)
./examples/run_benchmark.sh

# Run tests
cd build && ctest --output-on-failure
```

## CLI

```
dryad [--allow-net] [--vm] <script.dryad>
dryad compile <file> [-o out.dryadc] [--aot -o out.ll]
dryad analyze <file>
dryad check <file>
```

## Documentation

| Topic | Path |
|-------|------|
| Docs index | [docs/README.md](../docs/README.md) |
| Build guide | [docs/getting-started/building.md](../docs/getting-started/building.md) |
| Language spec | [dryad_theory/](../dryad_theory/) |
| Examples | [examples/README.md](examples/README.md) |
| Stdlib guides | [docs/stdlib/](../docs/stdlib/) |

## Project layout

```
dryad-cpp/
├── src/           # Compiler, runtime, AOT
├── include/       # Public headers
├── stdlib/@std/   # Standard library (100% Dryad)
├── tests/         # Unit + integration + stdlib tests
├── examples/      # Runnable feature examples + benchmark
└── build/         # CMake output (dryad binary at build/bin/)
```

## License

MIT — see [LICENSE](../LICENSE)
