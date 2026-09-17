# @std/async

Async runtime for Dryad: microtask-driven Promises, `async`/`await`, threading bridge, and epoll I/O polling (Linux).

## Status (Phase 5 complete)

| Feature | Status |
|---------|--------|
| Microtask-driven `Promise` settlement | ✅ |
| `async function` / `await` syntax | ✅ |
| `EventLoop` pump at end of script | ✅ |
| epoll I/O poller (`__runtime_io_*`) | ✅ Linux |
| `Thread_startAsync` / `ThreadPool.submitAsync` | ✅ |
| `Promise_await` / `__runtime_await_promise` | ✅ |

## Quick start

```dryad
import { Promise_resolve, Promise_await } from "@std/async/promise";
import { Thread_startAsync } from "@std/async/threading/threading";

async function main() {
    let x = await Promise_resolve(40);
    let y = await Thread_startAsync(function() { return 2; }, []);
    return x + y;
}

let result = main();
Promise_await(result); // or rely on CLI pump at script end
```

## Modules

| Module | Key API |
|--------|---------|
| `promise` | `Promise`, `Promise_flush`, `Promise_await`, `Promise_all` |
| `event_loop` | `EventLoop_schedule`, `EventLoop_pump` |
| `threading/thread` | `Thread`, `Thread_startAsync`, `Thread_parallel` |
| `threading/thread_pool` | `ThreadPool`, `submitAsync`, `run` |
| `async` | Barrel re-export |

## C++ intrinsics

| Intrinsic | Purpose |
|-----------|---------|
| `__runtime_schedule_microtask` | Queue microtask |
| `__runtime_event_loop_pump_all` | Drain microtasks + thread completions + I/O |
| `__runtime_await_promise` | Block until promise settles (pumps event loop) |
| `__runtime_io_register_fd` | epoll watch (Linux) |
| `__thread_spawn_async` | Worker thread → resolve/reject on main loop |
| `__set_nonblocking` | Set O_NONBLOCK on socket fd |

## Remaining limitations

- `await` blocks the interpreter while pumping (cooperative, not preemptive)
- epoll integration is foundation-level; socket async still uses microtask deferral for connect/send
- Worker threads use isolated interpreters — no shared mutable Dryad state
- `catch`/`finally` reserved in statement position — use `.catch()` / `.finally()` on promises

See [Phase 5 plan](../implementation/plans/2026-09-16-async-runtime.md).
