use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

#[test]
fn test_tokenize_left_shift_operator() {
    let mut lexer = Lexer::new("1 << 2");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_right_shift_operator() {
    let mut lexer = Lexer::new("4 >> 2");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(4.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">>".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_bitwise_and_operator() {
    let mut lexer = Lexer::new("0b1100 & 0b1010");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0)); // 0b1100 = 12
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("&".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0)); // 0b1010 = 10
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_bitwise_or_operator() {
    let mut lexer = Lexer::new("0b1100 | 0b1010");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0)); // 0b1100 = 12
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("|".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0)); // 0b1010 = 10
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_bitwise_xor_operator() {
    let mut lexer = Lexer::new("0b1100 ^ 0b1010");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0)); // 0b1100 = 12
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("^".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0)); // 0b1010 = 10
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_symmetric_right_shift_operator() {
    let mut lexer = Lexer::new("0b1010 >>> 1");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0)); // 0b1010 = 10
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">>>".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_tokenize_symmetric_left_shift_operator() {
    let mut lexer = Lexer::new("0b0101 <<< 1");
    
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(5.0)); // 0b0101 = 5
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Eof);
}

#[test]
fn test_differentiate_shift_from_comparison() {
    let mut lexer = Lexer::new("x < y");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("y".to_string()));
    
    let mut lexer = Lexer::new("x >> y");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">>".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("y".to_string()));
    
    let mut lexer = Lexer::new("x << y");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("x".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("y".to_string()));
}

#[test]
fn test_differentiate_bitwise_from_logical() {
    let mut lexer = Lexer::new("a && b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("&&".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
    
    let mut lexer = Lexer::new("a & b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("&".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
    
    let mut lexer = Lexer::new("a || b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("||".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
    
    let mut lexer = Lexer::new("a | b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("|".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
}

#[test]
fn test_differentiate_xor_from_exponentiation() {
    let mut lexer = Lexer::new("a ** b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("**".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
    
    let mut lexer = Lexer::new("a ^ b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("^".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
    
    let mut lexer = Lexer::new("a ^^ b");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("^^".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("b".to_string()));
}

#[test]
fn test_bitwise_operators_with_expressions() {
    let mut lexer = Lexer::new("(a << 2)");
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('('));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol(')'));
}

#[test]
fn test_exact_syntax_md_example() {
    // Teste deslocamento esquerda
    let mut lexer = Lexer::new("1 << 2");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    
    // Teste deslocamento direita
    let mut lexer = Lexer::new("4 >> 2");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(4.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">>".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
    
    // Teste bitwise AND
    let mut lexer = Lexer::new("0b1100 & 0b1010");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("&".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0));
    
    // Teste bitwise OR
    let mut lexer = Lexer::new("0b1100 | 0b1010");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("|".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0));
    
    // Teste bitwise XOR
    let mut lexer = Lexer::new("0b1100 ^ 0b1010");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(12.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("^".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0));
    
    // Teste deslocamento simétrico direita
    let mut lexer = Lexer::new("0b1010 >>> 1");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(10.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator(">>>".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
    
    // Teste deslocamento simétrico esquerda  
    let mut lexer = Lexer::new("0b0101 <<< 1");
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(5.0));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(1.0));
}

#[test]
fn test_complex_bitwise_expression() {
    let mut lexer = Lexer::new("result = a << 2");
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("result".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Symbol('='));
    assert_eq!(lexer.next_token().unwrap().token, Token::Identifier("a".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Operator("<<".to_string()));
    assert_eq!(lexer.next_token().unwrap().token, Token::Number(2.0));
}
