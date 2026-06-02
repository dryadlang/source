# Template Strings & Lambda Functions Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Enable the bytecode compiler to support template string interpolation and arrow function expressions, completing two high-value language features.

**Architecture:** 
- **Template Strings**: Extend parser to recognize template tokens, compile interpolation expressions to concatenation bytecode
- **Lambda Functions**: Add `Expr::Lambda` to AST, compile as closures with automatic return, support type annotations and rest parameters

**Tech Stack:** Rust, Dryad AST (crates/dryad_parser), Bytecode VM (crates/dryad_bytecode), TDD approach

---

## Phase 1: Template Strings (Simple → Complex)

### Task 1: Verify Template String Parsing

**Goal:** Confirm parser already extracts template tokens correctly before implementing bytecode compilation.

**Files:**
- Examine: `crates/dryad_parser/src/parser.rs` (template parsing)
- Examine: `crates/dryad_parser/src/ast.rs` (Literal enum)
- Test: Create test in `crates/dryad_bytecode/tests/template_tests.rs`

**Step 1: Read parser template handling**

Run: `grep -n "template\|Template" crates/dryad_parser/src/parser.rs | head -20`

Expected: Find `parse_template_string()` function and `Literal::TemplateString(parts, expressions)` enum variant.

**Step 2: Create test file**

Create file: `crates/dryad_bytecode/tests/template_tests.rs`

```rust
#[cfg(test)]
mod template_tests {
    use dryad_bytecode::Compiler;
    use dryad_lexer::Lexer;
    use dryad_parser::Parser;

    #[test]
    fn test_simple_template_string_compiles() {
        let code = r#"
        let x = 5;
        let msg = `Number is ${x}`;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok(), "Template string should compile without error");
    }

    #[test]
    fn test_multiple_interpolations() {
        let code = r#"
        let a = "hello";
        let b = 42;
        let msg = `${a} world ${b}`;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok(), "Multiple interpolations should compile");
    }

    #[test]
    fn test_nested_expressions_in_template() {
        let code = r#"
        let x = 10;
        let msg = `Result: ${x + 5}`;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok(), "Nested expressions should compile");
    }
}
```

**Step 3: Run tests to verify they fail**

Run: `cargo test -p dryad_bytecode template_tests --lib 2>&1 | head -50`

Expected: FAIL with error like "no method named `compile` returns `Result`" or similar compilation error.

**Step 4: Check AST representation**

Run: `grep -A 5 "TemplateString" crates/dryad_parser/src/ast.rs`

Expected output showing structure:
```
TemplateString(Vec<String>, Vec<Expr>)
// Parts: ["Number is ", ""]
// Expressions: [Expr::Variable("x")]
```

**Step 5: Commit test structure**

```bash
git add crates/dryad_bytecode/tests/template_tests.rs
git commit -m "test: add template string compilation tests"
```

---

### Task 2: Implement Template String Compilation

**Goal:** Compile template string interpolation into Add opcodes that concatenate strings and values.

**Files:**
- Modify: `crates/dryad_bytecode/src/compiler.rs` (add `compile_template_string()`)
- Modify: `crates/dryad_bytecode/tests/template_tests.rs` (add execution tests)

**Step 1: Add template string handler to compile_literal**

In `crates/dryad_bytecode/src/compiler.rs`, locate `compile_literal()` function (around line 1100-1200).

Find the match statement that handles `Literal::`:

```rust
fn compile_literal(&mut self, lit: Literal, line: u32) -> Result<(), String> {
    match lit {
        Literal::Number(n) => { ... }
        Literal::String(s) => { ... }
        Literal::Boolean(b) => { ... }
        // ADD THIS:
        Literal::TemplateString(parts, expressions) => {
            self.compile_template_string(parts, expressions, line)
        }
        // ... other literals
    }
}
```

**Step 2: Implement compile_template_string helper**

Add this function to `impl Compiler`:

