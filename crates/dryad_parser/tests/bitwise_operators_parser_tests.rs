use dryad_parser::{Parser, Expr, Stmt};
use dryad_lexer::{Lexer, token::Token};
use dryad_errors::DryadError;

fn parse_expression(source: &str) -> Result<Expr, DryadError> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let tok = lexer.next_token()?;
        if let Token::Eof = tok.token { tokens.push(tok); break; }
        tokens.push(tok);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    if let Some(Stmt::Expression(expr, _)) = program.statements.get(0) {
        Ok(expr.clone())
    } else {
        Err(DryadError::new(2001, "Expected expression"))
    }
}

#[test]
fn test_parse_left_shift_operator() {
    let expr = parse_expression("1 << 2").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "<<");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_right_shift_operator() {
    let expr = parse_expression("4 >> 2").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, ">>");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_bitwise_and_operator() {
    let expr = parse_expression("0b1100 & 0b1010").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "&");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_bitwise_or_operator() {
    let expr = parse_expression("0b1100 | 0b1010").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "|");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_bitwise_xor_operator() {
    let expr = parse_expression("0b1100 ^ 0b1010").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "^");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_symmetric_right_shift_operator() {
    let expr = parse_expression("0b1010 >>> 1").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, ">>>");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_parse_symmetric_left_shift_operator() {
    let expr = parse_expression("0b0101 <<< 1").unwrap();
    
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "<<<");
    } else {
        panic!("Expected binary expression");
    }
}

#[test]
fn test_exact_syntax_md_example() {
    // Teste deslocamento esquerda
    let expr = parse_expression("1 << 2").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "<<");
    }
    
    // Teste deslocamento direita
    let expr = parse_expression("4 >> 2").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, ">>");
    }
    
    // Teste bitwise AND
    let expr = parse_expression("0b1100 & 0b1010").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "&");
    }
    
    // Teste bitwise OR
    let expr = parse_expression("0b1100 | 0b1010").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "|");
    }
    
    // Teste bitwise XOR
    let expr = parse_expression("0b1100 ^ 0b1010").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "^");
    }
    
    // Teste deslocamento simétrico direita
    let expr = parse_expression("0b1010 >>> 1").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, ">>>");
    }
    
    // Teste deslocamento simétrico esquerda
    let expr = parse_expression("0b0101 <<< 1").unwrap();
    if let Expr::Binary(_, operator, _, _) = expr {
        assert_eq!(operator, "<<<");
    }
}
