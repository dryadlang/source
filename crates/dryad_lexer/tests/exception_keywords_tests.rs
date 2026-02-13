#[cfg(test)]
mod exception_lexer_tests {
    use dryad_lexer::{lexer::Lexer, token::Token};

    #[test]
    fn test_try_keyword() {
        let input = "try";
        let mut lexer = Lexer::new(input);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Keyword("try".to_string()));
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Eof);
    }

    #[test]
    fn test_catch_keyword() {
        let input = "catch";
        let mut lexer = Lexer::new(input);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Keyword("catch".to_string()));
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Eof);
    }

    #[test]
    fn test_finally_keyword() {
        let input = "finally";
        let mut lexer = Lexer::new(input);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Keyword("finally".to_string()));
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Eof);
    }

    #[test]
    fn test_throw_keyword() {
        let input = "throw";
        let mut lexer = Lexer::new(input);
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Keyword("throw".to_string()));
        
        let token = lexer.next_token().unwrap();
        assert_eq!(token.token, Token::Eof);
    }

    #[test]
    fn test_exception_keywords_not_identifiers() {
        let keywords = ["try", "catch", "finally", "throw"];
        
        for keyword in &keywords {
            let mut lexer = Lexer::new(keyword);
            let token = lexer.next_token().unwrap();
            
            // Deve ser reconhecido como keyword, não como identifier
            match token.token {
                Token::Keyword(k) => assert_eq!(k, *keyword),
                Token::Identifier(_) => panic!("'{}' foi reconhecido como identifier em vez de keyword", keyword),
                _ => panic!("'{}' resultou em token inesperado: {:?}", keyword, token),
            }
        }
    }

    #[test]
    fn test_exception_keywords_case_sensitive() {
        let mixed_cases = ["Try", "CATCH", "Finally", "THROW"];
        
        for word in &mixed_cases {
            let mut lexer = Lexer::new(word);
            let token = lexer.next_token().unwrap();
            
            // Deve ser reconhecido como identifier, não como keyword
            match token.token {
                Token::Identifier(id) => assert_eq!(id, *word),
                Token::Keyword(_) => panic!("'{}' foi incorretamente reconhecido como keyword", word),
                _ => panic!("'{}' resultou em token inesperado: {:?}", word, token),
            }
        }
    }

    #[test]
    fn test_exception_keywords_with_underscores() {
        let words_with_underscores = ["try_", "_catch", "final_ly", "throw__"];
        
        for word in &words_with_underscores {
            let mut lexer = Lexer::new(word);
            let token = lexer.next_token().unwrap();
            
            // Deve ser reconhecido como identifier
            match token.token {
                Token::Identifier(id) => assert_eq!(id, *word),
                Token::Keyword(_) => panic!("'{}' foi incorretamente reconhecido como keyword", word),
                _ => panic!("'{}' resultou em token inesperado: {:?}", word, token),
            }
        }
    }

    #[test]
    fn test_try_catch_block_structure() {
        let input = "try { } catch (e) { }";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("try".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("catch".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("e".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    #[test]
    fn test_try_catch_finally_structure() {
        let input = "try { } catch (e) { } finally { }";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("try".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("catch".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("e".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("finally".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('{'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('}'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }

    #[test]
    fn test_throw_statement() {
        let input = "throw error;";
        let mut lexer = Lexer::new(input);
        
        assert_eq!(lexer.next_token().unwrap().token, Token::Keyword("throw".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("error".to_string()));
        assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(';'));
        assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
    }
}
