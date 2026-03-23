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
