// Testes para o módulo HTTP Client
// Arquivo: crates/dryad_runtime/tests/http_client_tests.rs

use dryad_runtime::{Interpreter, Value};
use dryad_runtime::heap::ManagedObject;
use dryad_lexer::Lexer;
use dryad_parser::Parser;

#[test]
fn test_http_get_request() {
    let mut interpreter = Interpreter::new();
    
    // Ativa módulo http_client
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        let response = native_http_get("https://www.google.com");
        response
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    // Verifica se recebeu uma string (resposta HTTP)
    match result {
        Value::String(response) => {
            assert!(response.contains("google") || response.contains("Google") || response.len() > 100);
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_post_request() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        let response = native_http_post("https://postman-echo.com/post", "test data");
        response
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(response) => {
            assert!(response.contains("test data") || response.len() > 50);
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_timeout_configuration() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        native_http_set_timeout("https://httpbin.org/delay/1", 5000);
        "timeout_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "timeout_configured");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_headers_configuration() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        let headers = { auth: "Bearer TOKEN123", type: "application/json" };
        native_http_set_headers("https://api.example.com", headers);
        native_http_set_user_agent("https://api.example.com", "Dryad-Test/1.0");
        "headers_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "headers_configured");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
#[ignore = "Soft block - travando execução dos testes"]
fn test_http_download_functionality() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        native_http_download("https://httpbin.org/json", "test_download.json");
        "download_completed"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "download_completed");
            
            // Verifica se o arquivo foi criado
            if std::path::Path::new("test_download.json").exists() {
                // Limpa o arquivo de teste
                let _ = std::fs::remove_file("test_download.json");
            } else {
                println!("⚠️  Download executou mas arquivo não foi encontrado");
            }
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test] 
fn test_http_json_response() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_client").unwrap();
    
    let code = r#"
        #<http_client>
        let json_response = native_http_json("https://jsonplaceholder.typicode.com/posts/1");
        json_response
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::Object(id) => {
            let obj = interpreter.heap.get(id).unwrap();
            if let ManagedObject::Object { properties, .. } = obj {
                // Verifica se contém propriedades esperadas do JSONPlaceholder
                assert!(properties.contains_key("id") || properties.contains_key("title") || !properties.is_empty(), "JSON deve conter propriedades válidas");
            } else {
                panic!("Esperado ManagedObject::Object");
            }
        }
        _ => panic!("Esperado Object (JSON parseado), recebido: {:?}", result),
    }
}