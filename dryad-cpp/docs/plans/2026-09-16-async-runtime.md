# Async Runtime — Phase 5 Plan

**Date**: 2026-09-16  
**Status**: Phase 5.1–5.5 complete  
**Depends on**: Phase 4.5 (`@std/async`, threading intrinsics)

---

## Current limitations (Phase 4.5)

| Limitation | Workaround today | Phase 5 target |
|------------|------------------|----------------|
| `catch`/`finally` reserved | `catchError`/`finallyDo` **or** `.catch()`/`.finally()` after `.` | ✅ `.catch()`/`.finally()` enabled in parser |
| Promise executor is synchronous | `then()` queues callbacks; settlement is immediate | Microtask queue + deferred settlement |
| No event loop | Blocking I/O + `Thread.join()` | `EventLoop` pump + I/O polling |
| Threading is blocking join | `ThreadPool.run()` blocks caller | `Promise` bridge from worker threads |
| Isolated interpreter per thread | No shared mutable state | Shared module registry + message passing |

---

## Phase 5.0 — Foundation (this sprint)

### Done
- [x] Parser: `catch` / `finally` allowed as **member names** (`obj.catch(fn)`, class methods)
- [x] `Promise.catch()` / `Promise.finally()` aliases (delegate to `catchError` / `finallyDo`)
- [x] C++ `EventLoop` singleton with microtask queue
- [x] Intrinsics: `__runtime_schedule_microtask`, `__runtime_event_loop_pending`
- [x] `@std/async/event_loop.dryad` wrapper

### Files
```
include/dryad/runtime/event_loop.h
src/runtime/event_loop.cpp
src/runtime/intrinsics_event_loop.cpp
stdlib/@std/async/event_loop.dryad
```

---

## Phase 5.1 — Microtask-driven Promises ✅

- `Promise_queueSettlement` schedules resolve/reject via `EventLoop_schedule`
- `Interpreter::execute()` pumps event loop at end of script
- `Promise_flush()` / `EventLoop_pump()` for explicit drain
- `then()` on settled promises also schedules via microtask

### Acceptance
```dryad
let order = [];
Promise_resolve(1).then(function(v) { order.push(1); });
order.push(0);
// pump → order == [0, 1]
```

---

## Phase 5.2 — `async` / `await` syntax ✅

- Parser: `async function`, `await expr` (`AwaitExpression` AST node)
- `async function` returns `Promise` via `PromiseRuntime::resolve`
- `await` via `__runtime_await_promise` (pumps until settled)
- Semantic analyzer: `await` only inside `async function`

### Acceptance
```dryad
async function fetchValue() {
    let x = await Promise_resolve(42);
    return x + 1;
}
let p = fetchValue(); // Promise<43>
```

---

## Phase 5.3 — I/O event loop (epoll/kqueue)

**Goal**: Non-blocking socket/file descriptors integrated with event loop.

### C++ intrinsics
- `syscall.epoll_create` / `epoll_ctl` / `epoll_wait` (Linux)
- `syscall.kqueue` / `kevent` (BSD/macOS) — platform abstraction layer

### Stdlib
- `Socket.connectAsync(host, port)` → `Promise<NetworkStream>`
- `NetworkStream.readAsync(buf)` → `Promise<int>`
- `@std/net/socket_async.dryad` upgraded from sync `Promise_resolve` wrappers

### Acceptance
- Single-threaded Dryad script: connect + read without blocking main interpreter between operations

---

## Phase 5.4 — Thread → Promise bridge

**Goal**: Background work returns `Promise` instead of requiring blocking `join()`.

### API
```dryad
function Thread_startAsync(fn, args) {
    return new Promise(function(resolve, reject) {
        let t = new Thread(fn, args);
        t.start();
        __thread_spawn(function() {
            let outcome = t.join();
            if (__runtime_object_get(outcome, "ok")) {
                resolve(__runtime_object_get(outcome, "value"));
            } else {
                reject(__runtime_object_get(outcome, "error"));
            }
        }, []);
    });
}
```

### C++ improvements
- `ThreadManager::spawn_promise(fn, args)` — resolve promise on worker completion via main-thread microtask
- Worker posts completion to main `EventLoop` (thread-safe queue)

### Isolated interpreter mitigation
- Module definitions are immutable after load → safe to share `ModuleLoader` cache
- Document: worker threads must not mutate shared Dryad objects; return values by copy only
- Future: `SharedArrayBuffer` / channel primitives for structured concurrency

---

## Phase 5.5 — Integration & polish

- [ ] `dryad check` warnings for blocking calls inside `async function`
- [ ] Integration test: async TCP echo (single-threaded event loop)
- [ ] Update `@std/async/README.md` — remove limitation entries as each phase ships
- [ ] Benchmark: microtask pump overhead vs sync executor

---

## Architecture diagram

```
┌─────────────────────────────────────────────────────────┐
│                    Main Interpreter                      │
│  ┌─────────────┐    ┌──────────────┐    ┌────────────┐ │
│  │ async/await │───▶│ Promise<T>   │───▶│ microtask │ │
│  │  desugar    │    │ (stdlib)     │    │  queue    │ │
│  └─────────────┘    └──────────────┘    └─────┬──────┘ │
│                                                 │        │
│  ┌─────────────┐    ┌──────────────┐           │        │
│  │ I/O poll    │◀───│  EventLoop   │◀──────────┘        │
│  │ epoll/kqueue│    │  pump_once() │                    │
│  └─────────────┘    └──────────────┘                    │
└───────────────────────────┬─────────────────────────────┘
                            │ completion microtask
┌───────────────────────────▼─────────────────────────────┐
│              Worker threads (ThreadPool)                 │
│  Isolated Interpreter per job → post result to main loop │
└─────────────────────────────────────────────────────────┘
```

---

## Migration guide (for stdlib authors)

| Phase 4.5 pattern | Phase 5 pattern |
|-------------------|-----------------|
| `promise.catchError(fn)` | `promise.catch(fn)` |
| `Promise_resolve(syncValue)` | `await asyncFn()` or deferred resolve |
| `thread.join()` blocking | `await Thread_startAsync(fn)` |
| `pool.run(fn)` blocking | `await pool.submitAsync(fn)` |

Keep `catchError`/`finallyDo` as permanent aliases for clarity in keyword-sensitive contexts.
