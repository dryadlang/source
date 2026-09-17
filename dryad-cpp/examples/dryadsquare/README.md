# DryadSquare — Turing-completeness benchmark

DryadSquare is a **mini compiler + virtual machine** written entirely in Dryad. It compiles the
**Square** language (see [SPEC.md](SPEC.md)) to stack bytecode and executes it.

## Why this exists

| Claim | Evidence |
|-------|----------|
| Square is Turing-complete | Unbounded variables + `while` + arithmetic (see SPEC) |
| Dryad can implement a compiler | Lexer, parser, code generator, VM — all `.dryad` |
| No host-language cheating | Zero C++ changes for this benchmark; stdlib imports only |

Self-hosting (Square compiling Square) is **phase 3+** on the roadmap — not required for the
Turing argument, but required before claiming “Dryad compiled Dryad end-to-end”.

## Rules

1. **Dryad only** — compiler, runtime, tests, docs.
2. **No language extensions** — if Square needs a feature, add it to Square, not to Dryad’s C++ core.
3. **Honest scope** — this proves computational universality + compiler expressiveness, not production readiness.

## Layout

```
dryadsquare/
├── SPEC.md              # Square language + bytecode spec
├── dryadsquare.dryad    # Golden tests (prints TURING_BENCHMARK_OK)
├── run_turing_benchmark.sh
├── programs/            # Example Square sources
└── square/              # Compiler implementation (100% Dryad)
    ├── string.dryad     # Character / substring helpers
    ├── token.dryad      # Lexer
    ├── parse.dryad      # Parser → AST
    ├── compiler.dryad   # AST → bytecode
    ├── vm.dryad         # Bytecode interpreter
    └── runtime.dryad    # compile + run pipeline
```

## Run

```bash
cd dryad-cpp
cmake -B build && cmake --build build -j$(nproc)
export DRYAD_STDLIB_DIR=stdlib

# Golden tests
./build/bin/dryad examples/dryadsquare/dryadsquare.dryad

# Full benchmark (interpreter + VM + AOT compile gate)
./examples/dryadsquare/run_turing_benchmark.sh
```

Success: output contains `TURING_BENCHMARK_OK`.

## Triple-mode gate

`run_turing_benchmark.sh` runs `dryadsquare.dryad` in:

- Interpreter (default)
- Bytecode VM (`--vm`)
- AOT compile (`compile --aot`)

All three must pass for the benchmark to be green.

## Roadmap

- [x] Square spec + Turing argument
- [x] Lexer / parser / compiler / VM in Dryad
- [x] Golden arithmetic + loop + Collatz tests
- [ ] Load `.sq` files from disk (VFS) instead of embedded strings
- [ ] Square functions + calls
- [ ] Self-host: compile `square/compiler.dryad` with DryadSquare
