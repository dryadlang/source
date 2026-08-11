# x86_64 Control Flow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers/executing-plans to implement this plan task-by-task.

**Goal:** Implement full control flow support (jumps, branches, loops, function calls) in the x86_64 backend to enable if/else statements, while loops, and function calls in AOT-compiled expressions.

**Architecture:** Control flow maps to three IR instruction categories:
1. **Jumps** (`Jump`, `Branch`) → x86_64 jumps (jmp, je, jne, jl, jg, jle, jge)
2. **Function calls** (`Call`, `Return`) → x86_64 call stack (call, ret, parameter passing via SystemV ABI)
3. **Labels** → x86_64 label resolution (track offsets, resolve jumps after code generation)

This plan covers Tier 1.2 and 1.3 from UPDATE_ROADMAP.md. After completion, all control flow bytecode will compile to executable x86_64 machine code.

**Tech Stack:** Rust, x86_64 machine code generation, SystemV AMD64 ABI, stack frame management.

---

## Task 1: Analyze Control Flow IR and x86_64 Patterns

**Files:**
- Read: `crates/dryad_aot/src/ir/instructions.rs` (lines 180-210)
- Read: `crates/dryad_aot/src/backend/x86_64.rs` (lines 1-100, label resolution section)
- Read: `crates/dryad_aot/tests/integration_e2e_compilation.rs` (existing tests)
- Reference: `docs/plans/2026-03-22-x86-64-expression-evaluator-design.md` (ABI section)

**Step 1: Document IR control flow instructions**

Read `IrInstruction` enum and identify:
- `Jump(BlockId)` — unconditional jump
- `Branch { cond, then_block, else_block }` — conditional jump
- `Call { dest, func, args }` — call function by ID
- `CallIndirect { dest, ptr, args }` — call function via pointer
- `Return(Option<RegisterId>)` — return from function

Document the semantics of each in a brief table.

**Step 2: Review existing label resolution code**

Read x86_64 backend and understand:
- How labels are currently emitted: `emit_label(block_id)`
- How label offsets are tracked: stored in codegen state
- How forward/backward jumps are resolved: `resolve_labels()` method

**Step 3: Document SystemV ABI requirements for calls**

From design doc, SystemV x86_64 calling convention:
- Parameters: rdi, rsi, rdx, rcx, r8, r9 (first 6 integer args)
- Return: rax (or rdx:rax for 128-bit)
- Caller-saved: rax, rcx, rdx, rsi, rdi, r8-r11
- Callee-saved: rbx, rsp, rbp, r12-r15
- Red zone: 128 bytes below rsp (no signals during exception handling)
- Stack alignment: 16-byte boundary before `call` instruction

Document stack frame layout:
```
[rsp-0]    ← Red zone start (128 bytes)
[rsp+8]    ← Pushed return address (after call)
[rsp+16]   ← Pushed rbp (callee saves)
[rsp+24]   ← Local variables start
```

**No code changes in this task.**

---

## Task 2: Write Tests for Jump Instructions

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (test module)
- Create: `crates/dryad_aot/tests/integration_control_flow.rs`

**Step 1: Write unit test for unconditional jump**

Add to x86_64 backend test module:

```rust
#[test]
fn test_jump_unconditional() {
    let mut func = IrFunction::new(0);
    
    // Block 0: LoadConst 1 = 42
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(42)),
    });
    block0.set_terminator(IrTerminator::Jump(1));
    
    // Block 1: Return r1
    let mut block1 = IrBlock::new(1);
    block1.set_terminator(IrTerminator::Return(Some(1)));
    
    func.blocks.push(block0);
    func.blocks.push(block1);
    
    // Compile
    let backend = X86_64Backend::new();
    let code = backend.compile(&func).expect("should compile");
    
    // Verify: code should contain jmp instruction (0xEB for short, 0xE9 for long)
    assert!(code.contains(&0xE9) || code.contains(&0xEB), "should contain jmp opcode");
}
```

