// crates/dryad_parser/tests/increment_decrement_parser_tests.rs
use dryad_parser::{Parser, Expr, Stmt};
use dryad_lexer::{Lexer, token::Token};

fn parse_program(source: &str) -> Result<Vec<Stmt>, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token()?;
        if let Token::Eof = tok.token { tokens.push(tok); break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    Ok(program.statements)
}

#[cfg(test)]
mod increment_decrement_parser_tests {
    use super::*;

    #[test]
    fn test_post_increment_statement() {
        let result = parse_program("x++;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Expression(Expr::PostIncrement(var, _), _) => {
                match var.as_ref() {
                    Expr::Variable(name, _) => assert_eq!(name, "x"),
                    _ => panic!("Expected variable in post-increment"),
                }
            },
            _ => panic!("Expected post-increment statement, got: {:?}", result[0]),
        }
    }

    #[test]
    fn test_post_decrement_statement() {
        let result = parse_program("y--;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Expression(Expr::PostDecrement(var, _), _) => {
                match var.as_ref() {
                    Expr::Variable(name, _) => assert_eq!(name, "y"),
                    _ => panic!("Expected variable in post-decrement"),
                }
            },
            _ => panic!("Expected post-decrement statement, got: {:?}", result[0]),
        }
    }

    #[test]
    fn test_pre_increment_statement() {
        let result = parse_program("++z;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Expression(Expr::PreIncrement(var, _), _) => {
                match var.as_ref() {
                    Expr::Variable(name, _) => assert_eq!(name, "z"),
                    _ => panic!("Expected variable in pre-increment"),
                }
            },
            _ => panic!("Expected pre-increment statement, got: {:?}", result[0]),
        }
    }

    #[test]
    fn test_pre_decrement_statement() {
        let result = parse_program("--w;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Expression(Expr::PreDecrement(var, _), _) => {
                match var.as_ref() {
                    Expr::Variable(name, _) => assert_eq!(name, "w"),
                    _ => panic!("Expected variable in pre-decrement"),
                }
            },
            _ => panic!("Expected pre-decrement statement, got: {:?}", result[0]),
        }
    }

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let result = parse_program("
            let contador = 0;
            contador++;
            contador--;
        ").unwrap();
        
        assert_eq!(result.len(), 3);
        
        // Primeira declaração
        match &result[0] {
            Stmt::VarDeclaration(name, value, _) => {
                assert_eq!(name, "contador");
                assert!(value.is_some());
            },
            _ => panic!("Expected variable declaration"),
        }
        
        // Segunda - incremento
        match &result[1] {
            Stmt::Expression(Expr::PostIncrement(_, _), _) => {},
            _ => panic!("Expected post-increment statement"),
        }
        
        // Terceira - decremento
        match &result[2] {
            Stmt::Expression(Expr::PostDecrement(_, _), _) => {},
            _ => panic!("Expected post-decrement statement"),
        }
    }

    #[test]
    fn test_increment_decrement_in_expressions() {
        let result = parse_program("result = x++ + --y;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Assignment(variable, value, _) => {
                assert_eq!(variable, "result");
                // A expressão deve ser uma adição entre post-increment e pre-decrement
                match value {
                    Expr::Binary(left, operator, right, _) => {
                        assert_eq!(operator, "+");
                        match left.as_ref() {
                            Expr::PostIncrement(_, _) => {},
                            _ => panic!("Expected post-increment on left side"),
                        }
                        match right.as_ref() {
                            Expr::PreDecrement(_, _) => {},
                            _ => panic!("Expected pre-decrement on right side"),
                        }
                    },
                    _ => panic!("Expected binary expression"),
                }
            },
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_multiple_increment_decrement() {
        let result = parse_program("
            a++;
            ++b;
            c--;
            --d;
        ").unwrap();
        
        assert_eq!(result.len(), 4);
        
        // a++
        match &result[0] {
            Stmt::Expression(Expr::PostIncrement(var, _), _) => {
                if let Expr::Variable(name, _) = var.as_ref() {
                    assert_eq!(name, "a");
                }
            },
            _ => panic!("Expected post-increment for a"),
        }
        
        // ++b
        match &result[1] {
            Stmt::Expression(Expr::PreIncrement(var, _), _) => {
                if let Expr::Variable(name, _) = var.as_ref() {
                    assert_eq!(name, "b");
                }
            },
            _ => panic!("Expected pre-increment for b"),
        }
        
        // c--
        match &result[2] {
            Stmt::Expression(Expr::PostDecrement(var, _), _) => {
                if let Expr::Variable(name, _) = var.as_ref() {
                    assert_eq!(name, "c");
                }
            },
            _ => panic!("Expected post-decrement for c"),
        }
        
        // --d
        match &result[3] {
            Stmt::Expression(Expr::PreDecrement(var, _), _) => {
                if let Expr::Variable(name, _) = var.as_ref() {
                    assert_eq!(name, "d");
                }
            },
            _ => panic!("Expected pre-decrement for d"),
        }
    }

    #[test]
    fn test_increment_decrement_with_parentheses() {
        let result = parse_program("result = (x++) + (--y);").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Assignment(variable, value, _) => {
                assert_eq!(variable, "result");
                match value {
                    Expr::Binary(left, operator, right, _) => {
                        assert_eq!(operator, "+");
                        // Parênteses devem ser transparentes
                        match left.as_ref() {
                            Expr::PostIncrement(_, _) => {},
                            _ => panic!("Expected post-increment inside parentheses"),
                        }
                        match right.as_ref() {
                            Expr::PreDecrement(_, _) => {},
                            _ => panic!("Expected pre-decrement inside parentheses"),
                        }
                    },
                    _ => panic!("Expected binary expression"),
                }
            },
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_chained_increment_decrement() {
        let result = parse_program("
            x++;
            x++;
            x--;
            x--;
        ").unwrap();
        
        assert_eq!(result.len(), 4);
        
        for (i, stmt) in result.iter().enumerate() {
            match stmt {
                Stmt::Expression(expr, _) => {
                    match expr {
                        Expr::PostIncrement(_, _) | Expr::PostDecrement(_, _) => {},
                        _ => panic!("Expected increment/decrement at position {}", i),
                    }
                },
                _ => panic!("Expected expression statement at position {}", i),
            }
        }
    }

    #[test]
    fn test_increment_decrement_precedence() {
        // Testa precedência: x++ deve ter alta precedência
        let result = parse_program("result = x++ * 2;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Assignment(variable, value, _) => {
                assert_eq!(variable, "result");
                // Deve ser (x++) * 2, não x++ * 2 agrupado diferente
                match value {
                    Expr::Binary(left, operator, right, _) => {
                        assert_eq!(operator, "*");
                        match left.as_ref() {
                            Expr::PostIncrement(_, _) => {},
                            _ => panic!("Expected post-increment in multiplication"),
                        }
                        match right.as_ref() {
                            Expr::Literal(dryad_parser::Literal::Number(n), _) => assert_eq!(*n, 2.0),
                            _ => panic!("Expected number 2"),
                        }
                    },
                    _ => panic!("Expected binary expression"),
                }
            },
            _ => panic!("Expected assignment statement"),
        }
    }

    #[test]
    fn test_increment_decrement_complex_expression() {
        let result = parse_program("total = ++start + end-- - middle;").unwrap();
        
        assert_eq!(result.len(), 1);
        match &result[0] {
            Stmt::Assignment(variable, value, _) => {
                assert_eq!(variable, "total");
                // Deve formar: ((++start + end--) - middle)
                match value {
                    Expr::Binary(left, op1, right, _) => {
                        assert_eq!(op1, "-");
                        match left.as_ref() {
                            Expr::Binary(inner_left, op2, inner_right, _) => {
                                assert_eq!(op2, "+");
                                match inner_left.as_ref() {
                                    Expr::PreIncrement(_, _) => {},
                                    _ => panic!("Expected pre-increment"),
                                }
                                match inner_right.as_ref() {
                                    Expr::PostDecrement(_, _) => {},
                                    _ => panic!("Expected post-decrement"),
                                }
                            },
                            _ => panic!("Expected nested binary expression"),
                        }
                        match right.as_ref() {
                            Expr::Variable(name, _) => assert_eq!(name, "middle"),
                            _ => panic!("Expected variable middle"),
                        }
                    },
                    _ => panic!("Expected binary expression"),
                }
            },
            _ => panic!("Expected assignment statement"),
        }
    }
}
