# Dryad — C++ Implementation

Modern dynamic programming language with self-hosting standard library and minimal runtime.

## Quick start

```bash
cmake -B build -DCMAKE_BUILD_TYPE=Release -DDRYAD_ENABLE_TLS=ON
cmake --build build -j$(nproc)

# Run a script
./build/bin/dryad ../docs/examples/01-language/basics.dryad

# Run all examples (benchmark)
../docs/examples/run_benchmark.sh

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
| **Docs hub** | [docs/README.md](../docs/README.md) |
| Quick start | [docs/getting-started/quick-start.md](../docs/getting-started/quick-start.md) |
| Language reference | [docs/language/](../docs/language/README.md) |
| Standard library | [docs/stdlib/](../docs/stdlib/README.md) |
| User libraries | [docs/libraries/](../docs/libraries/README.md) |
| CLI reference | [docs/tooling/cli.md](../docs/tooling/cli.md) |
| Implementation | [docs/implementation/](../docs/implementation/README.md) |
| Diagrams (PlantUML) | [docs/language/diagrams.md](../docs/language/diagrams.md) |
| Full sitemap | [docs/SITEMAP.md](../docs/SITEMAP.md) |
| Examples | [docs/examples/README.md](../docs/examples/README.md) |
| Language spec (PDF) | [docs/specification/theory/](../docs/specification/theory/) |

## Project layout

```
src/
├── src/           # Compiler, runtime, AOT
├── include/       # Public headers
├── stdlib/@std/   # Standard library (100% Dryad)
├── tests/         # Unit + integration + stdlib tests
└── build/         # CMake output (dryad binary at build/bin/)
```

Runnable examples and the full documentation live in the `docs/` module repository (`../docs/`).

## License

MIT — see [LICENSE](../LICENSE)
