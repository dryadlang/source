# Phase 4 Standard Library - Progress Report

**Date**: 2026-09-16  
**Status**: Phase 4.5 complete (Async + Networking P2)

---

## Summary

Phase 4 delivers the Dryad standard library foundation: core types, I/O, collections, networking, and async primitives. All planned modules for Phases 4.1–4.5 are implemented with full unit and integration test coverage.

### Completed phases

| Phase | Scope | Status |
|-------|-------|--------|
| 4.1 | Core Foundation (Buffer, VFS, exceptions, imports, CLI) | ✅ Complete |
| 4.2 | File I/O & Streams (FileStream, MemoryStream, Path) | ✅ Complete |
| 4.3 | Collections (List, Map, Set, Queue, IEnumerable) | ✅ Complete |
| 4.4 | Networking TCP (Socket, TcpClient, TcpListener) | ✅ Complete |
| 4.5 | Async + Networking P2 (Promise, Threading, UDP, DNS, TLS, HTTP) | ✅ Complete |

---

## Test Results

**362/362 tests passing (100%)**

```
Unit tests:         344 ✅
Integration tests:   18 ✅
```

---

## Deliverables

### Core Foundation (`@std/core`, `@std/buffers`)
- Exception hierarchy with stack trace intrinsic
- Buffer API (allocate, read/write, slice, resize, string conversion)
- `internal` visibility for module-private functions
- `IEnumerable` interface for collection iteration

### I/O (`@std/io`)
- VFS with `MemoryBackend` and `NativeBackend`
- `FileStream`, `MemoryStream`, abstract `Stream` API
- `Path` utilities (join, normalize, parent, extension)
- High-level `file.dryad` with async wrappers
- Cooperative `ThreadSafeVFS` locks
- Module README and runnable examples

### Collections (`@std/collections`)
- `List`: dynamic array, `map`/`filter`/`reduce`
- `Map`: key-value store with `keys()` / `values()` → `List`
- `Set`: unique values via internal `Map`
- `Queue`: FIFO circular buffer
- Barrel: `import { List, Map, Set, Queue } from "@std/collections"`

### Networking (`@std/net`)
- **TCP**: `Socket`, `NetworkStream`, `TcpClient`, `TcpListener`
- **UDP**: `UdpClient` with `sendto`/`recvfrom` buffer intrinsics
- **DNS**: `resolve_host` via `getaddrinfo` (IPv4)
- **TLS**: optional OpenSSL client handshake (`DRYAD_ENABLE_TLS=ON`)
- **HTTP**: minimal `HttpClient` (GET/POST, headers, URL parsing)
- **Async wrappers**: `socket_async.dryad` (Promise over blocking I/O)
- Network gated by `--allow-net` / `DRYAD_ALLOW_NET`
- Architecture: `docs/plans/2026-09-16-net-architecture.md`

### Async (`@std/async`)
- **Promise**: deferred `then()` queues, `Promise_all`, `Promise_race`, `catchError`, `finallyDo`
- **Threading** (`@std/async/threading/`):
  - `Thread` — `start()` (non-blocking) + `join()` (blocking)
  - `Thread_parallel` — run multiple functions concurrently
  - `ThreadPool` — fixed worker pool with `submit`/`wait`/`run`
  - `ThreadPool_map` — parallel map over function array
- C++ `ThreadManager` — each worker uses isolated `Interpreter`
- Intrinsics: `__thread_spawn`, `__thread_join`, `__thread_pool_*`, `__thread_sleep_ms`, `__thread_hardware_concurrency`

### Tooling
- `dryad` CLI: script execution, `--allow-net`, `--help`, `--version`
- `DRYAD_STDLIB_DIR` for stdlib path resolution
- `dryad check` semantic validation

---

## Known Limitations

| Area | Limitation |
|------|------------|
| Language | `catch`/`finally` reserved → `catchError`/`finallyDo` |
| Async | No real event loop; Promise executor is synchronous |
| Threading | Blocking join model; no `await` syntax yet |
| TLS | Client-only, no rigorous cert verification |
| Network | Blocking I/O; IPv4-focused DNS resolution |
| Logic | `&&` does not short-circuit |

---

## Deferred (future phases)

- Real event loop / `await` syntax integration
- Non-blocking async I/O (epoll/kqueue)
- Server-side TLS
- Generics for collections
- Full LSP completion and hover
