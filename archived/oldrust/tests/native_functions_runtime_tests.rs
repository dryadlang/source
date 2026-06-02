use dryad_lexer::lexer::Lexer;
use dryad_parser::parser::Parser;
use dryad_runtime::interpreter::Interpreter;
use dryad_runtime::value::Value;

// Helper function para executar código Dryad e obter resultado
fn execute_dryad_code(code: &str) -> Result<Option<Value>, String> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize().map_err(|e| format!("Lexer error: {:?}", e))?;
    
    let mut parser = Parser::new(tokens);
    let ast = parser.parse().map_err(|e| format!("Parser error: {:?}", e))?;
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret(ast).map_err(|e| format!("Runtime error: {:?}", e))?;
    Ok(result)
}

#[test]
fn test_console_io_directive() {
    let code = r#"
        #<console_io>
        let msg = "Hello, World!";
        print(msg);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute console_io directive: {:?}", result.err());
}

#[test]
fn test_multiple_directives() {
    let code = r#"
        #<console_io>
        #<debug>
        let x = 42;
        debug(x);
        print("Debug test");
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute multiple directives: {:?}", result.err());
}

#[test]
fn test_unknown_module_directive() {
    let code = r#"
        #<unknown_module>
        let x = 1;
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err(), "Should fail with unknown module");
    
    if let Err(error_msg) = result {
        assert!(error_msg.contains("Unknown native module"), 
               "Error should mention unknown module: {}", error_msg);
    }
}

#[test]
fn test_native_function_call() {
    let code = r#"
        #<debug>
        let value = 123;
        debug(value);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to call native function: {:?}", result.err());
}

#[test]
fn test_file_io_directive() {
    let code = r#"
        #<file_io>
        let path = "test.txt";
        file_exists(path);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute file_io directive: {:?}", result.err());
}

#[test]
fn test_system_env_directive() {
    let code = r#"
        #<system_env>
        let home = get_env("HOME");
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute system_env directive: {:?}", result.err());
}

#[test]
fn test_terminal_ansi_directive() {
    let code = r#"
        #<terminal_ansi>
        let red_text = ansi_red("Error message");
        print(red_text);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute terminal_ansi directive: {:?}", result.err());
}

#[test]
fn test_date_time_directive() {
    let code = r#"
        #<date_time>
        let now = current_timestamp();
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute date_time directive: {:?}", result.err());
}

#[test]
fn test_crypto_directive() {
    let code = r#"
        #<crypto>
        let data = "hello";
        let hash = sha256(data);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute crypto directive: {:?}", result.err());
}

#[test]
fn test_binary_io_directive() {
    let code = r#"
        #<binary_io>
        let num = 255;
        let hex = to_hex(num);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute binary_io directive: {:?}", result.err());
}

#[test]
fn test_directive_with_regular_code() {
    let code = r#"
        let x = 10;
        let y = 20;
        
        #<console_io>
        print("Sum:");
        
        let sum = x + y;
        print(sum);
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_ok(), "Failed to execute directive with regular code: {:?}", result.err());
}

#[test]
fn test_call_function_without_directive() {
    let code = r#"
        // Tentando chamar função nativa sem carregar o módulo
        print("This should fail");
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err(), "Should fail when calling function without loading module");
}