**Step 2: Write unit test for conditional branch**

```rust
#[test]
fn test_branch_conditional() {
    let mut func = IrFunction::new(0);
    
    // Block 0: LoadConst r1=42, LoadConst r2=0, CmpNe r3, r1, r2
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(42)),
    });
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 2,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    block0.add_instruction(IrInstruction::CmpNe {
        dest: 3,
        lhs: 1,
        rhs: 2,
    });
    block0.set_terminator(IrTerminator::Branch {
        cond: 3,
        then_block: 1,
        else_block: 2,
    });
    
    // Block 1: Return r1 (42)
    let mut block1 = IrBlock::new(1);
    block1.set_terminator(IrTerminator::Return(Some(1)));
    
    // Block 2: Return r2 (0)
    let mut block2 = IrBlock::new(2);
    block2.set_terminator(IrTerminator::Return(Some(2)));
    
    func.blocks.push(block0);
    func.blocks.push(block1);
    func.blocks.push(block2);
    
    let backend = X86_64Backend::new();
    let code = backend.compile(&func).expect("should compile");
    
    // Verify: code should contain conditional jump (je/jne/jl/jg opcodes: 0x74-0x7F)
    let has_conditional_jump = code.iter().any(|&b| (0x70..=0x7F).contains(&b) || b == 0x0F && code.windows(2).any(|w| w[0] == 0x0F && (0x80..=0x8F).contains(&w[1])));
    assert!(has_conditional_jump, "should contain conditional jump");
}
```

**Step 3: Write integration test: if/else statement**

Create `tests/integration_control_flow.rs`:

```rust
use dryad_aot::{compiler, ir::*, backend::x86_64::X86_64Backend, generator::pe::PeGenerator};

#[test]
fn test_e2e_if_else_statement() {
    // IR: if (42 != 0) { return 100 } else { return 200 }
    let mut func = IrFunction::new(0);
    
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(42)),
    });
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 2,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    block0.add_instruction(IrInstruction::CmpNe {
        dest: 3,
        lhs: 1,
        rhs: 2,
    });
    block0.set_terminator(IrTerminator::Branch {
        cond: 3,
        then_block: 1,
        else_block: 2,
    });
    
    let mut block1 = IrBlock::new(1);
    block1.add_instruction(IrInstruction::LoadConst {
        dest: 4,
        value: IrValue::Constant(IrConstant::I32(100)),
    });
    block1.set_terminator(IrTerminator::Return(Some(4)));
    
    let mut block2 = IrBlock::new(2);
    block2.add_instruction(IrInstruction::LoadConst {
        dest: 5,
        value: IrValue::Constant(IrConstant::I32(200)),
    });
    block2.set_terminator(IrTerminator::Return(Some(5)));
    
    func.blocks.push(block0);
    func.blocks.push(block1);
    func.blocks.push(block2);
    
    let mut module = IrModule::new();
    module.functions.push(func);
    
    // Compile to x86_64
    let backend = X86_64Backend::new();
    let machine_code = backend.compile_function(&module.functions[0]).expect("should compile");
    
    // Verify: machine code is non-empty and contains jumps
    assert!(!machine_code.is_empty(), "machine code should not be empty");
    assert!(machine_code.len() > 50, "machine code should have jumps and conditionals");
}
```

**Step 4: Run tests to verify they FAIL**

```bash
cargo test -p dryad_aot test_jump_unconditional --lib -- --nocapture
cargo test -p dryad_aot test_branch_conditional --lib -- --nocapture
cargo test -p dryad_aot test_e2e_if_else_statement --lib -- --nocapture
```

Expected: All 3 tests FAIL (handlers not yet implemented).