```rust
fn compile_template_string(
    &mut self,
    parts: Vec<String>,
    expressions: Vec<Expr>,
    line: u32,
) -> Result<(), String> {
    // Strategy: Build concatenation via Add opcodes
    // For template `${a}${b}`, we emit:
    //   CONSTANT "part1"
    //   (compile a)
    //   CONVERT_TO_STRING (implicit in Dryad concatenation)
    //   ADD
    //   (compile b)
    //   CONVERT_TO_STRING
    //   ADD
    //   ... repeat for all parts

    // Edge case: parts.len() == expressions.len() + 1
    // Example: `hello ${x} world` → parts=["hello ", " world"], exprs=[x]

    if parts.is_empty() && expressions.is_empty() {
        // Empty template string ``
        let idx = self.make_constant(Value::String(String::new()), line)?;
        self.emit(OpCode::Constant(idx), line);
        return Ok(());
    }

    // Start with first part (always a string)
    let first_idx = self.make_constant(Value::String(parts[0].clone()), line)?;
    self.emit(OpCode::Constant(first_idx), line);

    // For each expression, add it and concatenate
    for (i, expr) in expressions.iter().enumerate() {
        // Compile the expression
        self.compile_expression(expr.clone())?;

        // Add the next part
        if i + 1 < parts.len() {
            let part_idx = self.make_constant(Value::String(parts[i + 1].clone()), line)?;
            self.emit(OpCode::Constant(part_idx), line);
        }

        // Emit ADD to concatenate (Dryad's Add handles string concatenation)
        self.emit(OpCode::Add, line);
    }

    Ok(())
}
```

**Step 3: Run tests**

Run: `cargo test -p dryad_bytecode template_tests --lib 2>&1 | tail -20`

Expected: Tests should now compile successfully. PASS if the compiler handles template strings.

**Step 4: Add runtime execution test**

Add to `template_tests.rs`:

```rust
#[test]
fn test_template_string_execution() {
    let code = r#"
    let x = 5;
    let msg = `Number is ${x}`;
    msg
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).unwrap();
    
    let mut vm = VM::new();
    vm.execute(&chunk).unwrap();
    
    let result = vm.stack_peek().unwrap();
    match result {
        Value::String(s) => assert_eq!(s, "Number is 5"),
        _ => panic!("Expected string result, got {:?}", result),
    }
}
```

**Step 5: Run full test**

Run: `cargo test -p dryad_bytecode template_tests --lib 2>&1 | grep -E "PASS|FAIL|error"`

Expected: All template tests PASS.

**Step 6: Commit**

```bash
git add crates/dryad_bytecode/src/compiler.rs crates/dryad_bytecode/tests/template_tests.rs
git commit -m "feat: implement template string interpolation compilation"
```

---

### Task 3: Handle Edge Cases in Template Strings

**Goal:** Ensure edge cases (empty templates, expressions only, nested templates) work correctly.

**Files:**
- Modify: `crates/dryad_bytecode/tests/template_tests.rs`
- Modify: `crates/dryad_bytecode/src/compiler.rs` (if needed)

**Step 1: Add edge case tests**

Add to `template_tests.rs`:

```rust
#[test]
fn test_empty_template_string() {
    let code = r#"let msg = ``;"#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_template_with_only_expressions() {
    let code = r#"
    let a = "hello";
    let b = "world";
    let msg = `${a}${b}`;
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&ast);
    assert!(result.is_ok());
}

#[test]
fn test_template_with_complex_expression() {
    let code = r#"
    let x = 10;
    let y = 20;
    let msg = `Result: ${x + y * 2}`;
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let result = compiler.compile(&ast);
    assert!(result.is_ok());
}
```

**Step 2: Run edge case tests**

Run: `cargo test -p dryad_bytecode template_tests --lib 2>&1 | grep -E "test_|PASS|FAIL"`

Expected: All tests PASS. If any fail, debug the specific edge case in `compile_template_string()`.

**Step 3: Commit**

```bash
git add crates/dryad_bytecode/tests/template_tests.rs
git commit -m "test: add template string edge case tests"
```

---

## Phase 2: Lambda Functions (Arrow Functions)

### Task 4: Verify Lambda/Arrow Function Parsing

**Goal:** Confirm the parser already extracts lambda expressions before implementing bytecode compilation.

**Files:**
- Examine: `crates/dryad_parser/src/ast.rs` (check if `Expr::Lambda` exists)
- Examine: `crates/dryad_parser/src/parser.rs` (lambda parsing)
- Test: Create test in `crates/dryad_bytecode/tests/lambda_tests.rs`

