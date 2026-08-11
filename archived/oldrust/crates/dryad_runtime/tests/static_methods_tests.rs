// crates/dryad_runtime/tests/static_methods_tests.rs
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
fn test_simple_static_method() {
    let code = r#"
        class MathUtils {
            static function pi() {
                return 3.14159;
            }
        }
        
        MathUtils.pi()
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(3.14159));
}

#[test]
fn test_static_method_with_parameters() {
    let code = r#"
        class Calculadora {
            static function somar(a, b) {
                return a + b;
            }
        }
        
        Calculadora.somar(5, 3)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_static_method_calling_another_static_method() {
    let code = r#"
        class Calculadora {
            static function pi() {
                return 3.14159;
            }
            
            static function circunferencia(raio) {
                return 2 * Calculadora.pi() * raio;
            }
        }
        
        Calculadora.circunferencia(5)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    let expected = 2.0 * 3.14159 * 5.0;
    assert_eq!(result, Value::Number(expected));
}

#[test]
fn test_exact_syntax_md_example() {
    let code = r#"
        class Calculadora {
            static function pi() {
                return 3.14159;
            }
            
            static function circunferencia(raio) {
                return 2 * Calculadora.pi() * raio;
            }
        }

        let circ = Calculadora.circunferencia(5);
        circ
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    let expected = 2.0 * 3.14159 * 5.0;
    assert_eq!(result, Value::Number(expected));
}

#[test]
fn test_error_calling_non_static_method_as_static() {
    let code = r#"
        class Test {
            function instanceMethod() {
                return "instance";
            }
        }
        
        Test.instanceMethod()
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert!(error.message().contains("não é estático"));
}

#[test]
fn test_multiple_static_methods() {
    let code = r#"
        class MathUtils {
            static function add(a, b) {
                return a + b;
            }
            
            static function multiply(a, b) {
                return a * b;
            }
            
            static function calculate(x, y) {
                let sum = MathUtils.add(x, y);
                let product = MathUtils.multiply(x, y);
                return sum + product;
            }
        }
        
        MathUtils.calculate(3, 4)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    // sum = 3 + 4 = 7, product = 3 * 4 = 12, result = 7 + 12 = 19
    assert_eq!(result, Value::Number(19.0));
}
