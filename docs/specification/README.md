# Language Specification

The canonical Dryad language specification lives in `dryad_theory/`.

## Primary document

| Document | Description |
|----------|-------------|
| [dryad_theoretical_foundation_v2.pdf](../../dryad_theory/dryad_theoretical_foundation_v2.pdf) | **Canonical spec** (v2.0, 34 pages) |
| [dryad_theoretical_foundation_v2.tex](../../dryad_theory/dryad_theoretical_foundation_v2.tex) | LaTeX source |

## Supporting documents

| Document | Description |
|----------|-------------|
| [Theory index](../../dryad_theory/README.md) | Full index with reading guide |
| [REWORK_OVERVIEW.md](../../dryad_theory/REWORK_OVERVIEW.md) | C++ reimplementation strategy |
| [intrinsics_architecture_summary.md](../../dryad_theory/intrinsics_architecture_summary.md) | Intrinsics-based runtime design |
| [self_hosted_native_system.md](../../dryad_theory/self_hosted_native_system.md) | Self-hosting stdlib design |
| [native_functions_alternatives.md](../../dryad_theory/native_functions_alternatives.md) | 8 native binding proposals |
| [TASK_CHECKLIST.md](../../dryad_theory/TASK_CHECKLIST.md) | Spec vs implementation checklist |
| [V2_REVISION_SUMMARY.md](../../dryad_theory/V2_REVISION_SUMMARY.md) | v2.0 changelog |

## Practical reference (implementation-aligned)

These docs describe what is **actually implemented** in `dryad-cpp/` today:

| Guide | Description |
|-------|-------------|
| [Syntax reference](../language/syntax.md) | Surface syntax |
| [Modules](../language/modules.md) | import/export |
| [Execution modes](../language/execution-modes.md) | Triple execution |
| [Diagrams](../language/diagrams.md) | PlantUML visual reference |
| [Standard library](../stdlib/README.md) | @std/* APIs |

## Relationship

```
dryad_theory/          Formal specification (what Dryad should be)
docs/language/         Practical reference (what the compiler supports today)
docs/implementation/   How the C++ code implements the spec
```

When the spec and implementation diverge, the implementation docs note the gap (e.g. `try/catch` lexed but not parsed).
