# Bytecode Test AST Compatibility Fixes Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Update all 4 integration test files to match the current parser AST structure (SourceLocation fields, Stmt::Print → Expr::Call, FunctionDeclaration params structure).

**Architecture:** 
1. Update SourceLocation helper in each test file to include `position: 0, source_line: None`
2. Replace all `Stmt::Print(expr, loc)` with `Stmt::Expression(Expr::Call(Box::new(Expr::Variable("print".to_string(), loc)), vec![expr], loc), loc)`
3. Update all FunctionDeclaration params from `(String, Option<Type>)` to `(String, Option<Type>, None)` 
4. Add `rest_param: None` field to all FunctionDeclaration structs

**Tech Stack:** Rust, AST manipulation, bytecode compiler tests

---

## Task 1: Fix array_tests.rs SourceLocation + Stmt::Print

**Files:**
- Modify: `crates/dryad_bytecode/tests/array_tests.rs`

**Step 1: Update dummy_loc() helper**

In array_tests.rs, find the `dummy_loc()` function (lines 8-15) and replace with:

```rust
fn dummy_loc() -> SourceLocation {
    SourceLocation {
        line: 1,
        column: 1,
        file: None,
        position: 0,
        source_line: None,
    }
}
```

Expected: File saves without errors.

**Step 2: Replace first Stmt::Print (test_array_indexing, line ~71)**

Find:
```rust
// print arr[1];
Stmt::Print(
    Expr::Index(
        Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
        Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Replace with:
```rust
// print arr[1];
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::Index(
            Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
            Box::new(Expr::Literal(Literal::Number(1.0), dummy_loc())),
            dummy_loc(),
        )],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Expected: File saves without errors.

**Step 3: Replace second Stmt::Print (test_array_mutation, line ~123)**

Find:
```rust
// print arr[0];
Stmt::Print(
    Expr::Index(
        Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
        Box::new(Expr::Literal(Literal::Number(0.0), dummy_loc())),
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Replace with:
```rust
// print arr[0];
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::Index(
            Box::new(Expr::Variable("arr".to_string(), dummy_loc())),
            Box::new(Expr::Literal(Literal::Number(0.0), dummy_loc())),
            dummy_loc(),
        )],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Expected: File saves without errors.

**Step 4: Replace third Stmt::Print (test_tuple_access, line ~203)**

Find:
```rust
// print t.1;
Stmt::Print(
    Expr::TupleAccess(
        Box::new(Expr::Variable("t".to_string(), dummy_loc())),
        1,
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Replace with:
```rust
// print t.1;
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::TupleAccess(
            Box::new(Expr::Variable("t".to_string(), dummy_loc())),
            1,
            dummy_loc(),
        )],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Expected: File saves without errors.

**Step 5: Compile and verify**

Run: `cargo test -p dryad_bytecode --test array_tests 2>&1 | tail -20`

Expected: All tests compile successfully (runtime may fail if print not defined, but that's OK).

**Step 6: Commit**

```bash
git add crates/dryad_bytecode/tests/array_tests.rs
git commit -m "fix: update array_tests.rs for current AST structure (SourceLocation, Stmt::Print)"
```

---

## Task 2: Fix exception_tests.rs SourceLocation + Stmt::Print

**Files:**
- Modify: `crates/dryad_bytecode/tests/exception_tests.rs`

**Step 1: Update dummy_loc()**

Same as Task 1 Step 1.

**Step 2: Replace all Stmt::Print instances in catch blocks**

exception_tests.rs has 2 Stmt::Print calls in the try/catch/finally test (line ~84 and ~101).

Find first:
```rust
Stmt::Print(
    Expr::Literal(Literal::String("Capturado".to_string()), dummy_loc()),
    dummy_loc(),
),
```

Replace with:
```rust
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::Literal(Literal::String("Capturado".to_string()), dummy_loc())],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Find second:
```rust
Stmt::Print(
    Expr::Literal(Literal::String("Sempre executa".to_string()), dummy_loc()),
    dummy_loc(),
),
```

Replace with:
```rust
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::Literal(Literal::String("Sempre executa".to_string()), dummy_loc())],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Expected: File saves without errors.

**Step 3: Compile and verify**

Run: `cargo test -p dryad_bytecode --test exception_tests 2>&1 | tail -20`

Expected: All tests compile successfully.

**Step 4: Commit**

```bash
git add crates/dryad_bytecode/tests/exception_tests.rs
git commit -m "fix: update exception_tests.rs for current AST structure (SourceLocation, Stmt::Print)"
```

---

