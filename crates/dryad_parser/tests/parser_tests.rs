// crates/dryad_parser/tests/parser_tests.rs
use dryad_parser::{Parser, Expr, Literal, Stmt};
use dryad_lexer::{Lexer, token::Token};
use dryad_errors::DryadError;

#[cfg(test)]
mod parser_tests {
    use super::*;

    fn parse_expression(source: &str) -> Result<Expr, DryadError> {
        let mut lexer = Lexer::new(source);
        let mut tokens = Vec::new();
        
        loop {
            let t = lexer.next_token()?;
            if let Token::Eof = t.token { tokens.push(t); break; }
            tokens.push(t);
        }
        
        let mut parser = Parser::new(tokens);
        let program = parser.parse()?;
        
        // Para testes de expressão, esperamos um statement de expressão
        if program.statements.is_empty() {
            return Err(DryadError::new(2001, "Nenhum statement encontrado"));
        }
        
        match &program.statements[0] {
            Stmt::Expression(expr, _) => Ok(expr.clone()),
            _ => Err(DryadError::new(2001, "Esperado statement de expressão")),
        }
    }

    // Testes de Literais
    #[test]
    fn test_parse_number() {
        let expr = parse_expression("42").unwrap();
        match expr {
            Expr::Literal(Literal::Number(n), _) => assert_eq!(n, 42.0),
            _ => panic!("Esperado número literal"),
        }
    }

    #[test]
    fn test_parse_string() {
        let expr = parse_expression("\"Hello\"").unwrap();
        match expr {
            Expr::Literal(Literal::String(s), _) => assert_eq!(s, "Hello"),
            _ => panic!("Esperado string literal"),
        }
    }

    #[test]
    fn test_parse_boolean_true() {
        let expr = parse_expression("true").unwrap();
        match expr {
            Expr::Literal(Literal::Bool(b), _) => assert_eq!(b, true),
            _ => panic!("Esperado boolean literal"),
        }
    }

    #[test]
    fn test_parse_boolean_false() {
        let expr = parse_expression("false").unwrap();
        match expr {
            Expr::Literal(Literal::Bool(b), _) => assert_eq!(b, false),
            _ => panic!("Esperado boolean literal"),
        }
    }

    #[test]
    fn test_parse_null() {
        let expr = parse_expression("null").unwrap();
        match expr {
            Expr::Literal(Literal::Null, _) => {},
            _ => panic!("Esperado null literal"),
        }
    }

    // Testes de Variáveis
    #[test]
    fn test_parse_variable() {
        let expr = parse_expression("variable_name").unwrap();
        match expr {
            Expr::Variable(name, _) => assert_eq!(name, "variable_name"),
            _ => panic!("Esperado variável"),
        }
    }

    // Testes de Operações Aritméticas
    #[test]
    fn test_parse_addition() {
        let expr = parse_expression("2 + 3").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "+");
                match (*left, *right) {
                    (Expr::Literal(Literal::Number(2.0), _), Expr::Literal(Literal::Number(3.0), _)) => {},
                    _ => panic!("Operandos incorretos"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    #[test]
    fn test_parse_subtraction() {
        let expr = parse_expression("5 - 2").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "-");
                match (*left, *right) {
                    (Expr::Literal(Literal::Number(5.0), _), Expr::Literal(Literal::Number(2.0), _)) => {},
                    _ => panic!("Operandos incorretos"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    #[test]
    fn test_parse_multiplication() {
        let expr = parse_expression("3 * 4").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "*");
                match (*left, *right) {
                    (Expr::Literal(Literal::Number(3.0), _), Expr::Literal(Literal::Number(4.0), _)) => {},
                    _ => panic!("Operandos incorretos"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    #[test]
    fn test_parse_division() {
        let expr = parse_expression("8 / 2").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "/");
                match (*left, *right) {
                    (Expr::Literal(Literal::Number(8.0), _), Expr::Literal(Literal::Number(2.0), _)) => {},
                    _ => panic!("Operandos incorretos"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    // Testes de Precedência
    #[test]
    fn test_operator_precedence() {
        let expr = parse_expression("2 + 3 * 4").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "+");
                // Deve ser (2 + (3 * 4))
                    match *left {
                        Expr::Literal(Literal::Number(2.0), _) => {},
                    _ => panic!("Lado esquerdo incorreto"),
                }
                    match *right {
                        Expr::Binary(_, mult_op, _, _) => assert_eq!(mult_op, "*"),
                    _ => panic!("Lado direito deve ser multiplicação"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    // Testes de Comparação
    #[test]
    fn test_parse_equality() {
        let expr = parse_expression("5 == 5").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "=="),
            _ => panic!("Esperado comparação de igualdade"),
        }
    }

    #[test]
    fn test_parse_inequality() {
        let expr = parse_expression("3 != 4").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "!="),
            _ => panic!("Esperado comparação de desigualdade"),
        }
    }

    #[test]
    fn test_parse_less_than() {
        let expr = parse_expression("2 < 5").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "<"),
            _ => panic!("Esperado comparação menor que"),
        }
    }

    #[test]
    fn test_parse_greater_than() {
        let expr = parse_expression("7 > 3").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, ">"),
            _ => panic!("Esperado comparação maior que"),
        }
    }

    // Testes de Operadores Lógicos
    #[test]
    fn test_parse_logical_and() {
        let expr = parse_expression("true && false").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "&&"),
            _ => panic!("Esperado operador lógico AND"),
        }
    }

