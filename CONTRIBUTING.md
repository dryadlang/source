# Contributing to Dryad

Thanks for considering a contribution. This project is small and opinionated — the rules below keep it that way.

## Development workflow

### 1. Test-first (TDD is enforced)

Write the failing test before the implementation:

1. Add a test under `dryad-cpp/tests/` that describes the desired behavior.
2. Run it and confirm it fails for the right reason.
3. Implement the feature in `dryad-cpp/src/`.
4. Re-run the test until green, then run the full suite.

Targets: >90% coverage, no memory leaks (sanitizers are available via `-DDRYAD_ENABLE_SANITIZERS=ON`).

### 2. Small, focused commits

- One logical change per commit.
- Commit messages: imperative, summary line under 72 characters, body explaining *why* when it is not obvious. Example:

  ```
  stdlib: add bounds-checked byte read to Buffer

  Reads now return an error string instead of exposing out-of-bounds
  access, matching the error-as-value contract in exceptions.dryad.
  ```

- Run the full test suite before committing.

### 3. Keep the suite green

```bash
cd build
cmake --build .
ctest --output-on-failure
```

The suite currently has 7 documented pre-existing failures (see `CHANGELOG`). Do not add new failures on top of them; if your change is affected by one, mention it in the commit message.

## Code conventions

### C++ (runtime)

- C++17, `-Wall -Wextra -Werror` — no warnings allowed.
- Classes are PascalCase (`MemoryManager`), functions and variables camelCase.
- No unnecessary comments. Comments explain *why*, not *what*.
- Do not suppress diagnostics with `as any`-style escapes; there are none in C++, but do not weaken the build flags to get code to compile.

### Dryad (standard library)

- Functions are camelCase (`Buffer_FromString`).
- Stdlib errors are **values** (strings defined in `@std/core/exceptions`), never thrown exceptions.
- The stdlib never calls C++ directly — it only goes through intrinsics (`syscall.*` / `__*`), so it stays portable and testable.
- No unnecessary comments.

### Intrinsics

- New runtime capabilities are exposed as intrinsics (`syscall.*`), registered in `intrinsics_registry.cpp`.
- Keep the surface small: prefer composing existing intrinsics in Dryad over adding new ones.

## What not to do

- Do not commit the `build/` directory or any generated files.
- Do not commit probe/diagnostic artifacts (`build/probe*`, `build/flags.*`).
- Do not rewrite history on `main` (this project works directly on `main`).

## Submitting changes

This project lives on GitLab: `https://gitlab.com/dryadlang/source`.

- Push your branch and open a merge request, or if you have push access, commit directly to `main` per the workflow above.
- Reference any relevant issue in the merge request description.
- The specification lives in `dryad_theory/` (PDF + LaTeX). If a change contradicts the spec, update the spec or explain the deviation in the merge request.

## Reporting issues

- Check the `CHANGELOG` and existing issues first — some failures are already documented.
- Include: what you did, what you expected, what happened, and the build/test output.
- For spec errors, cite the document version, section, and page.
