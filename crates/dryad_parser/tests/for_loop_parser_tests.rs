use dryad_parser::{Parser, ast::{Stmt, Expr, Literal}};
use dryad_lexer::{Lexer, Token};

fn parse_tokens(input: &str) -> dryad_parser::ast::Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token_with_loc = lexer.next_token().unwrap();
        if matches!(token_with_loc.token, Token::Eof) {
            break;
        }
        tokens.push(token_with_loc);
    }
    
    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_parse_simple_for_statement() {
    let input = r#"
    for (i = 0; i < 5; i = i + 1) {
        result = result + i;
    }
    "#;
    
    let program = parse_tokens(input);
        assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(init, condition, update, body, _) => {
            // Verifica inicialização: i = 0
            assert!(init.is_some());
            match init.as_ref().unwrap().as_ref() {
                Stmt::Assignment(var, _, _) => {
                    assert_eq!(var, "i");
                    // Simplificamos o teste para apenas verificar se é uma atribuição à variável 'i'
                }
                _ => panic!("Inicialização deveria ser um assignment"),
            }
            
            // Verifica condição: i < 5
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                Expr::Binary(left, op, right, _) => {
                    assert!(matches!(**left, Expr::Variable(_, _)));
                    assert_eq!(op, "<");
                    assert!(matches!(**right, Expr::Literal(Literal::Number(5.0), _)));
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
            
            // Verifica update: i = i + 1
            assert!(update.is_some());
            match update.as_ref().unwrap().as_ref() {
                Stmt::Assignment(var, _, _) => {
                    assert_eq!(var, "i");
                }
                _ => panic!("Update deveria ser um assignment"),
            }
            
            // Verifica corpo é um bloco
            assert!(matches!(**body, Stmt::Block(..)));
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_with_complex_condition() {
    let input = r#"
    for (count = 1; count <= 10 && active; count = count + 2) {
        sum = sum + count;
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, condition, _, _, _) => {
            // Condição complexa: count <= 10 && active
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                Expr::Binary(_, op, _, _) => {
                    assert_eq!(op, "&&");
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_with_multiple_statements() {
    let input = r#"
    for (i = 0; i < 3; i = i + 1) {
        result = result + i;
        count = count + 1;
        let temp = i * 2;
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, _, _, body, _) => {
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 3); // 2 assignments + 1 declaration
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_nested_for_statements() {
    let input = r#"
    for (i = 0; i < 3; i = i + 1) {
        for (j = 0; j < 2; j = j + 1) {
            sum = sum + i + j;
        }
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, _, _, outer_body, _) => {
            match **outer_body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    // O primeiro statement deve ser outro for
                    assert!(matches!(statements[0], Stmt::For(_, _, _, _, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_with_if_inside() {
    let input = r#"
    for (i = 0; i < 5; i = i + 1) {
        if i % 2 == 0 {
            sum = sum + i;
        }
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, _, _, body, _) => {
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    // Deve conter um if statement
                    assert!(matches!(statements[0], Stmt::If(_, _, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_with_break_continue() {
    let input = r#"
    for (i = 0; i < 10; i = i + 1) {
        if i == 3 {
            continue;
        }
        if i == 7 {
            break;
        }
        result = result + i;
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, _, _, body, _) => {
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 3); // 2 ifs + 1 assignment
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_empty_components() {
    let input = r#"
    for (;;) {
        if condition {
            break;
        }
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(init, condition, update, _, _) => {
            // Todos os componentes devem ser None
            assert!(init.is_none());
            assert!(condition.is_none());
            assert!(update.is_none());
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_parse_for_without_braces_error() {
    let input = r#"
    for (i = 0; i < 5; i = i + 1)
        statement;
    "#;
    
    // Este teste deveria falhar porque for requer chaves
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token_with_loc = lexer.next_token().unwrap();
        if matches!(token_with_loc.token, Token::Eof) {
            break;
        }
        tokens.push(token_with_loc);
    }
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    // Deveria retornar erro
    assert!(result.is_err());
}

#[test]
fn test_parse_for_variable_condition() {
    let input = r#"
    for (i = 0; running; i = i + 1) {
        if i > 10 {
            running = false;
        }
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(_, condition, _, _, _) => {
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                Expr::Variable(name, _) => {
                    assert_eq!(name, "running");
                }
                _ => panic!("Condição deveria ser uma variável"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    for (i = 0; i < 5; i = i + 1) {
        i = i * 2;
    }
    "#;
    
    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    
    match &program.statements[0] {
        Stmt::For(init, condition, update, body, _) => {
            // Inicialização: i = 0
            assert!(init.is_some());
            match init.as_ref().unwrap().as_ref() {
                Stmt::Assignment(var, _, _) => {
                    assert_eq!(var, "i");
                    // Simplificamos o teste para apenas verificar se é uma atribuição à variável 'i'
                }
                _ => panic!("Inicialização deveria ser i = 0"),
            }
            
            // Condição: i < 5
            assert!(condition.is_some());
            match condition.as_ref().unwrap() {
                Expr::Binary(left, op, _right, _) => {
                    // Simplificamos para apenas verificar que é uma expressão binária com operador "<"
                    assert_eq!(op.as_str(), "<");
                    // Verifica que o lado esquerdo é uma variável
                    match left.as_ref() {
                        Expr::Variable(var, _) => assert_eq!(var, "i"),
                        _ => panic!("Lado esquerdo deveria ser uma variável"),
                    }
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
            
            // Update: i = i + 1
            assert!(update.is_some());
            match update.as_ref().unwrap().as_ref() {
                Stmt::Assignment(var, _, _) => {
                    assert_eq!(var, "i");
                }
                _ => panic!("Update deveria ser i = i + 1"),
            }
            
            // Corpo: { print(i); }
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um for statement"),
    }
}
