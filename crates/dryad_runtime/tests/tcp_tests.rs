#[cfg(test)]
mod tests {
    use dryad_lexer::Lexer;
    use dryad_parser::Parser;
    use dryad_runtime::Interpreter;

    fn setup_interpreter_with_tcp() -> Interpreter {
        let mut interpreter = Interpreter::new();
        // Ativa os módulos TCP e Time
        interpreter
            .activate_native_category("tcp")
            .expect("Falha ao ativar módulo TCP");
        interpreter
            .activate_native_category("time")
            .expect("Falha ao ativar módulo Time");
        interpreter
            .activate_native_category("console_io")
            .expect("Falha ao ativar módulo Console I/O");
        interpreter
    }

    fn execute_dryad_code(code: &str) -> Result<(), String> {
        let mut interpreter = setup_interpreter_with_tcp();

        // Gerar tokens usando next_token em loop
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        loop {
            let token = lexer
                .next_token()
                .map_err(|e| format!("Erro léxico: {}", e))?;
            let is_eof = matches!(token.token, dryad_lexer::Token::Eof);
            tokens.push(token);
            if is_eof {
                break;
            }
        }

        let mut parser = Parser::new(tokens);
        let program = parser
            .parse()
            .map_err(|e| format!("Erro de parsing: {}", e))?;

        interpreter
            .execute(&program)
            .map_err(|e| format!("Erro de runtime: {}", e))?;
        Ok(())
    }

    #[test]
    fn test_tcp_server_creation() {
        let code = r#"
            #<tcp>
            tcp_server_create("test_server", "127.0.0.1", 9001, 5);
        "#;

        execute_dryad_code(code).expect("Falha ao criar servidor TCP");
    }

    #[test]
    fn test_tcp_client_creation() {
        let code = r#"
            #<tcp>
            tcp_client_create("test_client", "127.0.0.1", 9002);
        "#;

        execute_dryad_code(code).expect("Falha ao criar cliente TCP");
    }

    #[test]
    fn test_tcp_port_availability() {
        let code = r#"
            #<tcp>
            let available = tcp_port_available(9003);
            print("Porta 9003 disponível: " + available);
        "#;

        execute_dryad_code(code).expect("Falha ao verificar disponibilidade da porta");
    }

    #[test]
    fn test_tcp_server_lifecycle() {
        let code = r#"
            #<tcp>
            tcp_server_create("lifecycle_server", "127.0.0.1", 9004, 3);
            
            let status_before = tcp_server_status("lifecycle_server");
            print("Status antes de iniciar: " + status_before.is_running);
            
            tcp_server_start("lifecycle_server");
            
            let status_after_start = tcp_server_status("lifecycle_server");
            print("Status após iniciar: " + status_after_start.is_running);
            
            native_sleep(100); // Aguarda 100ms
            
            tcp_server_stop("lifecycle_server");
            
            let status_after_stop = tcp_server_status("lifecycle_server");
            print("Status após parar: " + status_after_stop.is_running);
        "#;

        execute_dryad_code(code).expect("Falha no ciclo de vida do servidor TCP");
    }

    #[test]
    fn test_tcp_client_configuration() {
        let code = r#"
            #<tcp>
            tcp_client_create("config_client", "127.0.0.1", 9005);
            
            tcp_client_set_timeout("config_client", 15);
            
            let status = tcp_client_status("config_client");
            print("Cliente criado: " + status.client_id);
            print("Timeout configurado: " + status.timeout_secs);
        "#;

        execute_dryad_code(code).expect("Falha na configuração do cliente TCP");
    }

    #[test]
    fn test_tcp_utilities() {
        let code = r#"
            #<tcp>
            
            // Testa verificação de porta disponível
            let port_8080_available = tcp_port_available(8080);
            print("Porta 8080 disponível: " + port_8080_available);
            
            // Testa obter IP local
            let local_ip = tcp_get_local_ip();
            print("IP local: " + local_ip);
            
            // Testa resolução de hostname
            let google_ip = tcp_resolve_hostname("google.com");
            print("Google IP: " + google_ip);
        "#;

        execute_dryad_code(code).expect("Falha nas funções utilitárias TCP");
    }

    #[test]
    fn test_tcp_server_max_clients_setting() {
        let code = r#"
            #<tcp>
            tcp_server_create("max_clients_server", "127.0.0.1", 9006, 2);
            
            tcp_server_set_max_clients("max_clients_server", 10);
            
            let status = tcp_server_status("max_clients_server");
            print("Máximo de clientes configurado: " + status.max_clients);
        "#;

        execute_dryad_code(code).expect("Falha ao configurar máximo de clientes");
    }

    #[test]
    fn test_tcp_error_handling() {
        let code = r#"
            #<tcp>
            
            try {
                // Tentar obter status de servidor inexistente
                tcp_server_status("inexistente");
            } catch (e) {
                print("Erro capturado corretamente: " + e);
            }
            
            try {
                // Tentar conectar cliente inexistente
                tcp_client_connect("inexistente");
            } catch (e) {
                print("Erro de cliente capturado: " + e);
            }
        "#;

        execute_dryad_code(code).expect("Falha no tratamento de erros TCP");
    }

    #[test]
    #[ignore = "Teste de integração TCP com problemas de timing"]
    fn test_tcp_client_server_integration() {
        // Esse teste é mais complexo e simula uma comunicação real
        let code = r#"
            #<tcp>
            
            // Criar servidor
            tcp_server_create("integration_server", "127.0.0.1", 9007, 5);
            tcp_server_start("integration_server");
            
            native_sleep(200); // Aguarda servidor inicializar
            
            // Criar cliente
            tcp_client_create("integration_client", "127.0.0.1", 9007);
            
            let connected = tcp_client_connect("integration_client");
            print("Cliente conectado: " + connected);
            
            if (connected) {
                // Enviar dados
                let sent = tcp_client_send("integration_client", "Hello TCP Server!");
                print("Dados enviados: " + sent);
                
                // Tentar receber resposta
                let response = tcp_client_receive("integration_client");
                print("Resposta recebida: " + response);
                
                // Desconectar
                tcp_client_disconnect("integration_client");
            }
            
            // Parar servidor
            tcp_server_stop("integration_server");
        "#;

        execute_dryad_code(code).expect("Falha na integração cliente-servidor TCP");
    }
}
