// crates/tests/integration_foreach_test.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn execute_dryad_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            Token::Eof => break,
            token => tokens.push(token),
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let mut interpreter = Interpreter::new();
    interpreter.execute_and_return_value(&program)
}

#[test]
fn test_integration_foreach_complete_example() {
    let code = r#"
        let sum = 0;
        let numbers = [10, 20, 30, 40, 50];
        
        for num in numbers {
            sum = sum + num;
        }
        
        sum
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(150.0));
}

#[test]
fn test_integration_foreach_nested_with_arrays() {
    let code = r#"
        let total = 0;
        let matrix = [[1, 2], [3, 4], [5, 6]];
        
        for row in matrix {
            for cell in row {
                total = total + cell;
            }
        }
        
        total
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(21.0)); // 1+2+3+4+5+6 = 21
}

#[test]
fn test_integration_foreach_string_iteration() {
    let code = r#"
        let char_count = 0;
        let text = "Hello";
        
        for c in text {
            char_count = char_count + 1;
        }
        
        char_count
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_integration_foreach_with_break_and_continue() {
    let code = r#"
        let sum = 0;
        
        for x in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] {
            if x == 5 {
                continue;
            }
            if x == 8 {
                break;
            }
            sum = sum + x;
        }
        
        sum
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    // Soma: 1 + 2 + 3 + 4 + 6 + 7 = 23 (pula 5, para em 8)
    assert_eq!(result, Value::Number(23.0));
}

#[test]
fn test_integration_foreach_tuple_mixed_types() {
    let code = r#"
        let count = 0;
        let mixed = (1, "text", true, 42);
        
        for item in mixed {
            count = count + 1;
        }
        
        count
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_integration_foreach_vs_traditional_for() {
    let code_foreach = r#"
        let sum_foreach = 0;
        for x in [1, 2, 3, 4, 5] {
            sum_foreach = sum_foreach + x;
        }
        sum_foreach
    "#;
    
    let code_traditional = r#"
        let sum_traditional = 0;
        for i = 1; i <= 5; i = i + 1 {
            sum_traditional = sum_traditional + i;
        }
        sum_traditional
    "#;
    
    let result_foreach = execute_dryad_code(code_foreach).unwrap();
    let result_traditional = execute_dryad_code(code_traditional).unwrap();
    
    // Ambos devem dar o mesmo resultado: 15
    assert_eq!(result_foreach, Value::Number(15.0));
    assert_eq!(result_traditional, Value::Number(15.0));
    assert_eq!(result_foreach, result_traditional);
}

#[test]
fn test_integration_foreach_empty_collections() {
    let code = r#"
        let count = 0;
        
        for x in [] {
            count = count + 1;
        }
        
        for y in () {
            count = count + 1;
        }
        
        for z in "" {
            count = count + 1;
        }
        
        count
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(0.0)); // Nenhuma iteração
}
