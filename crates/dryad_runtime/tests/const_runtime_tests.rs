// crates/dryad_runtime/tests/const_runtime_tests.rs
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
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&program)?;
    Ok(Value::Null)
}

fn execute_and_get_variable(code: &str, var_name: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        match token.token {
            Token::Eof => break,
            _ => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.execute(&program)?;
    
    // Acessa a constante diretamente
    if let Some(value) = interpreter.constants.get(var_name) {
        Ok(value.clone())
    } else if let Some(value) = interpreter.variables.get(var_name) {
        Ok(value.clone())
    } else {
        Err(dryad_errors::DryadError::new(3001, &format!("Variável '{}' não encontrada", var_name)))
    }
}

#[test]
fn test_const_number_declaration() {
    let result = execute_dryad_code("const PI = 3.14159;");
    assert!(result.is_ok());
    
    let value = execute_and_get_variable("const PI = 3.14159;", "PI").unwrap();
    match value {
        Value::Number(n) => assert_eq!(n, 3.14159),
        _ => panic!("Esperado número"),
    }
}

#[test]
fn test_const_string_declaration() {
    let result = execute_dryad_code("const APP_NAME = \"Dryad Language\";");
    assert!(result.is_ok());
    
    let value = execute_and_get_variable("const APP_NAME = \"Dryad Language\";", "APP_NAME").unwrap();
    match value {
        Value::String(s) => assert_eq!(s, "Dryad Language"),
        _ => panic!("Esperado string"),
    }
}

#[test]
fn test_const_boolean_declaration() {
    let result = execute_dryad_code("const DEBUG_MODE = true;");
    assert!(result.is_ok());
    
    let value = execute_and_get_variable("const DEBUG_MODE = true;", "DEBUG_MODE").unwrap();
    match value {
        Value::Bool(b) => assert_eq!(b, true),
        _ => panic!("Esperado boolean"),
    }
}

#[test]
fn test_const_expression_evaluation() {
    let result = execute_dryad_code("const MAX_SIZE = 100 + 50;");
    assert!(result.is_ok());
    
    let value = execute_and_get_variable("const MAX_SIZE = 100 + 50;", "MAX_SIZE").unwrap();
    match value {
        Value::Number(n) => assert_eq!(n, 150.0),
        _ => panic!("Esperado número"),
    }
}

#[test]
fn test_const_redeclaration_error() {
    let code = r#"
        const PI = 3.14159;
        const PI = 2.71828;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    
    if let Err(err) = result {
        assert!(err.message().contains("já foi declarada"));
    }
}

#[test]
fn test_const_assignment_error() {
    let code = r#"
        const PI = 3.14159;
        PI = 2.71828;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    
    if let Err(err) = result {
        assert!(err.message().contains("Não é possível modificar a constante"));
    }
}

#[test]
fn test_const_usage_in_expression() {
    let code = r#"
        const PI = 3.14159;
        const RADIUS = 5.0;
        let area = PI * RADIUS * RADIUS;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok());
    
    let area_value = execute_and_get_variable(code, "area").unwrap();
    match area_value {
        Value::Number(n) => {
            let expected = 3.14159 * 5.0 * 5.0;
            assert!((n - expected).abs() < 0.001);
        },
        _ => panic!("Esperado número"),
    }
}

#[test]
fn test_const_and_var_different_namespaces() {
    let code = r#"
        const VALUE = 100;
        let variable = VALUE + 50;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok());
    
    let const_value = execute_and_get_variable(code, "VALUE").unwrap();
    let var_value = execute_and_get_variable(code, "variable").unwrap();
    
    match (const_value, var_value) {
        (Value::Number(c), Value::Number(v)) => {
            assert_eq!(c, 100.0);
            assert_eq!(v, 150.0);
        },
        _ => panic!("Esperado números"),
    }
}

#[test]
fn test_multiple_const_declarations() {
    let code = r#"
        const PI = 3.14159;
        const E = 2.71828;
        const GOLDEN_RATIO = 1.618;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok());
    
    let pi = execute_and_get_variable(code, "PI").unwrap();
    let e = execute_and_get_variable(code, "E").unwrap();
    let golden = execute_and_get_variable(code, "GOLDEN_RATIO").unwrap();
    
    match (pi, e, golden) {
        (Value::Number(p), Value::Number(e_val), Value::Number(g)) => {
            assert_eq!(p, 3.14159);
            assert_eq!(e_val, 2.71828);
            assert_eq!(g, 1.618);
        },
        _ => panic!("Esperado números"),
    }
}