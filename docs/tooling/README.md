# Tooling

CLI commands and developer tools for the Dryad compiler.

| Guide | Description |
|-------|-------------|
| [CLI reference](cli.md) | All `dryad` commands and flags |
| [dryad.toml](../implementation/tooling/DRYAD_TOML.md) | Package manifest format |
| [Examples](../examples/README.md) | Runnable example catalog + benchmark |

## Quick reference

```bash
dryad script.dryad              # Run (interpreter)
dryad --vm script.dryad         # Run (bytecode VM)
dryad check script.dryad        # Syntax validation
dryad analyze script.dryad      # Fast/slow path report
dryad compile script.dryad --aot -o out.ll
dryad compile --lib mod.dyd -o mod.dyl
dryad install                   # Install package dependencies
```

## Benchmark

```bash
cd dryad-cpp
./examples/run_benchmark.sh           # All examples
./examples/run_benchmark.sh --triple  # Interpreter + VM + AOT
```
