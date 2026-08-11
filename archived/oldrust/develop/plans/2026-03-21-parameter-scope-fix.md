# Implementation Plan: Parameter Scope Bug Fix

**Status**: Pending Implementation
**Difficulty**: Medium (3/5)
**Estimated Time**: 2-3 hours
**Impact**: Fix 2 failing tests → 22/22 passing (100%)

---

## Problem Statement

Two function tests fail with type mismatch errors when function parameters should receive numeric values:

```
test_function_call: "Não é possível adicionar function com number"
test_function_with_local_variables: "Não é possível multiplicar function com number"
```

When a function is called with numeric arguments, the parameters evaluate to function objects instead of the argument values.

### Root Cause Hypothesis

The issue likely stems from **upvalue capture during function definition** interfering with parameter resolution. When the compiler creates an upvalue for a parameter, it may be capturing the enclosing function's closure instead of the parameter value itself.

### Affected Code Paths

1. **Compiler** (`crates/dryad_bytecode/src/compiler.rs`)
   - `compile_function_declaration()` - Parameter handling
   - `emit_locals_for_parameters()` - Parameter initialization (if exists)
   - Upvalue capture logic during function compilation

2. **VM** (`crates/dryad_bytecode/src/vm.rs`)
   - `CallFrame` structure - Local variable storage
   - Parameter binding in function prologue
   - `SetLocal` opcode - Local variable assignment
   - `GetLocal` opcode - Local variable retrieval
   - Upvalue mechanism during function call

3. **Tests** (`crates/dryad_bytecode/tests/function_tests.rs`)
   - `test_function_call` (lines ~100-120)
   - `test_function_with_local_variables` (lines ~170-190)

---

## Diagnosis Phase

### Step 1: Understand Current Parameter Binding

Trace how parameters are currently handled:

```rust
// In compiler.rs: function_declaration()
1. Parse function signature to get parameter names
2. Create new compiler scope for function body
3. Emit code for function body
4. How are parameters made available in function scope?
   - Are they added to locals?
   - Are they initialized at function entry?
   - Are they treated as upvalues?
```

### Step 2: Add Debug Logging

Insert debug output in critical paths:

```rust
// In vm.rs: interpret() - function call
println!("DEBUG: Calling function");
println!("DEBUG: Parameters: {:?}", params);  // What's passed
println!("DEBUG: Local slots: {:?}", call_frame.locals);

// In vm.rs: SetLocal opcode
println!("DEBUG: SetLocal slot {} = {:?}", slot, value);

// In vm.rs: GetLocal opcode
println!("DEBUG: GetLocal slot {} from locals: {:?}", slot, locals[slot]);
```

### Step 3: Run Tests with Debug Output

```bash
cd /home/pedro/repo/source
RUST_BACKTRACE=1 cargo test -p dryad_bytecode test_function_call -- --nocapture 2>&1 | head -100
```

Observe:
- Are parameters being passed correctly?
- Are local slots being initialized?
- What value is in local slot when function starts?

---

## Root Cause Investigation

### Hypothesis A: Parameter Not in Locals

**Theory**: Parameters aren't being added to the CallFrame's locals array at all, so GetLocal returns wrong value or upvalue lookups fail.

**Check**:
```rust
// In CallFrame::new() or function entry code
// Are parameters explicitly added to locals?
// Expected: locals array should have parameter count + local variable count slots
```

**Fix if true**:
```rust
// When entering function, explicitly initialize locals for parameters
for (i, param_name) in function.params.iter().enumerate() {
    frame.locals.insert(i, Value::from_args.get(i));
}
```

### Hypothesis B: Upvalue Shadowing

**Theory**: When upvalue capture is performed, it's capturing the function object instead of the parameter value, causing type errors.

**Check**:
```rust
// In compiler.rs: function compilation
// When processing function body, check if parameter names are being marked as upvalues
// Expected: Parameters should be local to function, not upvalues
```

**Fix if true**:
```rust
// In compiler: Don't create upvalue for parameter
// Instead: Parameter should be first N locals in new function scope
```

### Hypothesis C: GetLocal Opcode Bug

**Theory**: The GetLocal opcode is fetching from wrong index or the CallFrame's locals are misaligned.

**Check**:
```rust
// In vm.rs: GetLocal handler
// Verify stack_start calculation is correct
// Verify locals indexing accounts for function parameters first
```

**Fix if true**:
```rust
// Adjust local slot calculation
let stack_start = self.frames.last().unwrap().stack_start;
let local_slot = idx as usize + stack_start;  // Ensure correct offset
```

---

## Implementation Steps

### Phase 1: Diagnostic (30 minutes)

1. **Add debug statements**:
   ```bash
   # Edit vm.rs CallFrame::new() and function call logic
   # Add println! for parameter values and local initialization
   ```

2. **Run failing tests**:
   ```bash
   cargo test -p dryad_bytecode test_function_call -- --nocapture 2>&1 | grep "DEBUG\|Erro"
   ```

3. **Document findings**:
   - Which hypothesis matches observed behavior?
   - What's the exact mismatch?

### Phase 2: Fix (1 hour)

