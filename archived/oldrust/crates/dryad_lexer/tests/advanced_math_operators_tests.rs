// crates/dryad_lexer/tests/advanced_math_operators_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[cfg(test)]
mod advanced_math_operators_tests {
    use super::*;

    #[test]
    fn test_tokenize_modulo_operator() {
        let mut lexer = Lexer::new("%");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("%".to_string()));
    }

    #[test]
    fn test_tokenize_exponentiation_operator() {
        let mut lexer = Lexer::new("**");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("**".to_string()));
    }

    #[test]
    fn test_tokenize_nth_root_operator() {
        let mut lexer = Lexer::new("^^");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("^^".to_string()));
    }

    #[test]
    fn test_tokenize_safe_modulo_operator() {
        let mut lexer = Lexer::new("%%");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("%%".to_string()));
    }

    #[test]
    fn test_tokenize_power_of_ten_operator() {
        let mut lexer = Lexer::new("##");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("##".to_string()));
    }

    #[test]
    fn test_differentiate_single_vs_double_operators() {
        // Testa * vs **
        let mut lexer = Lexer::new("* **");
        
        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1.token, Token::Operator("*".to_string()));
        
        let token2 = lexer.next_token().unwrap(); // espaço é ignorado
        assert_eq!(token2.token, Token::Operator("**".to_string()));
    }

    #[test]
    fn test_differentiate_modulo_vs_safe_modulo() {
        // Testa % vs %%
        let mut lexer = Lexer::new("% %%");
        
        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1.token, Token::Operator("%".to_string()));
        
        let token2 = lexer.next_token().unwrap(); // espaço é ignorado
        assert_eq!(token2.token, Token::Operator("%%".to_string()));
    }

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let code = r#"
let modulo = 10 % 3; // Resto da divisão (1)
let exponenciacao = 2 ** 3; // 2 elevado a 3 (8)
let raizEnésima = 27 ^^ (1/3); // Raiz cúbica de 27 (3)
let moduloSeguro = 10 %% 3; // Sempre positivo (1)
let potenciaBase10 = 10 ## 3; // 1000
"#;
        
        let mut lexer = Lexer::new(code);
        let mut operators = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if matches!(op.as_str(), "%" | "**" | "^^" | "%%" | "##") {
                    operators.push(op);
                }
            }
        }
        
        // Verifica se todos os operadores foram encontrados
        assert!(operators.contains(&"%".to_string()), "Operador % não foi encontrado");
        assert!(operators.contains(&"**".to_string()), "Operador ** não foi encontrado");
        assert!(operators.contains(&"^^".to_string()), "Operador ^^ não foi encontrado");
        assert!(operators.contains(&"%%".to_string()), "Operador %% não foi encontrado");
        assert!(operators.contains(&"##".to_string()), "Operador ## não foi encontrado");
    }

    #[test]
    fn test_advanced_operators_in_expressions() {
        let code = "let x = 5 % 3 ** 2 ^^ 0.5 %% 7 ## 2;";
        let mut lexer = Lexer::new(code);
        let mut operator_count = 0;
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if matches!(op.as_str(), "%" | "**" | "^^" | "%%" | "##") {
                    operator_count += 1;
                }
            }
        }
        
        assert_eq!(operator_count, 5, "Deveria ter encontrado 5 operadores avançados");
    }

    #[test]
    fn test_operators_with_parentheses() {
        let code = "(10 % 3) ** (2 ^^ 1)";
        let mut lexer = Lexer::new(code);
        let mut found_operators = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if matches!(op.as_str(), "%" | "**" | "^^") {
                    found_operators.push(op);
                }
            }
        }
        
        assert_eq!(found_operators.len(), 3);
        assert_eq!(found_operators[0], "%");
        assert_eq!(found_operators[1], "**");
        assert_eq!(found_operators[2], "^^");
    }

    #[test]
    fn test_complex_mathematical_expression() {
        let code = "result = sqrt(a ** 2 + b ** 2) % prime ^^ root %% mod ## power;";
        let mut lexer = Lexer::new(code);
        let mut advanced_ops = 0;
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                match op.as_str() {
                    "**" | "^^" | "%%" | "##" | "%" => advanced_ops += 1,
                    _ => {}
                }
            }
        }
        
        assert_eq!(advanced_ops, 6, "Deveria ter encontrado 6 operadores avançados");
    }

    #[test]
    fn test_comments_are_properly_skipped() {
        // Verifica se comentários são corretamente ignorados
        let code = "let x = 10; // isto é um comentário\nlet y = 20;";
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se apenas os tokens válidos foram capturados
        assert!(tokens.len() >= 6); // let, x, =, 10, ;, let, y, =, 20, ;
        
        // Verifica se não há nenhum operador de divisão dupla
        for token in &tokens {
            if let Token::Operator(op) = &token.token {
                assert_ne!(op, "//", "Não deveria encontrar operador de divisão dupla");
            }
        }
    }

    #[test]
    fn test_hash_operators_vs_comments() {
        // O operador ## não deve ser confundido com possíveis comentários futuros
        let code = "power = 10 ## 3;";
        let mut lexer = Lexer::new(code);
        let mut found_hash_power = false;
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if op == "##" {
                    found_hash_power = true;
                    break;
                }
            }
        }
        
        assert!(found_hash_power, "Operador ## deveria ter sido encontrado");
    }

    #[test]
    fn test_caret_operators() {
        // Testa ^ vs ^^
        let code = "a = b ^ c ^^ d;";
        let mut lexer = Lexer::new(code);
        let mut operators = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if matches!(op.as_str(), "^" | "^^") {
                    operators.push(op);
                }
            }
        }
        
        assert_eq!(operators.len(), 2);
        assert_eq!(operators[0], "^");
        assert_eq!(operators[1], "^^");
    }
}
