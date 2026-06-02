# Dryad Language Improvements Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Improve Dryad language to overcome current limitations that prevent writing clean graphics library code.

**Architecture:** The plan addresses 5 core language issues:
1. Division returns float, breaks array indexing
2. Array literal `[]` doesn't work properly
3. Index assignment `arr[i] = x` fails
4. Class syntax issues in certain contexts
5. While condition requires unnecessary parentheses

**Tech Stack:** Rust (Dryad compiler/interpreter), Dryad test files

---

## Problem Summary

During IPE graphics library development, these issues blocked writing clean code:

| Issue | Symptom | Root Cause |
|-------|---------|------------|
| Division | `cols = avail / min_c` returns float, breaks `%` modulo | Division returns `f64`, not integer |
| Array literal | `let arr = []` fails; `push()` works but `arr[0] = x` fails | Parser/interpreter array handling |
| Index assignment | `arr[0] = value` gives "not an object" error | Target evaluation in IndexAssignment |
| Class syntax | `class Foo { }` in certain contexts fails | Parser class handling |
| While parens | `while i < 10 {}` fails, needs `while (i < 10) {}` | Parser requires parentheses |

---

## Task 1: Fix Division to Return Integer

**Files:**
- Modify: `crates/dryad_runtime/src/interpreter.rs:1668-1682`

**Step 1: Write failing test**

Create test file `tests/div_integer.dryad`:
```dryad
let x = 10 / 3;
let arr = [1, 2, 3];
// This should work but fails because x is float
let idx = x;
print(idx);  // Should print 3, not 3.333...
```

**Step 2: Run test to verify failure**

Run: `cargo run --bin dryad -- run tests/div_integer.dryad`
Expected: Should show float value (3.333...)

**Step 3: Implement integer division**

Modify `divide_values` in interpreter.rs:

```rust
fn divide_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err(DryadError::new(3007, "Divisão por zero"))
            } else {
                // TRUNCATE to integer (floor toward zero)
                let result = (a / b).trunc();
                Ok(Value::Number(result))
            }
        }
        _ => Err(DryadError::new(
            3008,
            "Operação '/' só é válida para números",
        )),
    }
}
```

**Step 4: Add operator for float division**

Add new operator `//` for explicit float division in parser and interpreter:

```rust
// In token.rs, add:
// DoubleSlash = "//"

// In parser.rs, add binary operator handling
"//" => Expr::Binary(left, Operator::FloorDiv, right, loc)

// In interpreter.rs, add:
"/" => self.divide_values(left_val, right_val),       // integer div
"//" => self.floating_divide_values(left_val, right_val), // float div

fn floating_divide_values(&self, left: Value, right: Value) -> Result<Value, DryadError> {
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => {
            if b == 0.0 {
                Err(DryadError::new(3007, "Divisão por zero"))
            } else {
                Ok(Value::Number(a / b))
            }
        }
        _ => Err(DryadError::new(3008, "Operação '//' só é válida para números"))
    }
}
```

**Step 5: Run test to verify fix**

Run: `cargo run --bin dryad -- run tests/div_integer.dryad`
Expected: Should print "3"

**Step 6: Commit**

```bash
git add crates/dryad_runtime/src/interpreter.rs
git commit -m "fix: make / return integer, add // for float division"
```

---

## Task 2: Fix Array Literal and Index Assignment

**Files:**
- Modify: `crates/dryad_runtime/src/interpreter.rs:1155` (eval_array)
- Modify: `crates/dryad_runtime/src/interpreter.rs:3924` (execute_index_assignment)
- Modify: `crates/dryad_parser/src/parser.rs`

**Step 1: Write failing test**

Create test file `tests/array_index.dryad`:
```dryad
let arr = [];
arr.push(1);
arr.push(2);
arr[0] = 10;  // This fails
print(arr[0]);  // Should print 10
```

**Step 2: Run test to verify failure**

Run: `cargo run --bin dryad -- run tests/array_index.dryad`
Expected: Error about "not an object"

**Step 3: Debug execute_index_assignment**