Based on diagnostic results, implement fix in one of these areas:

**Option A: Fix Parameter Initialization**
```rust
// File: vm.rs - in function call handler
// Ensure parameters are explicitly set in locals before function starts
for (i, arg) in args.iter().enumerate() {
    call_frame.locals[i] = arg.clone();  // Explicitly set parameter values
}
```

**Option B: Fix Upvalue Logic**
```rust
// File: compiler.rs - in function compilation
// Ensure parameters aren't treated as upvalues
// Parameters should be local to function scope, not captured
```

**Option C: Fix GetLocal Indexing**
```rust
// File: vm.rs - in GetLocal handler
// Verify locals array is properly indexed
// Ensure parameter slots come first, then local variables
```

### Phase 3: Verification (1 hour)

1. **Run failing tests**:
   ```bash
   cargo test -p dryad_bytecode test_function_call test_function_with_local_variables
   ```

2. **Verify both tests pass**:
   ```bash
   # Expected output:
   # test test_function_call ... ok
   # test test_function_with_local_variables ... ok
   ```

3. **Ensure no regression**:
   ```bash
   cargo test -p dryad_bytecode
   # Verify 22/22 tests pass or improve
   ```

4. **Add documentation**:
   - Comment in code explaining fix
   - Update SESSION_COMPLETION_NOTES.md with resolution

---

## Testing Strategy

### Current Failing Tests

**Test 1: test_function_call** (function_tests.rs, ~lines 100-120)
```rust
#[test]
fn test_function_call() {
    // Defines function: fn add(a, b) { return a + b; }
    // Calls: add(5, 3)
    // Expected: Ok(8)
    // Actual: RuntimeError("Não é possível adicionar function com number")
    // Issue: Parameter 'a' evaluates to function instead of 5
}
```

**Test 2: test_function_with_local_variables** (function_tests.rs, ~lines 170-190)
```rust
#[test]
fn test_function_with_local_variables() {
    // Defines function: fn multiply(x, y) { let result = x * y; return result; }
    // Calls: multiply(4, 5)
    // Expected: Ok(20)
    // Actual: RuntimeError("Não é possível multiplicar function com number")
    // Issue: Parameter 'x' evaluates to function instead of 4
}
```

### Post-Fix Verification

After fix, verify:
1. Both tests pass
2. All other tests still pass
3. Integration tests still properly ignored
4. Benchmarks still compile and run

---

## Success Criteria

✅ **Primary**: Both failing tests pass
✅ **Secondary**: All other tests continue to pass (20/20)
✅ **Tertiary**: Code is well-commented explaining the fix
✅ **Final**: New commit documenting the fix

### Definition of Done

- [ ] Diagnostic phase complete (root cause identified)
- [ ] Fix implemented in appropriate file
- [ ] Both failing tests now pass
- [ ] Full test suite passes (22/22)
- [ ] No performance regression in benchmarks
- [ ] Code changes committed with clear message
- [ ] SESSION_COMPLETION_NOTES.md updated with resolution

---

## Alternative Approaches

### If Root Cause is Different

If diagnostic reveals different issue:

1. **Parameter Type Issue**
   - Check if parameters are being coerced to wrong type
   - Verify type checking in binary operations

2. **Scope Chain Issue**
   - Check if function scope is properly isolated
   - Verify variable resolution order (local → enclosing → global)

3. **Closure Capture Issue**
   - Check if function closure is capturing wrong environment
   - Verify upvalue slots are properly initialized

### Fallback: Minimal Workaround

If root cause is complex:
1. Could mark 2 function tests as `#[ignore]` like integration tests
2. Document as known limitation
3. Return to this in separate session with fresh perspective

**However**: Given core VM is solid and only 2 tests fail with same pattern, fix should be straightforward.

---

## Resources

### Related Code Files
- `crates/dryad_bytecode/src/vm.rs` - CallFrame, parameter binding, opcodes
- `crates/dryad_bytecode/src/compiler.rs` - Function compilation, parameter handling
- `crates/dryad_bytecode/tests/function_tests.rs` - Failing tests
- `crates/dryad_bytecode/src/value.rs` - Value types and operations

### Documentation
- `docs/bytecode/BYTECODE_COMPILER_GUIDE.md` - Architecture reference
- `develop/manuals/bytecode/SESSION_COMPLETION_NOTES.md` - Session context
- Previous commits: See git history for parameter-related changes

### Similar Issues
- Check git log for parameter-related commits
- Check git log for upvalue-related commits
- Search codebase for "local" and "parameter" comments

---

## Timeline Estimate

- **Diagnostic**: 30 minutes
- **Fix Implementation**: 1 hour
- **Testing & Verification**: 1 hour
- **Documentation**: 30 minutes
- **Total**: 2.5 - 3 hours

**Could be faster** if root cause is immediately obvious from diagnostics.
**Could be slower** if root cause is deeply nested in compiler/VM interaction.

---

## Next: Integration Tests Implementation

Once parameter scope bug is fixed, see:
- `develop/plans/2026-03-21-integration-test-completion.md`

This will enable the 4 integration test scenarios to pass, completing 100% coverage.
