# CLI Reference

The `dryad` binary provides script execution, compilation, analysis, and package management.

**Binary location:** `dryad-cpp/build/bin/dryad`

## Run

```
dryad [--allow-net] [--vm] <script.dryad|.dyd>
```

| Flag | Effect |
|------|--------|
| `--vm` | Use bytecode VM instead of interpreter |
| `--allow-net` | Enable network syscalls (sockets, DNS, HTTP) |

## Check

```
dryad check <file>
```

Validates syntax and reports diagnostics. Does not execute.

## Analyze

```
dryad analyze <file> [--no-profile]
```

Reports fast-path vs slow-path analysis for AOT compilation.

## Compile

```
dryad compile <file> [options]
```

| Option | Effect |
|--------|--------|
| `-o <path>` | Output path |
| `--aot` | Emit LLVM IR (`.ll`) |
| `--link` | Link native shared object (`.so`) |
| `--executable` | Link native executable |
| `--entry <fn>` | Entry function name (default: first function or `__dryad_main`) |
| `--no-profile` | Skip interpreter profiling pass |
| `--lib` | Compile as library → `.dyl` |
| `--native` | With `--lib`: also emit native `.so` |
| `--link-lib <name>` | Link user library into AOT output |

### Examples

```bash
# Bytecode artifact
dryad compile app.dryad -o app.dryadc

# LLVM IR
dryad compile app.dryad --aot -o app.ll

# Native shared library
dryad compile app.dryad --aot --link -o app.so

# Native executable
dryad compile app.dryad --aot --executable -o app

# User library
dryad compile --lib libs/@my/utils.dyd -o libs/@my/utils.dyl
dryad compile --lib --native libs/@my/utils.dyd -o libs/@my/utils.so
```

## Install

```
dryad install [--target <dir>]
```

Reads `dryad.toml` and installs dependencies.

## Link

```
dryad link --shared <input> -o <output.so>
```

Links a compiled library to a native shared object.

## Environment variables

| Variable | Purpose |
|----------|---------|
| `DRYAD_STDLIB_DIR` | Override stdlib path |
| `DRYAD_LIBRARY_PATH` | Colon-separated library search paths |
| `DRYAD_ALLOW_NET` | Set to `1` to enable network (alternative to `--allow-net`) |

## Companion artifacts

When running AOT-compiled scripts, the runtime auto-loads:

- `<script>.dryadc` — bytecode fallback
- `<script>.so` — native module

## See also

- [Building guide](../getting-started/building.md)
- [Execution modes](../language/execution-modes.md)
