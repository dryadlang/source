// crates/dryad_runtime/tests/advanced_math_runtime_tests.rs
use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::Lexer;

#[cfg(test)]
mod advanced_math_runtime_tests {
    use super::*;

    fn evaluate_expression(code: &str) -> f64 {
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, dryad_lexer::Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        let mut parser = Parser::new(tokens);
        let expr = parser.expression().unwrap();
        let mut interpreter = Interpreter::new();
        
        match interpreter.evaluate(&expr).unwrap() {
            dryad_runtime::Value::Number(n) => n,
            _ => panic!("Esperado valor numérico"),
        }
    }

    #[test]
    fn test_modulo_operator_basic() {
        let result = evaluate_expression("10 % 3");
        assert_eq!(result, 1.0, "10 % 3 deveria ser 1");
    }

    #[test]
    fn test_modulo_operator_zero_remainder() {
        let result = evaluate_expression("10 % 5");
        assert_eq!(result, 0.0, "10 % 5 deveria ser 0");
    }

    #[test]
    fn test_modulo_operator_negative() {
        let result = evaluate_expression("10 % -3");
        assert_eq!(result, 1.0, "10 % -3 deveria ser 1");
    }

    #[test]
    fn test_exponentiation_operator_basic() {
        let result = evaluate_expression("2 ** 3");
        assert_eq!(result, 8.0, "2 ** 3 deveria ser 8");
    }

    #[test]
    fn test_exponentiation_operator_zero() {
        let result = evaluate_expression("5 ** 0");
        assert_eq!(result, 1.0, "5 ** 0 deveria ser 1");
    }

    #[test]
    fn test_exponentiation_operator_negative() {
        let result = evaluate_expression("2 ** -2");
        assert_eq!(result, 0.25, "2 ** -2 deveria ser 0.25");
    }

    #[test]
    fn test_nth_root_operator_basic() {
        let result = evaluate_expression("8 ^^ 3");
        assert!((result - 2.0).abs() < 0.0001, "8 ^^ 3 deveria ser aproximadamente 2 (raiz cúbica)");
    }

    #[test]
    fn test_nth_root_operator_square() {
        let result = evaluate_expression("9 ^^ 2");
        assert_eq!(result, 3.0, "9 ^^ 2 deveria ser 3 (raiz quadrada)");
    }

    #[test]
    fn test_safe_modulo_operator_positive() {
        let result = evaluate_expression("10 %% 3");
        assert_eq!(result, 1.0, "10 %% 3 deveria ser 1");
    }

    #[test]
    fn test_safe_modulo_operator_negative_dividend() {
        let result = evaluate_expression("-10 %% 3");
        assert_eq!(result, 2.0, "-10 %% 3 deveria ser 2 (sempre positivo)");
    }

    #[test]
    fn test_safe_modulo_operator_negative_divisor() {
        let result = evaluate_expression("10 %% -3");
        assert_eq!(result, 1.0, "10 %% -3 deveria ser 1 (sempre positivo)");
    }

    #[test]
    fn test_power_of_ten_operator_basic() {
        let result = evaluate_expression("3 ## 0");
        assert_eq!(result, 3.0, "3 ## 0 deveria ser 3 * 10^0 = 3");
    }

    #[test]
    fn test_power_of_ten_operator_positive() {
        let result = evaluate_expression("5 ## 2");
        assert_eq!(result, 500.0, "5 ## 2 deveria ser 5 * 10^2 = 500");
    }

    #[test]
    fn test_power_of_ten_operator_negative() {
        let result = evaluate_expression("25 ## -1");
        assert_eq!(result, 2.5, "25 ## -1 deveria ser 25 * 10^-1 = 2.5");
    }

    #[test]
    fn test_single_caret_operator() {
        let result = evaluate_expression("2 ^ 4");
        assert_eq!(result, 6.0, "2 ^ 4 deveria ser 6 (XOR bitwise: 2 XOR 4 = 6)");
    }

    #[test]
    fn test_operator_precedence_exponentiation() {
        // ** deve ter precedência mais alta que *
        let result = evaluate_expression("2 * 3 ** 2");
        assert_eq!(result, 18.0, "2 * 3 ** 2 deveria ser 2 * (3 ** 2) = 2 * 9 = 18");
    }

    #[test]
    fn test_operator_precedence_modulo() {
        // % deve ter mesma precedência que *
        let result = evaluate_expression("10 + 7 % 3");
        assert_eq!(result, 11.0, "10 + 7 % 3 deveria ser 10 + (7 % 3) = 10 + 1 = 11");
    }

    #[test]
    fn test_complex_expression() {
        // Teste complexo com múltiplos operadores
        let result = evaluate_expression("2 ** 3 + 10 % 3 * 4");
        // 2 ** 3 = 8
        // 10 % 3 = 1
        // 1 * 4 = 4
        // 8 + 4 = 12
        assert_eq!(result, 12.0, "Expressão complexa deveria resultar em 12");
    }

    #[test]
    fn test_parentheses_override_precedence() {
        let result = evaluate_expression("(2 + 3) ** 2");
        assert_eq!(result, 25.0, "(2 + 3) ** 2 deveria ser 5 ** 2 = 25");
    }

    #[test]
    fn test_exact_syntax_md_examples() {
        // Exemplos diretos do SYNTAX.md
        assert_eq!(evaluate_expression("10 % 3"), 1.0, "Resto da divisão");
        assert_eq!(evaluate_expression("2 ** 3"), 8.0, "2 elevado a 3");
        assert_eq!(evaluate_expression("10 %% 3"), 1.0, "Sempre positivo");
        assert_eq!(evaluate_expression("10 ## 3"), 10000.0, "10 * 10^3 = 10000");
    }

    #[test]
    fn test_chained_operations() {
        // Teste de operações encadeadas com parênteses para clareza
        let result = evaluate_expression("(16 ^^ 2) ^^ 2");
        // (16 ^^ 2) = 4, depois 4 ^^ 2 = 2
        assert_eq!(result, 2.0, "Operações encadeadas devem funcionar");
    }

    #[test]
    fn test_mixed_with_existing_operators() {
        // Testa interação com operadores existentes
        let result = evaluate_expression("5 + 3 * 2 ** 2 - 1");
        // 2 ** 2 = 4
        // 3 * 4 = 12
        // 5 + 12 = 17
        // 17 - 1 = 16
        assert_eq!(result, 16.0, "Mistura com operadores existentes");
    }
}
