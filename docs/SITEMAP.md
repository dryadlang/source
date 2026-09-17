# Documentation Sitemap

Complete inventory of Dryad project documentation. **Active docs** live under `docs/`; **canonical spec** in `dryad_theory/`; **archived** material in `archived/`.

---

## 1. User documentation (`docs/`)

### Getting started

| File | Description |
|------|-------------|
| [getting-started/README.md](getting-started/README.md) | Section index |
| [getting-started/quick-start.md](getting-started/quick-start.md) | Install, first script, examples |
| [getting-started/building.md](getting-started/building.md) | CMake, CLI, env vars, tests |
| [getting-started/contributing.md](getting-started/contributing.md) | → root CONTRIBUTING.md |

### Language

| File | Description |
|------|-------------|
| [language/README.md](language/README.md) | Language docs hub |
| [language/syntax.md](language/syntax.md) | Surface syntax reference |
| [language/modules.md](language/modules.md) | import/export, paths |
| [language/execution-modes.md](language/execution-modes.md) | Triple execution |
| [language/diagrams.md](language/diagrams.md) | PlantUML index |

### Specification

| File | Description |
|------|-------------|
| [specification/README.md](specification/README.md) | → dryad_theory/ canonical PDF |

### Standard library

| File | Description |
|------|-------------|
| [stdlib/README.md](stdlib/README.md) | @std module tree |
| [stdlib/core.md](stdlib/core.md) | IDisposable, exceptions |
| [stdlib/buffers.md](stdlib/buffers.md) | Buffer API |
| [stdlib/collections.md](stdlib/collections.md) | List, Map, Set, Queue |
| [stdlib/io.md](stdlib/io.md) | VFS, streams, files |
| [stdlib/async.md](stdlib/async.md) | Promise, event loop |
| [stdlib/async-threading.md](stdlib/async-threading.md) | Thread, ThreadPool |
| [stdlib/net.md](stdlib/net.md) | TCP/UDP, TLS, HTTP |

### Libraries & packages

| File | Description |
|------|-------------|
| [libraries/README.md](libraries/README.md) | User libraries hub |
| [libraries/overview.md](libraries/overview.md) | .dyd/.dyl system |
| [libraries/packages.md](libraries/packages.md) | dryad.toml, install |

### Tooling

| File | Description |
|------|-------------|
| [tooling/README.md](tooling/README.md) | Tooling hub |
| [tooling/cli.md](tooling/cli.md) | CLI reference |
| [implementation/tooling/DRYAD_TOML.md](implementation/tooling/DRYAD_TOML.md) | Manifest format (detailed) |

### Examples

| File | Description |
|------|-------------|
| [examples/README.md](examples/README.md) | → dryad-cpp/examples/ |
| [../dryad-cpp/examples/README.md](../dryad-cpp/examples/README.md) | Full catalog |
| [../dryad-cpp/examples/08-libraries/README.md](../dryad-cpp/examples/08-libraries/README.md) | Library example |

### Diagrams

| Path | Description |
|------|-------------|
| [diagrams/dryad-language/dryad-language.puml](diagrams/dryad-language/dryad-language.puml) | Master PlantUML index |
| [diagrams/dryad-language/parts/01–11](diagrams/dryad-language/parts/) | 11 detailed diagrams |

---

## 2. Implementation docs (`docs/implementation/`)

### Architecture

| File | Description |
|------|-------------|
| [architecture/STDLIB_ARCHITECTURE.md](implementation/architecture/STDLIB_ARCHITECTURE.md) | Stdlib design |
| [architecture/STDLIB_IMPLEMENTATION_PLAN.md](implementation/architecture/STDLIB_IMPLEMENTATION_PLAN.md) | Phase 4 plan |
| [architecture/LIBRARY_SYSTEM.md](implementation/architecture/LIBRARY_SYSTEM.md) | Library platform spec |

### Phases

| File | Phase |
|------|-------|
| [phases/PHASE_0_COMPLETE.md](implementation/phases/PHASE_0_COMPLETE.md) | Project setup |
| [phases/PHASE_1_COMPLETE.md](implementation/phases/PHASE_1_COMPLETE.md) | Lexer & parser |
| [phases/PHASE_2_COMPLETE.md](implementation/phases/PHASE_2_COMPLETE.md) | Interpreter |
| [phases/PHASE_3_COMPLETE.md](implementation/phases/PHASE_3_COMPLETE.md) | Intrinsics |
| [phases/PHASE_3_FINAL.md](implementation/phases/PHASE_3_FINAL.md) | Intrinsics (final) |
| [phases/PHASE_4_PROGRESS_REPORT.md](implementation/phases/PHASE_4_PROGRESS_REPORT.md) | Stdlib |
| [phases/PHASE_6_TRIPLE_MODEL.md](implementation/phases/PHASE_6_TRIPLE_MODEL.md) | Triple execution |

### Plans

