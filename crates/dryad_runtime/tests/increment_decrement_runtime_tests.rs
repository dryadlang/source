// crates/dryad_runtime/tests/increment_decrement_runtime_tests.rs
use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};
use dryad_errors::DryadError;

fn execute_program(source: &str) -> Result<String, DryadError> {
    let mut lexer = Lexer::new(source);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token()?;
        if matches!(token.token, Token::Eof) {
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
mod increment_decrement_runtime_tests {
    use super::*;

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let result = execute_program("
            let contador = 0;
            contador++;  // Incrementa 1 (agora contador é 1)
            contador--;  // Decrementa 1 (agora contador é 0)
            contador;    // retorna o valor final
        ").unwrap();
        
        assert_eq!(result, "0");
    }

    #[test]
    fn test_post_increment() {
        let result = execute_program("
            let x = 5;
            x++;
        ").unwrap();
        
        // Post-increment retorna o valor original
        assert_eq!(result, "5");
        
        // Verifica se a variável foi incrementada
        let result2 = execute_program("
            let x = 5;
            x++;
            x;
        ").unwrap();
        
        assert_eq!(result2, "6");
    }

    #[test]
    fn test_post_decrement() {
        let result = execute_program("
            let x = 5;
            x--;
        ").unwrap();
        
        // Post-decrement retorna o valor original
        assert_eq!(result, "5");
        
        // Verifica se a variável foi decrementada
        let result2 = execute_program("
            let x = 5;
            x--;
            x;
        ").unwrap();
        
        assert_eq!(result2, "4");
    }

    #[test]
    fn test_pre_increment() {
        let result = execute_program("
            let x = 5;
            ++x;
        ").unwrap();
        
        // Pre-increment retorna o valor novo
        assert_eq!(result, "6");
        
        // Verifica se a variável foi incrementada
        let result2 = execute_program("
            let x = 5;
            ++x;
            x;
        ").unwrap();
        
        assert_eq!(result2, "6");
    }

    #[test]
    fn test_pre_decrement() {
        let result = execute_program("
            let x = 5;
            --x;
        ").unwrap();
        
        // Pre-decrement retorna o valor novo
        assert_eq!(result, "4");
        
        // Verifica se a variável foi decrementada
        let result2 = execute_program("
            let x = 5;
            --x;
            x;
        ").unwrap();
        
        assert_eq!(result2, "4");
    }

    #[test]
    fn test_difference_between_pre_and_post() {
        // Testando a diferença entre pré e pós incremento
        let post_result = execute_program("
            let x = 10;
            let result = x++;
            result;
        ").unwrap();
        
        assert_eq!(post_result, "10"); // x++ retorna valor original
        
        let pre_result = execute_program("
            let x = 10;
            let result = ++x;
            result;
        ").unwrap();
        
        assert_eq!(pre_result, "11"); // ++x retorna valor novo
    }

    #[test]
    fn test_increment_decrement_in_expressions() {
        let result = execute_program("
            let x = 5;
            let y = 3;
            let result = x++ + --y;
            result;
        ").unwrap();
        
        // x++ retorna 5, --y retorna 2, então 5 + 2 = 7
        assert_eq!(result, "7");
        
        // Verifica os valores finais das variáveis
        let final_x = execute_program("
            let x = 5;
            let y = 3;
            let result = x++ + --y;
            x;
        ").unwrap();
        
        assert_eq!(final_x, "6"); // x foi incrementado
        
        let final_y = execute_program("
            let x = 5;
            let y = 3;
            let result = x++ + --y;
            y;
        ").unwrap();
        
        assert_eq!(final_y, "2"); // y foi decrementado
    }

    #[test]
    fn test_multiple_increments_decrements() {
        let result = execute_program("
            let x = 0;
            x++;    // x = 1
            x++;    // x = 2
            ++x;    // x = 3
            x--;    // x = 2
            --x;    // x = 1
            x;
        ").unwrap();
        
        assert_eq!(result, "1");
    }

    #[test]
    fn test_increment_decrement_with_multiplication() {
        let result = execute_program("
            let x = 3;
            let result = x++ * 2;
            result;
        ").unwrap();
        
        // x++ retorna 3, então 3 * 2 = 6
        assert_eq!(result, "6");
        
        // Verifica que x foi incrementado
        let final_x = execute_program("
            let x = 3;
            let result = x++ * 2;
            x;
        ").unwrap();
        
        assert_eq!(final_x, "4");
    }

    #[test]
    fn test_complex_expression_with_increment_decrement() {
        let result = execute_program("
            let a = 10;
            let b = 5;
            let c = 2;
            let result = ++a + b-- - --c;
            result;
        ").unwrap();
        
        // ++a = 11, b-- = 5, --c = 1
        // 11 + 5 - 1 = 15
        assert_eq!(result, "15");
        
        // Verifica valores finais
        let final_a = execute_program("
            let a = 10;
            let b = 5;
            let c = 2;
            let result = ++a + b-- - --c;
            a;
        ").unwrap();
        assert_eq!(final_a, "11");
        
        let final_b = execute_program("
            let a = 10;
            let b = 5;
            let c = 2;
            let result = ++a + b-- - --c;
            b;
        ").unwrap();
        assert_eq!(final_b, "4");
        
        let final_c = execute_program("
            let a = 10;
            let b = 5;
            let c = 2;
            let result = ++a + b-- - --c;
            c;
        ").unwrap();
        assert_eq!(final_c, "1");
    }

    #[test]
    fn test_increment_decrement_with_parentheses() {
        let result = execute_program("
            let x = 5;
            let y = 3;
            let result = (x++) + (--y);
            result;
        ").unwrap();
        
        // (x++) = 5, (--y) = 2, então 5 + 2 = 7
        assert_eq!(result, "7");
    }

    #[test]
    fn test_floating_point_increment_decrement() {
        let result = execute_program("
            let x = 2.5;
            x++;
            x;
        ").unwrap();
        
        assert_eq!(result, "3.5");
        
        let result2 = execute_program("
            let x = 2.5;
            x--;
            x;
        ").unwrap();
        
        assert_eq!(result2, "1.5");
    }

    #[test]
    fn test_negative_number_increment_decrement() {
        let result = execute_program("
            let x = -5;
            x++;
            x;
        ").unwrap();
        
        assert_eq!(result, "-4");
        
        let result2 = execute_program("
            let x = -1;
            ++x;
        ").unwrap();
        
        assert_eq!(result2, "0");
    }

    #[test]
    fn test_increment_decrement_error_non_number() {
        let result = execute_program("
            let x = \"hello\";
            x++;
        ");
        
        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.message().contains("só é válido para números"));
        }
    }

    #[test]
    fn test_increment_decrement_error_undefined_variable() {
        let result = execute_program("
            y++;
        ");
        
        assert!(result.is_err());
        if let Err(err) = result {
            // Aceita qualquer mensagem que indique erro de variável não definida
            assert!(err.message().contains("y") || err.message().contains("undefined") || err.message().contains("variável") || err.message().contains("não"));
        }
    }

    #[test]
    fn test_chained_operations() {
        let result = execute_program("
            let x = 5;
            let a = x++;    // a = 5, x = 6
            let b = ++x;    // b = 7, x = 7
            let c = x--;    // c = 7, x = 6
            let d = --x;    // d = 5, x = 5
            
            a + b + c + d;  // 5 + 7 + 7 + 5 = 24
        ").unwrap();
        
        assert_eq!(result, "24");
    }

    #[test]
    fn test_increment_in_assignment() {
        let result = execute_program("
            let x = 10;
            let y = 0;
            y = x++;    // y recebe valor original de x (10), x vira 11
            y;
        ").unwrap();
        
        assert_eq!(result, "10");
        
        let result2 = execute_program("
            let x = 10;
            let y = 0;
            y = x++;
            x;
        ").unwrap();
        
        assert_eq!(result2, "11");
    }

    #[test]
    fn test_step_by_step_counter() {
        // Simula exatamente o exemplo do SYNTAX.md passo a passo
        let step1 = execute_program("
            let contador = 0;
            contador++;
        ").unwrap();
        assert_eq!(step1, "0"); // retorna valor original
        
        let step2 = execute_program("
            let contador = 0;
            contador++;
            contador;
        ").unwrap();
        assert_eq!(step2, "1"); // agora contador é 1
        
        let step3 = execute_program("
            let contador = 1;
            contador--;
        ").unwrap();
        assert_eq!(step3, "1"); // retorna valor original
        
        let step4 = execute_program("
            let contador = 1;
            contador--;
            contador;
        ").unwrap();
        assert_eq!(step4, "0"); // agora contador é 0
    }

    #[test]
    fn test_precedence_increment_vs_arithmetic() {
        let result = execute_program("
            let x = 2;
            let result = x++ * 3 + 1;
            result;
        ").unwrap();
        
        // Deve ser (x++) * 3 + 1 = 2 * 3 + 1 = 7
        assert_eq!(result, "7");
        
        let result2 = execute_program("
            let x = 2;
            let result = ++x * 3;
            result;
        ").unwrap();
        
        // Deve ser (++x) * 3 = 3 * 3 = 9
        assert_eq!(result2, "9");
    }

    #[test]
    fn test_double_increment_different_variables() {
        let result = execute_program("
            let a = 1;
            let b = 2;
            let c = 3;
            
            a++; // a = 2
            ++b; // b = 3
            c--; // c = 2
            
            a + b + c; // 2 + 3 + 2 = 7
        ").unwrap();
        
        assert_eq!(result, "7");
    }
}
