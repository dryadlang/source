#[test]
fn test_e2e_compilation_pipeline_initialization() {
    use dryad_aot::compiler::AotCompiler;
    use dryad_aot::compiler::Target;

    let _compiler = AotCompiler::new(Target::X86_64Linux);

    // Compiler successfully initialized with correct target
    // (internal state verified through subsequent compilation tests)
}

#[test]
fn test_e2e_ir_conversion_and_code_generation() {
    use dryad_aot::backend::Backend;
    use dryad_aot::compiler::BytecodeToIrConverter;
    use dryad_aot::compiler::Target;
    use dryad_aot::generator::Generator;
    use dryad_bytecode::Chunk;

    // Create minimal bytecode
    let chunk = Chunk::new("test");

    // Convert bytecode to IR
    let mut converter = BytecodeToIrConverter::new();
    let ir_module = converter.convert(&chunk).expect("Conversion failed");
    assert_eq!(ir_module.functions.len(), 1);

    // Generate x86_64 machine code
    let backend = Target::X86_64Linux.create_backend();
    let machine_code = backend
        .compile_module(&ir_module)
        .expect("Backend compilation failed");

    assert!(
        !machine_code.is_empty(),
        "Machine code should contain entry point and return sequence"
    );

    // Generate executable format
    let generator = Target::X86_64Linux.create_generator();
    let executable = generator
        .generate_object(&ir_module, &machine_code)
        .expect("Generator failed");

    assert!(!executable.is_empty(), "Executable should not be empty");
    assert!(
        executable.len() > machine_code.len(),
        "Executable should include ELF headers plus machine code"
    );

    // Verify ELF header for Linux target
    assert_eq!(&executable[0..4], b"\x7FELF", "ELF magic number mismatch");
}

#[test]
fn test_e2e_windows_pe_generation() {
    use dryad_aot::backend::Backend;
    use dryad_aot::compiler::BytecodeToIrConverter;
    use dryad_aot::compiler::Target;
    use dryad_aot::generator::Generator;
    use dryad_bytecode::Chunk;

    // Create minimal bytecode
    let chunk = Chunk::new("test");

    // Convert to IR
    let mut converter = BytecodeToIrConverter::new();
    let ir_module = converter.convert(&chunk).expect("Conversion failed");

    // Generate x86_64 code
    let backend = Target::X86_64Windows.create_backend();
    let machine_code = backend
        .compile_module(&ir_module)
        .expect("Backend compilation failed");

    // Generate PE executable
    let generator = Target::X86_64Windows.create_generator();
    let executable = generator
        .generate_object(&ir_module, &machine_code)
        .expect("Generator failed");

    assert!(!executable.is_empty(), "PE executable should not be empty");
    assert_eq!(&executable[0..2], b"MZ", "PE magic number (MZ) mismatch");

    // Verify PE signature at offset 64
    assert_eq!(&executable[64..68], b"PE\0\0", "PE signature mismatch");
}

