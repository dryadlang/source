// crates/dryad_runtime/tests/function_runtime_tests.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};

fn execute_dryad_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        match token.token {
            Token::Eof => break,
            _ => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let mut interpreter = Interpreter::new();
    interpreter.execute_and_return_value(&program)
}

#[test]
fn test_simple_function_declaration_and_call() {
    let code = r#"
        function test() {
            return 42;
        }
        
        test()
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_function_with_parameters() {
    let code = r#"
        function dobrar(x) {
            return x * 2;
        }
        
        dobrar(21)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_function_with_multiple_parameters() {
    let code = r#"
        function somar(a, b, c) {
            return a + b + c;
        }
        
        somar(10, 20, 12)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(42.0));
}

#[test]
fn test_function_without_return() {
    let code = r#"
        function semRetorno() {
            let x = 10;
        }
        
        semRetorno()
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_function_with_string_concatenation() {
    let code = r#"
        function saudacao(nome) {
            return "Olá, " + nome + "!";
        }
        
        saudacao("Maria")
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("Olá, Maria!".to_string()));
}

#[test]
fn test_function_calling_another_function() {
    let code = r#"
        function dobrar(x) {
            return x * 2;
        }
        
        function quadruplar(x) {
            return dobrar(dobrar(x));
        }
        
        quadruplar(10)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(40.0));
}

#[test]
fn test_recursive_function() {
    let code = r#"
        function fatorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        fatorial(5)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_function_with_local_variables() {
    let code = r#"
        function calcular() {
            let a = 10;
            let b = 20;
            let c = a + b;
            return c * 2;
        }
        
        calcular()
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(60.0));
}

#[test]
fn test_function_scope_isolation() {
    let code = r#"
        let global = 100;
        
        function modificar() {
            let global = 50;
            return global;
        }
        
        let resultado = modificar();
        global + resultado
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(150.0)); // 100 + 50
}

#[test]
fn test_function_early_return() {
    let code = r#"
        function verificar(x) {
            if x > 10 {
                return "grande";
            }
            return "pequeno";
        }
        
        verificar(15)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("grande".to_string()));
}

#[test]
fn test_function_with_expression_arguments() {
    let code = r#"
        function somar(a, b) {
            return a + b;
        }
        
        somar(5 * 2, 8 + 4)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(22.0)); // 10 + 12
}

#[test]
fn test_function_call_error_wrong_arguments() {
    let code = r#"
        function test(a, b) {
            return a + b;
        }
        
        test(1)
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    // Deve retornar erro de número incorreto de argumentos
}

#[test]
fn test_undefined_function_call() {
    let code = r#"
        inexistente()
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    // Deve retornar erro de função não encontrada
}
