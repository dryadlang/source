// crates/dryad_lexer/tests/assignment_operators_tests.rs
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};
use dryad_errors::DryadError;

#[cfg(test)]
mod assignment_operators_tests {
    use super::*;

    #[test]
    fn test_tokenize_addition_assignment() {
        let mut lexer = Lexer::new("+=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("+=".to_string()));
    }

    #[test]
    fn test_tokenize_subtraction_assignment() {
        let mut lexer = Lexer::new("-=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("-=".to_string()));
    }

    #[test]
    fn test_tokenize_multiplication_assignment() {
        let mut lexer = Lexer::new("*=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("*=".to_string()));
    }

    #[test]
    fn test_tokenize_division_assignment() {
        let mut lexer = Lexer::new("/=");
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("/=".to_string()));
    }

    #[test]
    fn test_assignment_operators_in_code() {
        let source = "x += 5; y -= 3; z *= 2; w /= 4;";
        let mut lexer = Lexer::new(source);
        
        let expected_tokens = vec![
            Token::Identifier("x".to_string()),
            Token::Operator("+=".to_string()),
            Token::Number(5.0),
            Token::Symbol(';'),
            Token::Identifier("y".to_string()),
            Token::Operator("-=".to_string()),
            Token::Number(3.0),
            Token::Symbol(';'),
            Token::Identifier("z".to_string()),
            Token::Operator("*=".to_string()),
            Token::Number(2.0),
            Token::Symbol(';'),
            Token::Identifier("w".to_string()),
            Token::Operator("/=".to_string()),
            Token::Number(4.0),
            Token::Symbol(';'),
            Token::Eof,
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, expected_token);
        }
    }

    #[test]
    fn test_differentiate_assignment_from_operators() {
        // Testa que += é diferente de + seguido de =
        let mut lexer = Lexer::new("x + = y");
        
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Operator("+".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('='));
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("y".to_string()));
    }

    #[test]
    fn test_complete_assignment_program() {
        let source = "let x = 10; x += 5; x -= 2; x *= 3; x /= 6;";
        let mut lexer = Lexer::new(source);
        
        let expected_tokens = vec![
            Token::Keyword("let".to_string()),
            Token::Identifier("x".to_string()),
            Token::Symbol('='),
            Token::Number(10.0),
            Token::Symbol(';'),
            Token::Identifier("x".to_string()),
            Token::Operator("+=".to_string()),
            Token::Number(5.0),
            Token::Symbol(';'),
            Token::Identifier("x".to_string()),
            Token::Operator("-=".to_string()),
            Token::Number(2.0),
            Token::Symbol(';'),
            Token::Identifier("x".to_string()),
            Token::Operator("*=".to_string()),
            Token::Number(3.0),
            Token::Symbol(';'),
            Token::Identifier("x".to_string()),
            Token::Operator("/=".to_string()),
            Token::Number(6.0),
            Token::Symbol(';'),
            Token::Eof,
        ];

        for expected_token in expected_tokens {
            let token = lexer.next_token().unwrap();
            assert_eq!(token.token, expected_token);
        }
    }
}
