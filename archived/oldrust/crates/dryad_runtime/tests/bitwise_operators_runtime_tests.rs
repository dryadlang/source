use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::Lexer;

fn evaluate_expression(source: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        if matches!(token.token, dryad_lexer::Token::Eof) {
            tokens.push(token);
            break;
        }
        tokens.push(token);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    let mut interpreter = Interpreter::new();
    let result = interpreter.execute(&program)?;
    
    // Parse the result string back to a number
    let number: f64 = result.parse()?;
    Ok(number)
}

#[test]
fn test_left_shift_operator_evaluation() {
    let result = evaluate_expression("1 << 2").unwrap();
    assert_eq!(result, 4.0); // 1 << 2 = 1 * 2^2 = 4
}

#[test]
fn test_right_shift_operator_evaluation() {
    let result = evaluate_expression("4 >> 2").unwrap();
    assert_eq!(result, 1.0); // 4 >> 2 = 4 / 2^2 = 1
}

#[test]
fn test_bitwise_and_operator_evaluation() {
    let result = evaluate_expression("0b1100 & 0b1010").unwrap();
    assert_eq!(result, 8.0); // 12 & 10 = 8 (0b1000)
}

#[test]
fn test_bitwise_or_operator_evaluation() {
    let result = evaluate_expression("0b1100 | 0b1010").unwrap();
    assert_eq!(result, 14.0); // 12 | 10 = 14 (0b1110)
}

#[test]
fn test_bitwise_xor_operator_evaluation() {
    let result = evaluate_expression("0b1100 ^ 0b1010").unwrap();
    assert_eq!(result, 6.0); // 12 ^ 10 = 6 (0b0110)
}

#[test]
fn test_symmetric_right_shift_operator_evaluation() {
    let result = evaluate_expression("0b1010 >>> 1").unwrap();
    assert_eq!(result, 5.0); // 10 >>> 1 = 5 (0b0101)
}

#[test]
fn test_symmetric_left_shift_operator_evaluation() {
    let result = evaluate_expression("0b0101 <<< 1").unwrap();
    assert_eq!(result, 10.0); // 5 <<< 1 = 10 (0b1010)
}

#[test]
fn test_bitwise_operators_with_different_numbers() {
    // Teste com números decimais
    let result = evaluate_expression("12 & 10").unwrap();
    assert_eq!(result, 8.0);
    
    let result = evaluate_expression("12 | 10").unwrap();
    assert_eq!(result, 14.0);
    
    let result = evaluate_expression("12 ^ 10").unwrap();
    assert_eq!(result, 6.0);
}

#[test]
fn test_shift_operators_with_different_numbers() {
    // Teste com diferentes valores de shift
    let result = evaluate_expression("8 << 1").unwrap();
    assert_eq!(result, 16.0); // 8 * 2^1 = 16
    
    let result = evaluate_expression("16 >> 1").unwrap();
    assert_eq!(result, 8.0); // 16 / 2^1 = 8
    
    let result = evaluate_expression("8 << 3").unwrap();
    assert_eq!(result, 64.0); // 8 * 2^3 = 64
    
    let result = evaluate_expression("64 >> 3").unwrap();
    assert_eq!(result, 8.0); // 64 / 2^3 = 8
}

#[test]
fn test_bitwise_operators_precedence() {
    // Teste precedência: & tem precedência maior que |
    let result = evaluate_expression("12 | 8 & 15").unwrap();
    // Deve ser interpretado como: 12 | (8 & 15) = 12 | 8 = 12
    assert_eq!(result, 12.0);
    
    // Shift tem precedência menor que aritmética
    let result = evaluate_expression("2 + 3 << 1").unwrap();
    // Deve ser interpretado como: (2 + 3) << 1 = 5 << 1 = 10
    assert_eq!(result, 10.0);
}

