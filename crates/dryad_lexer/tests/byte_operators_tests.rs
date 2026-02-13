// crates/dryad_lexer/tests/byte_operators_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[cfg(test)]
mod byte_operators_tests {
    use super::*;

    #[test]
    fn test_tokenize_binary_number() {
        let mut lexer = Lexer::new("0b1010");
        let token = lexer.next_token().unwrap();
        
        // 0b1010 = 10 em decimal
        assert_eq!(token.token, Token::Number(10.0));
    }

    #[test]
    fn test_tokenize_octal_number() {
        let mut lexer = Lexer::new("0o12");
        let token = lexer.next_token().unwrap();
        
        // 0o12 = 10 em decimal
        assert_eq!(token.token, Token::Number(10.0));
    }

    #[test]
    fn test_tokenize_hexadecimal_number() {
        let mut lexer = Lexer::new("0xA");
        let token = lexer.next_token().unwrap();
        
        // 0xA = 10 em decimal
        assert_eq!(token.token, Token::Number(10.0));
    }

    #[test]
    fn test_binary_numbers_various_values() {
        // Testa diferentes valores binários
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
            let mut lexer = Lexer::new(input);
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, Token::Number(expected), "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_octal_numbers_various_values() {
        // Testa diferentes valores octais
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
            let mut lexer = Lexer::new(input);
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, Token::Number(expected), "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_hexadecimal_numbers_various_values() {
        // Testa diferentes valores hexadecimais
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
            let mut lexer = Lexer::new(input);
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, Token::Number(expected), "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_hexadecimal_case_insensitive() {
        // Testa que hex funciona com letras maiúsculas e minúsculas
        let test_cases = vec![
            ("0xa", 10.0),
            ("0xA", 10.0),
            ("0xb", 11.0),
            ("0xB", 11.0),
            ("0xc", 12.0),
            ("0xC", 12.0),
            ("0xd", 13.0),
            ("0xD", 13.0),
            ("0xe", 14.0),
            ("0xE", 14.0),
            ("0xf", 15.0),
            ("0xF", 15.0),
            ("0xff", 255.0),
            ("0xFF", 255.0),
            ("0xFf", 255.0),
            ("0xfF", 255.0),
        ];

        for (input, expected) in test_cases {
            let mut lexer = Lexer::new(input);
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, Token::Number(expected), "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_byte_operators_in_expressions() {
        let code = "let a = 0b1010; let b = 0o12; let c = 0xA;";
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se encontramos pelo menos os 3 números (10, 10, 10)
        let numbers: Vec<f64> = tokens.iter()
            .filter_map(|t| if let Token::Number(n) = &t.token { Some(*n) } else { None })
            .collect();
        
        assert!(numbers.contains(&10.0));
        assert_eq!(numbers.iter().filter(|&&n| n == 10.0).count(), 3);
    }

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let code = r#"
let byte1 = 0b1010; // 10 em binário
let byte2 = 0o12;   // 10 em octal
let byte3 = 0xA;    // 10 em hexadecimal
"#;
        
        let mut lexer = Lexer::new(code);
        let mut byte_numbers = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Number(n) = token.token {
                // Só adiciona se for 10.0 (nossos números de byte)
                if n == 10.0 {
                    byte_numbers.push(n);
                }
            }
        }
        
        // Devemos ter encontrado exatamente 3 números com valor 10
        assert_eq!(byte_numbers.len(), 3, "Deveria ter encontrado 3 números de byte com valor 10");
        for num in byte_numbers {
            assert_eq!(num, 10.0, "Todos os números de byte deveriam ser 10");
        }
    }

    #[test]
    fn test_invalid_binary_digits() {
        // Testa que dígitos inválidos em binário geram erro
        let invalid_cases = vec!["0b2", "0b8", "0b19", "0bA"];
        
        for input in invalid_cases {
            let mut lexer = Lexer::new(input);
            let result = lexer.next_token();
            assert!(result.is_err(), "Input '{}' deveria gerar erro", input);
        }
    }

    #[test]
    fn test_invalid_octal_digits() {
        // Testa que dígitos inválidos em octal geram erro
        let invalid_cases = vec!["0o8", "0o9", "0oA"];
        
        for input in invalid_cases {
            let mut lexer = Lexer::new(input);
            let result = lexer.next_token();
            assert!(result.is_err(), "Input '{}' deveria gerar erro", input);
        }
    }

    #[test]
    fn test_invalid_hexadecimal_digits() {
        // Testa que dígitos inválidos em hexadecimal geram erro
        let invalid_cases = vec!["0xG", "0xZ", "0x1G"];
        
        for input in invalid_cases {
            let mut lexer = Lexer::new(input);
            let result = lexer.next_token();
            assert!(result.is_err(), "Input '{}' deveria gerar erro", input);
        }
    }

    #[test]
    fn test_empty_number_literals() {
        // Testa que literais vazios geram erro
        let invalid_cases = vec!["0b", "0o", "0x"];
        
        for input in invalid_cases {
            let mut lexer = Lexer::new(input);
            let result = lexer.next_token();
            assert!(result.is_err(), "Input '{}' deveria gerar erro", input);
        }
    }

    #[test]
    fn test_byte_numbers_with_operations() {
        // Testa operações com números de byte
        let code = "0b1010 + 0o12 - 0xA";
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se os tokens estão corretos
        assert!(matches!(tokens[0].token, Token::Number(10.0))); // 0b1010
        assert!(matches!(tokens[1].token, Token::Operator(ref op) if op == "+")); 
        assert!(matches!(tokens[2].token, Token::Number(10.0))); // 0o12
        assert!(matches!(tokens[3].token, Token::Operator(ref op) if op == "-"));
        assert!(matches!(tokens[4].token, Token::Number(10.0))); // 0xA
    }
}
