# Implementation Plan: Integration Tests Completion

**Status**: Pending Implementation
**Difficulty**: Medium-High (4/5)
**Estimated Time**: 4-8 hours (per test scenario)
**Impact**: Enable 4 realistic program scenarios to execute fully

---

## Problem Statement

Four integration test scenarios are marked as `#[ignore]` because they fail during VM execution. These tests document realistic Dryad programs combining multiple language features. Currently failing due to missing or incomplete VM features.

**Goal**: Enable these tests to pass, demonstrating a fully-functional bytecode compiler capable of executing real programs.

---

## Test Inventory

### Test 1: OOP Bank Account System

**Location**: `crates/dryad_bytecode/tests/integration_e2e.rs` (lines 20-202)

**What It Tests**:
- Class declaration with properties
- Class constructor (implicit initialization)
- Instance methods (withdraw, deposit)
- Property access and modification
- Method invocation on instances

**Expected Program Behavior**:
```dryad
class Account {
    balance = 0;  // Default property
    
    constructor(initialBalance) {
        this.balance = initialBalance;
    }
    
    withdraw(amount) {
        if (this.balance >= amount) {
            this.balance = this.balance - amount;
        }
    }
    
    deposit(amount) {
        this.balance = this.balance + amount;
    }
}

let account = new Account(1000);
account.withdraw(200);  // balance = 800
account.deposit(100);   // balance = 900
print(account.balance); // Output: 900
```

**Why It Fails**:
- Class properties with default values not properly initialized
- `SetProperty` opcode may not handle class-level property defaults
- Property initialization should happen in constructor or instance creation

**Required Features**:
1. Class property defaults (ClassMember::Property with default value)
2. SetProperty opcode proper implementation
3. Constructor mechanism (implicit __init__ method)
4. Property access chains (a.b.c)

**Implementation Steps**:
1. **Phase 1**: Implement ClassMember::Property defaults
   - Modify compiler to emit property initialization code
   - Store default values in class object
   
2. **Phase 2**: Ensure SetProperty works on instances
   - Verify SetProperty opcode handles Instance type
   - Add SetProperty for class properties during instantiation

3. **Phase 3**: Constructor support
   - Add implicit constructor generation
   - Or implement explicit `constructor` method

4. **Phase 4**: Test and verify
   - Run: `cargo test -p dryad_bytecode --test integration_e2e test_e2e_realistic_oop_program`
   - Expected: test passes

---

### Test 2: Complex Control Flow

**Location**: `crates/dryad_bytecode/tests/integration_e2e.rs` (lines 205-301)

**What It Tests**:
- Nested loops (while inside for)
- Conditional statements with complex expressions
- Arithmetic operators (%, ==)
- Variable mutations in loops

**Expected Program Behavior**:
```dryad
let sum = 0;
for (let i = 1; i <= 20; i = i + 1) {
    if (i % 2 == 0) {
        sum = sum + i;  // Add even numbers
    }
}
print(sum);  // Output: 110 (2+4+6+8+10+12+14+16+18+20)
```

**Why It Fails**:
- Nested control flow might not properly maintain loop state
- Modulo operator precision or implementation issue
- Break/continue scoping in nested loops

**Required Features**:
1. Nested loop support with proper state management
2. Modulo operator (%) correctness
3. Complex boolean expressions in if conditions
4. Variable mutation in loop bodies

**Implementation Steps**:
1. **Phase 1**: Verify loop state management
   - Check CallFrame or VM state during nested loops
   - Ensure loop counters don't interfere

2. **Phase 2**: Fix modulo operator if needed
   - Test: `5 % 2` should be `1`
   - Verify precision for various number types

3. **Phase 3**: Verify conditional evaluation
   - Check boolean expression evaluation
   - Ensure comparison operators work in complex expressions

4. **Phase 4**: Test and verify
   - Run: `cargo test -p dryad_bytecode --test integration_e2e test_e2e_complex_control_flow`
   - Expected: test passes

---

### Test 3: Exception Handling

**Location**: `crates/dryad_bytecode/tests/integration_e2e.rs` (lines 304-375)

