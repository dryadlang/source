# Standard Library (@std)

The Dryad standard library is **100% written in Dryad** and built on compiler intrinsics (`__syscall_*`, `__runtime_*`). Application code should use `@std/*` — never call intrinsics directly.

**Source:** `dryad-cpp/stdlib/@std/` (46 modules)

## Architecture

```
Application code
  ↓
@std/* high-level API (TcpClient, readFile, Promise, HttpClient)
  ↓
@std/* mid-level (Socket, VFS, Buffer, Stream)
  ↓
Compiler intrinsics (__socket, __open_read, __alloc_bytes, …)
  ↓
C++ runtime / OS syscalls
```

**Error model:** functions return error strings from `@std/core/exceptions` — no thrown exceptions across the language boundary.

## Module tree

```
@std/
├── core/           IDisposable, IEnumerable, exception factories
├── buffers/        Buffer — managed byte arrays
├── collections/    List, Map, Set, Queue (barrel)
├── io/             VFS, streams, paths, file I/O
├── async/          Promise, event loop, threads (barrel)
└── net/            sockets, TCP/UDP, TLS, HTTP (barrel)
```

## Guides

| Module | Guide | Barrel import |
|--------|-------|---------------|
| Core | [core.md](core.md) | `@std/core/exceptions`, etc. |
| Buffers | [buffers.md](buffers.md) | `@std/buffers/buffer` |
| Collections | [collections.md](collections.md) | `@std/collections` |
| I/O | [io.md](io.md) | `@std/io/file` (facade) |
| Async | [async.md](async.md) | `@std/async` |
| Threading | [async-threading.md](async-threading.md) | `@std/async/threading/threading` |
| Networking | [net.md](net.md) | `@std/net` |

## Quick examples

```dryad
import { List } from "@std/collections";
import { readFile, writeFile } from "@std/io/file";
import { Buffer_FromString } from "@std/buffers/buffer";
import { Promise_resolve } from "@std/async/promise";
```

## Design documents

| Document | Description |
|----------|-------------|
| [STDLIB_ARCHITECTURE.md](../implementation/architecture/STDLIB_ARCHITECTURE.md) | Design philosophy, namespace layout |
| [STDLIB_IMPLEMENTATION_PLAN.md](../implementation/architecture/STDLIB_IMPLEMENTATION_PLAN.md) | Phase 4 implementation plan |
| [self_hosted_native_system.md](../../dryad_theory/self_hosted_native_system.md) | Original VFS/HTTP/async design |

## Examples

| Category | Path |
|----------|------|
| Buffers | `dryad-cpp/examples/02-buffers/` |
| Collections | `dryad-cpp/examples/03-collections/` |
| I/O | `dryad-cpp/examples/04-io/` |
| Async | `dryad-cpp/examples/05-async/` |
| Networking | `dryad-cpp/examples/06-net/` |

## Diagram

See [stdlib PlantUML diagram](../diagrams/dryad-language/parts/11-stdlib.puml).