## Task 3: Fix increment_tests.rs SourceLocation + Stmt::Print + FunctionDeclaration

**Files:**
- Modify: `crates/dryad_bytecode/tests/increment_tests.rs`

**Step 1: Update dummy_loc()**

Same as Task 1 Step 1.

**Step 2: Replace Stmt::Print (line ~40)**

Find:
```rust
Stmt::Print(
    Expr::PostIncrement(
        Box::new(Expr::Variable("x".to_string(), dummy_loc())),
        dummy_loc(),
    ),
    dummy_loc(),
),
```

Replace with:
```rust
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::PostIncrement(
            Box::new(Expr::Variable("x".to_string(), dummy_loc())),
            dummy_loc(),
        )],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

**Step 3: Replace remaining Stmt::Print instances (line ~54, ~68, etc.)**

Repeat Step 2 for all remaining Stmt::Print → Stmt::Expression(Expr::Call(...)) replacements in file.

**Step 4: Fix FunctionDeclaration params (line ~117)**

Find:
```rust
FunctionDeclaration {
    name: "increment".to_string(),
    params: vec![
        ("x".to_string(), Some(Type::Number)),
    ],
    ...
}
```

Replace with:
```rust
FunctionDeclaration {
    name: "increment".to_string(),
    params: vec![
        ("x".to_string(), Some(Type::Number), None),
    ],
    rest_param: None,
    ...
}
```

Add `rest_param: None,` field if missing.

**Step 5: Compile and verify**

Run: `cargo test -p dryad_bytecode --test increment_tests 2>&1 | tail -20`

Expected: All tests compile successfully.

**Step 6: Commit**

```bash
git add crates/dryad_bytecode/tests/increment_tests.rs
git commit -m "fix: update increment_tests.rs for current AST structure (Stmt::Print, FunctionDeclaration params)"
```

---

## Task 4: Fix loop_tests.rs SourceLocation + Stmt::Print

**Files:**
- Modify: `crates/dryad_bytecode/tests/loop_tests.rs`

**Step 1: Update dummy_loc()**

Same as Task 1 Step 1.

**Step 2: Replace all Stmt::Print instances**

loop_tests.rs has multiple Stmt::Print calls in for-loop and while-loop tests (line ~48, ~109, ~188).

For each, find:
```rust
Stmt::Print(
    Expr::Variable("x".to_string(), dummy_loc()),
    dummy_loc(),
),
```

Replace with:
```rust
Stmt::Expression(
    Expr::Call(
        Box::new(Expr::Variable("print".to_string(), dummy_loc())),
        vec![Expr::Variable("x".to_string(), dummy_loc())],
        dummy_loc(),
    ),
    dummy_loc(),
),
```

(Adjust the expression inside vec![] based on what's being printed in each test.)

**Step 3: Compile and verify**

Run: `cargo test -p dryad_bytecode --test loop_tests 2>&1 | tail -20`

Expected: All tests compile successfully.

**Step 4: Commit**

```bash
git add crates/dryad_bytecode/tests/loop_tests.rs
git commit -m "fix: update loop_tests.rs for current AST structure (SourceLocation, Stmt::Print)"
```

---

## Task 5: Verify all tests compile and run full suite

**Files:**
- Affected: All 6 test files

**Step 1: Run full bytecode test suite**

Run: `cargo test -p dryad_bytecode 2>&1 | tail -30`

Expected: All test files compile successfully. Some tests may fail at runtime (if print() not defined), but compilation must pass.

**Step 2: Check library (non-test) compilation**

Run: `cargo test -p dryad_bytecode --lib 2>&1 | tail -5`

Expected: All library tests pass (10 passed).

**Step 3: Document results**

Create a summary of test results:
- class_tests.rs: X passed, Y failed
- array_tests.rs: X passed, Y failed
- function_tests.rs: X passed, Y failed
- exception_tests.rs: X passed, Y failed
- increment_tests.rs: X passed, Y failed
- loop_tests.rs: X passed, Y failed

**Step 4: Final commit**

```bash
git add -A
git commit -m "fix: all bytecode tests now compile with updated AST structure"
```

---

## Execution Path

**Next Steps:**
1. Execute Tasks 1-5 sequentially
2. After each task: compile and verify before moving to next
3. Commit frequently for easy rollback if errors occur
4. Upon completion: run full test suite and document results in TODO list

**Expected Duration:** 30-45 minutes for manual fixes + compilation verification

**Rollback:** If any task fails during compilation:
```bash
git checkout -- <filename>
```

Then restart that task.
