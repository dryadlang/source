// crates/dryad_runtime/tests/variable_tests.rs
use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::Lexer;
use dryad_errors::DryadError;

#[cfg(test)]
mod variable_tests {
    use super::*;

    fn execute_program(source: &str) -> Result<String, DryadError> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token()?;
            if matches!(token.token, dryad_lexer::Token::Eof) {
                tokens.push(token);
                break;
            }
            tokens.push(token);
        }
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        
        let mut interpreter = Interpreter::new();
        interpreter.execute(&program)
    }

    // Testes de Declaração de Variáveis
    #[test]
    fn test_variable_declaration_with_number() {
        assert_eq!(execute_program("let x = 42;").unwrap(), "null");
    }

    #[test]
    fn test_variable_declaration_with_string() {
        assert_eq!(execute_program("let name = \"João\";").unwrap(), "null");
    }

    #[test]
    fn test_variable_declaration_with_boolean() {
        assert_eq!(execute_program("let active = true;").unwrap(), "null");
    }

    #[test]
    fn test_variable_declaration_with_null() {
        assert_eq!(execute_program("let empty = null;").unwrap(), "null");
    }

    #[test]
    fn test_variable_declaration_without_value() {
        assert_eq!(execute_program("let x;").unwrap(), "null");
    }

    #[test]
    fn test_variable_declaration_with_expression() {
        assert_eq!(execute_program("let result = 2 + 3 * 4;").unwrap(), "null");
    }

    // Testes de Uso de Variáveis
    #[test]
    fn test_variable_usage() {
        let source = r#"
            let x = 10;
            x;
        "#;
        assert_eq!(execute_program(source).unwrap(), "10");
    }

    #[test]
    fn test_variable_in_expression() {
        let source = r#"
            let x = 5;
            let y = 3;
            x + y;
        "#;
        assert_eq!(execute_program(source).unwrap(), "8");
    }

    #[test]
    fn test_variable_in_complex_expression() {
        let source = r#"
            let a = 2;
            let b = 3;
            let c = 4;
            a + b * c;
        "#;
        assert_eq!(execute_program(source).unwrap(), "14");
    }

    #[test]
    fn test_string_variable_concatenation() {
        let source = r#"
            let first = "Hello";
            let second = "World";
            first + " " + second;
        "#;
        assert_eq!(execute_program(source).unwrap(), "Hello World");
    }

    #[test]
    fn test_boolean_variables_in_logic() {
        let source = r#"
            let a = true;
            let b = false;
            a && b;
        "#;
        assert_eq!(execute_program(source).unwrap(), "false");
    }

    // Testes de Múltiplas Declarações
    #[test]
    fn test_multiple_declarations() {
        let source = r#"
            let x = 10;
            let y = 20;
            let z = 30;
            x + y + z;
        "#;
        assert_eq!(execute_program(source).unwrap(), "60");
    }

    #[test]
    fn test_variable_shadowing_same_name() {
        let source = r#"
            let x = 10;
            let x = 20;
            x;
        "#;
        assert_eq!(execute_program(source).unwrap(), "20");
    }

    #[test]
    fn test_variable_declaration_using_previous() {
        let source = r#"
            let x = 5;
            let y = x * 2;
            y;
        "#;
        assert_eq!(execute_program(source).unwrap(), "10");
    }

    // Testes de Tipos Mistos
    #[test]
    fn test_mixed_types_in_expressions() {
        let source = r#"
            let num = 42;
            let text = "Number: ";
            text + num;
        "#;
        assert_eq!(execute_program(source).unwrap(), "Number: 42");
    }

    #[test]
    fn test_truthiness_with_variables() {
        let sources_and_expected = [
            ("let x = 5; let y = 0; x && y;", "false"),
            ("let x = true; let y = 1; x && y;", "true"),
            ("let empty = \"\"; let full = \"text\"; empty || full;", "true"),
        ];
        
        for (source, expected) in &sources_and_expected {
            assert_eq!(execute_program(source).unwrap(), *expected);
        }
    }

    // Testes de Comparação com Variáveis
    #[test]
    fn test_variable_comparisons() {
        let source = r#"
            let x = 10;
            let y = 5;
            x > y;
        "#;
        assert_eq!(execute_program(source).unwrap(), "true");
    }

    #[test]
    fn test_variable_equality() {
        let source = r#"
            let a = 42;
            let b = 42;
            a == b;
        "#;
        assert_eq!(execute_program(source).unwrap(), "true");
    }

    // Testes de Erros
    #[test]
    fn test_undefined_variable_error() {
        let result = execute_program("undefined_var;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3001, .. } => {}, // Variável não definida
            _ => panic!("Erro esperado: E3001"),
        }
    }

    #[test]
    fn test_undefined_variable_in_expression() {
        let result = execute_program("let x = 5; x + undefined_var;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3001, .. } => {}, // Variável não definida
            _ => panic!("Erro esperado: E3001"),
        }
    }

    #[test]
    fn test_using_variable_before_declaration() {
        let result = execute_program("y; let y = 10;");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3001, .. } => {}, // Variável não definida
            _ => panic!("Erro esperado: E3001"),
        }
    }

    // Testes de Edge Cases
    #[test]
    fn test_variable_with_null_in_expression() {
        let source = r#"
            let x = null;
            let y = 5;
            x == null;
        "#;
        assert_eq!(execute_program(source).unwrap(), "true");
    }

    #[test]
    fn test_complex_program_with_variables() {
        let source = r#"
            let base = 10;
            let height = 5;
            let area = base * height / 2;
            let message = "Área do triângulo: ";
            message + area;
        "#;
        assert_eq!(execute_program(source).unwrap(), "Área do triângulo: 25");
    }

    #[test]
    fn test_nested_variable_expressions() {
        let source = r#"
            let a = 2;
            let b = 3;
            let c = 4;
            let result = (a + b) * (c - 1);
            result;
        "#;
        assert_eq!(execute_program(source).unwrap(), "15");
    }

    #[test]
    fn test_return_last_expression() {
        let source = r#"
            let x = 10;
            let y = 20;
            x;
            y;
        "#;
        // Deve retornar o valor da última expressão
        assert_eq!(execute_program(source).unwrap(), "20");
    }

    #[test]
    fn test_variable_names_with_underscore() {
        let source = r#"
            let my_var = 42;
            let _private = "secret";
            my_var + 8;
        "#;
        assert_eq!(execute_program(source).unwrap(), "50");
    }
}
