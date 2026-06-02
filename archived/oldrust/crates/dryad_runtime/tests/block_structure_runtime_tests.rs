use dryad_runtime::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};

fn run_block_code(source: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if matches!(token.token, Token::Eof) {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Err(_) => break,
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    
    // Execute all statements in program
    let mut last_value = Value::Null;
    for stmt in program.statements {
        last_value = interpreter.execute_statement(&stmt)?;
    }
    
    Ok(last_value)
}

#[test]
fn test_empty_block_runtime() {
    let result = run_block_code("{ }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Null));
}

#[test]
fn test_simple_block_with_variable() {
    let result = run_block_code("{ let x = 42; }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Null));
}

#[test]
fn test_block_variable_scoping() {
    // Variables declared in blocks should be scoped to that block
    let result = run_block_code("{ let x = 10; } let y = 20;");
    assert!(result.is_ok());
    // The last statement (let y = 20) should return null
    assert!(matches!(result.unwrap(), Value::Null));
}

#[test]
fn test_nested_block_scoping() {
    let result = run_block_code("{ let x = 1; { let y = 2; } }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Null));
}

#[test]
fn test_block_with_expressions() {
    let result = run_block_code("{ 5 + 3; \"hello\"; true; }");
    assert!(result.is_ok());
    // Last expression should return true
    assert!(matches!(result.unwrap(), Value::Bool(true)));
}

#[test]
fn test_block_multiple_statements() {
    let result = run_block_code("{ let a = 5; let b = 10; a + b; }");
    assert!(result.is_ok());
    // Last expression should return 15
    assert!(matches!(result.unwrap(), Value::Number(15.0)));
}

#[test]
fn test_block_variable_access() {
    let result = run_block_code("{ let x = 42; x; }");
    assert!(result.is_ok());
    // Should return the value of x
    assert!(matches!(result.unwrap(), Value::Number(42.0)));
}

#[test]
fn test_block_variable_modification() {
    let result = run_block_code("{ let x = 10; x = 20; x; }");
    assert!(result.is_ok());
    // Should return the modified value
    assert!(matches!(result.unwrap(), Value::Number(20.0)));
}

#[test]
fn test_multiple_blocks_separate_scopes() {
    let result = run_block_code("{ let x = 1; } { let x = 2; x; }");
    assert!(result.is_ok());
    // Second block should return its own x value
    assert!(matches!(result.unwrap(), Value::Number(2.0)));
}

#[test]
fn test_deeply_nested_blocks() {
    let result = run_block_code("{ { { let x = 99; x; } } }");
    assert!(result.is_ok());
    // Should return the deeply nested value
    assert!(matches!(result.unwrap(), Value::Number(99.0)));
}

#[test]
fn test_block_with_complex_expressions() {
    let result = run_block_code("{ let a = 5; let b = 3; (a + b) * 2; }");
    assert!(result.is_ok());
    // Should return (5 + 3) * 2 = 16
    assert!(matches!(result.unwrap(), Value::Number(16.0)));
}

#[test]
fn test_block_with_string_operations() {
    let result = run_block_code("{ let greeting = \"Hello\"; let name = \"World\"; greeting + \" \" + name; }");
    assert!(result.is_ok());
    // Should return concatenated string
    if let Value::String(s) = result.unwrap() {
        assert_eq!(s, "Hello World");
    } else {
        panic!("Expected string result");
    }
}

#[test]
fn test_block_with_boolean_logic() {
    let result = run_block_code("{ let a = true; let b = false; a && !b; }");
    assert!(result.is_ok());
    // Should return true && !false = true
    assert!(matches!(result.unwrap(), Value::Bool(true)));
}

#[test]
fn test_block_sequence_evaluation() {
    let result = run_block_code("{ 1; 2; 3; }");
    assert!(result.is_ok());
    // Should return the last expression value
    assert!(matches!(result.unwrap(), Value::Number(3.0)));
}

#[test]
fn test_empty_statements_in_block() {
    let result = run_block_code("{ ; let x = 5; ; x; ; }");
    assert!(result.is_ok());
    // Should ignore empty statements and return x
    assert!(matches!(result.unwrap(), Value::Number(5.0)));
}

#[test]
fn test_block_variable_shadowing() {
    let result = run_block_code("let x = 1; { let x = 2; x; }");
    assert!(result.is_ok());
    // Inner block should shadow outer variable
    assert!(matches!(result.unwrap(), Value::Number(2.0)));
}

#[test]
fn test_block_preserves_outer_scope() {
    // This test checks that after a block ends, outer scope is restored
    let result = run_block_code("let x = 10; { let x = 20; } x;");
    assert!(result.is_ok());
    // After block ends, outer x should still be 10
    assert!(matches!(result.unwrap(), Value::Number(10.0)));
}

#[test]
fn test_mixed_nested_block_patterns() {
    let result = run_block_code("{ let a = 1; { let b = 2; { let c = 3; a + b + c; } } }");
    assert!(result.is_ok());
    // Should access variables from all scope levels: 1 + 2 + 3 = 6
    assert!(matches!(result.unwrap(), Value::Number(6.0)));
}

#[test]
fn test_block_with_mathematical_sequence() {
    let result = run_block_code("{ let sum = 0; sum = sum + 1; sum = sum + 2; sum = sum + 3; sum; }");
    assert!(result.is_ok());
    // Should return 0 + 1 + 2 + 3 = 6
    assert!(matches!(result.unwrap(), Value::Number(6.0)));
}

#[test]
fn test_syntax_md_block_patterns() {
    // Test patterns that will be used in functions, classes, and control flow
    
    // Simple block like function body
    let result = run_block_code("{ let result = 42; result; }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Number(42.0)));
    
    // Block like if condition body
    let result = run_block_code("{ let condition = true; condition; }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Bool(true)));
    
    // Block with multiple calculations like method body
    let result = run_block_code("{ let width = 10; let height = 5; width * height; }");
    assert!(result.is_ok());
    assert!(matches!(result.unwrap(), Value::Number(50.0)));
}