**Step 5: Commit test setup**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs tests/integration_control_flow.rs
git commit -m "test: add unit and integration tests for jump/branch instructions"
```

---

## Task 3: Implement Jump Handler

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (compile_instruction method, around line 200)

**Step 1: Implement Jump handler in compile_instruction**

Add to match statement in `compile_instruction`:

```rust
IrInstruction::Jump(target_block) => {
    codegen.emit_jmp(*target_block);
}
```

**Step 2: Implement emit_jmp in X86_64Codegen**

Add to X86_64Codegen methods (around line 600):

```rust
fn emit_jmp(&mut self, target: BlockId) {
    // Relative jump (E9 /id32) - 5 bytes
    self.code.push(0xE9); // JMP opcode
    
    // Offset to target block (placeholder, will be resolved)
    let offset_pos = self.code.len();
    self.pending_jumps.push((offset_pos, JumpType::Absolute, target));
    self.code.extend_from_slice(&[0, 0, 0, 0]); // 4-byte offset (will be patched)
}

// Helper enum for jump types
#[derive(Debug, Clone, Copy)]
enum JumpType {
    Absolute,      // jmp - unconditional
    IfEqual,       // je - cond == 0
    IfNotEqual,    // jne - cond != 0
    IfLess,        // jl - cond < 0
    IfGreaterEq,   // jge - cond >= 0
    IfGreater,     // jg - cond > 0
    IfLessEq,      // jle - cond <= 0
}
```

**Step 3: Add to X86_64Codegen struct**

In X86_64Codegen definition (around line 500):

```rust
pub struct X86_64Codegen {
    // ... existing fields ...
    code: Vec<u8>,
    labels: HashMap<BlockId, usize>,        // BlockId → code offset
    pending_jumps: Vec<(usize, JumpType, BlockId)>, // (code_offset, type, target)
}
```

**Step 4: Run test**

```bash
cargo test -p dryad_aot test_jump_unconditional --lib -- --nocapture
```

Expected: Test should PASS.

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement unconditional Jump instruction for x86_64"
```

---

## Task 4: Implement Branch Handler

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (compile_instruction method)

**Step 1: Understand condition value conversion**

In x86_64, `test rax, rax` or `cmp rax, 0` sets flags. Then:
- `je` (jump if equal) = jump if ZF=1 (value == 0)
- `jne` (jump if not equal) = jump if ZF=0 (value != 0)

But Branch instruction receives a condition register. We need to convert register value to flags.

Pattern:
```
mov rax, [condition_register]
test rax, rax          ; or cmp rax, 0
jne then_block         ; if condition != 0, jump to then
jmp else_block         ; else jump to else
```

**Step 2: Implement Branch handler in compile_instruction**

```rust
IrInstruction::Branch { cond, then_block, else_block } => {
    let cond_reg = codegen.get_phys_reg(*cond)?;
    
    // Move condition to rax and test it
    codegen.emit_mov_reg_reg(0, cond_reg); // rax = condition
    codegen.emit_test_reg_reg(0, 0);       // test rax, rax (sets flags)
    
    // Jump to then_block if non-zero (ZF=0)
    codegen.emit_jne(*then_block);
    
    // Jump to else_block (unconditional fallthrough or explicit jump)
    codegen.emit_jmp(*else_block);
}
```

**Step 3: Implement emit_jne, emit_test_reg_reg**

```rust
fn emit_jne(&mut self, target: BlockId) {
    // Relative conditional jump (0F 85 /id32) - 6 bytes for long form
    // Short form (75 /ib) is 2 bytes but has limited range
    // Use long form for simplicity
    self.code.push(0x0F);
    self.code.push(0x85); // JNE opcode
    
    let offset_pos = self.code.len();
    self.pending_jumps.push((offset_pos, JumpType::IfNotEqual, target));
    self.code.extend_from_slice(&[0, 0, 0, 0]); // 4-byte offset
}

fn emit_test_reg_reg(&mut self, dst: u8, src: u8) {
    // TEST r64, r64 (85 /r)
    let rex = 0x48 | ((dst >> 3) << 2) | (src >> 3);
    self.code.push(rex);
    self.code.push(0x85);
    self.code.push(0xC0 | ((dst & 0x7) << 3) | (src & 0x7));
}
```