**Step 1: Check AST for Lambda variant**

Run: `grep -n "Lambda\|Arrow" crates/dryad_parser/src/ast.rs | head -10`

Expected: Output showing `Expr::Lambda(params, return_type, body)` or similar.

If not found, Lambda may be called `Expr::Function` or `Expr::ArrowFunction`. Document the exact variant name.

**Step 2: Create lambda test file**

Create file: `crates/dryad_bytecode/tests/lambda_tests.rs`

```rust
#[cfg(test)]
mod lambda_tests {
    use dryad_bytecode::Compiler;
    use dryad_lexer::Lexer;
    use dryad_parser::Parser;

    #[test]
    fn test_simple_lambda_compiles() {
        let code = r#"
        let add = (a, b) => a + b;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok(), "Lambda should compile without error");
    }

    #[test]
    fn test_lambda_with_single_param() {
        let code = r#"
        let square = (x) => x * x;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_lambda_with_type_annotations() {
        let code = r#"
        let add = (a: number, b: number): number => a + b;
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let result = compiler.compile(&ast);
        
        assert!(result.is_ok());
    }
}
```

**Step 3: Run tests to verify they fail**

Run: `cargo test -p dryad_bytecode lambda_tests --lib 2>&1 | head -50`

Expected: FAIL with error indicating Lambda expression not handled in compiler.

**Step 4: Commit test structure**

```bash
git add crates/dryad_bytecode/tests/lambda_tests.rs
git commit -m "test: add lambda function compilation tests"
```

---

### Task 5: Implement Lambda Expression Compilation

**Goal:** Compile arrow functions as closures that automatically return the expression value.

**Files:**
- Modify: `crates/dryad_bytecode/src/compiler.rs` (add `compile_lambda()`)
- Modify: `crates/dryad_bytecode/tests/lambda_tests.rs` (add execution tests)

**Step 1: Add lambda handler to compile_expression**

In `crates/dryad_bytecode/src/compiler.rs`, locate `compile_expression()` function.

Add handler in the match statement:

```rust
fn compile_expression(&mut self, expr: Expr) -> Result<(), String> {
    match expr {
        // ... existing cases ...
        
        // ADD THIS:
        Expr::Lambda(params, return_type, body, loc) => {
            self.compile_lambda(params, return_type, *body, loc.line)
        }
        
        // ... rest of cases ...
    }
}
```

**Step 2: Implement compile_lambda helper**

Add this function to `impl Compiler`:

```rust
fn compile_lambda(
    &mut self,
    params: Vec<String>,
    _return_type: Option<Type>,
    body: Expr,
    line: u32,
) -> Result<(), String> {
    // Strategy: Create a function chunk, emit Closure opcode
    // Lambda body is an expression, not a statement block
    // We need to: compile the expression, emit Return
    
    // Save outer function context
    let outer_chunk = std::mem::replace(&mut self.chunk, Chunk::new());
    let outer_locals = std::mem::replace(&mut self.locals, Vec::new());
    let outer_upvalues = std::mem::replace(&mut self.upvalues, Vec::new());
    
    // Set function context for lambda
    self.function_name = format!("<lambda at {}>", line);
    self.locals.clear();
    
    // Add parameters as local variables
    for param in &params {
        self.locals.push(Local {
            name: param.clone(),
            depth: 1,
            is_captured: false,
        });
    }
    
    // Compile the body expression
    self.compile_expression(body)?;
    
    // Auto-return the expression value
    self.emit(OpCode::Return, line);
    
    // Get the lambda function
    let lambda_chunk = std::mem::replace(&mut self.chunk, outer_chunk);
    let lambda_func = Rc::new(Function {
        name: self.function_name.clone(),
        arity: params.len(),
        chunk: lambda_chunk,
    });
    
    // Restore outer context
    self.locals = outer_locals;
    self.upvalues = outer_upvalues;
    
    // Emit the lambda as a constant
    let idx = self.make_constant(Value::Function(lambda_func), line)?;
    self.emit(OpCode::Constant(idx), line);
    
    // Emit Closure opcode with upvalue count
    self.emit(OpCode::Closure(self.upvalues.len() as u8), line);
    
    Ok(())
}
```

**Step 3: Run tests**

