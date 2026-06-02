// crates/dryad_runtime/tests/assignment_runtime_tests.rs
use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::Lexer;
use dryad_errors::DryadError;

fn execute_program(source: &str) -> Result<String, DryadError> {
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
    interpreter.execute(&program)
}

#[cfg(test)]
mod assignment_runtime_tests {
    use super::*;

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let result = execute_program("
            let x = 5;
            x += 2;  // x = x + 2 (agora x é 7)
            x -= 3;  // x = x - 3 (agora x é 4)
            x *= 2;  // x = x * 2 (agora x é 8)
            x /= 4;  // x = x / 4 (agora x é 2)
            x;       // retorna o valor final
        ").unwrap();
        
        assert_eq!(result, "2");
    }

    #[test]
    fn test_simple_assignment() {
        let result = execute_program("
            let x = 10;
            x = 20;
            x;
        ").unwrap();
        
        assert_eq!(result, "20");
    }

    #[test]
    fn test_addition_assignment() {
        let result = execute_program("
            let x = 10;
            x += 5;
            x;
        ").unwrap();
        
        assert_eq!(result, "15");
    }

    #[test]
    fn test_subtraction_assignment() {
        let result = execute_program("
            let x = 10;
            x -= 3;
            x;
        ").unwrap();
        
        assert_eq!(result, "7");
    }

    #[test]
    fn test_multiplication_assignment() {
        let result = execute_program("
            let x = 4;
            x *= 3;
            x;
        ").unwrap();
        
        assert_eq!(result, "12");
    }

    #[test]
    fn test_division_assignment() {
        let result = execute_program("
            let x = 20;
            x /= 4;
            x;
        ").unwrap();
        
        assert_eq!(result, "5");
    }

    #[test]
    fn test_assignment_with_expression() {
        let result = execute_program("
            let x = 10;
            let y = 5;
            x += y * 2;
            x;
        ").unwrap();
        
        assert_eq!(result, "20"); // 10 + (5 * 2)
    }

    #[test]
    fn test_chained_assignments() {
        let result = execute_program("
            let total = 100;
            total -= 20;    // 80
            total /= 4;     // 20
            total += 15;    // 35
            total *= 2;     // 70
            total;
        ").unwrap();
        
        assert_eq!(result, "70");
    }

    #[test]
    fn test_assignment_with_variables() {
        let result = execute_program("
            let a = 10;
            let b = 5;
            let x = 0;
            
            x += a;     // x = 0 + 10 = 10
            x -= b;     // x = 10 - 5 = 5
            x *= a;     // x = 5 * 10 = 50
            x /= b;     // x = 50 / 5 = 10
            
            x;
        ").unwrap();
        
        assert_eq!(result, "10");
    }

    #[test]
    fn test_multiple_variables_assignments() {
        let result = execute_program("
            let x = 10;
            let y = 20;
            let z = 30;
            
            x += 5;     // 15
            y *= 2;     // 40
            z -= 10;    // 20
            
            z;          // retorna z
        ").unwrap();
        
        assert_eq!(result, "20");
    }

    #[test]
    fn test_assignment_with_floating_point() {
        let result = execute_program("
            let x = 10.5;
            x += 2.5;       // 13.0
            x *= 2;         // 26.0
            x /= 4;         // 6.5
            x -= 1.5;       // 5.0
            x;
        ").unwrap();
        
        assert_eq!(result, "5");
    }

    #[test]
    fn test_assignment_with_negative_numbers() {
        let result = execute_program("
            let x = -5;
            x += 10;        // 5
            x -= -3;        // 8
            x *= -2;        // -16
            x /= -4;        // 4
            x;
        ").unwrap();
        
        assert_eq!(result, "4");
    }

    #[test]
    fn test_assignment_precedence() {
        let result = execute_program("
            let x = 2;
            let y = 3;
            x += y * 4;     // x = 2 + (3 * 4) = 14
            x;
        ").unwrap();
        
        assert_eq!(result, "14");
    }

    #[test]
    fn test_complex_assignment_expressions() {
        let result = execute_program("
            let a = 5;
            let b = 3;
            let c = 2;
            
            a += b + c;     // a = 5 + (3 + 2) = 10
            b *= a - c;     // b = 3 * (10 - 2) = 24
            c += a * b;     // c = 2 + (10 * 24) = 242
            
            c;
        ").unwrap();
        
        assert_eq!(result, "242");
    }

    #[test]
    fn test_assignment_error_undefined_variable() {
        let result = execute_program("
            x += 5;
        ");
        
        assert!(result.is_err());
        if let Err(err) = result {
            // Aceita qualquer mensagem que indique variável não definida
            assert!(err.message().contains("x") || err.message().contains("undefined") || err.message().contains("variável"));
        }
    }

    #[test]
    fn test_division_by_zero_in_assignment() {
        let result = execute_program("
            let x = 10;
            x /= 0;
        ");
        
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.message().contains("Divisão por zero"));
        }
    }

    #[test]
    fn test_assignment_returns_assigned_value() {
        // Verifica que o assignment retorna o valor atribuído
        let result = execute_program("
            let x = 10;
            x += 5;     // Deve retornar 15
        ").unwrap();
        
        assert_eq!(result, "15");
    }

    #[test]
    fn test_all_operators_sequence() {
        let result = execute_program("
            let value = 8;
            
            value += 2;     // 10
            value -= 1;     // 9
            value *= 3;     // 27
            value /= 9;     // 3
            
            value;
        ").unwrap();
        
        assert_eq!(result, "3");
    }

    #[test]
    fn test_assignment_with_parentheses() {
        let result = execute_program("
            let x = 10;
            let y = 2;
            let z = 3;
            
            x += (y + z) * 2;   // x = 10 + ((2 + 3) * 2) = 10 + 10 = 20
            x;
        ").unwrap();
        
        assert_eq!(result, "20");
    }

    #[test]
    fn test_step_by_step_calculation() {
        // Simula o exemplo do SYNTAX.md passo a passo
        let result = execute_program("
            let x = 5;
            x += 2;     // x = 7, retorna 7
        ").unwrap();
        assert_eq!(result, "7");

        let result = execute_program("
            let x = 7;
            x -= 3;     // x = 4, retorna 4
        ").unwrap();
        assert_eq!(result, "4");

        let result = execute_program("
            let x = 4;
            x *= 2;     // x = 8, retorna 8
        ").unwrap();
        assert_eq!(result, "8");

        let result = execute_program("
            let x = 8;
            x /= 4;     // x = 2, retorna 2
        ").unwrap();
        assert_eq!(result, "2");
    }
}
