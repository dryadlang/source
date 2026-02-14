# Structural Refactor Implementation Plan (Phase 2)

The goal is to address technical debt and architectural weaknesses by modularizing the interpreter and establishing a clean separation of concerns.

## Proposed Changes

### Interpreter Restructuring

- **[NEW] [environment.rs](file:///C:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/environment.rs)**:
  - Define `Environment` struct to manage `variables`, `constants`, `classes`, and `imported_modules`.
  - Implement scope management (cloning/restoring) within `Environment`.
- **[NEW] [native_registry.rs](file:///C:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/native_registry.rs)**:
  - Define `NativeRegistry` to encapsulate `NativeModuleManager` and module activation logic.
- **[MODIFY] [interpreter.rs](file:///C:/Users/Pedro%20Jesus/Downloads/source-main/source-main/crates/dryad_runtime/src/interpreter.rs)**:
  - Replace internal maps with `Environment` and `NativeRegistry`.
  - Refactor evaluation logic to delegate state access and native calls.
  - Update Garbage Collection (GC) roots to traverse the modular structure.

## Verification Plan

### Automated Tests

- Full dryad_runtime test suite: `cargo test -p dryad_runtime`
- Compilation check: `cargo check -p dryad_runtime`
