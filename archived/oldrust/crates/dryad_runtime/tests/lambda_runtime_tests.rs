// crates/dryad_runtime/tests/lambda_runtime_tests.rs

use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn execute_dryad_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        if token.token == Token::Eof {
            break;
        }
        tokens.push(token);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&program)?;
    Ok(Value::String(result))
}

#[test]
fn test_simple_lambda() {
    let code = r#"
        let quadrado = x => x * x;
        quadrado(5);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute simple lambda: {:?}", result.err());
}

#[test]
fn test_lambda_with_multiple_params() {
    let code = r#"
        let soma = (a, b) => a + b;
        soma(3, 4);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute multi-param lambda: {:?}", result.err());
}

#[test]
fn test_lambda_with_zero_params() {
    let code = r#"
        let resposta = () => 42;
        resposta();
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute zero-param lambda: {:?}", result.err());
}

#[test]
fn test_lambda_assignment_and_call() {
    let code = r#"
        let duplicar = x => x * 2;
        let resultado = duplicar(21);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute lambda assignment and call: {:?}", result.err());
}

#[test]
fn test_lambda_with_string_concatenation() {
    let code = r#"
        let saudar = nome => "Olá, " + nome + "!";
        saudar("Mundo");
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute lambda with string concatenation: {:?}", result.err());
}

#[test]
fn test_lambda_scope() {
    let code = r#"
        let x = 10;
        let funcao = y => x + y;
        funcao(5);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute lambda with scope: {:?}", result.err());
}

#[test]
fn test_nested_lambda_calls() {
    let code = r#"
        let adicionar = x => y => x + y;
        let adicionar5 = adicionar(5);
        adicionar5(3);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute nested lambda calls: {:?}", result.err());
}

#[test]
fn test_lambda_as_parameter() {
    let code = r#"
        function aplicar(f, x) {
            return f(x);
        }
        
        let quadrado = x => x * x;
        aplicar(quadrado, 4);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute lambda as parameter: {:?}", result.err());
}

#[test]
fn test_lambda_return_lambda() {
    let code = r#"
        let criarMultiplicador = n => x => n * x;
        let triple = criarMultiplicador(3);
        triple(7);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute lambda returning lambda: {:?}", result.err());
}

#[test]
fn test_lambda_immediate_invocation() {
    let code = r#"
        let resultado = (x => x + 10)(5);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Lambda immediate invocation should work: {:?}", result.err());
}

#[test]
fn test_lambda_error_wrong_arity() {
    let code = r#"
        let soma = (a, b) => a + b;
        soma(1);  // Erro: número incorreto de argumentos
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err(), "Should fail with wrong number of arguments");
}

#[test]
fn test_lambda_error_not_function() {
    let code = r#"
        let x = 42;
        x(5);  // Erro: x não é uma função
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err(), "Should fail when calling non-function");
}