The issue is at line 3930:
```rust
let target = self.evaluate(array_expr)?;
```

When `array_expr` is just an identifier like `arr`, `evaluate` returns the VALUE of `arr`, which should be `Value::Array(id)`. But there might be an issue with how the target is being handled.

Add debug logging or check: When assigning `arr[0] = x`, the `array_expr` should evaluate to the array value directly (not copy it).

**Step 4: Fix index assignment for arrays**

The fix should ensure we get a mutable reference to the array:

```rust
fn execute_index_assignment(
    &mut self,
    array_expr: &Expr,
    index_value: Value,
    value: Value,
) -> Result<Value, DryadError> {
    // For identifiers, we need to get the value from environment
    // then modify it in place
    match array_expr {
        Expr::Identifier(name, _) => {
            // Get the array from environment
            let array_val = self.env.get(name).cloned().ok_or_else(|| {
                DryadError::new(3001, &format!("Variável '{}' não definida", name))
            })?;
            
            // Now handle as array
            match array_val {
                Value::Array(id) => {
                    // ... existing array assignment logic
                }
                _ => Err(DryadError::new(3034, "Tentativa de atribuir índice a valor que não é array"))
            }
        }
        // Existing logic for complex expressions
        _ => {
            let target = self.evaluate(array_expr)?;
            // ... existing logic
        }
    }
}
```

**Step 5: Run test to verify fix**

Run: `cargo run --bin dryad -- run tests/array_index.dryad`
Expected: Should print "10"

**Step 6: Commit**

```bash
git add crates/dryad_runtime/src/interpreter.rs
git commit -m "fix: array index assignment for identifiers"
```

---

## Task 3: Fix While Condition Parentheses

**Files:**
- Modify: `crates/dryad_parser/src/parser.rs`

**Step 1: Write failing test**

Create test file `tests/while_no_parens.dryad`:
```dryad
let i = 0;
while i < 5 {
    print(i);
    i = i + 1;
}
```

**Step 2: Run test to verify failure**

Run: `cargo run --bin dryad -- run tests/while_no_parens.dryad`
Expected: "Esperado '(' após 'while'"

**Step 3: Find while parsing in parser**

Search for While in parser.rs:

**Step 4: Modify parser to accept both forms**

Find the while statement parsing and modify:

```rust
// Current (requires parens):
Stmt::While(condition, body, loc) => {
    // Parse: while (condition) { body }
}

// Modify to accept optional parentheses:
Stmt::While(condition, body, loc) => {
    // If next token is '(', consume it, parse condition, expect ')'
    // Otherwise, parse condition until '{' or newline
}
```

The key is in parsing the condition:
- If we see `(`, consume it, parse expression, expect `)`
- If we don't see `(`, parse expression until we hit `{` or newline

**Step 5: Run test to verify fix**

Run: `cargo run --bin dryad -- run tests/while_no_parens.dryad`
Expected: Should print 0, 1, 2, 3, 4

**Step 6: Commit**

```bash
git add crates/dryad_parser/src/parser.rs
git commit -m "feat: make while condition parentheses optional"
```

---

## Task 4: Test All Fixes Together

**Step 1: Update layout test**

Update `ipe/tests/test_layout.dryad` to use the new features:

```dryad
// Now should work with integer division
let cols = avail_w / min_c;  // Returns integer now

// Array index assignment should work
let cards = [];
cards.push({"label": "Card 1"});
cards[0]["label"] = "Updated";  // Should work

// While without parens
while i < 6 {
    // ...
}
```

**Step 2: Run the layout test**

Run: `cargo run --bin dryad -- run ipe/tests/test_layout.dryad`
Expected: Should show responsive grid that adapts to window resize

**Step 3: Commit**

```bash
git add ipe/tests/test_layout.dryad
git commit -m "test: update layout demo to use new language features"
```

---

## Execution Options

**Plan complete and saved. Two execution options:**

**1. Subagent-Driven (this session)** - I dispatch fresh subagent per task, review between tasks, fast iteration

**2. Parallel Session (separate)** - Open new session with executing-plans, batch execution with checkpoints

**Which approach?**