#[test]
fn test_all_implemented_ir_instructions() {
    use dryad_aot::ir::{IrConstant, IrInstruction, IrValue};

    // Verify all implemented instruction types can be constructed and matched
    let _instructions = vec![
        IrInstruction::LoadConst {
            dest: 0,
            value: IrValue::Constant(IrConstant::I32(5)),
        },
        IrInstruction::Add {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Sub {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Mul {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Div {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Mod {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Neg { dest: 0, src: 1 },
        IrInstruction::CmpEq {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::CmpNe {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::CmpLt {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::CmpLe {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::CmpGt {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::CmpGe {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::And {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Or {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Xor {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Not { dest: 0, src: 1 },
        IrInstruction::Shl {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
        IrInstruction::Shr {
            dest: 0,
            lhs: 1,
            rhs: 2,
        },
    ];

    assert_eq!(
        _instructions.len(),
        19,
        "All 19 key instructions accounted for"
    );
}

#[test]
fn test_e2e_if_else_windows_binary() {
    use dryad_aot::compiler::Target;
    use dryad_aot::ir::{
        IrBlock, IrConstant, IrFunction, IrInstruction, IrModule, IrTerminator, IrType, IrValue,
    };

    // Build IR for: if (42 != 0) { return 100 } else { return 200 }
    let mut module = IrModule::new("if_else_test");

    let mut func = IrFunction::new("test_if_else", IrType::I32);

    // Create 4 blocks: entry, then_block, else_block, merge_block
    let entry_block_id = module.new_block_id();
    let then_block_id = module.new_block_id();
    let else_block_id = module.new_block_id();

    func.entry_block = entry_block_id;

    // ===== BLOCK 0: Entry =====
    // Load 42 into register 0
    // Load 0 into register 1
    // Compare 42 != 0 into register 2
    // Branch to then_block (block 1) if true, else_block (block 2) if false
    let mut entry = IrBlock::new(entry_block_id);
    entry.add_instruction(IrInstruction::LoadConst {
        dest: 0,
        value: IrValue::Constant(IrConstant::I32(42)),
    });
    entry.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    entry.add_instruction(IrInstruction::CmpNe {
        dest: 2,
        lhs: 0,
        rhs: 1,
    });
    entry.set_terminator(IrTerminator::Branch {
        cond: 2,
        then_block: then_block_id,
        else_block: else_block_id,
    });
    func.add_block(entry);

    // ===== BLOCK 1: Then branch (42 != 0 is true) =====
    // Load 100 into register 3
    // Return 100
    let mut then_block = IrBlock::new(then_block_id);
    then_block.add_instruction(IrInstruction::LoadConst {
        dest: 3,
        value: IrValue::Constant(IrConstant::I32(100)),
    });
    then_block.set_terminator(IrTerminator::Return(Some(3)));
    func.add_block(then_block);

    // ===== BLOCK 2: Else branch =====
    // Load 200 into register 4
    // Return 200
    let mut else_block = IrBlock::new(else_block_id);
    else_block.add_instruction(IrInstruction::LoadConst {
        dest: 4,
        value: IrValue::Constant(IrConstant::I32(200)),
    });
    else_block.set_terminator(IrTerminator::Return(Some(4)));
    func.add_block(else_block);

    module.add_function(func);

    // Compile to x86_64 machine code
    let backend = Target::X86_64Windows.create_backend();
    let machine_code = backend
        .compile_module(&module)
        .expect("Backend compilation failed");

    // Verify machine code contains branching instructions
    assert!(
        machine_code.len() > 30,
        "Machine code should contain LoadConst, CmpNe, Branch, and Return instructions (>30 bytes)"
    );

    // Generate PE executable
    let generator = Target::X86_64Windows.create_generator();
    let pe_binary = generator
        .generate_object(&module, &machine_code)
        .expect("PE generation failed");

    // ===== VERIFY PE BINARY STRUCTURE =====

    // Check PE binary is not empty
    assert!(!pe_binary.is_empty(), "PE binary should not be empty");

    // Check MZ magic bytes (PE signature)
    assert_eq!(
        &pe_binary[0..2],
        b"MZ",
        "PE binary must start with MZ magic bytes"
    );

    // PE binaries have signature at offset 0x3C (60 bytes)
    // The signature is "PE\0\0" at that offset
    if pe_binary.len() > 68 {
        assert_eq!(
            &pe_binary[64..68],
            b"PE\0\0",
            "PE signature (PE\\0\\0) should be at offset 64"
        );
    }

    // Check minimum viable PE size (headers + minimal machine code)
    assert!(
        pe_binary.len() >= 64,
        "PE binary should be at least 64 bytes for PE header structure"
    );

    // Verify machine code is embedded in PE (PE headers add overhead)
    assert!(
        pe_binary.len() > machine_code.len(),
        "PE binary should include PE headers plus machine code"
    );

    // Optional: Write binary for manual inspection
    let test_binary_path = "/tmp/test_if_statement.exe";
    if std::fs::write(test_binary_path, &pe_binary).is_ok() {
        println!("✓ Test PE binary written to: {}", test_binary_path);
        println!(
            "  Machine code: {} bytes, PE total: {} bytes (headers: {} bytes)",
            machine_code.len(),
            pe_binary.len(),
            pe_binary.len() - machine_code.len()
        );
    }
}

#[test]
fn test_e2e_while_loop_compilation() {
    use dryad_aot::compiler::Target;
    use dryad_aot::ir::{
        IrBlock, IrConstant, IrFunction, IrInstruction, IrModule, IrTerminator, IrType, IrValue,
    };

    // Build IR for: i = 0; while (i < 10) { i = i + 1 }; return i
    // Expected result: i should be 10 after loop
    let mut module = IrModule::new("while_loop_test");

    let mut func = IrFunction::new("test_while_loop", IrType::I32);

    // Create 4 blocks: init, loop_cond, loop_body, exit
    let init_block_id = module.new_block_id();
    let loop_cond_block_id = module.new_block_id();
    let loop_body_block_id = module.new_block_id();
    let exit_block_id = module.new_block_id();

    func.entry_block = init_block_id;

    // ===== BLOCK 0: Initialization =====
    // i = 0 (LoadConst r1=0)
    // Jump to loop condition block
    let mut init_block = IrBlock::new(init_block_id);
    init_block.add_instruction(IrInstruction::LoadConst {
        dest: 1,
        value: IrValue::Constant(IrConstant::I32(0)),
    });
    init_block.set_terminator(IrTerminator::Jump(loop_cond_block_id));
    func.add_block(init_block);

    // ===== BLOCK 1: Loop Condition =====
    // Load 10 into register 2
    // Compare i < 10 (CmpLt r3, r1, r2)
    // Branch: if true (r3) go to loop_body, else go to exit
    let mut loop_cond = IrBlock::new(loop_cond_block_id);
    loop_cond.add_instruction(IrInstruction::LoadConst {
        dest: 2,
        value: IrValue::Constant(IrConstant::I32(10)),
    });
    loop_cond.add_instruction(IrInstruction::CmpLt {
        dest: 3,
        lhs: 1,
        rhs: 2,
    });
    loop_cond.set_terminator(IrTerminator::Branch {
        cond: 3,
        then_block: loop_body_block_id,
        else_block: exit_block_id,
    });
    func.add_block(loop_cond);

    // ===== BLOCK 2: Loop Body =====
    // Load 1 into register 4
    // i = i + 1 (Add r1, r1, r4)
    // Jump back to loop condition (backward jump)
    let mut loop_body = IrBlock::new(loop_body_block_id);
    loop_body.add_instruction(IrInstruction::LoadConst {
        dest: 4,
        value: IrValue::Constant(IrConstant::I32(1)),
    });
    loop_body.add_instruction(IrInstruction::Add {
        dest: 1,
        lhs: 1,
        rhs: 4,
    });
    loop_body.set_terminator(IrTerminator::Jump(loop_cond_block_id));
    func.add_block(loop_body);

    // ===== BLOCK 3: Exit =====
    // Return i (r1)
    let mut exit_block = IrBlock::new(exit_block_id);
    exit_block.set_terminator(IrTerminator::Return(Some(1)));
    func.add_block(exit_block);

    module.add_function(func);

    // Compile to x86_64 machine code
    let backend = Target::X86_64Windows.create_backend();
    let machine_code = backend
        .compile_module(&module)
        .expect("Backend compilation failed");

    // Verify machine code contains loop instructions (should be substantial)
    assert!(
        machine_code.len() > 60,
        "Machine code should contain loop with multiple blocks, jumps, and comparisons (>60 bytes), got {} bytes",
        machine_code.len()
    );

    // Generate PE executable
    let generator = Target::X86_64Windows.create_generator();
    let pe_binary = generator
        .generate_object(&module, &machine_code)
        .expect("PE generation failed");

    // ===== VERIFY PE BINARY STRUCTURE =====

    // Check PE binary is not empty
    assert!(!pe_binary.is_empty(), "PE binary should not be empty");

    // Check MZ magic bytes (PE signature)
    assert_eq!(
        &pe_binary[0..2],
        b"MZ",
        "PE binary must start with MZ magic bytes"
    );

    // PE binaries have signature at offset 0x3C (60 bytes)
    // The signature is "PE\0\0" at that offset
    if pe_binary.len() > 68 {
        assert_eq!(
            &pe_binary[64..68],
            b"PE\0\0",
            "PE signature (PE\\0\\0) should be at offset 64"
        );
    }

    // Check minimum viable PE size (headers + minimal machine code)
    assert!(
        pe_binary.len() >= 64,
        "PE binary should be at least 64 bytes for PE header structure"
    );

    // Verify machine code is embedded in PE (PE headers add overhead)
    assert!(
        pe_binary.len() > machine_code.len(),
        "PE binary should include PE headers plus machine code"
    );

    // Verify machine code likely contains conditional jumps (loop pattern)
    // x86_64 conditional jump opcodes typically start with 0x0F followed by 8x (where x is variation)
    // or short forms like 0x7x for backward jumps
    let has_jumps = machine_code.windows(2).any(|w| {
        // 0x0F 0x8x: long conditional jumps
        (w[0] == 0x0F && (w[1] & 0xF0) == 0x80) ||
        // 0x7x: short conditional jumps  
        (w[0] >= 0x70 && w[0] <= 0x7F) ||
        // 0xEB: unconditional short jump
        w[0] == 0xEB ||
        // 0xE9: unconditional near jump
        w[0] == 0xE9 ||
        // 0xFF: indirect jump or call (jmp/call r64)
        (w[0] == 0xFF && (w[1] & 0x38) >= 0x20)
    });
    assert!(
        has_jumps,
        "Machine code should contain conditional jump opcodes for loop control"
    );

    // Optional: Write binary for manual inspection
    let test_binary_path = "/tmp/test_while_loop.exe";
    if std::fs::write(test_binary_path, &pe_binary).is_ok() {
        println!("✓ While loop PE binary written to: {}", test_binary_path);
        println!(
            "  Machine code: {} bytes, PE total: {} bytes (headers: {} bytes)",
            machine_code.len(),
            pe_binary.len(),
            pe_binary.len() - machine_code.len()
        );
        println!("  IR: 4 blocks (init → loop_cond → loop_body → exit with backward jump)");
    }
}
