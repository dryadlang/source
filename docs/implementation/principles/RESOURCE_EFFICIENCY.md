# Resource Efficiency

Dryad targets hosts from desktops to constrained embedded systems. Resource use is a
first-class design constraint, not an afterthought.

## Execution tiers (cost model)

| Tier | Compile cost | Runtime RAM | CPU | Use when |
|------|-------------|-------------|-----|----------|
| Interpreter | None | Low | Slowest | Bootstrap, REPL, module load |
| Bytecode VM | Medium | Medium | Fast | Default dev / CI |
| AOT native | High (LLVM) | Lowest* | Fastest | Release, firmware-adjacent |

\* AOT uses a bump arena for boxed values; arrays still allocate separately.

## Runtime safeguards

| Variable | Default | Purpose |
|----------|---------|---------|
| `DRYAD_VM_MAX_STEPS` | unlimited | Abort VM after N dispatch steps (runaway loop guard) |
| `DRYAD_ARENA_BYTES` | 65536 | Cap AOT box arena; OOM falls back to static sentinel |
| `DRYAD_RUN_TIMEOUT` | 30 | Benchmark / example execution timeout (seconds) |
| `DRYAD_COMPILE_TIMEOUT` | 120 | AOT compile timeout in `run_benchmark.sh` |

## AOT runtime rules

1. **Arena boxing** — `dryad_box_*` uses a single bump arena per process (no per-value `malloc`).
2. **Literal strings** — LLVM emits `dryad_box_lit_cstring` (no `strdup`) for compile-time literals.
3. **Heap arrays** — `dryad_new_array` still uses heap; future: optional static/arena arrays for embedded profiles.
4. **No C++ in linked runtime** — `aot_runtime.c` stays freestanding-friendly.

## Compiler / VM rules

1. **Imports always interpret** — module loading must not trigger bytecode compile of the user program (prevents compile storms during `import`).
2. **JIT off by default** — enable only when a delegate is attached; embedded builds should omit JIT entirely.
3. **Profiler optional** — `--no-profile` skips interpreter profiling pass on compile.

## Embedded roadmap (next steps)

- [ ] `DRYAD_EMBEDDED` CMake profile: no LLVM, no JIT, fixed arena sizes
- [ ] Stack slots without boxing for typed locals in AOT (I32/F64 fast path end-to-end)
- [ ] Optional `DRYAD_AOT_MAX_LOOP_ITERS` injected at LLVM lower time
- [ ] Freestanding libc: replace `printf` with hook `dryad_write(int fd, const char*, size_t)`
- [ ] Cross-compile AOT on host, flash bytecode or native blob to device

## Debugging runaway processes

Hung native tests (e.g. old `/tmp/cf` from a prior AOT link) spin at ~80% CPU. They are
**stale executables**, not the `dryad` compiler itself.

```bash
pkill -f '/tmp/cf'    # or: kill <pid>
rm -f /tmp/cf /tmp/cf_native
```

Re-link after codegen fixes: `dryad compile file.dryad --aot --link --executable -o /tmp/out`
