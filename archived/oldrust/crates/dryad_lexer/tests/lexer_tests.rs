// crates/dryad_lexer/tests/lexer_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};
use dryad_errors::DryadError;

#[cfg(test)]
mod lexer_tests {
    use super::*;

    // Testes de Números
    #[test]
    fn test_tokenize_integer() {
        let mut lexer = Lexer::new("42");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(42.0));
    }

    #[test]
    fn test_tokenize_float() {
        let mut lexer = Lexer::new("3.14");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(3.14));
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut lexer = Lexer::new("-5");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("-".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(5.0));
    }

    // Testes de Strings
    #[test]
    fn test_tokenize_string() {
        let mut lexer = Lexer::new("\"Hello World\"");
        assert_eq!(lexer.next_token().unwrap().token, Token::String("Hello World".to_string()));
    }

    #[test]
    fn test_tokenize_string_with_escapes() {
        let mut lexer = Lexer::new("\"Hello\\nWorld\"");
        assert_eq!(lexer.next_token().unwrap().token, Token::String("Hello\nWorld".to_string()));
    }

    #[test]
    fn test_unterminated_string_error() {
        let mut lexer = Lexer::new("\"Hello World");
        let result = lexer.next_token();
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Lexer { code: 1002, .. } => {}, // E1002 - Unterminated String
            _ => panic!("Erro esperado: E1002"),
        }
    }

    // Testes de Identificadores e Palavras-chave
    #[test]
    fn test_tokenize_identifier() {
        let mut lexer = Lexer::new("variable_name");
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("variable_name".to_string()));
    }

    #[test]
    fn test_tokenize_keywords() {
        let keywords = ["let", "if", "else", "function", "class", "return"];
        for keyword in &keywords {
            let mut lexer = Lexer::new(keyword);
            assert_eq!(lexer.next_token().unwrap().token, Token::Keyword(keyword.to_string()));
        }
    }

    #[test]
    fn test_tokenize_booleans() {
        let mut lexer = Lexer::new("true false");
        assert_eq!(lexer.next_token().unwrap().token, Token::Boolean(true));
        assert_eq!(lexer.next_token().unwrap().token, Token::Boolean(false));
    }

    #[test]
    fn test_tokenize_null() {
        let mut lexer = Lexer::new("null");
        assert_eq!(lexer.next_token().unwrap().token, Token::Literal("null".to_string()));
    }

    // Testes de Operadores
    #[test]
    fn test_tokenize_arithmetic_operators() {
        let mut lexer = Lexer::new("+ - * /");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("+".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("-".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("*".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("/".to_string()));
    }

    #[test]
    fn test_tokenize_comparison_operators() {
        let mut lexer = Lexer::new("== != < > <= >=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("==".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("!=".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<=".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">=".to_string()));
    }

    #[test]
    fn test_tokenize_logical_operators() {
        let mut lexer = Lexer::new("&& || !");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("&&".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("||".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("!".to_string()));
    }

    // Testes de Símbolos
    #[test]
    fn test_tokenize_symbols() {
        let mut lexer = Lexer::new("(){};,=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(';'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(','));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('='));
    }

    // Testes de Comentários
    #[test]
    fn test_skip_line_comment() {
        let mut lexer = Lexer::new("42 // comentário\n24");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(42.0));
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(24.0));
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    #[test]
    fn test_skip_block_comment() {
        let mut lexer = Lexer::new("42 /* comentário */ 24");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(42.0));
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(24.0));
    }

    #[test]
    fn test_unterminated_block_comment_error() {
        let mut lexer = Lexer::new("42 /* comentário não fechado");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(42.0));
        let result = lexer.next_token();
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Lexer { code: 1003, .. } => {}, // E1003 - Unterminated Comment
            _ => panic!("Erro esperado: E1003"),
        }
    }

    // Testes de Tratamento de Espaços
    #[test]
    fn test_skip_whitespace() {
        let mut lexer = Lexer::new("  \t\n  42   \r\n  ");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(42.0));
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    // Testes de Erros
    #[test]
    fn test_unexpected_character_error() {
        let mut lexer = Lexer::new("@");
        let result = lexer.next_token();
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Lexer { code: 1001, .. } => {}, // E1001 - Unexpected Character
            _ => panic!("Erro esperado: E1001"),
        }
    }

    #[test]
    fn test_invalid_escape_sequence_error() {
        let mut lexer = Lexer::new("\"\\k\"");
        let result = lexer.next_token();
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Lexer { code: 1005, .. } => {}, // E1005 - Invalid Escape Sequence
            _ => panic!("Erro esperado: E1005"),
        }
    }

    // Testes de Programa Completo
    #[test]
    fn test_complete_program() {
        let source = r#"
            let x = 42;
            if x > 10 {
                print("Grande");
            }
        "#;
        
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        loop {
            let token = lexer.next_token().unwrap();
            if matches!(token.token, Token::Eof) {
                break;
            }
            tokens.push(token);
        }
        
        // Verifica se todos os tokens esperados estão presentes
        assert!(tokens.len() > 10); // Deve ter pelo menos alguns tokens
        assert!(tokens.iter().any(|t| matches!(&t.token, Token::Keyword(k) if k == "let")));
        assert!(tokens.iter().any(|t| matches!(&t.token, Token::Keyword(k) if k == "if")));
        assert!(tokens.iter().any(|t| matches!(t.token, Token::Number(42.0))));
    }

    // Testes de Performance e Edge Cases
    #[test]
    fn test_empty_input() {
        let mut lexer = Lexer::new("");
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    #[test]
    fn test_only_whitespace() {
        let mut lexer = Lexer::new("   \t\n\r   ");
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    #[test]
    fn test_large_number() {
        let mut lexer = Lexer::new("123456789.987654321");
        assert_eq!(lexer.next_token().unwrap().token, Token::Number(123456789.987654321));
    }
}