#[test]
fn test_bitwise_operators_with_parentheses() {
    let result = evaluate_expression("(12 & 8) | 4").unwrap();
    // (12 & 8) = 8, então 8 | 4 = 12
    assert_eq!(result, 12.0);
    
    let result = evaluate_expression("12 & (8 | 4)").unwrap();
    // (8 | 4) = 12, então 12 & 12 = 12
    assert_eq!(result, 12.0);
}

#[test]
fn test_shift_operators_with_zero() {
    let result = evaluate_expression("5 << 0").unwrap();
    assert_eq!(result, 5.0); // 5 * 2^0 = 5
    
    let result = evaluate_expression("5 >> 0").unwrap();
    assert_eq!(result, 5.0); // 5 / 2^0 = 5
}

#[test]
fn test_bitwise_operators_with_zero() {
    let result = evaluate_expression("5 & 0").unwrap();
    assert_eq!(result, 0.0); // 5 & 0 = 0
    
    let result = evaluate_expression("5 | 0").unwrap();
    assert_eq!(result, 5.0); // 5 | 0 = 5
    
    let result = evaluate_expression("5 ^ 0").unwrap();
    assert_eq!(result, 5.0); // 5 ^ 0 = 5
}

#[test]
fn test_complex_bitwise_expression() {
    let result = evaluate_expression("(0b1100 & 0b1010) | (0b0011 ^ 0b0101)").unwrap();
    // (12 & 10) | (3 ^ 5) = 8 | 6 = 14
    assert_eq!(result, 14.0);
}

#[test]
fn test_mixed_operations_with_bitwise() {
    let result = evaluate_expression("10 + 5 & 12").unwrap();
    // 10 + 5 = 15, então 15 & 12 = 12
    assert_eq!(result, 12.0);
    
    let result = evaluate_expression("10 * 2 << 1").unwrap();
    // 10 * 2 = 20, então 20 << 1 = 40
    assert_eq!(result, 40.0);
}

#[test]
fn test_exact_syntax_md_examples() {
    // Testa exatamente os exemplos do SYNTAX.md
    let result = evaluate_expression("1 << 2").unwrap();
    assert_eq!(result, 4.0); // 4 (1 * 2^2)
    
    let result = evaluate_expression("4 >> 2").unwrap();
    assert_eq!(result, 1.0); // 1 (4 / 2^2)
    
    let result = evaluate_expression("0b1100 & 0b1010").unwrap();
    assert_eq!(result, 8.0); // 0b1000 (8)
    
    let result = evaluate_expression("0b1100 | 0b1010").unwrap();
    assert_eq!(result, 14.0); // 0b1110 (14)
    
    let result = evaluate_expression("0b1100 ^ 0b1010").unwrap();
    assert_eq!(result, 6.0); // 0b0110 (6)
    
    let result = evaluate_expression("0b1010 >>> 1").unwrap();
    assert_eq!(result, 5.0); // 0b0101 (5)
    
    let result = evaluate_expression("0b0101 <<< 1").unwrap();
    assert_eq!(result, 10.0); // 0b1010 (10)
}

#[test]
fn test_floating_point_bitwise_operations() {
    // Operações bitwise com números de ponto flutuante (devem ser truncados para inteiros)
    let result = evaluate_expression("12.7 & 10.3").unwrap();
    assert_eq!(result, 8.0); // int(12.7) & int(10.3) = 12 & 10 = 8
    
    let result = evaluate_expression("12.1 | 10.9").unwrap();
    assert_eq!(result, 14.0); // int(12.1) | int(10.9) = 12 | 10 = 14
}

#[test]
fn test_large_numbers_bitwise() {
    let result = evaluate_expression("255 & 128").unwrap();
    assert_eq!(result, 128.0); // 0xFF & 0x80 = 0x80
    
    let result = evaluate_expression("1024 >> 2").unwrap();
    assert_eq!(result, 256.0); // 1024 / 4 = 256
    
    let result = evaluate_expression("1 << 8").unwrap();
    assert_eq!(result, 256.0); // 1 * 256 = 256
}