**What It Tests**:
- Try/catch/finally block structure
- Exception throwing and catching
- Finally block execution regardless of exception
- Stack unwinding through function calls

**Expected Program Behavior**:
```dryad
try {
    print("In try block");
    throw "Error message";
    print("After throw");  // Should not execute
} catch (e) {
    print("Caught: " + e);
} finally {
    print("In finally");  // Always executes
}
```

**Why It Fails**:
- Exception propagation through try/catch frames
- Finally block execution timing
- Stack unwinding mechanism
- Exception value binding in catch clause

**Required Features**:
1. TryBegin/TryEnd opcodes implementation
2. Exception propagation in call stack
3. Finally block guaranteed execution
4. Catch variable binding

**Implementation Steps**:
1. **Phase 1**: Review TryFrame structure
   - Check if TryFrame is properly pushed/popped
   - Verify catch_ip and finally_ip are correct

2. **Phase 2**: Implement exception propagation
   - When throw is executed, search TryFrame stack
   - Jump to catch block if found
   - Execute finally before catch if present

3. **Phase 3**: Implement catch variable binding
   - Bind exception value to catch variable name
   - Make available in catch block scope

4. **Phase 4**: Test and verify
   - Run: `cargo test -p dryad_bytecode --test integration_e2e test_e2e_exception_handling`
   - Expected: test passes

---

### Test 4: Array Operations

**Location**: `crates/dryad_bytecode/tests/integration_e2e.rs` (lines 378-449)

**What It Tests**:
- Array creation and literal syntax
- ForEach loop (for-in iteration)
- Array indexing and access
- Accumulation pattern with arrays

**Expected Program Behavior**:
```dryad
let numbers = [1, 2, 3, 4, 5];
let sum = 0;

for (let num in numbers) {
    sum = sum + num;
}

print(sum);  // Output: 15
```

**Why It Fails**:
- ForEach loop not fully implemented
- Array iteration protocol missing
- Index variable handling in for-in loops

**Required Features**:
1. ForEach opcode (or For loop variant for arrays)
2. Array iteration protocol
3. Loop variable binding for each iteration
4. Array indexing in expressions

**Implementation Steps**:
1. **Phase 1**: Implement ForEach support
   - Add ForEach to opcode set if not present
   - Implement VM handler for ForEach
   - Handle iterable objects (arrays, strings)

2. **Phase 2**: Implement array iteration
   - When ForEach encounters array, iterate elements
   - Bind each element to loop variable
   - Execute loop body for each element

3. **Phase 3**: Verify array operations
   - Array literals work
   - Array indexing works
   - Array length available

4. **Phase 4**: Test and verify
   - Run: `cargo test -p dryad_bytecode --test integration_e2e test_e2e_array_operations`
   - Expected: test passes

---

## Priority & Effort Estimation

| Test | Priority | Difficulty | Estimated Hours | Prerequisites |
|------|----------|------------|-----------------|----------------|
| Complex Control Flow | 1 (Low) | 2/5 | 1-2 | None |
| Array Operations | 2 (Medium) | 3/5 | 2-4 | ForEach opcode |
| OOP Bank Account | 3 (Medium) | 4/5 | 3-6 | Property defaults |
| Exception Handling | 4 (High) | 5/5 | 4-8 | Try frame logic |

**Recommended Order**:
1. **First**: Complex Control Flow (simplest, validates loop mechanism)
2. **Second**: Array Operations (builds on loops, adds iteration)
3. **Third**: OOP Bank Account (builds on property system)
4. **Fourth**: Exception Handling (most complex, requires stack unwinding)

---

## Implementation Approach

### Phased Rollout

Each test can be fixed independently. Recommended approach:

```
Week 1:
  Day 1: Complex Control Flow
  Day 2: Array Operations

Week 2:
  Day 3: OOP Bank Account System
  Day 4: Exception Handling
```

### Verification for Each Test

After implementing each test:

```bash
# Run specific test
cargo test -p dryad_bytecode --test integration_e2e test_e2e_<name> -- --nocapture

# Verify no regression
cargo test -p dryad_bytecode

# Run benchmarks to ensure no performance regression
cargo bench -p dryad_bytecode --bench compiler_performance
```

