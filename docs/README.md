# Dryad Documentation

Central documentation index for the Dryad programming language project.

## Quick links

| Topic | Path |
|-------|------|
| Build & run | [getting-started/building.md](getting-started/building.md) |
| Contributing | [getting-started/contributing.md](getting-started/contributing.md) |
| Language spec (v2) | [specification/README.md](specification/README.md) |
| Implementation phases | [implementation/phases/](implementation/phases/) |
| Architecture & plans | [implementation/architecture/](implementation/architecture/) |
| Standard library | [stdlib/](stdlib/) |
| Code examples | [../dryad-cpp/examples/README.md](../dryad-cpp/examples/README.md) |
| Archived (Rust era) | [archive/README.md](archive/README.md) |

## Structure

```
docs/
├── README.md                          ← you are here
├── getting-started/
│   ├── building.md                    # CMake, CLI, environment
│   └── contributing.md                # TDD workflow, commit style
├── specification/
│   └── README.md                      # Index to dryad_theory/ (canonical PDF)
├── implementation/
│   ├── phases/                        # Phase 0–4 completion reports
│   ├── architecture/                  # Stdlib design & implementation plan
│   ├── principles/                    # SOLID principles for C++ code
│   └── plans/                         # Feature implementation plans
│       ├── syscall-intrinsics
│       ├── buffer-runtime
│       ├── io-vfs
│       ├── vm-dispatcher / vm-implementation
│       ├── async-runtime
│       ├── net-architecture
│       ├── triple-execution-model
│       └── opcode-coverage (6.11)
├── stdlib/
│   ├── async.md
│   ├── async-threading.md
│   ├── collections.md
│   ├── io.md
│   └── net.md
└── archive/
    └── README.md                      # Pointer to archived/ (old Rust impl)
```

## Implementation plans

| Plan | Description |
|------|-------------|
| [syscall-intrinsics](implementation/plans/2025-06-21-syscall-intrinsics.md) | ~50 syscall primitives (file, network, memory, process) |
| [buffer-runtime](implementation/plans/2026-06-21-buffer-runtime-infrastructure.md) | Buffer class API, bounds checking, GC |
| [io-vfs](implementation/plans/2026-06-21-io-vfs-implementation.md) | Virtual File System (Native + Memory backends) |
| [vm-dispatcher](implementation/plans/2026-06-21-vm-dispatcher-design.md) | Stack-based VM architecture |
| [vm-implementation](implementation/plans/2026-06-21-vm-implementation-plan.md) | 100+ opcodes, INTRINSIC_SYSCALL |
| [async-runtime](implementation/plans/2026-09-16-async-runtime.md) | Microtasks, epoll, thread→Promise bridge |
| [net-architecture](implementation/plans/2026-09-16-net-architecture.md) | TCP/UDP, NetworkStream, HTTP client |
| [triple-execution-model](implementation/plans/2026-09-16-triple-execution-model.md) | Interpreter + bytecode VM + AOT LLVM IR |
| [opcode-coverage](implementation/plans/2026-09-17-opcode-coverage.md) | Arrays, `for`, class methods in bytecode (6.11) |
| [bytecode-coverage matrix](implementation/plans/BYTECODE_COVERAGE.md) | Full opcode + language coverage status (6.12) |
| [stdlib-via-intrinsics](implementation/plans/2026-09-17-stdlib-via-intrinsics.md) | `CALL_GLOBAL` for stdlib from bytecode (6.13) |
| [native-binary-output](implementation/plans/2026-09-17-native-binary-output.md) | ELF executable + `binary` bytes in compile result (6.14) |

## Phase reports

| Phase | Report |
|-------|--------|
| 0 — Project setup | [PHASE_0_COMPLETE.md](implementation/phases/PHASE_0_COMPLETE.md) |
| 1 — Lexer & parser | [PHASE_1_COMPLETE.md](implementation/phases/PHASE_1_COMPLETE.md) |
| 2 — Interpreter | [PHASE_2_COMPLETE.md](implementation/phases/PHASE_2_COMPLETE.md) |
| 3 — Intrinsics | [PHASE_3_COMPLETE.md](implementation/phases/PHASE_3_COMPLETE.md) / [PHASE_3_FINAL.md](implementation/phases/PHASE_3_FINAL.md) |
| 4 — Stdlib | [PHASE_4_PROGRESS_REPORT.md](implementation/phases/PHASE_4_PROGRESS_REPORT.md) |
| 6 — Triple model | [PHASE_6_TRIPLE_MODEL.md](implementation/phases/PHASE_6_TRIPLE_MODEL.md) |
