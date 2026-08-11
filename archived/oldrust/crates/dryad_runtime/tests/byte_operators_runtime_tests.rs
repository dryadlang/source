// crates/dryad_runtime/tests/byte_operators_runtime_tests.rs
use dryad_lexer::{Lexer, token::Token};
use dryad_parser::Parser;
use dryad_runtime::Interpreter;

fn evaluate_expression(code: &str) -> f64 {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        if matches!(token.token, Token::Eof) {
            break;
        }
        tokens.push(token);
    }
    
    let mut parser = Parser::new(tokens);
    let expr = parser.expression().unwrap();
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.evaluate(&expr).unwrap();
    
    match result {
        dryad_runtime::Value::Number(n) => n,
        _ => panic!("Expected number result"),
    }
}

#[cfg(test)]
mod byte_operators_runtime_tests {
    use super::*;

    #[test]
    fn test_binary_number_evaluation() {
        let result = evaluate_expression("0b1010");
        assert_eq!(result, 10.0, "0b1010 deveria ser 10");
    }

    #[test]
    fn test_octal_number_evaluation() {
        let result = evaluate_expression("0o12");
        assert_eq!(result, 10.0, "0o12 deveria ser 10");
    }

    #[test]
    fn test_hexadecimal_number_evaluation() {
        let result = evaluate_expression("0xA");
        assert_eq!(result, 10.0, "0xA deveria ser 10");
    }

    #[test]
    fn test_byte_numbers_arithmetic() {
        // Todos resultam em 10
        let result = evaluate_expression("0b1010 + 0o12 + 0xA");
        assert_eq!(result, 30.0, "10 + 10 + 10 deveria ser 30");
    }

    #[test]
    fn test_byte_numbers_mixed_operations() {
        // 0b10 = 2, 0x3 = 3, 0o4 = 4
        let result = evaluate_expression("0b10 + 0x3 * 0o4"); // 2 + 3 * 4 = 2 + 12 = 14
        assert_eq!(result, 14.0, "2 + 3 * 4 deveria ser 14");
    }

    #[test]
    fn test_binary_numbers_various_values() {
        let test_cases = vec![
            ("0b0", 0.0),
            ("0b1", 1.0),
            ("0b10", 2.0),
            ("0b11", 3.0),
            ("0b100", 4.0),
            ("0b101", 5.0),
            ("0b110", 6.0),
            ("0b111", 7.0),
            ("0b1000", 8.0),
            ("0b1111", 15.0),
            ("0b10000", 16.0),
        ];

        for (input, expected) in test_cases {
            let result = evaluate_expression(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_octal_numbers_various_values() {
        let test_cases = vec![
            ("0o0", 0.0),
            ("0o1", 1.0),
            ("0o7", 7.0),
            ("0o10", 8.0),
            ("0o11", 9.0),
            ("0o12", 10.0),
            ("0o17", 15.0),
            ("0o20", 16.0),
            ("0o77", 63.0),
            ("0o100", 64.0),
        ];

        for (input, expected) in test_cases {
            let result = evaluate_expression(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_hexadecimal_numbers_various_values() {
        let test_cases = vec![
            ("0x0", 0.0),
            ("0x1", 1.0),
            ("0x9", 9.0),
            ("0xA", 10.0),
            ("0xB", 11.0),
            ("0xC", 12.0),
            ("0xD", 13.0),
            ("0xE", 14.0),
            ("0xF", 15.0),
            ("0x10", 16.0),
            ("0xFF", 255.0),
            ("0x100", 256.0),
        ];

        for (input, expected) in test_cases {
            let result = evaluate_expression(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_hexadecimal_case_insensitive() {
        let test_cases = vec![
            ("0xa", 10.0),
            ("0xA", 10.0),
            ("0xb", 11.0),
            ("0xB", 11.0),
            ("0xf", 15.0),
            ("0xF", 15.0),
            ("0xff", 255.0),
            ("0xFF", 255.0),
        ];

        for (input, expected) in test_cases {
            let result = evaluate_expression(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_exact_syntax_md_examples() {
        // Testa exatamente os exemplos do SYNTAX.md
        assert_eq!(evaluate_expression("0b1010"), 10.0, "0b1010 deveria ser 10 em binário");
        assert_eq!(evaluate_expression("0o12"), 10.0, "0o12 deveria ser 10 em octal");
        assert_eq!(evaluate_expression("0xA"), 10.0, "0xA deveria ser 10 em hexadecimal");
    }

    #[test]
    fn test_byte_numbers_with_parentheses() {
        let result = evaluate_expression("(0b1010 + 0o12) * 0xA"); // (10 + 10) * 10 = 200
        assert_eq!(result, 200.0, "(10 + 10) * 10 deveria ser 200");
    }

    #[test]
    fn test_byte_numbers_complex_expression() {
        // Mix de diferentes bases numa expressão complexa
        let result = evaluate_expression("0xFF - 0o377 + 0b11111111"); // 255 - 255 + 255 = 255
        assert_eq!(result, 255.0, "255 - 255 + 255 deveria ser 255");
    }

    #[test]
    fn test_byte_numbers_with_advanced_math() {
        // Testa com operadores matemáticos avançados
        let result = evaluate_expression("0b100 ** 0x2"); // 4 ** 2 = 16
        assert_eq!(result, 16.0, "4 ** 2 deveria ser 16");
    }

    #[test]
    fn test_byte_numbers_with_modulo() {
        let result = evaluate_expression("0x10 % 0o12"); // 16 % 10 = 6
        assert_eq!(result, 6.0, "16 % 10 deveria ser 6");
    }

    #[test]
    fn test_large_byte_numbers() {
        // Testa números maiores
        let result = evaluate_expression("0xFFFF"); // 65535
        assert_eq!(result, 65535.0, "0xFFFF deveria ser 65535");
        
        let result = evaluate_expression("0b1111111111111111"); // 65535
        assert_eq!(result, 65535.0, "0b1111111111111111 deveria ser 65535");
        
        let result = evaluate_expression("0o177777"); // 65535
        assert_eq!(result, 65535.0, "0o177777 deveria ser 65535");
    }

    #[test]
    fn test_byte_numbers_precedence() {
        // Testa precedência de operadores com números de byte
        let result = evaluate_expression("0b10 + 0x3 * 0o4 - 0b1"); // 2 + 3 * 4 - 1 = 2 + 12 - 1 = 13
        assert_eq!(result, 13.0, "2 + 3 * 4 - 1 deveria ser 13");
    }

    #[test]
    fn test_byte_numbers_floating_operations() {
        // Testa divisão que resulta em float
        let result = evaluate_expression("0b1010 / 0o4"); // 10 / 4 = 2.5
        assert_eq!(result, 2.5, "10 / 4 deveria ser 2.5");
    }
}
