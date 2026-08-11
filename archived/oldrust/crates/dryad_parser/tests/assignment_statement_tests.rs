// crates/dryad_parser/tests/assignment_statement_tests.rs
use dryad_parser::{Parser, Program, Stmt, Expr, Literal};
use dryad_lexer::{Lexer, token::Token};
use dryad_errors::DryadError;

fn parse_program(source: &str) -> Result<Program, DryadError> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token()?;
        if let Token::Eof = tok.token { tokens.push(tok); break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse()
}

#[cfg(test)]
mod assignment_statement_tests {
    use super::*;

    #[test]
    fn test_simple_assignment() {
        let program = parse_program("x = 5;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            assert!(matches!(expr, Expr::Literal(Literal::Number(5.0), _)));
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_addition_assignment() {
        let program = parse_program("x += 5;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            // Deve ser x = x + 5
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "+");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
                assert!(matches!(**right, Expr::Literal(Literal::Number(5.0), _)));
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_subtraction_assignment() {
        let program = parse_program("x -= 3;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "-");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
                assert!(matches!(**right, Expr::Literal(Literal::Number(3.0), _)));
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_multiplication_assignment() {
        let program = parse_program("x *= 2;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "*");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
                assert!(matches!(**right, Expr::Literal(Literal::Number(2.0), _)));
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_division_assignment() {
        let program = parse_program("x /= 4;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "/");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
                assert!(matches!(**right, Expr::Literal(Literal::Number(4.0), _)));
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_assignment_with_expression() {
        let program = parse_program("x += y * 2;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            // Deve ser x = x + (y * 2)
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "+");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
                
                // O lado direito deve ser y * 2
                if let Expr::Binary(right_left, right_op, right_right, _) = &**right {
                    assert_eq!(right_op, "*");
                    assert!(matches!(**right_left, Expr::Variable(ref var_name, _) if var_name == "y"));
                    assert!(matches!(**right_right, Expr::Literal(Literal::Number(2.0), _)));
                } else {
                    panic!("Esperado Binary expression no lado direito");
                }
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_multiple_assignments() {
        let program = parse_program("
            x = 10;
            x += 5;
            x -= 2;
            x *= 3;
            x /= 6;
        ").unwrap();
        assert_eq!(program.statements.len(), 5);

        // Verifica que todos são assignments
        for (i, stmt) in program.statements.iter().enumerate() {
            if let Stmt::Assignment(name, _, _) = stmt {
                assert_eq!(name.identifier_name().unwrap(), "x");
            } else {
                panic!("Statement {} deveria ser Assignment", i);
            }
        }
    }

    #[test]
    fn test_assignment_vs_declaration() {
        let program = parse_program("
            let x = 5;
            x = 10;
            x += 3;
        ").unwrap();
        assert_eq!(program.statements.len(), 3);

        // Primeiro deve ser VarDeclaration
        assert!(matches!(program.statements[0], Stmt::VarDeclaration(..)));
        
        // Os outros dois devem ser Assignment
        assert!(matches!(program.statements[1], Stmt::Assignment(..)));
        assert!(matches!(program.statements[2], Stmt::Assignment(..)));
    }

    #[test]
    fn test_assignment_with_variables() {
        let program = parse_program("x = y + z;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::Assignment(name, expr, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "x");
            if let Expr::Binary(left, op, right, _) = expr {
                assert_eq!(op, "+");
                assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "y"));
                assert!(matches!(**right, Expr::Variable(ref var_name, _) if var_name == "z"));
            } else {
                panic!("Esperado Binary expression");
            }
        } else {
            panic!("Esperado Assignment statement");
        }
    }

    #[test]
    fn test_chained_assignment_operations() {
        let program = parse_program("
            let total = 0;
            total += 10;
            total *= 2;
            total -= 5;
            total /= 3;
        ").unwrap();
        assert_eq!(program.statements.len(), 5);

        // Primeiro é declaração
        if let Stmt::VarDeclaration(name, _, _, _) = &program.statements[0] {
            assert_eq!(name.identifier_name().unwrap(), "total");
        } else {
            panic!("Primeiro statement deveria ser VarDeclaration");
        }

        // Os outros são assignments
        let expected_ops = ["+", "*", "-", "/"];
        for (i, expected_op) in expected_ops.iter().enumerate() {
            if let Stmt::Assignment(name, expr, _) = &program.statements[i + 1] {
                assert_eq!(name.identifier_name().unwrap(), "total");
                if let Expr::Binary(_, op, _, _) = expr {
                    assert_eq!(op, expected_op);
                } else {
                    panic!("Assignment {} deveria ter Binary expression", i + 1);
                }
            } else {
                panic!("Statement {} deveria ser Assignment", i + 1);
            }
        }
    }
}
