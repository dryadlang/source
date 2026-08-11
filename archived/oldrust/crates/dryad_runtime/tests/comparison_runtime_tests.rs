// crates/dryad_runtime/tests/comparison_runtime_tests.rs
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
mod comparison_runtime_tests {
    use super::*;

    #[test]
    fn test_exact_syntax_md_example() {
        // Testa exatamente o exemplo do SYNTAX.md
        let result = execute_program("
            let x = 5;
            let y = 10;

            let igual = x == y;        // false
            let diferente = x != y;    // true
            let menor = x < y;         // true
            let maior = x > y;         // false
            let menorIgual = x <= y;   // true
            let maiorIgual = x >= y;   // false
            
            // Retorna o último valor
            maiorIgual;
        ").unwrap();
        
        assert_eq!(result, "false");
    }

    #[test]
    fn test_number_equality() {
        let result = execute_program("
            let a = 42;
            let b = 42;
            let c = 43;
            
            let igual1 = a == b;
            let igual2 = a == c;
            let diferente1 = a != b;
            let diferente2 = a != c;
            
            diferente2;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_number_comparisons() {
        let result = execute_program("
            let x = 10;
            let y = 20;
            
            let menor = x < y;
            let maior = x > y;
            let menorIgual1 = x <= y;
            let menorIgual2 = x <= x;
            let maiorIgual1 = x >= y;
            let maiorIgual2 = x >= x;
            
            maiorIgual2;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_string_comparisons() {
        let result = execute_program("
            let s1 = \"hello\";
            let s2 = \"hello\";
            let s3 = \"world\";
            
            let igual = s1 == s2;
            let diferente = s1 != s3;
            
            diferente;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_boolean_comparisons() {
        let result = execute_program("
            let t = true;
            let f = false;
            
            let igual1 = t == true;
            let igual2 = f == false;
            let diferente1 = t != f;
            let diferente2 = t == f;
            
            diferente1;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_null_comparisons() {
        let result = execute_program("
            let n1 = null;
            let n2 = null;
            let x = 42;
            
            let igual = n1 == n2;
            let diferente = n1 != x;
            
            diferente;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_mixed_type_comparisons() {
        let result = execute_program("
            let num = 42;
            let str = \"42\";
            let bool = true;
            let nul = null;
            
            let numStr = num == str;        // false (different types)
            let numBool = num == bool;      // false (different types) 
            let numNull = num == nul;       // false (different types)
            let strBool = str == bool;      // false (different types)
            let strNull = str == nul;       // false (different types)
            let boolNull = bool == nul;     // false (different types)
            
            // All should be false, testing the last one
            boolNull;
        ").unwrap();
        
        assert_eq!(result, "false");
    }

    #[test]
    fn test_comparison_with_expressions() {
        let result = execute_program("
            let x = 5;
            let y = 3;
            
            let resultado1 = (x + y) == 8;           // true
            let resultado2 = (x * y) > 10;           // true (15 > 10)
            let resultado3 = (x - y) <= 2;           // true (2 <= 2)
            let resultado4 = (x / y) != 1;           // true (1.666... != 1)
            
            resultado4;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_chained_comparisons_with_logical_operators() {
        let result = execute_program("
            let a = 1;
            let b = 2;
            let c = 3;
            
            let ascendente = a < b && b < c;         // true
            let descendente = a > b || b > c;        // false
            let igualdade = a == 1 && b == 2;        // true
            let misturado = a < b && b != c;         // true
            
            misturado;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_zero_and_negative_comparisons() {
        let result = execute_program("
            let zero = 0;
            let positivo = 5;
            let negativo = -3;
            
            let zeroMenor = zero < positivo;         // true
            let zeroMaior = zero > negativo;         // true
            let negativoMenor = negativo < zero;     // true
            let positivoMaior = positivo > zero;     // true
            
            positivoMaior;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_floating_point_comparisons() {
        let result = execute_program("
            let pi = 3.14159;
            let e = 2.71828;
            let approxPi = 3.14;
            
            let piMaior = pi > e;                    // true
            let piMenorAprox = pi < approxPi;        // false
            let eMenor = e < pi;                     // true
            let piDiferente = pi != approxPi;        // true
            
            piDiferente;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_comparison_precedence() {
        let result = execute_program("
            let x = 5;
            let y = 10;
            
            // Testing that comparison has lower precedence than arithmetic
            let resultado1 = x + 2 < y - 3;         // (5 + 2) < (10 - 3) => 7 < 7 => false
            let resultado2 = x * 2 == y;             // (5 * 2) == 10 => 10 == 10 => true
            let resultado3 = x + y > x * 2;          // (5 + 10) > (5 * 2) => 15 > 10 => true
            
            resultado3;
        ").unwrap();
        
        assert_eq!(result, "true");
    }

    #[test]
    fn test_comparison_edge_cases() {
        let result = execute_program("
            let x = 5;
            let y = 5;
            
            // Testing edge cases with equal values
            let igual = x == y;                      // true
            let menorIgual = x <= y;                 // true
            let maiorIgual = x >= y;                 // true
            let menor = x < y;                       // false
            let maior = x > y;                       // false
            let diferente = x != y;                  // false
            
            // All of: true, true, true, false, false, false
            // Return equal (first true)
            igual;
        ").unwrap();
        
        assert_eq!(result, "true");
    }
}