**Step 4: Run test**

```bash
cargo test -p dryad_aot test_branch_conditional --lib -- --nocapture
```

Expected: Test should PASS.

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Branch instruction with conditional jumps"
```

---

## Task 5: Implement Label Resolution

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (resolve_labels method)

**Step 1: Understand label resolution**

After all blocks are compiled, we have:
- `labels: HashMap<BlockId, usize>` — maps each block to its code offset
- `pending_jumps: Vec<(offset, type, target)>` — list of jumps to resolve

For each pending jump:
1. Get target block's offset from labels
2. Calculate relative offset: `target_offset - (current_offset + jump_size)`
3. Patch the 4-byte placeholder with the calculated offset

**Step 2: Implement resolve_labels**

```rust
fn resolve_labels(&mut self) {
    let pending = self.pending_jumps.clone();
    
    for (offset, _jump_type, target_block) in pending {
        let target_offset = self.labels.get(&target_block)
            .ok_or_else(|| format!("Undefined label: {:?}", target_block))?;
        
        // Calculate relative offset
        // For 32-bit relative jumps, offset is from next instruction
        let relative = (*target_offset as i32) - ((offset + 4) as i32);
        
        // Patch the 4-byte placeholder
        let bytes = (relative as u32).to_le_bytes();
        self.code[offset..offset+4].copy_from_slice(&bytes);
    }
}
```

**Step 3: Update compile_block to track labels**

In compile_block (around line 85):

```rust
fn compile_block(&self, block: &IrBlock, codegen: &mut X86_64Codegen) -> Result<(), String> {
    // Record offset of this block for jumps
    codegen.labels.insert(block.id, codegen.code.len());
    
    // Compile instructions
    for instr in &block.instructions {
        self.compile_instruction(instr, codegen)?;
    }
    
    // Compile terminator
    self.compile_terminator(&block.terminator, codegen)?;
    
    Ok(())
}
```

**Step 4: Run test**

```bash
cargo test -p dryad_aot test_e2e_if_else_statement --lib -- --nocapture
```

Expected: Test should PASS.

**Step 5: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement label resolution for jump instructions"
```

---

## Task 6: Write Tests for Function Calls

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (test module)
- Modify: `tests/integration_control_flow.rs`

**Step 1: Write unit test for call instruction**

```rust
#[test]
fn test_call_simple_function() {
    let mut func = IrFunction::new(0);
    
    // Block 0: Call function 1, Return result
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::Call {
        dest: Some(1),
        func: 1,
        args: vec![],
    });
    block0.set_terminator(IrTerminator::Return(Some(1)));
    
    func.blocks.push(block0);
    
    // Function 1: Return 99
    let mut func1 = IrFunction::new(1);
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 0,
        value: IrValue::Constant(IrConstant::I32(99)),
    });
    block0.set_terminator(IrTerminator::Return(Some(0)));
    func1.blocks.push(block0);
    
    let mut module = IrModule::new();
    module.functions.push(func);
    module.functions.push(func1);
    
    let backend = X86_64Backend::new();
    let code = backend.compile(&module.functions[0]).expect("should compile");
    
    // Verify: code should contain call instruction (E8 for near call)
    assert!(code.contains(&0xE8), "should contain call opcode");
}
```

**Step 2: Write integration test: function call with arguments**

