// crates/dryad_parser/tests/advanced_math_parser_tests.rs
use dryad_parser::{Parser, ast::Expr};
use dryad_lexer::{Lexer, Token};

#[cfg(test)]
mod advanced_math_parser_tests {
    use super::*;

    use dryad_lexer::token::TokenWithLocation;
    fn tokenize_code(code: &str) -> Vec<TokenWithLocation> {
        let mut lexer = Lexer::new(code);
        let mut tokens = Vec::new();
        loop {
            let token = lexer.next_token().unwrap();
            if token.token == Token::Eof {
                break;
            }
            tokens.push(token);
        }
        tokens
    }

    #[test]
    fn test_parse_modulo_operator() {
        let code = "a % b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        
        // Usar parse_expression que devemos implementar
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "%");
            }
            _ => panic!("Esperado Binary para operador %"),
        }
    }

    #[test]
    fn test_parse_exponentiation_operator() {
        let code = "a ** b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "**");
            }
            _ => panic!("Esperado Binary para operador **"),
        }
    }

    #[test]
    fn test_parse_nth_root_operator() {
        let code = "a ^^ b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "^^");
            }
            _ => panic!("Esperado Binary para operador ^^"),
        }
    }

    #[test]
    fn test_parse_safe_modulo_operator() {
        let code = "a %% b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "%%");
            }
            _ => panic!("Esperado Binary para operador %%"),
        }
    }

    #[test]
    fn test_parse_power_of_ten_operator() {
        let code = "a ## b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "##");
            }
            _ => panic!("Esperado Binary para operador ##"),
        }
    }

    #[test]
    fn test_parse_single_caret_operator() {
        let code = "a ^ b";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(_, operator, _, _) => {
                assert_eq!(operator, "^");
            }
            _ => panic!("Esperado Binary para operador ^"),
        }
    }

    #[test]
    fn test_exact_syntax_md_example() {
        let code = "10 % 3";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(left, operator, right, _) => {
                assert_eq!(operator, "%");
                if let Expr::Literal(dryad_parser::ast::Literal::Number(left_val), _) = *left {
                    assert_eq!(left_val, 10.0);
                } else {
                    panic!("Lado esquerdo deveria ser Number(10)");
                }
                if let Expr::Literal(dryad_parser::ast::Literal::Number(right_val), _) = *right {
                    assert_eq!(right_val, 3.0);
                } else {
                    panic!("Lado direito deveria ser Number(3)");
                }
            }
            _ => panic!("Esperado Binary para 10 % 3"),
        }
    }
}
