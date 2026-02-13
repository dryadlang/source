// crates/dryad_runtime/tests/array_runtime_tests.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::{Parser, ast::Expr};
use dryad_lexer::{Lexer, token::Token};

fn parse_expression(input: &str) -> Expr {
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
    
    // Pega a primeira declaração de variável e retorna sua expressão
    match &program.statements[0] {
        dryad_parser::ast::Stmt::VarDeclaration(_, Some(expr), _) => expr.clone(),
        _ => panic!("Esperado declaração de variável com expressão"),
    }
}

#[test]
fn test_empty_array() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression("let arr = [];");
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Array(elements) => {
            assert_eq!(elements.len(), 0);
        },
        _ => panic!("Esperado array vazio, encontrado {:?}", result),
    }
}

#[test]
fn test_array_with_numbers() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression("let numeros = [1, 2, 3];");
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Array(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Value::Number(1.0));
            assert_eq!(elements[1], Value::Number(2.0));
            assert_eq!(elements[2], Value::Number(3.0));
        },
        _ => panic!("Esperado array com números, encontrado {:?}", result),
    }
}

#[test]
fn test_array_with_mixed_types() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression(r#"let misto = [1, "dois", true];"#);
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Array(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Value::Number(1.0));
            assert_eq!(elements[1], Value::String("dois".to_string()));
            assert_eq!(elements[2], Value::Bool(true));
        },
        _ => panic!("Esperado array misto, encontrado {:?}", result),
    }
}

#[test]
fn test_nested_array() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression("let matriz = [[1, 2], [3, 4]];");
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Array(elements) => {
            assert_eq!(elements.len(), 2);
            
            match &elements[0] {
                Value::Array(sub_elements) => {
                    assert_eq!(sub_elements.len(), 2);
                    assert_eq!(sub_elements[0], Value::Number(1.0));
                    assert_eq!(sub_elements[1], Value::Number(2.0));
                },
                _ => panic!("Esperado sub-array"),
            }
            
            match &elements[1] {
                Value::Array(sub_elements) => {
                    assert_eq!(sub_elements.len(), 2);
                    assert_eq!(sub_elements[0], Value::Number(3.0));
                    assert_eq!(sub_elements[1], Value::Number(4.0));
                },
                _ => panic!("Esperado sub-array"),
            }
        },
        _ => panic!("Esperado array aninhado, encontrado {:?}", result),
    }
}

#[test]
fn test_array_access() {
    let mut interpreter = Interpreter::new();
    
    // Primeiro cria o array
    interpreter.set_variable("arr".to_string(), Value::Array(vec![
        Value::Number(10.0),
        Value::Number(20.0),
        Value::Number(30.0),
    ]));
    
    let expr = parse_expression("let valor = arr[1];");
    let result = interpreter.evaluate(&expr).unwrap();
    
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_array_access_out_of_bounds() {
    let mut interpreter = Interpreter::new();
    
    // Cria array pequeno
    interpreter.set_variable("arr".to_string(), Value::Array(vec![
        Value::Number(10.0),
    ]));
    
    let expr = parse_expression("let valor = arr[5];");
    let result = interpreter.evaluate(&expr);
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3082); // Índice fora dos limites
}

#[test]
fn test_empty_tuple() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression("let vazio = ();");
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Tuple(elements) => {
            assert_eq!(elements.len(), 0);
        },
        _ => panic!("Esperado tupla vazia, encontrado {:?}", result),
    }
}

#[test]
fn test_tuple_with_mixed_types() {
    let mut interpreter = Interpreter::new();
    let expr = parse_expression(r#"let tupla = (1, "dois", 3.0);"#);
    
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        Value::Tuple(elements) => {
            assert_eq!(elements.len(), 3);
            assert_eq!(elements[0], Value::Number(1.0));
            assert_eq!(elements[1], Value::String("dois".to_string()));
            assert_eq!(elements[2], Value::Number(3.0));
        },
        _ => panic!("Esperado tupla mista, encontrado {:?}", result),
    }
}

#[test]
fn test_tuple_access() {
    let mut interpreter = Interpreter::new();
    
    // Primeiro cria a tupla
    interpreter.set_variable("tupla".to_string(), Value::Tuple(vec![
        Value::Number(1.0),
        Value::String("dois".to_string()),
        Value::Number(3.0),
    ]));
    
    let expr = parse_expression("let valor = tupla.1;");
    let result = interpreter.evaluate(&expr).unwrap();
    
    assert_eq!(result, Value::String("dois".to_string()));
}

#[test]
fn test_tuple_access_out_of_bounds() {
    let mut interpreter = Interpreter::new();
    
    // Cria tupla pequena
    interpreter.set_variable("tupla".to_string(), Value::Tuple(vec![
        Value::Number(1.0),
    ]));
    
    let expr = parse_expression("let valor = tupla.5;");
    let result = interpreter.evaluate(&expr);
    
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3084); // Índice fora dos limites da tupla
}