    #[test]
    fn test_parse_logical_or() {
        let expr = parse_expression("true || false").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "||"),
            _ => panic!("Esperado operador lógico OR"),
        }
    }

    // Testes de Parênteses
    #[test]
    fn test_parse_parentheses() {
        let expr = parse_expression("(2 + 3) * 4").unwrap();
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "*");
                // Verifica se os parênteses alteraram a precedência
                    match *left {
                        Expr::Binary(_, add_op, _, _) => assert_eq!(add_op, "+"),
                    _ => panic!("Lado esquerdo deve ser adição"),
                }
                    match *right {
                        Expr::Literal(Literal::Number(4.0), _) => {},
                    _ => panic!("Lado direito incorreto"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    // Testes de Erros
    #[test]
    fn test_unexpected_token_error() {
        let result = parse_expression("2 +");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Parser { code: 2001, .. } => {}, // E2001 - Unexpected Token
            _ => panic!("Erro esperado: E2001"),
        }
    }

    #[test]
    fn test_missing_closing_parenthesis() {
        let result = parse_expression("(2 + 3");
        assert!(result.is_err());
        match result.unwrap_err() {
            DryadError::Parser { code: 2005, .. } => {}, // E2005 - Missing Closing Parenthesis
            _ => panic!("Erro esperado: E2005"),
        }
    }

    // Testes de Expressões Complexas
    #[test]
    fn test_complex_expression() {
        let expr = parse_expression("(2 + 3) * 4 == 20 && true").unwrap();
        // Deve ser parseado como: ((((2 + 3) * 4) == 20) && true)
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "&&"),
            _ => panic!("Operador principal deve ser &&"),
        }
    }

    #[test]
    fn test_string_concatenation_precedence() {
        let expr = parse_expression("\"Hello\" + \" \" + \"World\"").unwrap();
        // Deve ser parseado da esquerda para a direita: (("Hello" + " ") + "World")
        match expr {
            Expr::Binary(left, op, right, _) => {
                assert_eq!(op, "+");
                match *left {
                    Expr::Binary(_, inner_op, _, _) => assert_eq!(inner_op, "+"),
                    _ => panic!("Lado esquerdo deve ser concatenação"),
                }
                match *right {
                    Expr::Literal(Literal::String(s), _) => assert_eq!(s, "World"),
                    _ => panic!("Lado direito incorreto"),
                }
            },
            _ => panic!("Esperado expressão binária"),
        }
    }

    // Testes de Edge Cases
    #[test]
    fn test_empty_input() {
        let result = parse_expression("");
        assert!(result.is_err());
    }

    #[test]
    fn test_only_operator() {
        let result = parse_expression("+");
        assert!(result.is_err());
    }

    #[test]
    fn test_nested_parentheses() {
        let expr = parse_expression("((2 + 3))").unwrap();
        match expr {
            Expr::Binary(_, op, _, _) => assert_eq!(op, "+"),
            _ => panic!("Esperado expressão de adição aninhada"),
        }
    }
}
