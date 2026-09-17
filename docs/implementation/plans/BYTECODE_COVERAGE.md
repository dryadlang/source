# Bytecode Coverage Matrix

> **Status:** Complete for bytecode-eligible language subset (phase 6.12)  
> **Tests:** 403+ unit tests (see [6.13 stdlib via intrinsics](2026-09-17-stdlib-via-intrinsics.md))

## Opcode inventory (50 opcodes)

| Opcode | VM | Compiler | AOT fast-path | Notes |
|--------|:--:|:--------:|:-------------:|-------|
| `PUSH_NULL/TRUE/FALSE/INT/FLOAT/CONST` | ✅ | ✅ | ✅ const | |
| `POP`, `DUP`, `SWAP` | ✅ | ✅ partial | — | `SWAP` VM-only |
| `ADD`…`MOD` (generic) | ✅ | ✅ | slow | |
| `ADD/SUB/MUL/DIV_I32` | ✅ | ✅ | ✅ | typed i32 |
| `ADD/SUB/MUL/DIV_F64` | ✅ | ✅ | ✅ | typed f64 |
| `NOT` | ✅ | ✅ | slow | unary `!` |
| `EQ`…`GTE` | ✅ | ✅ | partial | |
| `JUMP`, `JUMP_IF_FALSE` | ✅ | ✅ | — | control flow |
| `JUMP_IF_TRUE` | ✅ | ✅ | — | `\|\|` short-circuit |
| `HALT`, `CALL`, `RET` | ✅ | ✅ | CALL slow | |
| `LOAD_FAST`, `STORE_FAST` | ✅ | ✅ | ✅ | |
| `NEW_ARRAY`…`NEW_INSTANCE` | ✅ | ✅ | slow | arrays + classes |
| `CALL_GLOBAL` | ✅ | ✅ | slow | stdlib / imported functions |
| `GET_ELEMENT`, `GET_ELEMENT_F64` | ✅ | ✅ | slow / f64 | |
| `INTRINSIC_SYSCALL` | ✅ | ✅ | slow | `__syscall`, `__runtime_*` |

## Language constructs

| Construct | Interpreter | Bytecode | Notes |
|-----------|:-----------:|:--------:|-------|
| Literals | ✅ | ✅ | |
| Variables `let`/`const` | ✅ | ✅ | |
| Arithmetic / compare | ✅ | ✅ | specialized when profiled |
| Logical `&&` `\|\|` | ✅ | ✅ | short-circuit |
| Unary `!`, `-` | ✅ | ✅ | |
| `if` / `else` | ✅ | ✅ | |
| `while` | ✅ | ✅ | |
| `for (init; cond; incr)` | ✅ | ✅ | |
| `break` / `continue` | ✅ | ✅ | |
| `return` | ✅ | ✅ | |
| Array `[…]`, `arr[i]` | ✅ | ✅ | |
| `new Class()`, methods | ✅ | ✅ | via `VmRuntimeDelegate` |
| `this` | ✅ | ✅ | requires `this` slot/param |
| Direct `fn()` calls | ✅ | ✅ | |
| Intrinsic `__read`, `__runtime_*` | ✅ | ✅ | `INTRINSIC_SYSCALL` |
| `@compile` blocks | ✅ | ✅ | inline body |

## Explicitly interpreter-only (by design)

| Construct | Reason |
|-----------|--------|
| `class` / `interface` declarations | Setup phase; not in function bytecode |
| `import` / module loading | Program-level |
| `await` / async | Event loop integration |
| `super` | Constructor dispatch |
| Function expressions / closures | No `MAKE_CLOSURE` yet |
| Object literals `{a:1}` | No `ObjectExpression` in parser |
| Computed method call `obj[x]()` | Rare; deferred |

## Gate for next phases

Bytecode coverage is **complete** for:

1. Numeric hot paths → AOT/JIT
2. Stdlib syscall surface via `INTRINSIC_SYSCALL`
3. Class instance methods in `@compile("bytecode")` functions
4. Control flow including `break`/`continue`

Stdlib integration via `CALL_GLOBAL` is complete (6.13). Next: tier-2 JIT, closures.
