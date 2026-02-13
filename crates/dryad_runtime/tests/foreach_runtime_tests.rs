// crates/dryad_runtime/tests/foreach_runtime_tests.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};

fn parse_and_execute(input: &str) -> Result<Value, dryad_errors::DryadError> {
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
fn test_foreach_array_basic() {
    let input = r#"
        let sum = 0;
        for (x in [1, 2, 3] ) {
            sum = sum + x;
        }
        sum
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
fn test_foreach_array_variable() {
    let input = r#"
        let numbers = [10, 20, 30];
        let total = 0;
        for (num in numbers ) {
            total = total + num;
        }
        total
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(60.0));
}

#[test]
fn test_foreach_tuple_basic() {
    let input = r#"
        let count = 0;
        for (item in (1, 2, 3, 4) ) {
            count = count + 1;
        }
        count
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(4.0));
}

#[test]
fn test_foreach_string_iteration() {
    let input = r#"
        let count = 0;
        for (char in "abc" ) {
            count = count + 1;
        }
        count
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_foreach_variable_scope() {
    let input = r#"
        let x = 99;
        for (x in [1, 2, 3] ) {
            x = x + 10;
        }
        x
    "#;
    
    let result = parse_and_execute(input).unwrap();
    // x deve voltar ao valor original após o foreach
    assert_eq!(result, Value::Number(99.0));
}

#[test]
fn test_foreach_nested_loops() {
    let input = r#"
        let result = 0;
        for (outer in [1, 2] ) {
            for (inner in [10, 20] ) {
                result = result + outer + inner;
            }
        }
        result
    "#;
    
    let result = parse_and_execute(input).unwrap();
    // (1+10) + (1+20) + (2+10) + (2+20) = 11 + 21 + 12 + 22 = 66
    assert_eq!(result, Value::Number(66.0));
}

#[test]
fn test_foreach_empty_array() {
    let input = r#"
        let count = 0;
        for (item in [] ) {
            count = count + 1;
        }
        count
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_foreach_empty_tuple() {
    let input = r#"
        let count = 0;
        for (item in () ) {
            count = count + 1;
        }
        count
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::Number(0.0));
}

#[test]
fn test_foreach_with_break() {
    let input = r#"
        let sum = 0;
        for (x in [1, 2, 3, 4, 5] ) {
            if x == 3 {
                break;
            }
            sum = sum + x;
        }
        sum
    "#;
    
    let result = parse_and_execute(input).unwrap();
    // Soma 1 + 2 = 3, para no 3
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_foreach_with_continue() {
    let input = r#"
        let sum = 0;
        for (x in [1, 2, 3, 4, 5] ) {
            if x == 3 {
                continue;
            }
            sum = sum + x;
        }
        sum
    "#;
    
    let result = parse_and_execute(input).unwrap();
    // Soma 1 + 2 + 4 + 5 = 12, pula o 3
    assert_eq!(result, Value::Number(12.0));
}

#[test]
fn test_foreach_non_iterable_error() {
    let input = r#"
        for (x in 42 ) {
            x = x + 1;
        }
    "#;
    
    let result = parse_and_execute(input);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3030); // "Valor não é iterável"
}

#[test]
fn test_foreach_mixed_types_tuple() {
    let input = r#"
        let types = "";
        for (item in (1, "text", true) ) {
            if item == 1 {
                types = types + "num";
            }
        }
        types
    "#;
    
    let result = parse_and_execute(input).unwrap();
    assert_eq!(result, Value::String("num".to_string()));
}

#[test]
fn test_foreach_array_index_access() {
    let input = r#"
        let data = [[1, 2], [3, 4]];
        let sum = 0;
        for (row in data ) {
            for (cell in row ) {
                sum = sum + cell;
            }
        }
        sum
    "#;
    
    let result = parse_and_execute(input).unwrap();
    // 1 + 2 + 3 + 4 = 10
    assert_eq!(result, Value::Number(10.0));
}


