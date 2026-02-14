use dryad_lexer::{token::Token, Lexer};
use dryad_parser::{
    ast::{Expr, Literal, Stmt},
    Parser,
};

fn parse_tokens(input: &str) -> dryad_parser::ast::Program {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token {
            break;
        }
        tokens.push(tok);
    }

    let mut parser = Parser::new(tokens);
    parser.parse().unwrap()
}

#[test]
fn test_parse_simple_do_while_statement() {
    let input = r#"
    do {
        i = i + 1;
    } while (i < 5);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(program.statements[0], Stmt::DoWhile(..)));

    match &program.statements[0] {
        Stmt::DoWhile(body, condition, _) => {
            // Verifica o corpo é um bloco
            assert!(matches!(**body, Stmt::Block(..)));

            // Verifica a condição: i < 5
            match condition {
                Expr::Binary(left, op, right, _) => {
                    assert!(matches!(**left, Expr::Variable(_, _)));
                    assert_eq!(op, "<");
                    assert!(matches!(**right, Expr::Literal(Literal::Number(5.0), _)));
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_with_complex_condition() {
    let input = r#"
    do {
        x = x - 1;
    } while (x > 0 && y < 10);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);
    assert!(matches!(program.statements[0], Stmt::DoWhile(..)));

    match &program.statements[0] {
        Stmt::DoWhile(_, condition, _) => {
            // Condição complexa: x > 0 && y < 10
            match condition {
                Expr::Binary(_, op, _, _) => {
                    assert_eq!(op, "&&");
                }
                _ => panic!("Condição deveria ser uma expressão binária"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_with_multiple_statements() {
    let input = r#"
    do {
        result = counter;
        counter = counter + 1;
        let temp = counter;
    } while (counter < 3);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(body, _, _) => {
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 3); // result assignment, counter assignment, declaration
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_nested_do_while_statements() {
    let input = r#"
    do {
        do {
            result = result + 1;
        } while (inner < 3);
    } while (outer < 2);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(outer_body, _, _) => {
            match **outer_body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    // O primeiro statement deve ser outro do-while
                    assert!(matches!(statements[0], Stmt::DoWhile(..)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_with_if_inside() {
    let input = r#"
    do {
        if condition {
            break;
        }
    } while (running);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(body, _, _) => {
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    // Deve conter um if statement
                    assert!(matches!(statements[0], Stmt::If(_, _, _)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_with_single_statement_block() {
    let input = r#"
    do {
        counter = counter + 1;
    } while (active);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(body, condition, _) => {
            // Condição simples: active
            match condition {
                Expr::Variable(name, _) => {
                    assert_eq!(name, "active");
                }
                _ => panic!("Condição deveria ser uma variável"),
            }

            // Corpo com um statement
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 1);
                    assert!(matches!(statements[0], Stmt::Assignment(..)));
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_without_braces_error() {
    let input = r#"
    do
        statement;
    while condition;
    "#;

    // Este teste deveria falhar porque do-while requer chaves
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let tok = lexer.next_token().unwrap();
        if let Token::Eof = tok.token {
            break;
        }
        tokens.push(tok);
    }

    let mut parser = Parser::new(tokens);
    let result = parser.parse();

    // Deveria retornar erro
    assert!(result.is_err());
}

#[test]
fn test_parse_do_while_boolean_conditions() {
    let input = r#"
    do {
        break;
    } while (true);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(_, condition, _) => {
            match condition {
                Expr::Literal(Literal::Bool(true), _) => {
                    // Correto
                }
                _ => panic!("Condição deveria ser true literal"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_parse_do_while_variable_condition() {
    let input = r#"
    do {
        result = result + 1;
    } while (running);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        Stmt::DoWhile(_, condition, _) => match condition {
            Expr::Variable(name, _) => {
                assert_eq!(name, "running");
            }
            _ => panic!("Condição deveria ser uma variável"),
        },
        _ => panic!("Esperava um do-while statement"),
    }
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    do {
        result = i;
        i = i + 1;
    } while (i < 5);
    "#;

    let program = parse_tokens(input);
    assert_eq!(program.statements.len(), 2); // let declaration + do-while

    // Primeiro statement: let i = 0;
    match &program.statements[0] {
        Stmt::VarDeclaration(pattern, _, Some(expr), _) => {
            assert_eq!(pattern.identifier_name().unwrap(), "i");
            match expr {
                Expr::Literal(Literal::Number(0.0), _) => {
                    // Correto
                }
                _ => panic!("Valor inicial deveria ser 0"),
            }
        }
        _ => panic!("Esperava declaração de variável"),
    }

    // Segundo statement: do-while loop
    match &program.statements[1] {
        Stmt::DoWhile(body, condition, _) => {
            // Condição: i < 5
            match condition {
                Expr::Binary(left, op, right, _) => {
                    assert!(matches!(**left, Expr::Variable(_, _)));
                    assert_eq!(op, "<");
                    assert!(matches!(**right, Expr::Literal(Literal::Number(5.0), _)));
                }
                _ => panic!("Condição deveria ser i < 5"),
            }

            // Corpo: { result = i; i = i + 1; }
            match **body {
                Stmt::Block(ref statements, _) => {
                    assert_eq!(statements.len(), 2);
                }
                _ => panic!("Corpo deveria ser um bloco"),
            }
        }
        _ => panic!("Esperava um do-while statement"),
    }
}
