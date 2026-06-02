use dryad_lexer::{Lexer, Token};
use dryad_parser::{
    ast::{Expr, Literal, Stmt},
    Parser,
};

fn parse_tokens(input: &str) -> dryad_parser::ast::Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().unwrap();
        if token.token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_parse_simple_while_statement() {
    let input = r#"
    while (x < 10) {
        x = x + 1;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(condition, body, _) => {
            match condition {
                Expr::Binary(left, op, right, _) => {
                    assert!(matches!(**left, Expr::Variable(_, _)));
                    assert_eq!(op, "<");
                    assert!(matches!(**right, Expr::Literal(Literal::Number(10.0), _)));
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
            assert!(matches!(**body, Stmt::Block(..)));
        }
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_with_complex_condition() {
    let input = r#"
    while (x > 0 && y < 10) {
        x = x - 1;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(condition, _, _) => match condition {
            Expr::Binary(_, op, _, _) => {
                assert_eq!(op, "&&");
            }
            _ => panic!("Condição deveria ser uma expressão binária"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_with_multiple_statements() {
    let input = r#"
    while (counter < 3) {
        result = counter;
        counter = counter + 1;
        let temp = counter;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(_, body, _) => match **body {
            Stmt::Block(ref statements, _) => {
                assert_eq!(statements.len(), 3);
            }
            _ => panic!("Corpo deveria ser um bloco"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_nested_while_statements() {
    let input = r#"
    while (outer < 2) {
        while (inner < 3) {
            result = result + 1;
        }
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(_, outer_body, _) => match **outer_body {
            Stmt::Block(ref statements, _) => {
                assert_eq!(statements.len(), 1);
                assert!(matches!(statements[0], Stmt::While(..)));
            }
            _ => panic!("Corpo deveria ser um bloco"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_with_if_inside() {
    let input = r#"
    while (running) {
        if condition {
            break;
        }
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(_, body, _) => match **body {
            Stmt::Block(ref statements, _) => {
                assert_eq!(statements.len(), 1);
                assert!(matches!(statements[0], Stmt::If(..)));
            }
            _ => panic!("Corpo deveria ser um bloco"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_with_single_statement_block() {
    let input = r#"
    while (active) {
        counter = counter + 1;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(condition, body, _) => {
            match condition {
                Expr::Variable(name, _) => {
                    assert_eq!(name, "active");
                }
                _ => panic!("Condição deveria ser uma variável"),
            }
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    assert!(matches!(statements[0], Stmt::Assignment(..)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_without_braces_error() {
    let input = r#"
    while (condition
        statement;
    "#;

    // Este teste deveria falhar porque while requer chaves
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().unwrap();
        if token.token == Token::Eof {
            break;
        }
        tokens.push(token);
    }

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Deveria retornar erro
    assert!(result.is_err());
}

#[test]
fn test_parse_while_boolean_conditions() {
    let input = r#"
    while (true) {
        break;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(condition, _, _) => match condition {
            Expr::Literal(Literal::Bool(true), _) => {}
            _ => panic!("Condição deveria ser true literal"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_parse_while_variable_condition() {
    let input = r#"
    while (running) {
        result = result + 1;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::While(condition, _, _) => match condition {
            Expr::Variable(name, _) => {
                assert_eq!(name, "running");
            }
            _ => panic!("Condição deveria ser uma variável"),
        },
        _ => panic!("Esperava um while statement"),
    }
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    while (i < 5) {
        result = i;
        i = i + 1;
    }
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 2); // let declaration + while

    // Primeiro statement: let i = 0;
    match &program.statements[0] {
        Stmt::VarDeclaration(name, _, Some(expr), _) => {
            assert_eq!(name.identifier_name().unwrap(), "i");
            match expr {
                Expr::Literal(Literal::Number(0.0), _) => {}
                _ => panic!("Valor inicial deveria ser 0"),
            }
        }
        _ => panic!("Esperava declaração de variável"),
    }
    match &program.statements[1] {
        Stmt::While(condition, body, _) => {
            match condition {
                Expr::Binary(left, op, right, _) => {
                    assert!(matches!(**left, Expr::Variable(_, _)));
                    assert_eq!(op, "<");
                    assert!(matches!(**right, Expr::Literal(Literal::Number(5.0), _)));
                }
                _ => panic!("Condição deveria ser i < 5"),
            }
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 2);
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um while statement"),
    }
}
