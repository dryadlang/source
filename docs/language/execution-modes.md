# Execution Modes

Dryad supports three execution tiers for the same language semantics. This is the **triple execution model**.

## Modes

| Mode | CLI / Decorator | Artifact | Characteristics |
|------|-----------------|----------|-----------------|
| **Interpreter** | default / `@compile("interpreter")` | — | Fastest startup, tree-walk evaluation |
| **Bytecode VM** | `--vm` / `@compile("bytecode")` | `.dryadc` | Profile-guided opcodes, ~10× interpreter |
| **AOT native** | `compile --aot` / `@compile("AOT")` | `.ll` / `.so` | LLVM IR → native code, lowest runtime overhead |

## Selecting a mode

### Per-function decorator

```dryad
@compile("bytecode")
function hotLoop(n) {
    let sum = 0;
    for (let i = 0; i < n; i = i + 1) {
        sum = sum + i;
    }
    return sum;
}
```

### Compile block

```dryad
@compile("AOT") {
    function entry() { return compute(); }
}
```

### CLI default

```bash
dryad script.dryad          # interpreter
dryad --vm script.dryad     # bytecode VM
```

## How it works

1. **Interpreter** runs first (optionally) to profile types via `TypeProfiler`
2. **BytecodeCompiler** emits specialized opcodes (`ADD_I32`, `GET_ELEMENT_F64`, …)
3. **AOT** lowers bytecode → Micro-IR → LLVM IR → native `.so`

Type observations flow: interpreter → bytecode → AOT.

## Companion artifacts

When you run `script.dryad` with `@compile("AOT")`, the runtime auto-loads:

- `script.dryadc` — bytecode fallback
- `script.so` — native module via `dlopen`

## Async functions

`async function` bodies always execute on the **interpreter** path (event loop integration).

## Script entry quirk (VM)

Scripts with only top-level statements are wrapped as `__dryad_main` for VM execution. Top-level `let` bindings are **not** visible in `global_env` after VM script entry — wrap verification logic in a named function.

## Triple coverage gate

All examples must pass interpreter, `--vm`, and `compile --aot`:

```bash
cd dryad-cpp
./examples/run_benchmark.sh --triple
```

## See also

- [Phase 6 report](../implementation/phases/PHASE_6_TRIPLE_MODEL.md)
- [Triple execution plan](../implementation/plans/2026-09-16-triple-execution-model.md)
- [CLI compile options](../tooling/cli.md)
