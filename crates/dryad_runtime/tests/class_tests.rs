// crates/dryad_runtime/tests/class_tests.rs

use dryad_runtime::interpreter::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn execute_dryad_code(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token() {
            Ok(token_with_location) if token_with_location.token == Token::Eof => break,
            Ok(token) => tokens.push(token),
            Err(e) => return Err(format!("Lexer error: {:?}", e)),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(prog) => prog,
        Err(e) => return Err(format!("Parser error: {:?}", e)),
    };
    
    let mut interpreter = Interpreter::new();
    match interpreter.execute(&program) {
        Ok(result) => Ok(result),
        Err(e) => Err(format!("Runtime error: {:?}", e)),
    }
}

#[test]
fn test_simple_class_declaration() {
    let code = r#"
        class Pessoa {
            function init(nome, idade) {
                this.nome = nome;
                this.idade = idade;
            }
        }
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to declare simple class: {:?}", result.err());
}

#[test]
fn test_class_instantiation() {
    let code = r#"
        class Pessoa {
            function init(nome, idade) {
                this.nome = nome;
                this.idade = idade;
            }
        }
        
        let pessoa = Pessoa("João", 30);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to instantiate class: {:?}", result.err());
}

#[test]
fn test_method_call() {
    let code = r#"
        class Pessoa {
            function init(nome, idade) {
                this.nome = nome;
                this.idade = idade;
            }
            
            function apresentar() {
                return "Meu nome é " + this.nome + " e tenho " + this.idade + " anos.";
            }
        }
        
        let pessoa = Pessoa("Maria", 25);
        pessoa.apresentar();
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to call method: {:?}", result.err());
}

#[test]
fn test_property_access() {
    let code = r#"
        class Pessoa {
            function init(nome, idade) {
                this.nome = nome;
                this.idade = idade;
            }
        }
        
        let pessoa = Pessoa("Ana", 35);
        pessoa.nome;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to access property: {:?}", result.err());
}

#[test]
fn test_method_with_parameters() {
    let code = r#"
        class Calculadora {
            function somar(a, b) {
                return a + b;
            }
        }
        
        let calc = Calculadora();
        calc.somar(5, 3);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to call method with parameters: {:?}", result.err());
}

#[test]
fn test_class_with_properties() {
    let code = r#"
        class Contador {
            let valor = 0;
            
            function incrementar() {
                this.valor = this.valor + 1;
                return this.valor;
            }
        }
        
        let contador = Contador();
        contador.incrementar();
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed with class properties: {:?}", result.err());
}
