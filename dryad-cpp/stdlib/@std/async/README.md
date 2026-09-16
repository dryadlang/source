# @std/async

Async primitives and OS threading for Dryad. The runtime uses a **synchronous Promise executor** (no real event loop). Threading runs Dryad functions on native OS threads with a **blocking join** model.

## Modules

| Module | Description |
|--------|-------------|
| `promise` | `Promise<T>`, `Promise_resolve`, `Promise_all`, `Promise_race` |
| `threading/thread` | `Thread`, `Thread_parallel`, `Thread_sleepMs`, `Thread_hardwareConcurrency` |
| `threading/thread_pool` | `ThreadPool`, `ThreadPool_map` |
| `threading/threading` | Barrel for threading sub-library |
| `async` | Top-level barrel re-export |

## Promise API

```dryad
import { Promise, Promise_resolve, Promise_all } from "@std/async/promise";

// Immediate resolve
let p = Promise_resolve(42);
p.then(function(v) { return v + 1; }); // chained promise → 43

// Deferred resolve
let holder = __runtime_create_object();
let pending = new Promise(function(resolve, reject) {
    __runtime_object_set(holder, "resolve", resolve);
});
pending.then(function(v) { return v * 2; });
__runtime_object_get(holder, "resolve")(21); // chained → 42

// Wait for all (preserves input order)
Promise_all([Promise_resolve(1), Promise_resolve(2)]);
```

| Method | Description |
|--------|-------------|
| `then(onFulfilled, onRejected?)` | Chain on fulfillment/rejection; queues if pending |
| `catchError(onRejected)` | Shorthand for `then(null, onRejected)` |
| `finallyDo(onFinally)` | Run cleanup after settle (no value change) |
| `Promise_all(promises)` | Resolve when all fulfilled; rejects on first failure |
| `Promise_race(promises)` | Resolve/reject with first settled promise |

## Threading API

```dryad
import { Thread, ThreadPool, Thread_parallel } from "@std/async/async";

function worker(a, b) { return a + b; }

// Single thread
let t = new Thread(worker, [10, 32]);
t.start();
let outcome = t.join();
if (__runtime_object_get(outcome, "ok")) {
    print(__runtime_object_get(outcome, "value")); // 42
}

// Parallel (non-blocking start, blocking join)
Thread_parallel([worker, worker]);

// Thread pool
let pool = new ThreadPool(4);
let result = pool.run(worker, [1, 1]); // blocking submit+wait
pool.destroy();
```

| Class / Function | Description |
|------------------|-------------|
| `Thread(fn, args?)` | Wraps a single OS thread |
| `Thread.start()` | Spawn thread (non-blocking) |
| `Thread.join()` | Block until complete; returns `{ ok, value }` or `{ ok, error }` |
| `Thread_parallel(fns)` | Start all, then join all |
| `ThreadPool(n)` | Fixed-size worker pool |
| `ThreadPool.submit(fn, args?)` | Queue job, return job id |
| `ThreadPool.wait(jobId)` | Block until job completes |
| `ThreadPool.run(fn, args?)` | `submit` + `wait` |
| `ThreadPool_map(pool, fns)` | Run array of functions in parallel |

## C++ Intrinsics

| Intrinsic | Description |
|-----------|-------------|
| `__thread_spawn(fn, args[])` | Spawn detached thread, return job id |
| `__thread_join(jobId)` | Block and return result (throws on failure) |
| `__thread_pool_create(workers)` | Create pool handle |
| `__thread_pool_destroy(handle)` | Shutdown pool |
| `__thread_pool_submit(handle, fn, args[])` | Queue job |
| `__thread_pool_wait(handle, jobId)` | Block for result |
| `__thread_sleep_ms(ms)` | Sleep current thread |
| `__thread_hardware_concurrency()` | CPU thread count |

## Known limitations

- `catch` / `finally` are reserved keywords → use `catchError` / `finallyDo`
- Promises resolve synchronously when the executor runs; `then()` queues callbacks for pending promises
- Each worker thread uses a separate `Interpreter` instance (no shared mutable state)
- No `await` syntax — use `then()` chaining or blocking `join()`/`wait()`
- TLS certificate verification is not rigorous (see `@std/net`)

## Running tests

```bash
./build/unit_tests --gtest_filter="PromiseTest.*:ThreadingTest.*"
```
