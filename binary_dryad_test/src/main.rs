use dryad_aot::compiler::BytecodeToIrConverter;
use dryad_aot::generator::pe::PeGenerator;
use dryad_aot::generator::Generator;
use dryad_bytecode::Compiler;
use dryad_lexer::Lexer;
use dryad_parser::Parser;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Ler código Dryad
    let source_code = fs::read_to_string("main.dryad")?;
    println!("📄 Código Dryad lido ({} bytes)", source_code.len());
    println!("─────────────────────────────────────");
    println!("{}", source_code);
    println!("─────────────────────────────────────\n");

    // 2. Lexer: Código → Tokens
    println!("🔤 Fase 1: Tokenização (Lexer)...");
    let mut lexer = Lexer::new(&source_code);
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if token.token == dryad_lexer::Token::Eof {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Err(e) => return Err(format!("Lexer error: {:?}", e).into()),
        }
    }
    println!("✓ {} tokens gerados\n", tokens.len());

    // 3. Parser: Tokens → AST
    println!("🌳 Fase 2: Parsing (Parser)...");
    let mut parser = Parser::new(tokens);
    let ast = parser.parse()?;
    println!("✓ AST construída ({} statements)\n", ast.statements.len());

    // 4. Compiler: AST → Bytecode
    println!("📦 Fase 3: Compilação para Bytecode...");
    let mut compiler = Compiler::new();
    let bytecode_chunk = compiler.compile(ast)?;
    println!("✓ Bytecode gerado ({} opcodes)", bytecode_chunk.code.len());

    // Debug: mostrar alguns opcodes
    println!("\nPrimeiros opcodes:");
    for (i, op) in bytecode_chunk.code.iter().take(10).enumerate() {
        println!("  [{}] {:?}", i, op);
    }
    println!("  ...\n");

    // 5. BytecodeToIrConverter: Bytecode → IR
    println!("🔵 Fase 4: Conversão para IR (Intermediate Representation)...");
    let mut ir_converter = BytecodeToIrConverter::new();
    let ir_module = ir_converter.convert(&bytecode_chunk)?;
    println!("✓ IR gerada com {} funcções", ir_module.functions.len());
    for (i, func) in ir_module.functions.iter().enumerate() {
        println!("  Função {}: {}", i, func.name);
    }
    println!();

    // 6. PeGenerator: IR → PE Binary
    println!("🪟 Fase 5: Geração de Binário PE (Windows)...");
    let gen = PeGenerator::new();

    // Gerar código de máquina dummy (para este teste)
    let machine_code = vec![0x90; 1024]; // NOPs - instruções nulas

    let pe_binary = gen.generate_object(&ir_module, &machine_code)?;
    println!("✓ PE binary gerado ({} bytes)", pe_binary.len());

    // Verificações do PE
    println!("\n📋 Verificação do PE Binary:");
    println!("  Magic: {:?}", &pe_binary[0..2]);
    println!(
        "  PE Signature: {:?}",
        std::str::from_utf8(&pe_binary[64..68]).unwrap_or("???")
    );
    println!("  Tamanho mínimo (512 bytes): {}", pe_binary.len() >= 512);
    println!();

    // 7. Salvar binário PE
    println!("💾 Fase 6: Salvando PE binary...");
    fs::write("test_program.exe", &pe_binary)?;
    println!("✓ Binário salvo em: test_program.exe");
    println!(
        "  Tamanho final: {} bytes ({:.2} KB)",
        pe_binary.len(),
        pe_binary.len() as f64 / 1024.0
    );

    println!("\n✅ Compilação completa!");
    println!("═══════════════════════════════════");
    println!("Pipeline: Dryad → Bytecode → IR → PE");

    Ok(())
}
