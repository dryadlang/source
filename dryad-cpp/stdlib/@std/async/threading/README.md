# @std/async/threading

OS-level threading sub-library. Each thread runs Dryad code in an isolated `Interpreter` instance.

## When to use

| Use case | API |
|----------|-----|
| Run one function on a background thread | `Thread` |
| Run multiple functions in parallel | `Thread_parallel` |
| Reuse workers for many short jobs | `ThreadPool` |
| Sleep without blocking the whole process | `Thread_sleepMs` |

## Thread lifecycle

1. `new Thread(fn, args)` — create (does not start)
2. `thread.start()` — spawn OS thread (non-blocking)
3. `thread.join()` — block until complete, return outcome object

`join()` returns a runtime object:

```dryad
let outcome = thread.join();
let ok = __runtime_object_get(outcome, "ok");
let value = __runtime_object_get(outcome, "value");
let error = __runtime_object_get(outcome, "error");
```

## ThreadPool lifecycle

1. `new ThreadPool(workerCount)` — start N worker threads
2. `pool.submit(fn, args)` → job id (non-blocking)
3. `pool.wait(jobId)` → result (blocking)
4. `pool.run(fn, args)` — shorthand for submit + wait
5. `pool.destroy()` — shutdown workers

## Implementation notes

- Workers call `pthread`-backed `std::thread` via C++ `ThreadManager`
- `stdlib_root` is propagated to worker interpreters via static registry
- `pool_wait` does not hold the global mutex during blocking wait (avoids deadlock with `set_stdlib_root`)
- Jobs are tracked by `shared_ptr<ThreadJob>` in the queue to avoid races between `wait()` and worker completion

## Limitations

- Blocking join/wait model only (no futures/promises integration yet)
- No thread cancellation or timeout
- GC/thread safety across interpreters not fully validated
- Functions passed to threads must not depend on caller's mutable closure state