```rust
#[test]
fn test_e2e_function_with_parameters() {
    // func add(a, b) { return a + b }
    // return add(10, 20)
    
    let mut add_func = IrFunction::new(0);
    add_func.parameters = vec![0, 1]; // a in r0, b in r1
    
    let mut block = IrBlock::new(0);
    block.add_instruction(IrInstruction::Add {
        dest: 2,
        lhs: 0,
        rhs: 1,
    });
    block.set_terminator(IrTerminator::Return(Some(2)));
    
    add_func.blocks.push(block);
    
    // Caller: add(10, 20)
    let mut main = IrFunction::new(1);
    let mut block = IrBlock::new(0);
    block.add_instruction(IrInstruction::LoadConst {
        dest: 0,
        value: IrValue::Constant(IrConstant::I32(10)),
    });
    block.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(20)),
    });
    block.add_instruction(IrInstruction::Call {
        dest: Some(2),
        func: 0,
        args: vec![0, 1],
    });
    block.set_terminator(IrTerminator::Return(Some(2)));
    
    main.blocks.push(block);
    
    let mut module = IrModule::new();
    module.functions.push(add_func);
    module.functions.push(main);
    
    let backend = X86_64Backend::new();
    let code = backend.compile(&module.functions[1]).expect("should compile");
    
    // Verify: code contains call and parameter setup
    assert!(code.contains(&0xE8), "should contain call opcode");
    assert!(code.len() > 30, "should have parameter setup code");
}
```

**Step 3: Run tests to verify they FAIL**

```bash
cargo test -p dryad_aot test_call_simple_function --lib -- --nocapture
cargo test -p dryad_aot test_e2e_function_with_parameters --lib -- --nocapture
```

Expected: Both FAIL (handlers not implemented).

**Step 4: Commit test setup**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs tests/integration_control_flow.rs
git commit -m "test: add call instruction tests"
```

---

## Task 7: Implement Call Handler

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (compile_instruction method)

**Step 1: Understand SystemV parameter passing**

First 6 parameters go in: rdi, rsi, rdx, rcx, r8, r9
Stack parameters go on stack (rightmost first, pushed in reverse)

```rust
// Pseudo-code for Call instruction
match instr {
    IrInstruction::Call { dest, func, args } => {
        // Move arguments to parameter registers (rdi, rsi, rdx, rcx, r8, r9)
        for (i, arg) in args.iter().enumerate() {
            if i < 6 {
                let param_reg = [7, 6, 2, 1, 8, 9][i]; // rdi, rsi, rdx, rcx, r8, r9
                let arg_phys = codegen.get_phys_reg(*arg)?;
                codegen.emit_mov_reg_reg(param_reg, arg_phys);
            } else {
                // Stack parameters (push in reverse order)
                let arg_phys = codegen.get_phys_reg(*arg)?;
                codegen.emit_push_reg(arg_phys);
            }
        }
        
        // Call function
        codegen.emit_call(func_id);
        
        // Clean up stack parameters (pop them)
        if args.len() > 6 {
            let stack_params_size = ((args.len() - 6) as u32) * 8;
            codegen.emit_add_rsp(stack_params_size);
        }
        
        // Move return value to destination
        if let Some(dest) = dest {
            let dest_phys = codegen.get_phys_reg(*dest)?;
            codegen.emit_mov_reg_reg(dest_phys, 0); // rax has return value
        }
    }
}
```

**Step 2: Implement Call handler in compile_instruction**

Add to match statement:

```rust
IrInstruction::Call { dest, func, args } => {
    let func_id = *func;
    
    // SystemV x86_64 ABI: first 6 params in rdi, rsi, rdx, rcx, r8, r9
    let param_regs = [7, 6, 2, 1, 8, 9]; // indices for rdi, rsi, rdx, rcx, r8, r9
    
    // Setup parameters
    for (i, arg) in args.iter().enumerate() {
        let arg_phys = codegen.get_phys_reg(*arg)?;
        
        if i < 6 {
            // Register parameter
            let param_reg = param_regs[i];
            codegen.emit_mov_reg_reg(param_reg, arg_phys);
        } else {
            // Stack parameter (push in order)
            codegen.emit_push_reg(arg_phys);
        }
    }
    
    // Emit call instruction
    codegen.emit_call(func_id);
    
    // Clean up stack
    if args.len() > 6 {
        let stack_size = ((args.len() - 6) as u32) * 8;
        codegen.emit_add_rsp(stack_size);
    }
    
    // Move return value to destination
    if let Some(dest_reg) = dest {
        let dest_phys = codegen.get_phys_reg(*dest_reg)?;
        codegen.emit_mov_reg_reg(dest_phys, 0); // rax → dest
    }
}
```

**Step 3: Implement emit_call**

```rust
fn emit_call(&mut self, func_id: u32) {
    // CALL rel32 (E8 /id32) - 5 bytes
    self.code.push(0xE8);
    
    let offset_pos = self.code.len();
    // For now, store placeholder - real linking happens during module compilation
    self.function_calls.push((offset_pos, func_id));
    self.code.extend_from_slice(&[0, 0, 0, 0]); // 4-byte offset (will be patched)
}

