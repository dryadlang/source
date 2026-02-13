// crates/dryad_errors/tests/integration_tests.rs

use dryad_errors::*;
use std::path::PathBuf;
use std::collections::HashMap;

#[cfg(test)]
mod integration_tests {
    use super::*;

    /// Test-Driven Development: Teste que demonstra como o novo sistema
    /// de erros facilita o debug durante o desenvolvimento
    #[test]
    fn test_error_driven_development_workflow() {
        // Cenário: Desenvolvedor está criando uma função de divisão
        // e quer implementar tratamento de erro robusto
        
        // 1. Primeiro, criamos um erro de runtime detalhado
        let location = SourceLocation::new(
            Some(PathBuf::from("src/calculator.dryad")),
            15,
            20,
            350
        ).with_source_line("result = numerator / denominator;".to_string());
        
        // 2. Criamos stack trace realístico
        let mut stack_trace = StackTrace::new();
        
        stack_trace.push_frame(StackFrame::new(
            "main".to_string(),
            SourceLocation::new(Some(PathBuf::from("src/calculator.dryad")), 1, 1, 0)
        ));
        
        stack_trace.push_frame(StackFrame::new(
            "calculate_average".to_string(),
            SourceLocation::new(Some(PathBuf::from("src/calculator.dryad")), 8, 5, 150)
        ).with_context("processing array of values".to_string()));
        
        stack_trace.push_frame(StackFrame::new(
            "safe_divide".to_string(),
            SourceLocation::new(Some(PathBuf::from("src/calculator.dryad")), 12, 8, 250)
        ).with_context("within validation block".to_string()));
        
        // 3. Adicionamos contexto de variáveis
        let mut variables = HashMap::new();
        variables.insert("numerator".to_string(), "100.5".to_string());
        variables.insert("denominator".to_string(), "0.0".to_string());
        variables.insert("values_count".to_string(), "5".to_string());
        
        let debug_context = DebugContext::new()
            .with_variables(variables)
            .with_suggestion("Implemente validação: if (denominator == 0) { throw error; }".to_string())
            .with_suggestion("Considere retornar um valor especial como Infinity ou NaN".to_string())
            .with_suggestion("Use try-catch para capturar este erro em contexto superior".to_string())
            .with_help_url("https://docs.dryad.com/error-handling#division-by-zero".to_string());
        
        let error = DryadError::runtime(
            3001,
            "Divisão por zero detectada durante cálculo de média",
            location,
            stack_trace
        ).with_debug_context(debug_context);
        
        // 4. Verificamos se todas as informações estão presentes
        assert_eq!(error.code(), 3001);
        assert!(error.message().contains("Divisão por zero"));
        assert_eq!(error.location().line, 15);
        assert_eq!(error.location().column, 20);
        
        // 5. Verificamos se o stack trace está correto
        if let DryadError::Runtime { stack_trace, .. } = &error {
            assert_eq!(stack_trace.frames.len(), 3);
            assert_eq!(stack_trace.frames[2].function_name, "safe_divide");
            assert_eq!(stack_trace.frames[2].context, Some("within validation block".to_string()));
        }
        
        // 6. Testamos a saída formatada
        let error_output = format!("{}", error);
        assert!(error_output.contains("🚨 E3001: Erro de Runtime"));
        assert!(error_output.contains("📍 Local: src/calculator.dryad:15:20"));
        assert!(error_output.contains("📚 Stack Trace:"));
        assert!(error_output.contains("🔍 Variáveis locais:"));
        assert!(error_output.contains("numerator = 100.5"));
        assert!(error_output.contains("denominator = 0.0"));
        assert!(error_output.contains("💡 Sugestões:"));
        assert!(error_output.contains("try-catch"));
    }
    
