// crates/dryad_runtime/tests/native_functions_tests.rs

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
fn test_native_directive_console_io() {
    let code = r#"
        #<console_io>
        native_print("Hello World")
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Null); // native_print retorna null
}

#[test]
fn test_native_directive_file_io() {
    let code = r#"
        #<file_io>
        let exists = native_file_exists("nonexistent_file.txt");
        exists
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Bool(false));
}

#[test]
fn test_native_directive_debug() {
    let code = r#"
        #<debug>
        let type_name = native_typeof(42);
        type_name
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("number".to_string()));
}

#[test]
fn test_native_directive_system_env() {
    let code = r#"
        #<system_env>
        let platform = native_platform();
        platform
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    // O resultado depende da plataforma, mas deve ser uma string
    match result {
        Value::String(platform) => {
            assert!(platform == "windows" || platform == "linux" || platform == "macos");
        }
        _ => panic!("Esperado string"),
    }
}

#[test]
fn test_multiple_native_directives() {
    let code = r#"
        #<console_io>
        #<debug>
        #<system_env>
        
        let platform = native_platform();
        let type_name = native_typeof(platform);
        type_name
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("string".to_string()));
}

#[test]
fn test_native_directive_date_time() {
    let code = r#"
        #<date_time>
        let timestamp = native_timestamp();
        timestamp
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    match result {
        Value::Number(n) => assert!(n > 0.0),
        _ => panic!("Esperado número"),
    }
}

#[test]
fn test_native_print_with_expression() {
    let code = r#"
        #<console_io>
        native_print("Value: " + (5 + 3))
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_native_functions_with_variables() {
    let code = r#"
        #<debug>
        let x = 42;
        let y = "hello";
        let z = true;
        
        let type_x = native_typeof(x);
        let type_y = native_typeof(y);
        let type_z = native_typeof(z);
        
        type_x + "," + type_y + "," + type_z
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("number,string,boolean".to_string()));
}

#[test]
fn test_error_unknown_native_module() {
    let code = r#"
        #<unknown_module>
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 6001);
    assert!(error.message().contains("Módulo nativo desconhecido: unknown_module"));
}

#[test]
fn test_error_native_function_without_directive() {
    let code = r#"
        native_print("Hello")
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3003);
    assert!(error.message().contains("não definida"));
}

#[test]
fn test_native_function_with_wrong_arguments() {
    let code = r#"
        #<file_io>
        native_read_file()
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3004);
    assert!(error.message().contains("espera 1 argumento"));
}

#[test]
fn test_native_directive_crypto() {
    let code = r#"
        #<crypto>
        let uuid = native_uuid();
        uuid
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    match result {
        Value::String(uuid) => {
            // UUID deve ter o formato básico xxxx-xxxx-xxxx-xxxx
            assert!(uuid.len() > 10);
            assert!(uuid.contains("-"));
        }
        _ => panic!("Esperado string"),
    }
}

#[test]
fn test_native_typeof_all_types() {
    let code = r#"
        #<debug>
        
        let number_type = native_typeof(42);
        let string_type = native_typeof("hello");
        let bool_type = native_typeof(true);
        let null_type = native_typeof(null);
        
        number_type + "," + string_type + "," + bool_type + "," + null_type
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("number,string,boolean,null".to_string()));
}

#[test]
fn test_native_sleep_function() {
    let code = r#"
        #<date_time>
        native_sleep(1)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Null);
}

#[test]
fn test_native_uptime_function() {
    let code = r#"
        #<date_time>
        let uptime = native_uptime();
        uptime
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    match result {
        Value::Number(n) => assert!(n >= 0.0),
        _ => panic!("Esperado número"),
    }
}
