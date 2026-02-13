// Testes para o módulo HTTP Server
// Arquivo: crates/dryad_runtime/tests/http_server_tests.rs

use dryad_runtime::{Interpreter, Value};
use dryad_lexer::Lexer;
use dryad_parser::Parser;

#[test]
fn test_http_server_creation() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("test_server", "127.0.0.1", 8081);
        "server_created"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "server_created");
            println!("✅ Criação de servidor funcionou");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_route_configuration() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("test_server", "127.0.0.1", 8082);
        native_http_server_get("test_server", "/", "<h1>Homepage</h1>");
        native_http_server_get("test_server", "/api", '{"message": "API working"}');
        native_http_server_post("test_server", "/api/data", '{"status": "received"}');
        "routes_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "routes_configured");
            println!("✅ Configuração de rotas funcionou");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_html_content() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("html_server", "127.0.0.1", 8083);
        native_http_server_html("html_server", "/dashboard", 
            "<!DOCTYPE html>
            <html>
            <head><title>Dashboard</title></head>
            <body><h1>Dashboard Dryad</h1></body>
            </html>");
        "html_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "html_configured");
            println!("✅ Configuração de HTML funcionou");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_json_content() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("json_server", "127.0.0.1", 8084);
        native_http_server_json("json_server", "/api/status", 
            '{"server": "dryad", "version": "0.1.0", "status": "running"}');
        "json_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "json_configured");
            println!("✅ Configuração de JSON funcionou");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_basic_operations() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    // Testa criação e configuração básica (sem start que pode travar)
    let code = r#"
        #<http_server>
        native_http_server_create("basic_server", "127.0.0.1", 8085);
        native_http_server_get("basic_server", "/", "<h1>Test Server</h1>");
        "server_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "server_configured");
            println!("✅ Operações básicas do servidor funcionaram");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_status() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("status_server", "127.0.0.1", 8086);
        let status = native_http_server_status("status_server");
        status
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(status_json) => {
            // Verifica se é JSON válido
            let parsed: Result<serde_json::Value, _> = serde_json::from_str(&status_json);
            assert!(parsed.is_ok(), "Status não é JSON válido");
            
            // Verifica conteúdo do status
            let status: serde_json::Value = parsed.unwrap();
            assert_eq!(status["server_id"], "status_server");
            assert_eq!(status["host"], "127.0.0.1");
            assert_eq!(status["port"], 8086);
            
            println!("✅ Status do servidor funcionou: {}", status_json);
        }
        _ => panic!("Esperado String (JSON), recebido: {:?}", result),
    }
}

#[test]
fn test_http_server_static_content() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    // Cria arquivo de teste
    std::fs::write("test_file.html", "<html><body><h1>Static Test</h1></body></html>")
        .expect("Falha ao criar arquivo de teste");
    
    let code = r#"
        #<http_server>
        native_http_server_create("static_server", "127.0.0.1", 8087);
        native_http_server_static("static_server", "/test.html", "test_file.html");
        "static_configured"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "static_configured");
            println!("✅ Configuração de conteúdo estático funcionou");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
    
    // Limpa arquivo de teste
    let _ = std::fs::remove_file("test_file.html");
}

#[test]
fn test_http_server_multiple_instances() {
    let mut interpreter = Interpreter::new();
    interpreter.activate_native_category("http_server").unwrap();
    
    let code = r#"
        #<http_server>
        native_http_server_create("server_1", "127.0.0.1", 8088);
        native_http_server_create("server_2", "127.0.0.1", 8089);
        native_http_server_create("server_3", "127.0.0.1", 8090);
        
        native_http_server_get("server_1", "/", "Server 1");
        native_http_server_get("server_2", "/", "Server 2");
        native_http_server_get("server_3", "/", "Server 3");
        
        "multiple_servers_created"
    "#;
    
    let mut lexer = Lexer::new(code);
    let mut parser = Parser::new_from_lexer(&mut lexer).expect("Criação do parser falhou");
    let program = parser.parse().expect("Parsing falhou");
    
    let result = interpreter.execute_and_return_value(&program).expect("Execução falhou");
    
    match result {
        Value::String(s) => {
            assert_eq!(s, "multiple_servers_created");
            println!("✅ Múltiplas instâncias de servidor funcionaram");
        }
        _ => panic!("Esperado String, recebido: {:?}", result),
    }
}