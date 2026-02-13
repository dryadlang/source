// crates/dryad_runtime/tests/interpreter_tests.rs
use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::Lexer;
use dryad_errors::DryadError;

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    fn eval_expression(source: &str) -> Result<String, DryadError> {
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

    // Testes de Literais
    #[test]
    fn test_eval_number() {
        assert_eq!(eval_expression("42").unwrap(), "42");
        assert_eq!(eval_expression("3.14").unwrap(), "3.14");
        assert_eq!(eval_expression("0").unwrap(), "0");
    }

    #[test]
    fn test_eval_string() {
        assert_eq!(eval_expression("\"Hello\"").unwrap(), "Hello");
        assert_eq!(eval_expression("\"\"").unwrap(), "");
        assert_eq!(eval_expression("\"Hello World\"").unwrap(), "Hello World");
    }

    #[test]
    fn test_eval_boolean() {
        assert_eq!(eval_expression("true").unwrap(), "true");
        assert_eq!(eval_expression("false").unwrap(), "false");
    }

    #[test]
    fn test_eval_null() {
        assert_eq!(eval_expression("null").unwrap(), "null");
    }

    // Testes de Operações Aritméticas
    #[test]
    fn test_eval_addition() {
        assert_eq!(eval_expression("2 + 3").unwrap(), "5");
        assert_eq!(eval_expression("1.5 + 2.5").unwrap(), "4");
        assert_eq!(eval_expression("0 + 0").unwrap(), "0");
    }

    #[test]
    fn test_eval_subtraction() {
        assert_eq!(eval_expression("5 - 2").unwrap(), "3");
        assert_eq!(eval_expression("10 - 7").unwrap(), "3");
        assert_eq!(eval_expression("0 - 5").unwrap(), "-5");
    }

    #[test]
    fn test_eval_multiplication() {
        assert_eq!(eval_expression("3 * 4").unwrap(), "12");
        assert_eq!(eval_expression("2.5 * 2").unwrap(), "5");
        assert_eq!(eval_expression("0 * 100").unwrap(), "0");
    }

    #[test]
    fn test_eval_division() {
        assert_eq!(eval_expression("8 / 2").unwrap(), "4");
        assert_eq!(eval_expression("7 / 2").unwrap(), "3.5");
        assert_eq!(eval_expression("1 / 3").unwrap(), "0.3333333333333333");
    }

    #[test]
    fn test_eval_division_by_zero() {
        let result = eval_expression("5 / 0");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3007, .. } => {}, // E3007 - Divisão por zero
            _ => panic!("Erro esperado: E3007"),
        }
    }

    // Testes de Precedência de Operadores
    #[test]
    fn test_operator_precedence() {
        assert_eq!(eval_expression("2 + 3 * 4").unwrap(), "14");
        assert_eq!(eval_expression("2 * 3 + 4").unwrap(), "10");
        assert_eq!(eval_expression("10 - 4 / 2").unwrap(), "8");
    }

    #[test]
    fn test_parentheses_precedence() {
        assert_eq!(eval_expression("(2 + 3) * 4").unwrap(), "20");
        assert_eq!(eval_expression("2 * (3 + 4)").unwrap(), "14");
        assert_eq!(eval_expression("(10 - 4) / 2").unwrap(), "3");
    }

    // Testes de Concatenação de Strings
    #[test]
    fn test_string_concatenation() {
        assert_eq!(eval_expression("\"Hello\" + \" World\"").unwrap(), "Hello World");
        assert_eq!(eval_expression("\"\" + \"test\"").unwrap(), "test");
        assert_eq!(eval_expression("\"abc\" + \"def\"").unwrap(), "abcdef");
    }

    #[test]
    fn test_mixed_type_concatenation() {
        assert_eq!(eval_expression("\"Number: \" + 42").unwrap(), "Number: 42");
        assert_eq!(eval_expression("123 + \" is a number\"").unwrap(), "123 is a number");
        assert_eq!(eval_expression("\"Result: \" + true").unwrap(), "Result: true");
    }

    // Testes de Comparação
    #[test]
    fn test_equality() {
        assert_eq!(eval_expression("5 == 5").unwrap(), "true");
        assert_eq!(eval_expression("5 == 3").unwrap(), "false");
        assert_eq!(eval_expression("\"hello\" == \"hello\"").unwrap(), "true");
        assert_eq!(eval_expression("\"hello\" == \"world\"").unwrap(), "false");
        assert_eq!(eval_expression("true == true").unwrap(), "true");
        assert_eq!(eval_expression("true == false").unwrap(), "false");
        assert_eq!(eval_expression("null == null").unwrap(), "true");
    }

    #[test]
    fn test_inequality() {
        assert_eq!(eval_expression("5 != 3").unwrap(), "true");
        assert_eq!(eval_expression("5 != 5").unwrap(), "false");
        assert_eq!(eval_expression("\"hello\" != \"world\"").unwrap(), "true");
        assert_eq!(eval_expression("true != false").unwrap(), "true");
    }

    #[test]
    fn test_numeric_comparison() {
        assert_eq!(eval_expression("5 > 3").unwrap(), "true");
        assert_eq!(eval_expression("3 > 5").unwrap(), "false");
        assert_eq!(eval_expression("5 >= 5").unwrap(), "true");
        assert_eq!(eval_expression("3 >= 5").unwrap(), "false");
        assert_eq!(eval_expression("3 < 5").unwrap(), "true");
        assert_eq!(eval_expression("5 < 3").unwrap(), "false");
        assert_eq!(eval_expression("5 <= 5").unwrap(), "true");
        assert_eq!(eval_expression("5 <= 3").unwrap(), "false");
    }

    // Testes de Operadores Lógicos
    #[test]
    fn test_logical_and() {
        assert_eq!(eval_expression("true && true").unwrap(), "true");
        assert_eq!(eval_expression("true && false").unwrap(), "false");
        assert_eq!(eval_expression("false && true").unwrap(), "false");
        assert_eq!(eval_expression("false && false").unwrap(), "false");
    }

    #[test]
    fn test_logical_or() {
        assert_eq!(eval_expression("true || true").unwrap(), "true");
        assert_eq!(eval_expression("true || false").unwrap(), "true");
        assert_eq!(eval_expression("false || true").unwrap(), "true");
        assert_eq!(eval_expression("false || false").unwrap(), "false");
    }

    #[test]
    fn test_logical_not() {
        assert_eq!(eval_expression("!true").unwrap(), "false");
        assert_eq!(eval_expression("!false").unwrap(), "true");
    }

    // Testes de Truthiness
    #[test]
    fn test_truthiness_in_logical_ops() {
        // Números
        assert_eq!(eval_expression("5 && true").unwrap(), "true");
        assert_eq!(eval_expression("0 && true").unwrap(), "false");
        
        // Strings
        assert_eq!(eval_expression("\"hello\" && true").unwrap(), "true");
        assert_eq!(eval_expression("\"\" && true").unwrap(), "false");
        
        // Null
        assert_eq!(eval_expression("null && true").unwrap(), "false");
        assert_eq!(eval_expression("null || true").unwrap(), "true");
    }

    // Testes de Expressões Complexas
    #[test]
    fn test_complex_arithmetic() {
        assert_eq!(eval_expression("(2 + 3) * (4 - 1)").unwrap(), "15");
        assert_eq!(eval_expression("2 * 3 + 4 * 5").unwrap(), "26");
        assert_eq!(eval_expression("(10 + 5) / (3 * 1)").unwrap(), "5");
    }

    #[test]
    fn test_complex_logical() {
        assert_eq!(eval_expression("(5 > 3) && (2 < 4)").unwrap(), "true");
        assert_eq!(eval_expression("(5 < 3) || (2 < 4)").unwrap(), "true");
        assert_eq!(eval_expression("!(5 == 3) && (4 == 4)").unwrap(), "true");
    }

    #[test]
    fn test_mixed_operations() {
        assert_eq!(eval_expression("5 + 3 == 8").unwrap(), "true");
        assert_eq!(eval_expression("(2 * 3) > 5").unwrap(), "true");
        assert_eq!(eval_expression("\"a\" + \"b\" == \"ab\"").unwrap(), "true");
    }

    // Testes de Erros de Tipo
    #[test]
    fn test_invalid_subtraction_types() {
        let result = eval_expression("\"hello\" - \"world\"");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3005, .. } => {}, // Operação '-' só é válida para números
            _ => panic!("Erro esperado de tipo"),
        }
    }

    #[test]
    fn test_invalid_multiplication_types() {
        let result = eval_expression("\"hello\" * 5");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3006, .. } => {}, // Operação '*' só é válida para números
            _ => panic!("Erro esperado de tipo"),
        }
    }

    #[test]
    fn test_invalid_comparison_types() {
        let result = eval_expression("\"hello\" > 5");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Runtime { code: 3009, .. } => {}, // Comparação só é válida para números
            _ => panic!("Erro esperado de tipo"),
        }
    }

    // Testes de Edge Cases
    #[test]
    fn test_large_numbers() {
        assert_eq!(eval_expression("1000000 + 1000000").unwrap(), "2000000");
        assert_eq!(eval_expression("999999 * 999999").unwrap(), "999998000001");
    }

    #[test]
    fn test_floating_point_precision() {
        let result = eval_expression("0.1 + 0.2").unwrap();
        // Floating point arithmetic pode ter imprecisões
        assert!(result.starts_with("0.3"));
    }

    #[test]
    fn test_negative_numbers() {
        // Nota: números negativos são parseados como operador unário '-' + número
        // Por isso testamos expressões que resultam em negativos
        assert_eq!(eval_expression("0 - 5").unwrap(), "-5");
        assert_eq!(eval_expression("3 - 10").unwrap(), "-7");
    }

    #[test]
    fn test_boolean_equality_with_numbers() {
        // Diferentes tipos nunca são iguais
        assert_eq!(eval_expression("true == 1").unwrap(), "false");
        assert_eq!(eval_expression("false == 0").unwrap(), "false");
        assert_eq!(eval_expression("null == 0").unwrap(), "false");
    }
}
