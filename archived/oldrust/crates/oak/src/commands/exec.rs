use std::path::PathBuf;
use std::fs;
use dryad_lexer::Lexer;
use dryad_parser::Parser as DryadParser;
use dryad_runtime::Interpreter;
use crate::ui::*;

// Importar o OakModuleResolver aqui seria ideal mas ciclo de deps pode ser problemático
// Como Oak agora é um crate separado, podemos duplicar a lógica ou fazer CLI depender de oak
// Mas `dryad_cli` depende só do runtime.
// Assumindo que dryad_cli tem o adapter, OAK exec deveria delegar pra ele
// ou re-implementar o adapter aqui se quisermos autonomia.
// Vamos re-implementar simplificado pois OakModuleResolver está em dryad_cli (que é um bin, não lib?)
// Se dryad_cli for bin, não podemos usar ele como lib.
// Vamos fazer Exec rodar o interpretador básico.

pub fn execute_dryad_file(file: &str, _args: &[String], validate: bool) -> Result<(), Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file)?;
    
    // Lexer
    let mut lexer = Lexer::new(&content);
    let mut tokens = Vec::new();
    loop {
        let token = lexer.next_token()?;
        let is_eof = matches!(token.token, dryad_lexer::Token::Eof);
        tokens.push(token);
        if is_eof { break; }
    }

    // Parser
    let mut parser = DryadParser::new(tokens);
    let program = parser.parse()?;

    if validate {
        print_success("Sintaxe válida");
        return Ok(());
    }

    print_warning(&format!("'oak exec' executa sem suporte completo a módulos. Use 'dryad run {}' para melhor experiência.", file));
    
    // Execução básica
    let mut interpreter = Interpreter::new();
    interpreter.set_current_file(PathBuf::from(file));
    let result = interpreter.execute(&program)?;
    
    if result != "null" {
        println!("{}", result);
    }

    Ok(())
}