    /// Teste que demonstra como criar erros com diferentes níveis de contexto
    /// para facilitar o desenvolvimento incremental (TDD)
    #[test]
    fn test_progressive_error_context_building() {
        // Fase 1: Erro simples (compatibilidade com código antigo)
        let basic_error = DryadError::new(2001, "Token inesperado");
        assert_eq!(basic_error.code(), 2001);
        assert_eq!(basic_error.message(), "Token inesperado");
        
        // Fase 2: Adicionar localização
        let location = SourceLocation::new(None, 10, 5, 100);
        let error_with_location = DryadError::parser(
            2001,
            "Token inesperado",
            location,
            vec!["';'".to_string()],
            "'{'".to_string()
        );
        
        assert_eq!(error_with_location.location().line, 10);
        assert_eq!(error_with_location.location().column, 5);
        
        // Fase 3: Adicionar contexto de debug
        let debug_context = DebugContext::new()
            .with_suggestion("Adicione ';' no final da declaração".to_string());
            
        let full_error = error_with_location.with_debug_context(debug_context);
        
        let output = format!("{}", full_error);
        assert!(output.contains("💡 Sugestões:"));
        assert!(output.contains("Adicione ';'"));
    }
    
    /// Teste que verifica se warnings com diferentes severidades
    /// são tratados corretamente
    #[test]
    fn test_warning_severity_levels() {
        let location = SourceLocation::new(None, 1, 1, 0);
        
        let low_warning = DryadError::Warning {
            code: 8001,
            message: "Variável não utilizada".to_string(),
            location: location.clone(),
            severity: WarningSeverity::Low,
            debug_context: None,
        };
        
        let medium_warning = DryadError::Warning {
            code: 8002,
            message: "Função deprecated".to_string(),
            location: location.clone(),
            severity: WarningSeverity::Medium,
            debug_context: None,
        };
        
        let high_warning = DryadError::Warning {
            code: 8003,
            message: "Potencial vazamento de memória".to_string(),
            location: location,
            severity: WarningSeverity::High,
            debug_context: None,
        };
        
        // Testamos se os ícones de severidade são diferentes
        let low_output = format!("{}", low_warning);
        let medium_output = format!("{}", medium_warning);
        let high_output = format!("{}", high_warning);
        
        assert!(low_output.contains("⚠️"));
        assert!(medium_output.contains("🟡"));
        assert!(high_output.contains("🟠"));
    }
    
    /// Teste que simula um fluxo completo de TDD:
    /// 1. Escrever teste que falha
    /// 2. Implementar código mínimo 
    /// 3. Refatorar com tratamento de erro detalhado
    #[test]
    fn test_tdd_error_workflow() {
        // RED: Criamos um teste que falha com erro bem detalhado
        
        fn simulated_parser_function(code: &str) -> Result<String, DryadError> {
            if code.contains("invalid_syntax") {
                let location = SourceLocation::new(
                    Some(PathBuf::from("test_input.dryad")),
                    1,
                    code.find("invalid_syntax").unwrap() + 1,
                    0
                ).with_source_line(code.to_string());
                
                let debug_context = DebugContext::new()
                    .with_suggestion("Use 'let' para declarar variáveis".to_string())
                    .with_suggestion("Verifique a documentação de sintaxe".to_string())
                    .with_help_url("https://docs.dryad.com/syntax".to_string());
                
                return Err(DryadError::parser(
                    2050,
                    "Sintaxe de declaração de variável inválida",
                    location,
                    vec!["let".to_string(), "const".to_string()],
                    "invalid_syntax".to_string()
                ).with_debug_context(debug_context));
            }
            
            Ok("parsed successfully".to_string())
        }
        
        // GREEN: Testamos o comportamento esperado
        let valid_result = simulated_parser_function("let x = 5;");
        assert!(valid_result.is_ok());
        
        // Testamos o erro detalhado
        let error_result = simulated_parser_function("invalid_syntax x = 5;");
        assert!(error_result.is_err());
        
        let error = error_result.unwrap_err();
        assert_eq!(error.code(), 2050);
        
        let error_output = format!("{}", error);
        assert!(error_output.contains("Sintaxe de declaração"));
        assert!(error_output.contains("📝 Esperado: let, const"));
        assert!(error_output.contains("❌ Encontrado: invalid_syntax"));
        assert!(error_output.contains("💡 Sugestões:"));
        
        // REFACTOR: O erro já fornece contexto suficiente para debug
        println!("Error output for TDD debugging:\n{}", error_output);
    }
}