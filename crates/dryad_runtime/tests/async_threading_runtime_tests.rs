// crates/dryad_runtime/tests/async_threading_runtime_tests.rs
use dryad_runtime::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};
use dryad_errors::DryadError;

fn parse_and_execute(input: &str) -> Result<Value, DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        match token.token {
            Token::Eof => break,
            _ => tokens.push(token)
        }
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    interpreter.execute_and_return_value(&program)
}

#[test]
fn test_async_function_declaration() {
    let input = r#"
        async function getData() {
            return "async data";
        }
        
        getData
    "#;
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::AsyncFunction { name, params, .. } => {
            assert_eq!(name, "getData");
            assert_eq!(params.len(), 0);
        }
        _ => panic!("Esperado AsyncFunction, encontrado: {:?}", result),
    }
}

#[test]
fn test_thread_function_declaration() {
    let input = r#"
        thread function backgroundTask(data) {
            return data;
        }
        
        backgroundTask
    "#;
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::ThreadFunction { name, params, .. } => {
            assert_eq!(name, "backgroundTask");
            assert_eq!(params.len(), 1);
            assert_eq!(params[0], "data");
        }
        _ => panic!("Esperado ThreadFunction, encontrado: {:?}", result),
    }
}

#[test]
fn test_mutex_creation() {
    let input = "mutex()";
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::Mutex { id, locked } => {
            assert!(id > 0);
            assert!(!locked);
        }
        _ => panic!("Esperado Mutex, encontrado: {:?}", result),
    }
}

#[test]
fn test_thread_instantiation() {
    let input = r#"
        function simpleTask() {
            return "task result";
        }
        
        thread(simpleTask)
    "#;
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::Thread { id, is_running } => {
            assert!(id > 0);
            assert!(is_running);
        }
        _ => panic!("Esperado Thread, encontrado: {:?}", result),
    }
}

#[test] 
fn test_simple_await() {
    let input = r#"
        let value = "hello";
        await value
    "#;
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "hello");
        }
        _ => panic!("Esperado String, encontrado: {:?}", result),
    }
}

#[test]
fn test_mutex_and_thread_combo() {
    let input = r#"
        let lock = mutex();
        
        function incrementTask() {
            return "incremented";
        }
        
        let myThread = thread(incrementTask);
        lock
    "#;
    
    let result = parse_and_execute(input).expect("Deveria executar sem erro");
    
    match result {
        Value::Mutex { id, locked } => {
            assert!(id > 0);
            assert!(!locked);
        }
        _ => panic!("Esperado Mutex, encontrado: {:?}", result),
    }
}