use dryad_parser::{Parser, ast::Stmt};
use dryad_lexer::{Lexer, Token, TokenWithLocation};

fn parse_tokens(source: &str) -> Vec<TokenWithLocation> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    loop {
        match lexer.next_token() {
            Ok(token) => {
                if matches!(token.token, Token::Eof) {
                    tokens.push(token);
                    break;
                }
                tokens.push(token);
            }
            Err(_) => break,
        }
    }
    tokens
}

#[test]
fn test_parse_empty_block() {
    let tokens = parse_tokens("{ }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert!(statements.is_empty());
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_single_statement_block() {
    let tokens = parse_tokens("{ let x = 5; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 1);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_multiple_statements_block() {
    let tokens = parse_tokens("{ let x = 5; let y = 10; let z = 15; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 3);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
        assert!(matches!(statements[1], Stmt::VarDeclaration(_, _, _)));
        assert!(matches!(statements[2], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_nested_blocks() {
    let tokens = parse_tokens("{ { let x = 1; } { let y = 2; } }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 2);
        
        // First nested block
        if let Stmt::Block(inner1, _) = &statements[0] {
            assert_eq!(inner1.len(), 1);
            assert!(matches!(inner1[0], Stmt::VarDeclaration(_, _, _)));
        } else {
            panic!("Expected first nested Block statement");
        }
        
        // Second nested block
        if let Stmt::Block(inner2, _) = &statements[1] {
            assert_eq!(inner2.len(), 1);
            assert!(matches!(inner2[0], Stmt::VarDeclaration(_, _, _)));
        } else {
            panic!("Expected second nested Block statement");
        }
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_deeply_nested_blocks() {
    let tokens = parse_tokens("{ { { let x = 1; } } }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(level1, _) = &program.statements[0] {
        assert_eq!(level1.len(), 1);
        
        if let Stmt::Block(level2, _) = &level1[0] {
            assert_eq!(level2.len(), 1);
            
            if let Stmt::Block(level3, _) = &level2[0] {
                assert_eq!(level3.len(), 1);
                assert!(matches!(level3[0], Stmt::VarDeclaration(_, _, _)));
            } else {
                panic!("Expected level 3 Block statement");
            }
        } else {
            panic!("Expected level 2 Block statement");
        }
    } else {
        panic!("Expected level 1 Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_block_with_expressions() {
    let tokens = parse_tokens("{ 5; \"hello\"; true; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 3);
        assert!(matches!(statements[0], Stmt::Expression(_, _)));
        assert!(matches!(statements[1], Stmt::Expression(_, _)));
        assert!(matches!(statements[2], Stmt::Expression(_, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_block_with_mixed_statements() {
    let tokens = parse_tokens("{ let x = 5; 10; let y = \"test\"; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 3);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
        assert!(matches!(statements[1], Stmt::Expression(_, _)));
        assert!(matches!(statements[2], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_block_whitespace_handling() {
    let tokens = parse_tokens("{\n  let x = 5;\n  let y = 10;\n}");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 2);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
        assert!(matches!(statements[1], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_block_with_trailing_semicolon() {
    let tokens = parse_tokens("{ let x = 5; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 1);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test]
fn test_parse_block_without_trailing_semicolon() {
    // Block statements themselves don't need semicolons
    let tokens = parse_tokens("{ let x = 5 }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    if let Stmt::Block(statements, _) = &program.statements[0] {
        assert_eq!(statements.len(), 1);
        assert!(matches!(statements[0], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected Block statement, got {:?}", program.statements[0]);
    }
}

#[test] 
fn test_error_handling_unmatched_braces() {
    // Test missing closing brace
    let tokens = parse_tokens("{ let x = 5;");
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    assert!(result.is_err(), "Should error on missing closing brace");
}

#[test]
fn test_multiple_separate_blocks() {
    let tokens = parse_tokens("{ let x = 1; } { let y = 2; }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 2);
    
    // First block
    if let Stmt::Block(statements1, _) = &program.statements[0] {
        assert_eq!(statements1.len(), 1);
        assert!(matches!(statements1[0], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected first Block statement");
    }
    
    // Second block
    if let Stmt::Block(statements2, _) = &program.statements[1] {
        assert_eq!(statements2.len(), 1);
        assert!(matches!(statements2[0], Stmt::VarDeclaration(_, _, _)));
    } else {
        panic!("Expected second Block statement");
    }
}

#[test]
fn test_empty_blocks_sequence() {
    let tokens = parse_tokens("{ } { } { }");
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 3);
    for stmt in &program.statements {
        if let Stmt::Block(statements, _) = stmt {
            assert!(statements.is_empty());
        } else {
            panic!("Expected Block statement");
        }
    }
}