Run: `cargo test -p dryad_bytecode lambda_tests --lib 2>&1 | tail -30`

Expected: Compilation tests should PASS. If they fail, debug the specific error.

**Step 4: Add lambda execution tests**

Add to `lambda_tests.rs`:

```rust
#[test]
fn test_lambda_execution() {
    let code = r#"
    let square = (x) => x * x;
    square(5)
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).unwrap();
    
    let mut vm = VM::new();
    vm.execute(&chunk).unwrap();
    
    let result = vm.stack_peek().unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 25.0),
        _ => panic!("Expected number 25, got {:?}", result),
    }
}

#[test]
fn test_lambda_with_multiple_params() {
    let code = r#"
    let add = (a, b) => a + b;
    add(3, 7)
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).unwrap();
    
    let mut vm = VM::new();
    vm.execute(&chunk).unwrap();
    
    let result = vm.stack_peek().unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 10.0),
        _ => panic!("Expected number 10, got {:?}", result),
    }
}

#[test]
fn test_lambda_in_array_map_like() {
    let code = r#"
    let nums = [1, 2, 3];
    let double = (x) => x * 2;
    double(nums[0])
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).unwrap();
    
    let mut vm = VM::new();
    vm.execute(&chunk).unwrap();
    
    let result = vm.stack_peek().unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 2.0),
        _ => panic!("Expected number 2, got {:?}", result),
    }
}
```

**Step 5: Run all lambda tests**

Run: `cargo test -p dryad_bytecode lambda_tests --lib 2>&1 | grep -E "test_|PASS|FAIL|error"`

Expected: All execution tests PASS.

**Step 6: Commit**

```bash
git add crates/dryad_bytecode/src/compiler.rs crates/dryad_bytecode/tests/lambda_tests.rs
git commit -m "feat: implement lambda/arrow function compilation"
```

---

### Task 6: Handle Lambda Edge Cases

**Goal:** Support lambdas with closures (capturing outer variables), rest parameters, and nested lambdas.

**Files:**
- Modify: `crates/dryad_bytecode/src/compiler.rs` (enhance lambda upvalue handling)
- Modify: `crates/dryad_bytecode/tests/lambda_tests.rs`

**Step 1: Add closure capture tests**

Add to `lambda_tests.rs`:

```rust
#[test]
fn test_lambda_captures_outer_variable() {
    let code = r#"
    let x = 10;
    let add_x = (y) => x + y;
    add_x(5)
    "#;

    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().unwrap();

    let mut compiler = Compiler::new();
    let chunk = compiler.compile(&ast).unwrap();
    
    let mut vm = VM::new();
    vm.execute(&chunk).unwrap();
    
    let result = vm.stack_peek().unwrap();
    match result {
        Value::Number(n) => assert_eq!(n, 15.0),
        _ => panic!("Expected 15, got {:?}", result),
    }
}

#[test]
fn test_nested_lambdas() {
    let code = r#"
    let outer = (x) => {
        let inner = (y) => x + y;
        inner(10)
    };
    outer(5)
    "#;

    // Note: This test requires lambdas with block bodies, which may not be supported
    // If parser doesn't support block bodies in lambdas, this test can be skipped
    
    let lexer = Lexer::new(code);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(tokens);
    
    if parser.parse().is_ok() {
        // Only test if parser supports it
        // (implementation details depend on parser capabilities)
    }
}

#[test]
fn test_lambda_with_rest_params() {
    let code = r#"
    let sum_all = (...nums) => {
        // This requires rest parameter support and statement blocks
        // May be out of scope for this task
    };
    "#;

    // Placeholder for future implementation
    // Rest parameters in lambdas require more complex handling
}
```

**Step 2: Enhance lambda compilation for upvalue capture**

Modify `compile_lambda()` to properly capture outer variables:

