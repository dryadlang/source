// crates/dryad_parser/tests/byte_operators_parser_tests.rs
use dryad_lexer::{Lexer, token::{TokenWithLocation, Token}};
use dryad_parser::{Parser, ast::Expr};

fn tokenize_code(code: &str) -> Vec<TokenWithLocation> {
    let mut lexer = Lexer::new(code);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token { break; }
        tokens.push(tok);
    }
    
    tokens
}

#[cfg(test)]
mod byte_operators_parser_tests {
    use super::*;

    #[test]
    fn test_parse_binary_number() {
        let code = "0b1010";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Literal(dryad_parser::ast::Literal::Number(value), _) => {
                assert_eq!(value, 10.0); // 0b1010 = 10
            }
            _ => panic!("Esperado Number(10.0) para 0b1010"),
        }
    }

    #[test]
    fn test_parse_octal_number() {
        let code = "0o12";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Literal(dryad_parser::ast::Literal::Number(value), _) => {
                assert_eq!(value, 10.0); // 0o12 = 10
            }
            _ => panic!("Esperado Number(10.0) para 0o12"),
        }
    }

    #[test]
    fn test_parse_hexadecimal_number() {
        let code = "0xA";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Literal(dryad_parser::ast::Literal::Number(value), _) => {
                assert_eq!(value, 10.0); // 0xA = 10
            }
            _ => panic!("Esperado Number(10.0) para 0xA"),
        }
    }

    #[test]
    fn test_parse_byte_numbers_in_expression() {
        let code = "0b1010 + 0o12 + 0xA";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        // Verifica se é uma expressão binária de adição
        match ast {
            Expr::Binary(left, op1, right, _) => {
                assert_eq!(op1, "+");
                // Verifica se o lado esquerdo é também uma adição
                match *left {
                        Expr::Binary(left_left, op2, left_right, _) => {
                        assert_eq!(op2, "+");
                        // Verifica os números
                        match (*left_left, *left_right) {
                            (Expr::Literal(dryad_parser::ast::Literal::Number(n1), _), 
                             Expr::Literal(dryad_parser::ast::Literal::Number(n2), _)) => {
                                assert_eq!(n1, 10.0); // 0b1010
                                assert_eq!(n2, 10.0); // 0o12
                            }
                            _ => panic!("Esperado dois números"),
                        }
                    }
                    _ => panic!("Esperado expressão binária no lado esquerdo"),
                }
                // Verifica o lado direito
                    match *right {
                    Expr::Literal(dryad_parser::ast::Literal::Number(n3), _) => {
                        assert_eq!(n3, 10.0); // 0xA
                    }
                    _ => panic!("Esperado número no lado direito"),
                }
            }
            _ => panic!("Esperado expressão binária de adição"),
        }
    }

    #[test]
    fn test_parse_various_byte_formats() {
        let test_cases = vec![
            ("0b0", 0.0),
            ("0b1", 1.0),
            ("0b1111", 15.0),
            ("0o0", 0.0),
            ("0o7", 7.0),
            ("0o17", 15.0),
            ("0x0", 0.0),
            ("0x9", 9.0),
            ("0xF", 15.0),
            ("0xf", 15.0),
            ("0xFF", 255.0),
        ];

        for (code, expected) in test_cases {
            let tokens = tokenize_code(code);
            let mut parser = Parser::new(tokens);
            let ast = parser.expression().unwrap();
            
            match ast {
                Expr::Literal(dryad_parser::ast::Literal::Number(value), _) => {
                    assert_eq!(value, expected, "Failed for input: {}", code);
                }
                _ => panic!("Esperado Number({}) para {}", expected, code),
            }
        }
    }

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa o primeiro número do exemplo do SYNTAX.md
        let code = "0b1010";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Literal(dryad_parser::ast::Literal::Number(value), _) => {
                assert_eq!(value, 10.0, "0b1010 deveria ser 10 em decimal");
            }
            _ => panic!("Esperado literal numérico para 0b1010"),
        }
    }

    #[test]
    fn test_byte_numbers_with_parentheses() {
        let code = "(0b1010) * (0xA)";
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        match ast {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "*");
                // Verifica se ambos os lados são números 10
                          match (*left, *right) {
                          (Expr::Literal(dryad_parser::ast::Literal::Number(n1), _), 
                            Expr::Literal(dryad_parser::ast::Literal::Number(n2), _)) => {
                        assert_eq!(n1, 10.0);
                        assert_eq!(n2, 10.0);
                    }
                    _ => panic!("Esperado dois números"),
                }
            }
            _ => panic!("Esperado multiplicação"),
        }
    }

    #[test]
    fn test_byte_numbers_precedence() {
        let code = "0b10 + 0x3 * 0o4"; // 2 + 3 * 4 = 2 + 12 = 14
        let tokens = tokenize_code(code);
        let mut parser = Parser::new(tokens);
        let ast = parser.expression().unwrap();
        
        // Verifica a estrutura da árvore (precedência de multiplicação)
        match ast {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "+");
                match (*left, *right) {
                    (Expr::Literal(dryad_parser::ast::Literal::Number(n1), _), 
                     Expr::Binary(mult_left, mult_op, mult_right, _)) => {
                        assert_eq!(n1, 2.0); // 0b10
                        assert_eq!(mult_op, "*");
                        match (*mult_left, *mult_right) {
                            (Expr::Literal(dryad_parser::ast::Literal::Number(n2), _), 
                             Expr::Literal(dryad_parser::ast::Literal::Number(n3), _)) => {
                                assert_eq!(n2, 3.0); // 0x3
                                assert_eq!(n3, 4.0); // 0o4
                            }
                            _ => panic!("Esperado dois números na multiplicação"),
                        }
                    }
                    _ => panic!("Esperado número + (multiplicação)"),
                }
            }
            _ => panic!("Esperado adição"),
        }
    }
}