| File | Topic |
|------|-------|
| [plans/2025-06-21-syscall-intrinsics.md](implementation/plans/2025-06-21-syscall-intrinsics.md) | Syscalls |
| [plans/2026-06-21-buffer-runtime-infrastructure.md](implementation/plans/2026-06-21-buffer-runtime-infrastructure.md) | Buffers |
| [plans/2026-06-21-io-vfs-implementation.md](implementation/plans/2026-06-21-io-vfs-implementation.md) | VFS |
| [plans/2026-06-21-vm-dispatcher-design.md](implementation/plans/2026-06-21-vm-dispatcher-design.md) | VM design |
| [plans/2026-06-21-vm-implementation-plan.md](implementation/plans/2026-06-21-vm-implementation-plan.md) | VM impl |
| [plans/VM_DELIVERABLES_SUMMARY.md](implementation/plans/VM_DELIVERABLES_SUMMARY.md) | VM deliverables |
| [plans/2026-09-16-async-runtime.md](implementation/plans/2026-09-16-async-runtime.md) | Async |
| [plans/2026-09-16-net-architecture.md](implementation/plans/2026-09-16-net-architecture.md) | Networking |
| [plans/2026-09-16-triple-execution-model.md](implementation/plans/2026-09-16-triple-execution-model.md) | Triple model |
| [plans/2026-09-17-library-system-implementation.md](implementation/plans/2026-09-17-library-system-implementation.md) | Libraries |
| [plans/2026-09-17-opcode-coverage.md](implementation/plans/2026-09-17-opcode-coverage.md) | Opcodes |
| [plans/BYTECODE_COVERAGE.md](implementation/plans/BYTECODE_COVERAGE.md) | Coverage matrix |
| [plans/2026-09-17-stdlib-via-intrinsics.md](implementation/plans/2026-09-17-stdlib-via-intrinsics.md) | CALL_GLOBAL |
| [plans/2026-09-17-native-binary-output.md](implementation/plans/2026-09-17-native-binary-output.md) | Native binary |
| [plans/2026-09-17-example-triple-coverage.md](implementation/plans/2026-09-17-example-triple-coverage.md) | Example gate |

### Principles

| File | Description |
|------|-------------|
| [principles/SOLID_PRINCIPLES.md](implementation/principles/SOLID_PRINCIPLES.md) | SOLID |
| [principles/RESOURCE_EFFICIENCY.md](implementation/principles/RESOURCE_EFFICIENCY.md) | Resource efficiency |

---

## 3. Language specification (`dryad_theory/`)

| File | Description |
|------|-------------|
| [dryad_theoretical_foundation_v2.pdf](../dryad_theory/dryad_theoretical_foundation_v2.pdf) | **Canonical spec** (v2.0) |
| [dryad_theoretical_foundation_v2.tex](../dryad_theory/dryad_theoretical_foundation_v2.tex) | LaTeX source |
| [README.md](../dryad_theory/README.md) | Theory docs index |
| [QUICK_START.md](../dryad_theory/QUICK_START.md) | C++ reimplementation quick start |
| [REWORK_OVERVIEW.md](../dryad_theory/REWORK_OVERVIEW.md) | Rust → C++ migration |
| [intrinsics_architecture_summary.md](../dryad_theory/intrinsics_architecture_summary.md) | Intrinsics design |
| [self_hosted_native_system.md](../dryad_theory/self_hosted_native_system.md) | Self-hosting stdlib |
| [native_functions_alternatives.md](../dryad_theory/native_functions_alternatives.md) | 8 binding proposals |
| [TASK_CHECKLIST.md](../dryad_theory/TASK_CHECKLIST.md) | Implementation checklist |
| [V2_REVISION_SUMMARY.md](../dryad_theory/V2_REVISION_SUMMARY.md) | v2 changelog |
| [CORRECTIONS_APPLIED.md](../dryad_theory/CORRECTIONS_APPLIED.md) | v1.1 corrections |

---

## 4. Implementation code docs (`dryad-cpp/`)

| File | Description |
|------|-------------|
| [README.md](../dryad-cpp/README.md) | C++ project quick start |
| [examples/README.md](../dryad-cpp/examples/README.md) | Example catalog |
| [examples/dryadsquare/README.md](../dryad-cpp/examples/dryadsquare/README.md) | Turing benchmark |
| [examples/dryadsquare/SPEC.md](../dryad-cpp/examples/dryadsquare/SPEC.md) | DryadSquare spec |
| [examples/08-libraries/README.md](../dryad-cpp/examples/08-libraries/README.md) | Library example |

### Stdlib module READMEs (redirect to docs/stdlib/)

| File | Redirects to |
|------|--------------|
| [stdlib/@std/io/README.md](../dryad-cpp/stdlib/@std/io/README.md) | docs/stdlib/io.md |
| [stdlib/@std/collections/README.md](../dryad-cpp/stdlib/@std/collections/README.md) | docs/stdlib/collections.md |
| [stdlib/@std/async/README.md](../dryad-cpp/stdlib/@std/async/README.md) | docs/stdlib/async.md |
| [stdlib/@std/async/threading/README.md](../dryad-cpp/stdlib/@std/async/threading/README.md) | docs/stdlib/async-threading.md |
| [stdlib/@std/net/README.md](../dryad-cpp/stdlib/@std/net/README.md) | docs/stdlib/net.md |

---

## 5. Root-level

| File | Description |
|------|-------------|
| [README.md](../README.md) | Project overview |
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Contribution guide |
| [CHANGELOG](../CHANGELOG) | Release notes |

---

## 6. Archived (`archived/`)

| Path | Contents |
|------|----------|
| [archive/README.md](archive/README.md) | Archive index |
| [archived/oldrust/](../archived/oldrust/) | Rust interpreter, Oak PM, AOT, IPE |
| [archived/oldrust/docs/](../archived/oldrust/docs/) | Old language reference |
| [archived/docs_md/](../archived/docs_md/) | Reorganization reports (PT) |

> Archived docs may reference APIs that no longer exist. Use `docs/` and `dryad_theory/` for current information.

---

## Navigation by audience

| I am… | Start here |
|-------|------------|
| New user | [getting-started/quick-start.md](getting-started/quick-start.md) |
| Writing Dryad code | [language/syntax.md](language/syntax.md) → [stdlib/README.md](stdlib/README.md) |
| Building libraries | [libraries/README.md](libraries/README.md) |
| Language designer | [specification/README.md](specification/README.md) |
| C++ implementor | [implementation/README.md](implementation/README.md) |
| Visual learner | [language/diagrams.md](language/diagrams.md) |