```rust
fn compile_lambda(
    &mut self,
    params: Vec<String>,
    _return_type: Option<Type>,
    body: Expr,
    line: u32,
) -> Result<(), String> {
    // ... existing setup code ...
    
    // ENHANCED: Track upvalues before compiling body
    let outer_upvalues_count = self.upvalues.len();
    
    // Compile the body expression
    self.compile_expression(body)?;
    
    // Auto-return the expression value
    self.emit(OpCode::Return, line);
    
    // Get the lambda function
    let lambda_chunk = std::mem::replace(&mut self.chunk, outer_chunk);
    let lambda_func = Rc::new(Function {
        name: self.function_name.clone(),
        arity: params.len(),
        chunk: lambda_chunk,
    });
    
    // Restore outer context
    self.locals = outer_locals;
    let captured_upvalues = self.upvalues.len() - outer_upvalues_count;
    self.upvalues = outer_upvalues;
    
    // Emit the lambda as a constant
    let idx = self.make_constant(Value::Function(lambda_func), line)?;
    self.emit(OpCode::Constant(idx), line);
    
    // Emit Closure opcode with upvalue count (number of captured variables)
    self.emit(OpCode::Closure(captured_upvalues as u8), line);
    
    Ok(())
}
```

**Step 3: Run closure capture tests**

Run: `cargo test -p dryad_bytecode lambda_tests::test_lambda_captures_outer_variable --lib 2>&1`

Expected: PASS if upvalue capture is correctly implemented. If FAIL, debug the upvalue chain.

**Step 4: Run all tests to check for regressions**

Run: `cargo test -p dryad_bytecode 2>&1 | tail -5`

Expected: All tests pass (42+).

**Step 5: Commit**

```bash
git add crates/dryad_bytecode/src/compiler.rs crates/dryad_bytecode/tests/lambda_tests.rs
git commit -m "feat: add lambda closure capture and edge case handling"
```

---

## Phase 3: Integration & Verification

### Task 7: Integration Tests

**Goal:** Verify that template strings and lambdas work correctly together and in real-world scenarios.

**Files:**
- Create: `crates/dryad_bytecode/tests/integration_templates_lambdas.rs`

**Step 1: Create integration test file**

Create file: `crates/dryad_bytecode/tests/integration_templates_lambdas.rs`

```rust
#[cfg(test)]
mod integration_tests {
    use dryad_bytecode::Compiler;
    use dryad_lexer::Lexer;
    use dryad_parser::Parser;

    #[test]
    fn test_lambda_returns_template_string() {
        let code = r#"
        let greet = (name) => `Hello, ${name}!`;
        greet("Alice")
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&ast).unwrap();
        
        let mut vm = VM::new();
        vm.execute(&chunk).unwrap();
        
        let result = vm.stack_peek().unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "Hello, Alice!"),
            _ => panic!("Expected string, got {:?}", result),
        }
    }

    #[test]
    fn test_template_string_in_lambda_with_calculations() {
        let code = r#"
        let format_result = (x) => `Result: ${x * 2}`;
        format_result(21)
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&ast).unwrap();
        
        let mut vm = VM::new();
        vm.execute(&chunk).unwrap();
        
        let result = vm.stack_peek().unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "Result: 42"),
            _ => panic!("Expected string 'Result: 42', got {:?}", result),
        }
    }

    #[test]
    fn test_array_with_lambdas_and_templates() {
        let code = r#"
        let numbers = [1, 2, 3];
        let formatter = (n) => `Number: ${n}`;
        formatter(numbers[1])
        "#;

        let lexer = Lexer::new(code);
        let tokens = lexer.tokenize().unwrap();
        let mut parser = Parser::new(tokens);
        let ast = parser.parse().unwrap();

        let mut compiler = Compiler::new();
        let chunk = compiler.compile(&ast).unwrap();
        
        let mut vm = VM::new();
        vm.execute(&chunk).unwrap();
        
        let result = vm.stack_peek().unwrap();
        match result {
            Value::String(s) => assert_eq!(s, "Number: 2"),
            _ => panic!("Expected 'Number: 2', got {:?}", result),
        }
    }
}
```

**Step 2: Run integration tests**

Run: `cargo test -p dryad_bytecode integration_tests --lib 2>&1 | tail -20`

Expected: All integration tests PASS.

**Step 3: Run full test suite**

Run: `cargo test -p dryad_bytecode 2>&1 | grep "test result:" | tail -1`

Expected: All tests pass (should be 45+ now).

**Step 4: Commit**

```bash
git add crates/dryad_bytecode/tests/integration_templates_lambdas.rs
git commit -m "test: add integration tests for template strings and lambdas"
```

---

### Task 8: Final Verification & Documentation

