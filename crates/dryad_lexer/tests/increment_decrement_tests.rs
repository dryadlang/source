// crates/dryad_lexer/tests/increment_decrement_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[cfg(test)]
mod increment_decrement_tests {
    use super::*;

    #[test]
    fn test_tokenize_increment_operator() {
        let mut lexer = Lexer::new("++");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("++".to_string()));
    }

    #[test]
    fn test_tokenize_decrement_operator() {
        let mut lexer = Lexer::new("--");
        let token = lexer.next_token().unwrap();
        
        assert_eq!(token.token, Token::Operator("--".to_string()));
    }

    #[test]
    fn test_differentiate_increment_from_plus() {
        let mut lexer = Lexer::new("+ ++");
        
        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1.token, Token::Operator("+".to_string()));
        
        let token2 = lexer.next_token().unwrap(); // espaço é ignorado
        assert_eq!(token2.token, Token::Operator("++".to_string()));
    }

    #[test]
    fn test_differentiate_decrement_from_minus() {
        let mut lexer = Lexer::new("- --");
        
        let token1 = lexer.next_token().unwrap();
        assert_eq!(token1.token, Token::Operator("-".to_string()));
        
        let token2 = lexer.next_token().unwrap(); // espaço é ignorado
        assert_eq!(token2.token, Token::Operator("--".to_string()));
    }

    #[test]
    fn test_increment_decrement_in_code() {
        let code = "let x = 5; x++; x--;";
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se os tokens ++ e -- estão presentes
        let increment_found = tokens.iter().any(|t| matches!(&t.token, Token::Operator(op) if op == "++"));
        let decrement_found = tokens.iter().any(|t| matches!(&t.token, Token::Operator(op) if op == "--"));
        
        assert!(increment_found, "Token ++ não foi encontrado");
        assert!(decrement_found, "Token -- não foi encontrado");
    }

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let code = r#"
let contador = 0;
contador++;  // Incrementa 1 (agora contador e 1)
contador--;  // Decrementa 1 (agora contador e 0)
"#;
        
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se todos os tokens esperados estão presentes
        let let_found = tokens.iter().any(|t| matches!(&t.token, Token::Keyword(kw) if kw == "let"));
        let identifier_found = tokens.iter().any(|t| matches!(&t.token, Token::Identifier(id) if id == "contador"));
        let increment_found = tokens.iter().any(|t| matches!(&t.token, Token::Operator(op) if op == "++"));
        let decrement_found = tokens.iter().any(|t| matches!(&t.token, Token::Operator(op) if op == "--"));
        
        assert!(let_found, "Token let não foi encontrado");
        assert!(identifier_found, "Token contador não foi encontrado");
        assert!(increment_found, "Token ++ não foi encontrado");
        assert!(decrement_found, "Token -- não foi encontrado");
    }

    #[test]
    fn test_multiple_increments_decrements() {
        let code = "x++; y--; ++z; --w;";
        let mut lexer = Lexer::new(code);
        let mut operators = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                if op == "++" || op == "--" {
                    operators.push(op);
                }
            }
        }
        
        assert_eq!(operators.len(), 4);
        assert_eq!(operators[0], "++");
        assert_eq!(operators[1], "--");
        assert_eq!(operators[2], "++");
        assert_eq!(operators[3], "--");
    }

    #[test]
    fn test_increment_decrement_with_expressions() {
        let code = "result = x++ + --y;";
        let mut lexer = Lexer::new(code);
        let mut increment_count = 0;
        let mut decrement_count = 0;
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            if let Token::Operator(op) = token.token {
                match op.as_str() {
                    "++" => increment_count += 1,
                    "--" => decrement_count += 1,
                    _ => {}
                }
            }
        }
        
        assert_eq!(increment_count, 1, "Deveria ter 1 operador ++");
        assert_eq!(decrement_count, 1, "Deveria ter 1 operador --");
    }
}