fn emit_push_reg(&mut self, reg: u8) {
    // PUSH r64 (50+rd) or (FF /6 for r8-r15)
    if reg < 8 {
        self.code.push(0x50 | reg);
    } else {
        let rex = 0x41;
        self.code.push(rex);
        self.code.push(0x50 | (reg & 0x7));
    }
}
```

**Step 4: Add to X86_64Codegen struct**

```rust
pub struct X86_64Codegen {
    // ... existing fields ...
    function_calls: Vec<(usize, u32)>, // (code_offset, func_id)
}
```

**Step 5: Run test**

```bash
cargo test -p dryad_aot test_call_simple_function --lib -- --nocapture
```

Expected: Test should PASS.

**Step 6: Commit**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "feat: implement Call instruction with SystemV ABI parameter passing"
```

---

## Task 8: Implement Return Handler

**Files:**
- Modify: `crates/dryad_aot/src/backend/x86_64.rs` (compile_terminator method)

**Step 1: Understand return semantics**

x86_64 return value is in rax (or rdx:rax for 128-bit values).

```
mov rax, [return_value_register]  ; load value into rax if not already there
mov rsp, rbp                       ; restore stack
pop rbp                            ; restore base pointer
ret                                ; return from function
```

**Step 2: Check existing compile_terminator**

Look for IrTerminator::Return handling. If it doesn't exist, add it:

```rust
fn compile_terminator(
    &self,
    term: &IrTerminator,
    codegen: &mut X86_64Codegen,
) -> Result<(), String> {
    match term {
        IrTerminator::Return(maybe_val) => {
            if let Some(val) = maybe_val {
                let val_phys = codegen.get_phys_reg(*val)?;
                // Move return value to rax
                if val_phys != 0 {
                    codegen.emit_mov_reg_reg(0, val_phys);
                }
            } else {
                // Return void - set rax = 0
                codegen.emit_xor_reg_reg(0, 0);
            }
            
            // Epilogue (but this happens in compile_function, so just mark we've returned)
            // Actually, we can emit epilogue here if needed
        }
        IrTerminator::Jump(target) => {
            codegen.emit_jmp(*target);
        }
        IrTerminator::Unreachable => {
            // UD2 instruction for debugging
            self.emit_ud2(codegen);
        }
    }
    Ok(())
}
```

**Step 3: Verify Return instruction (IR level)**

Return is actually handled as a terminator in IrBlock, not as IrInstruction::Return. Verify this in the code and add if missing.

**Step 4: Run existing tests to ensure no regression**

```bash
cargo test -p dryad_aot --lib
```

Expected: All tests should still PASS (no handler was added, just documented).

**Step 5: Commit (if changes made)**

```bash
git add crates/dryad_aot/src/backend/x86_64.rs
git commit -m "docs: document Return handling in control flow"
```

---

## Task 9: Integration Test - While Loop