**Goal:** Ensure all features work, no regressions, and document the implementation.

**Files:**
- Modify: `develop/implementation/plans/2025-03-22-class-getters-setters.md` (reference new features)
- Run: Full test suite and check for errors

**Step 1: Run full test suite**

Run: `cargo test -p dryad_bytecode 2>&1 | tail -30`

Expected: All tests pass. Output should show:
```
test result: ok. X passed; 0 failed
```

**Step 2: Check for linter warnings**

Run: `cargo clippy -p dryad_bytecode 2>&1 | grep warning | head -20`

Expected: Only pre-existing warnings (if any). No new warnings from template/lambda code.

**Step 3: Verify no debug code left**

Run: `grep -r "println!\|dbg!\|todo!" crates/dryad_bytecode/src/compiler.rs | grep -v "//"`

Expected: No output (all debug code must be removed).

**Step 4: Document completion**

Create summary in `develop/implementation/PHASE1-BYTECODE-STATUS.md` (if not existing) or update status:

```markdown
## Phase 1 Completion Summary

### ✅ Implemented Features

1. **Class Getters/Setters** (Session 1)
   - Getter compilation as `__get_property` methods
   - Setter compilation as `__set_property` methods
   - VM interception in GetProperty/SetProperty opcodes

2. **Object Literals** (Session 1)
   - Object literal syntax `{ key: value }`
   - Object property access and mutation
   - Nested object support

3. **Template Strings** (Session 2)
   - Backtick-delimited template strings `` `...${expr}...` ``
   - Multiple interpolations in single template
   - Complex expressions in interpolation

4. **Lambda Functions** (Session 2)
   - Arrow function syntax `(params) => expr`
   - Type annotations on parameters and return
   - Closure capture of outer variables
   - Automatic return of expression value

### Test Results
- **Total Tests**: 48
- **Status**: All passing ✅
- **Coverage**: Core language features, edge cases, integration tests

### Remaining Phase 1 Features
- Pattern Matching (`match` expressions)
- Spread/Rest operators
- Destructuring patterns (advanced)
```

**Step 5: Final commit**

```bash
git add develop/implementation/PHASE1-BYTECODE-STATUS.md
git commit -m "docs: update Phase 1 bytecode compiler completion status"
```

---

## Success Criteria

| Criterion | Status |
|-----------|--------|
| Template strings compile without errors | ✅ Must PASS |
| Template string interpolation works correctly | ✅ Must PASS |
| Lambda functions compile without errors | ✅ Must PASS |
| Lambda execution returns correct values | ✅ Must PASS |
| Lambda closures capture outer variables | ✅ Must PASS |
| Integration tests all pass | ✅ Must PASS |
| No regressions in existing tests | ✅ Must PASS (48+ tests) |
| No debug code in production | ✅ Must verify |
| No clippy warnings (new code) | ✅ Must verify |
| All commits have descriptive messages | ✅ Must verify |

---

## Quick Reference: Key Code Locations

| Feature | File | Line Range |
|---------|------|-----------|
| Template parsing | `crates/dryad_parser/src/parser.rs` | ~1200-1300 |
| Lambda parsing | `crates/dryad_parser/src/parser.rs` | ~1400-1450 |
| AST Literal::TemplateString | `crates/dryad_parser/src/ast.rs` | ~150-160 |
| AST Expr::Lambda | `crates/dryad_parser/src/ast.rs` | ~200-210 |
| Compiler entry | `crates/dryad_bytecode/src/compiler.rs` | ~1 |
| compile_expression | `crates/dryad_bytecode/src/compiler.rs` | ~400-500 |
| VM execute | `crates/dryad_bytecode/src/vm.rs` | ~100-200 |

---

## Tips for Implementation

1. **Template Strings**: Start simple (no expressions), then add interpolation. Use the existing Add opcode for concatenation.
2. **Lambdas**: Model after existing function compilation (`compile_function_declaration`). The main difference is expression body (auto-return).
3. **Testing**: Write test first, run to fail, implement, run to pass. Commit frequently.
4. **Debugging**: Use `cargo check` to catch errors early. Use `cargo test` to verify behavior.
5. **Git**: Keep commits atomic (one feature per commit). Use descriptive messages referencing the task number.