---

## Success Criteria

### For Each Test

- [ ] Test compiles without errors
- [ ] Test executes without panics or runtime errors
- [ ] All assertions pass
- [ ] No output/logging errors
- [ ] All other tests continue to pass

### Overall Goal

- [ ] All 4 integration tests pass
- [ ] Full test suite: 24/24 passing (22 existing + 4 integration)
- [ ] Benchmarks show no performance regression
- [ ] Documentation updated with new capabilities

---

## Risk Mitigation

### Potential Issues

**Issue 1: Feature Interactions**
- Fixing one test might break another
- **Mitigation**: Run full test suite after each fix

**Issue 2: Compiler Changes**
- Changes to compiler might affect bytecode generation
- **Mitigation**: Test old programs still work

**Issue 3: VM Complexity**
- VM logic might get complicated with new features
- **Mitigation**: Add debug logging, keep changes focused

### Rollback Plan

If implementation causes regressions:

```bash
# Revert last commit
git reset --hard HEAD~1

# Identify root cause
# Re-implement with safer approach
```

---

## Definition of Done

### Per Integration Test

1. [ ] Implementation complete
2. [ ] Test passes locally
3. [ ] No regression in existing tests
4. [ ] Benchmarks run successfully
5. [ ] Code committed
6. [ ] Comments added explaining any complex logic

### Overall Completion

1. [ ] All 4 tests passing
2. [ ] Full test suite: 24/24
3. [ ] UPDATE: SESSION_COMPLETION_NOTES.md with final status
4. [ ] All implementation plans executed successfully

---

## Related Plans

### Prerequisites
- `2026-03-21-parameter-scope-fix.md` - Fix parameter binding first
  - Parameter scope fix should be completed before integration test work
  - Otherwise might confuse parameter issues with feature issues

### Related Issues
- SetProperty opcode for class properties
- ForEach loop implementation
- Try/catch/finally exception handling
- Array iteration protocol

---

## Testing Notes

### Test Execution

```bash
# Run single test
cargo test -p dryad_bytecode --test integration_e2e test_e2e_realistic_oop_program -- --nocapture

# Run all integration tests
cargo test -p dryad_bytecode --test integration_e2e

# Remove #[ignore] when ready to enable
# Edit: tests/integration_e2e.rs
#   Remove: #[ignore]
#   Keep: #[test]
```

### Debug Logging

Add debug output while implementing:

```rust
// In vm.rs during execution
println!("DEBUG: ForEach iteration, value={:?}", value);
println!("DEBUG: SetProperty {:?}[{}] = {:?}", obj, key, value);
println!("DEBUG: Exception caught: {:?}", exception);
```

Then remove before final commit.

---

## Timeline

**Optimal Path**: 1-2 weeks for all 4 tests
**Realistic Path**: 2-3 weeks with testing and debugging
**Conservative Path**: 3-4 weeks with buffer for complex issues

---

## Resources

### Code Files
- `crates/dryad_bytecode/tests/integration_e2e.rs` - Test scenarios
- `crates/dryad_bytecode/src/vm.rs` - VM implementation
- `crates/dryad_bytecode/src/compiler.rs` - Compiler implementation
- `crates/dryad_bytecode/src/opcode.rs` - Opcode definitions

### Documentation
- `docs/bytecode/BYTECODE_COMPILER_GUIDE.md` - Architecture
- `develop/manuals/bytecode/SESSION_COMPLETION_NOTES.md` - Context
- `crates/dryad_bytecode/src/opcode.rs` - Opcode details

### References
- Git history for similar features
- Related test files for patterns
- Issue tracker for known limitations

---

## Conclusion

The integration tests represent the next frontier for the bytecode compiler. Enabling them demonstrates:
1. Core VM is production-ready
2. Feature completeness for realistic programs
3. Integration between multiple language features
4. Real-world usage patterns

**Priority**: After fixing parameter scope bug, these tests should be the next implementation focus for the bytecode compiler project.
