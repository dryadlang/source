# Language Reference

User-facing documentation for the Dryad programming language.

## Canonical specification

The authoritative language definition is **[dryad_theoretical_foundation_v2.pdf](../../dryad_theory/dryad_theoretical_foundation_v2.pdf)** (v2.0). These guides summarize what is implemented in `dryad-cpp/` today.

| Document | Description |
|----------|-------------|
| [Syntax reference](syntax.md) | Keywords, types, control flow, OOP, decorators |
| [Modules & imports](modules.md) | `import`/`export`, paths, packages |
| [Execution modes](execution-modes.md) | Interpreter, VM, AOT — triple execution |
| [Diagrams](diagrams.md) | PlantUML language reference (core + stdlib) |

## Theory & design (dryad_theory/)

| Document | Description |
|----------|-------------|
| [Theory index](../../dryad_theory/README.md) | Full index of specification documents |
| [Intrinsics architecture](../../dryad_theory/intrinsics_architecture_summary.md) | ~50 syscall primitives design |
| [Self-hosted stdlib](../../dryad_theory/self_hosted_native_system.md) | VFS, HTTP, async I/O design |
| [C++ rework overview](../../dryad_theory/REWORK_OVERVIEW.md) | Migration from Rust implementation |
| [Implementation checklist](../../dryad_theory/TASK_CHECKLIST.md) | Spec vs implementation tracking |

## Implementation status

| Area | Status | Details |
|------|--------|---------|
| Lexer & parser | ✅ | Phase 1 |
| Interpreter | ✅ | Phase 2 |
| Intrinsics | ✅ | Phase 3 |
| Standard library | 🔄 | Phase 4 (growing) |
| Bytecode VM | ✅ | Phase 6 |
| AOT / LLVM | ✅ | Phase 6 |
| User libraries (.dyd/.dyl) | ✅ | L1–L4 |
| try/catch/throw | ❌ | Keywords lexed, not parsed |

See [implementation phases](../implementation/phases/) for detailed reports.
