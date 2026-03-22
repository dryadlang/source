#[test]
fn test_bytecode_to_pe_simple_arithmetic() {
    use dryad_aot::compiler::BytecodeToIrConverter;
    use dryad_aot::generator::pe::PeGenerator;
    use dryad_aot::generator::Generator;
    use dryad_bytecode::{Chunk, OpCode, Value};

    let mut chunk = Chunk::new("test");
    let _ = chunk.add_constant(Value::Number(5.0)).unwrap();
    let _ = chunk.add_constant(Value::Number(3.0)).unwrap();
    chunk.push_op(OpCode::Constant(0), 1);
    chunk.push_op(OpCode::Constant(1), 1);
    chunk.push_op(OpCode::Add, 1);
    chunk.push_op(OpCode::Return, 1);

    let mut converter = BytecodeToIrConverter::new();
    let ir_module = converter.convert(&chunk).expect("Convert failed");

    assert_eq!(ir_module.functions.len(), 1);

    let gen = PeGenerator::new();
    let pe_binary = gen
        .generate_object(&ir_module, &vec![0x90; 100])
        .expect("PE generation failed");

    assert_eq!(&pe_binary[0..2], b"MZ");
    assert!(pe_binary.len() >= 512);
}