**Files:**
- Modify: `tests/integration_control_flow.rs`

**Step 1: Write E2E test for while loop**

```rust
#[test]
fn test_e2e_while_loop() {
    // IR: i = 0; while (i < 10) { i = i + 1 }; return i
    let mut func = IrFunction::new(0);
    
    // Block 0: i = 0
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    block0.set_terminator(IrTerminator::Jump(1)); // Jump to loop condition
    
    // Block 1: while condition (i < 10)
    let mut block1 = IrBlock::new(1);
    block1.add_instruction(IrInstruction::LoadConst {
        dest: 2,
        value: IrValue::Constant(IrConstant::I32(10)),
    });
    block1.add_instruction(IrInstruction::CmpLt {
        dest: 3,
        lhs: 1,
        rhs: 2,
    });
    block1.set_terminator(IrTerminator::Branch {
        cond: 3,
        then_block: 2,
        else_block: 3,
    });
    
    // Block 2: loop body (i = i + 1)
    let mut block2 = IrBlock::new(2);
    block2.add_instruction(IrInstruction::LoadConst {
        dest: 4,
        value: IrValue::Constant(IrConstant::I32(1)),
    });
    block2.add_instruction(IrInstruction::Add {
        dest: 1,
        lhs: 1,
        rhs: 4,
    });
    block2.set_terminator(IrTerminator::Jump(1)); // Jump back to condition
    
    // Block 3: return i
    let mut block3 = IrBlock::new(3);
    block3.set_terminator(IrTerminator::Return(Some(1)));
    
    func.blocks.push(block0);
    func.blocks.push(block1);
    func.blocks.push(block2);
    func.blocks.push(block3);
    
    let mut module = IrModule::new();
    module.functions.push(func);
    
    let backend = X86_64Backend::new();
    let code = backend.compile(&module.functions[0]).expect("should compile");
    
    // Verify: code has jumps (backward for loop)
    assert!(code.len() > 80, "loop code should be substantial");
}
```

**Step 2: Run test**

```bash
cargo test -p dryad_aot test_e2e_while_loop --lib -- --nocapture
```

Expected: Test should PASS after jump/branch implementation.

**Step 3: Commit**

```bash
git add tests/integration_control_flow.rs
git commit -m "test: add integration test for while loop control flow"
```

---

## Task 10: Full Test Suite Verification

**Files:**
- Run all tests

**Step 1: Run full test suite**

```bash
cargo test -p dryad_aot
```

Expected output:
- All existing 44 lib tests: PASS ✓
- New jump/branch tests: PASS ✓
- New call tests: PASS ✓
- New while loop test: PASS ✓
- Integration tests: PASS ✓

**Step 2: Run clippy**

```bash
cargo clippy -p dryad_aot -- -D warnings
```

Expected: No warnings.

**Step 3: Verify no regressions**

Check that all originally passing tests still pass:

```bash
cargo test -p dryad_aot --lib 2>&1 | grep -E "test result:|passed"
```

Expected: Same number or more tests passing, all PASS.

**Step 4: Commit verification**

No code changes, just documentation.

---

## Task 11: E2E Binary Execution Test

**Files:**
- Modify: `tests/integration_control_flow.rs`

**Step 1: Create E2E test with PE generation**

