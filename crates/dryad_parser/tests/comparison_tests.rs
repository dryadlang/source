// crates/dryad_parser/tests/comparison_tests.rs
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
mod comparison_tests {
    use super::*;

    #[test]
    fn test_equality_operator() {
        let program = parse_program("let igual = 5 == 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "igual");
            assert_eq!(op, "==");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_inequality_operator() {
        let program = parse_program("let diferente = 5 != 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "diferente");
            assert_eq!(op, "!=");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_less_than_operator() {
        let program = parse_program("let menor = 5 < 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "menor");
            assert_eq!(op, "<");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_greater_than_operator() {
        let program = parse_program("let maior = 5 > 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "maior");
            assert_eq!(op, ">");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_less_than_or_equal_operator() {
        let program = parse_program("let menorIgual = 5 <= 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "menorIgual");
            assert_eq!(op, "<=");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_greater_than_or_equal_operator() {
        let program = parse_program("let maiorIgual = 5 >= 10;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "maiorIgual");
            assert_eq!(op, ">=");
            assert!(matches!(**left, Expr::Literal(Literal::Number(5.0), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_complex_comparison_with_variables() {
        let program = parse_program("
            let x = 5;
            let y = 10;
            let resultado = x < y;
        ").unwrap();
        assert_eq!(program.statements.len(), 3);

        // Verifica primeira declaração
        if let Stmt::VarDeclaration(name, Some(Expr::Literal(Literal::Number(5.0), _)), _) = &program.statements[0] {
            assert_eq!(name, "x");
        } else {
            panic!("Esperado VarDeclaration x = 5");
        }

        // Verifica segunda declaração  
        if let Stmt::VarDeclaration(name, Some(Expr::Literal(Literal::Number(10.0), _)), _) = &program.statements[1] {
            assert_eq!(name, "y");
        } else {
            panic!("Esperado VarDeclaration y = 10");
        }

        // Verifica comparação
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[2] {
            assert_eq!(name, "resultado");
            assert_eq!(op, "<");
            assert!(matches!(**left, Expr::Variable(ref var_name, _) if var_name == "x"));
            assert!(matches!(**right, Expr::Variable(ref var_name, _) if var_name == "y"));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_string_comparison() {
        let program = parse_program("let igual = \"hello\" == \"world\";").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "igual");
            assert_eq!(op, "==");
            assert!(matches!(**left, Expr::Literal(Literal::String(ref s), _) if s == "hello"));
            assert!(matches!(**right, Expr::Literal(Literal::String(ref s), _) if s == "world"));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_boolean_comparison() {
        let program = parse_program("let igual = true == false;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "igual");
            assert_eq!(op, "==");
            assert!(matches!(**left, Expr::Literal(Literal::Bool(true), _)));
            assert!(matches!(**right, Expr::Literal(Literal::Bool(false), _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_null_comparison() {
        let program = parse_program("let igual = null == null;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "igual");
            assert_eq!(op, "==");
            assert!(matches!(**left, Expr::Literal(Literal::Null, _)));
            assert!(matches!(**right, Expr::Literal(Literal::Null, _)));
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }

    #[test]
    fn test_chained_comparisons() {
        let program = parse_program("let resultado = 1 < 2 && 2 < 3;").unwrap();
        assert_eq!(program.statements.len(), 1);
        
        if let Stmt::VarDeclaration(name, Some(Expr::Binary(left, op, right, _)), _) = &program.statements[0] {
            assert_eq!(name, "resultado");
            assert_eq!(op, "&&");
            
            // Left side should be 1 < 2
            if let Expr::Binary(left_left, left_op, left_right, _) = &**left {
                assert_eq!(left_op, "<");
                assert!(matches!(**left_left, Expr::Literal(Literal::Number(1.0), _)));
                assert!(matches!(**left_right, Expr::Literal(Literal::Number(2.0), _)));
            } else {
                panic!("Esperado Binary expression no lado esquerdo");
            }

            // Right side should be 2 < 3
            if let Expr::Binary(right_left, right_op, right_right, _) = &**right {
                assert_eq!(right_op, "<");
                assert!(matches!(**right_left, Expr::Literal(Literal::Number(2.0), _)));
                assert!(matches!(**right_right, Expr::Literal(Literal::Number(3.0), _)));
            } else {
                panic!("Esperado Binary expression no lado direito");
            }
        } else {
            panic!("Esperado VarDeclaration com Binary expression");
        }
    }
}
