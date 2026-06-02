#[cfg(test)]
mod exception_parser_tests {
    use dryad_lexer::{lexer::Lexer, token::{Token, TokenWithLocation}};
    use dryad_parser::{Parser, ast::{Stmt, Expr, Literal}};

    fn parse_tokens(input: &str) -> Vec<TokenWithLocation> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();
        
        loop {
            match lexer.next_token() {
                Ok(token_with_location) => {
                    let is_eof = matches!(token_with_location.token, Token::Eof);
                    tokens.push(token_with_location);
                    if is_eof {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        
        tokens
    }

    #[test]
    fn test_simple_try_catch() {
        let input = "try { let x = 1; } catch (e) { let y = 2; }";
        let tokens = parse_tokens(input);
        let mut parser = Parser::new(tokens);
        
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Try(try_block, catch_clause, finally_clause, _location) => {
                // Verifica try block
                assert!(matches!(try_block.as_ref(), Stmt::Block(..)));
                
                // Verifica catch clause
                assert!(catch_clause.is_some());
                let (catch_var, catch_block) = catch_clause.as_ref().unwrap();
                assert_eq!(catch_var, "e");
                assert!(matches!(catch_block.as_ref(), Stmt::Block(..)));
                
                // Verifica que não há finally
                assert!(finally_clause.is_none());
            }
            _ => panic!("Esperado Statement::Try"),
        }
    }

    #[test]
    fn test_try_catch_finally() {
        let input = "try { let x = 1; } catch (err) { let y = 2; } finally { let z = 3; }";
        let tokens = parse_tokens(input);
        let mut parser = Parser::new(tokens);
        
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Try(try_block, catch_clause, finally_clause, _location) => {
                // Verifica try block
                assert!(matches!(try_block.as_ref(), Stmt::Block(..)));
                
                // Verifica catch clause
                assert!(catch_clause.is_some());
                let (catch_var, catch_block) = catch_clause.as_ref().unwrap();
                assert_eq!(catch_var, "err");
                assert!(matches!(catch_block.as_ref(), Stmt::Block(..)));
                
                // Verifica finally clause
                assert!(finally_clause.is_some());
                assert!(matches!(finally_clause.as_ref().unwrap().as_ref(), Stmt::Block(..)));
            }
            _ => panic!("Esperado Statement::Try"),
        }
    }

    #[test]
    fn test_try_finally_without_catch() {
        let input = "try { let x = 1; } finally { let y = 2; }";
        let tokens = parse_tokens(input);
        let mut parser = Parser::new(tokens);
        
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Try(try_block, catch_clause, finally_clause, _location) => {
                // Verifica try block
                assert!(matches!(try_block.as_ref(), Stmt::Block(..)));
                
                // Verifica que não há catch
                assert!(catch_clause.is_none());
                
                // Verifica finally clause
                assert!(finally_clause.is_some());
                assert!(matches!(finally_clause.as_ref().unwrap().as_ref(), Stmt::Block(..)));
            }
            _ => panic!("Esperado Statement::Try"),
        }
    }

    #[test]
    fn test_throw_statement() {
        let input = "throw \"error message\";";
        let tokens = parse_tokens(input);
        let mut parser = Parser::new(tokens);
        
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Throw(expr, _location) => {
                match expr {
                    Expr::Literal(Literal::String(s), _) => {
                        assert_eq!(s, "error message");
                    }
                    _ => panic!("Esperado literal string na expressão throw"),
                }
            }
            _ => panic!("Esperado Statement::Throw"),
        }
    }

    #[test]
    fn test_throw_variable() {
        let input = "throw errorVar;";
        let tokens = parse_tokens(input);
        let mut parser = Parser::new(tokens);
        
        let program = parser.parse().unwrap();
        assert_eq!(program.statements.len(), 1);
        
        match &program.statements[0] {
            Stmt::Throw(expr, _location) => {
                match expr {
                    Expr::Variable(name, _) => {
                        assert_eq!(name, "errorVar");
                    }
                    _ => panic!("Esperado variável na expressão throw"),
                }
            }
            _ => panic!("Esperado Statement::Throw"),
        }
    }
}
