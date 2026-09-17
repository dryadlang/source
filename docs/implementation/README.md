# Implementation Documentation

Internal documentation for Dryad compiler/runtime development (C++ in `dryad-cpp/`).

## Sections

| Section | Description |
|---------|-------------|
| [Phases](phases/) | Completion reports (Phase 0–6) |
| [Architecture](architecture/) | System design specifications |
| [Plans](plans/) | Feature implementation plans |
| [Principles](principles/) | C++ coding principles |
| [Tooling](tooling/) | Package manifest, build tooling |

## Phase reports

| Phase | Report | Topic |
|-------|--------|-------|
| 0 | [PHASE_0_COMPLETE](phases/PHASE_0_COMPLETE.md) | Project setup |
| 1 | [PHASE_1_COMPLETE](phases/PHASE_1_COMPLETE.md) | Lexer & parser |
| 2 | [PHASE_2_COMPLETE](phases/PHASE_2_COMPLETE.md) | Interpreter |
| 3 | [PHASE_3_COMPLETE](phases/PHASE_3_COMPLETE.md) / [FINAL](phases/PHASE_3_FINAL.md) | Intrinsics |
| 4 | [PHASE_4_PROGRESS](phases/PHASE_4_PROGRESS_REPORT.md) | Standard library |
| 6 | [PHASE_6_TRIPLE_MODEL](phases/PHASE_6_TRIPLE_MODEL.md) | Triple execution |

## Architecture

| Document | Description |
|----------|-------------|
| [STDLIB_ARCHITECTURE](architecture/STDLIB_ARCHITECTURE.md) | Stdlib design philosophy |
| [STDLIB_IMPLEMENTATION_PLAN](architecture/STDLIB_IMPLEMENTATION_PLAN.md) | Phase 4 plan |
| [LIBRARY_SYSTEM](architecture/LIBRARY_SYSTEM.md) | User libraries (.dyd/.dyl) |

## Active plans

| Plan | Description |
|------|-------------|
| [syscall-intrinsics](plans/2025-06-21-syscall-intrinsics.md) | ~50 syscall primitives |
| [buffer-runtime](plans/2026-06-21-buffer-runtime-infrastructure.md) | Buffer API, bounds checking |
| [io-vfs](plans/2026-06-21-io-vfs-implementation.md) | VFS (Native + Memory backends) |
| [vm-dispatcher](plans/2026-06-21-vm-dispatcher-design.md) | Stack-based VM design |
| [vm-implementation](plans/2026-06-21-vm-implementation-plan.md) | Opcode set, INTRINSIC_SYSCALL |
| [async-runtime](plans/2026-09-16-async-runtime.md) | Microtasks, epoll, thread bridge |
| [net-architecture](plans/2026-09-16-net-architecture.md) | TCP/UDP, HTTP client |
| [triple-execution](plans/2026-09-16-triple-execution-model.md) | Interpreter + VM + AOT |
| [library-system](plans/2026-09-17-library-system-implementation.md) | L1–L4 libraries |
| [opcode-coverage](plans/2026-09-17-opcode-coverage.md) | Bytecode language coverage |
| [BYTECODE_COVERAGE](plans/BYTECODE_COVERAGE.md) | Opcode matrix |
| [stdlib-via-intrinsics](plans/2026-09-17-stdlib-via-intrinsics.md) | CALL_GLOBAL for stdlib |
| [native-binary-output](plans/2026-09-17-native-binary-output.md) | ELF executable output |
| [example-triple-coverage](plans/2026-09-17-example-triple-coverage.md) | Examples triple gate |

## Principles

| Document | Description |
|----------|-------------|
| [SOLID_PRINCIPLES](principles/SOLID_PRINCIPLES.md) | SOLID for C++ codebase |
| [RESOURCE_EFFICIENCY](principles/RESOURCE_EFFICIENCY.md) | Memory & performance guidelines |

## Code layout

```
dryad-cpp/
├── include/dryad/     Public headers
│   ├── compiler/      Lexer, parser, AST, interpreter, bytecode
│   ├── runtime/       VM, values, modules, intrinsics
│   └── aot/           Micro-IR, LLVM, JIT, native linking
├── src/               Implementations
├── stdlib/@std/       Standard library (Dryad)
├── tests/             Unit + integration tests
└── examples/          Runnable demos
```

## Diagrams

[PlantUML language reference](../diagrams/dryad-language/dryad-language.puml)
