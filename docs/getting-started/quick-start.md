# Quick Start

Get Dryad running in a few minutes.

## 1. Build

```bash
cd dryad-cpp
cmake -B build -DCMAKE_BUILD_TYPE=Release -DDRYAD_ENABLE_TLS=ON
cmake --build build -j$(nproc)
```

Binary: `dryad-cpp/build/bin/dryad`

See [building.md](building.md) for options (sanitizers, TLS, curl).

## 2. Run a script

```bash
export DRYAD_STDLIB_DIR=dryad-cpp/stdlib

./build/bin/dryad dryad-cpp/examples/01-language/basics.dryad
```

## 3. Execution modes

```bash
# Interpreter (default)
./build/bin/dryad examples/01-language/basics.dryad

# Bytecode VM
./build/bin/dryad --vm examples/01-language/basics.dryad

# AOT compile to LLVM IR
./build/bin/dryad compile examples/01-language/basics.dryad --aot -o /tmp/basics.ll
```

See [execution modes](../language/execution-modes.md) for `@compile()` and when to use each tier.

## 4. Validate syntax

```bash
./build/bin/dryad check examples/01-language/basics.dryad
```

## 5. Run all examples

```bash
cd dryad-cpp
./examples/run_benchmark.sh

# Triple coverage: interpreter + VM + AOT
./examples/run_benchmark.sh --triple
```

## 6. Run tests

```bash
cd dryad-cpp/build
ctest --output-on-failure
```

## 7. Your first program

Create `hello.dryad`:

```dryad
function greet(name) {
    return "Hello, " + name + "!";
}

print(greet("Dryad"));
```

```bash
./build/bin/dryad hello.dryad
```

## Next steps

| Topic | Guide |
|-------|-------|
| Language syntax | [language/syntax.md](../language/syntax.md) |
| Imports & modules | [language/modules.md](../language/modules.md) |
| Standard library | [stdlib/README.md](../stdlib/README.md) |
| User libraries | [libraries/README.md](../libraries/README.md) |
| CLI reference | [tooling/cli.md](../tooling/cli.md) |
| Full spec (PDF) | [specification/README.md](../specification/README.md) |
