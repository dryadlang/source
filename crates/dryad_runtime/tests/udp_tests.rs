// Testes para o módulo UDP
// Testa funcionalidades de servidor e cliente UDP

#[cfg(test)]
mod udp_tests {
    use dryad_runtime::Interpreter;
    use dryad_parser::Parser;
    use dryad_lexer::{Lexer, token::Token};
    
    

    fn setup_interpreter_with_udp() -> Interpreter {
        let mut interpreter = Interpreter::new();
        interpreter.activate_native_category("udp").unwrap();
        interpreter.activate_native_category("time").unwrap();
        interpreter
    }

    fn execute_dryad_code(code: &str) -> Result<(), String> {
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token) => {
                    let is_eof = matches!(token.token, Token::Eof);
                    tokens.push(token);
                    if is_eof {
                        break;
                    }
                },
                Err(e) => return Err(format!("Lexer error: {:?}", e)),
            }
        }
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| format!("Parse error: {:?}", e))?;
        
        let mut interpreter = setup_interpreter_with_udp();
        interpreter.execute(&program).map_err(|e| format!("Runtime error: {:?}", e))?;
        
        Ok(())
    }

    #[test]
    fn test_udp_server_creation() {
        let code = r#"
            udp_server_create("test_server", "127.0.0.1", 9001);
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_client_creation() {
        let code = r#"
            udp_client_create("test_client", "127.0.0.1", 9002);
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_server_lifecycle() {
        let code = r#"
            udp_server_create("lifecycle_server", "127.0.0.1", 9003);
            udp_server_start("lifecycle_server");
            native_sleep(100);
            udp_server_stop("lifecycle_server");
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_client_configuration() {
        let code = r#"
            udp_client_create("config_client", "127.0.0.1", 9004);
            udp_client_set_timeout("config_client", 5000);
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_echo_communication() {
        let code = r#"
            // Criar servidor
            udp_server_create("echo_server", "127.0.0.1", 9005);
            udp_server_start("echo_server");
            
            // Aguardar inicialização
            native_sleep(200);
            
            // Criar cliente
            udp_client_create("echo_client", "127.0.0.1", 0);
            udp_client_bind("echo_client", 0);
            
            // Enviar mensagem usando send_to
            udp_client_send_to("echo_client", "Hello UDP!", "127.0.0.1", 9005);
            
            // Aguardar processamento
            native_sleep(100);
            
            // Limpar
            udp_server_stop("echo_server");
        "#;
        
        match execute_dryad_code(code) {
            Ok(_) => println!("✅ Teste passou"),
            Err(e) => {
                println!("❌ Erro no teste: {}", e);
                panic!("Teste falhou: {}", e);
            }
        }
    }

    #[test]
    fn test_udp_port_availability() {
        let code = r#"
            let available = udp_port_available(9006);
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_utilities() {
        let code = r#"
            let local_ip = udp_get_local_ip();
            let resolved = udp_resolve_hostname("localhost");
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_server_status() {
        let code = r#"
            udp_server_create("status_server", "127.0.0.1", 9007);
            let status = udp_server_status("status_server");
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }

    #[test]
    fn test_udp_client_status() {
        let code = r#"
            udp_client_create("status_client", "127.0.0.1", 9008);
            let status = udp_client_status("status_client");
        "#;
        
        assert!(execute_dryad_code(code).is_ok());
    }
}