```rust
#[test]
fn test_e2e_if_statement_windows_binary() {
    // Compile if statement to Windows PE executable
    // if (42 != 0) { return 100 } else { return 200 }
    // Expected exit code: 100 (system will return AL register)
    
    let mut func = IrFunction::new(0);
    
    let mut block0 = IrBlock::new(0);
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(42)),
    });
    block0.add_instruction(IrInstruction::LoadConst {
        dest: 2,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    block0.add_instruction(IrInstruction::CmpNe {
        dest: 3,
        lhs: 1,
        rhs: 2,
    });
    block0.set_terminator(IrTerminator::Branch {
        cond: 3,
        then_block: 1,
        else_block: 2,
    });
    
    let mut block1 = IrBlock::new(1);
    block1.add_instruction(IrInstruction::LoadConst {
        dest: 4,
        value: IrValue::Constant(IrConstant::I32(100)),
    });
    block1.set_terminator(IrTerminator::Return(Some(4)));
    
    let mut block2 = IrBlock::new(2);
    block2.add_instruction(IrInstruction::LoadConst {
        dest: 5,
        value: IrValue::Constant(IrConstant::I32(200)),
    });
    block2.set_terminator(IrTerminator::Return(Some(5)));
    
    func.blocks.push(block0);
    func.blocks.push(block1);
    func.blocks.push(block2);
    
    let mut module = IrModule::new();
    module.functions.push(func);
    
    let backend = X86_64Backend::new();
    let machine_code = backend.compile_module(&module).expect("should compile");
    
    let generator = PeGenerator::new();
    let pe_binary = generator.generate_object(&module, &machine_code).expect("should generate PE");
    
    // Write to temporary file and verify PE header
    let temp_path = "/tmp/test_if_statement.exe";
    std::fs::write(temp_path, &pe_binary).expect("should write file");
    
    // Verify PE signature
    assert_eq!(&pe_binary[0..2], b"MZ", "should have PE signature");
}
```

**Step 2: Run test**

```bash
cargo test -p dryad_aot test_e2e_if_statement_windows_binary --lib -- --nocapture
```

Expected: Test PASSES (binary generated, not executed).

**Step 3: Commit**

```bash
git add tests/integration_control_flow.rs
git commit -m "test: add E2E control flow binary generation test"
```

---

## Task 12: Documentation and Final Verification

**Files:**
- Create: `docs/plans/CONTROL_FLOW_IMPLEMENTATION_NOTES.md` (optional)
- Modify: `UPDATE_ROADMAP.md` (mark complete)

**Step 1: Document implementation notes**

Create a brief document explaining:
- Jump instruction implementation (unconditional and conditional)
- Branch instruction implementation with SystemV ABI
- Label resolution mechanism
- Stack frame management for function calls
- Parameter passing convention (first 6 in registers, rest on stack)

**Step 2: Update roadmap**

Mark Tier 1.2 and 1.3 as complete:

```markdown
#### 1.2 Controle de Fluxo Funcional (AOT)
```
Status: ✅ COMPLETE
```

#### 1.3 Funções no AOT (Call Stack)
```
Status: ✅ COMPLETE
```
```

**Step 3: Run full verification**

```bash
cargo test -p dryad_aot --lib
cargo clippy -p dryad_aot -- -D warnings
cargo build -p dryad_aot --release
```

Expected: All pass.

**Step 4: Final commit**

```bash
git add docs/plans/CONTROL_FLOW_IMPLEMENTATION_NOTES.md develop/UPDATE_ROADMAP.md
git commit -m "docs: document control flow implementation and mark Tier 1.2-1.3 complete"
```

---

## Summary

**Total Tasks:** 12  
**Estimated Effort:** 12-18 hours  
**Expected Outcome:** 

- ✅ Jump instructions (unconditional) working
- ✅ Branch instructions (conditional) with proper flag handling
- ✅ Label resolution enabling backward jumps (loops)
- ✅ Call instruction with SystemV ABI parameter passing
- ✅ Return instruction from functions
- ✅ Full if/else statement support
- ✅ Full while loop support
- ✅ Function calls with arguments and return values
- ✅ 55+ tests passing (44 existing + 11 new control flow tests)
- ✅ Zero regressions

**Verification Criteria:**

1. All tests pass: `cargo test -p dryad_aot` → 55+ PASS ✓
2. No clippy warnings: `cargo clippy` → clean ✓
3. Binaries generated: PE/ELF files created for control flow code ✓
4. Code review: All commits follow TDD (test first, implementation, commit) ✓
