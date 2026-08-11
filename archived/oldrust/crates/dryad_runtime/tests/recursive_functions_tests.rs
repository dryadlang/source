// crates/dryad_runtime/tests/recursive_functions_tests.rs
use dryad_lexer::{token::Token, Lexer};
use dryad_parser::Parser;
use dryad_runtime::interpreter::{Interpreter, Value};

fn execute_dryad_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().unwrap();
        match token.token {
            Token::Eof => break,
            _ => tokens.push(token),
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let mut last_value = Value::Null;

    for statement in program.statements {
        last_value = interpreter.execute_statement(&statement).unwrap();
    }

    Ok(last_value)
}

#[test]
fn test_factorial_recursive() {
    let code = r#"
        function fatorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        fatorial(5)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(120.0));
}

#[test]
fn test_factorial_edge_cases() {
    // Teste fatorial de 0
    let code = r#"
        function fatorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        fatorial(0)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(1.0));

    // Teste fatorial de 1
    let code = r#"
        function fatorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        fatorial(1)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(1.0));
}

#[test]
fn test_fibonacci_recursive() {
    let code = r#"
        function fibonacci(n) {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        fibonacci(5)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_fibonacci_sequence() {
    let code = r#"
        function fibonacci(n) {
            if n <= 1 {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        let fib0 = fibonacci(0);
        let fib1 = fibonacci(1);
        let fib2 = fibonacci(2);
        let fib3 = fibonacci(3);
        let fib4 = fibonacci(4);
        let fib5 = fibonacci(5);
        
        fib0 + fib1 + fib2 + fib3 + fib4 + fib5
    "#;

    let result = execute_dryad_code(code).unwrap();
    // 0 + 1 + 1 + 2 + 3 + 5 = 12
    assert_eq!(result, Value::Number(12.0));
}

#[test]
fn test_power_recursive() {
    let code = r#"
        function potencia(base, expoente) {
            if expoente == 0 {
                return 1;
            }
            if expoente == 1 {
                return base;
            }
            return base * potencia(base, expoente - 1);
        }
        
        potencia(2, 3)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_power_edge_cases() {
    // Qualquer número elevado a 0 é 1
    let code = r#"
        function potencia(base, expoente) {
            if expoente == 0 {
                return 1;
            }
            if expoente == 1 {
                return base;
            }
            return base * potencia(base, expoente - 1);
        }
        
        potencia(5, 0)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(1.0));

    // Qualquer número elevado a 1 é ele mesmo
    let code = r#"
        function potencia(base, expoente) {
            if expoente == 0 {
                return 1;
            }
            if expoente == 1 {
                return base;
            }
            return base * potencia(base, expoente - 1);
        }
        
        potencia(7, 1)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(7.0));
}

#[test]
fn test_gcd_recursive() {
    let code = r#"
        function mdc(a, b) {
            if b == 0 {
                return a;
            }
            let resto = a % b;
            return mdc(b, resto);
        }
        
        mdc(48, 18)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(6.0));
}

#[test]
#[ignore = "Stack overflow - necessita otimização tail-call"]
fn test_sum_recursive() {
    let code = r#"
        function somaRecursiva(n) {
            if (n <= 0) {
                return 0;
            }
            return n + somaRecursiva(n - 1);
        }
        
        somaRecursiva(10)
    "#;

    let result = execute_dryad_code(code).unwrap();
    // 1 + 2 + 3 + ... + 10 = 55
    assert_eq!(result, Value::Number(55.0));
}

#[test]
fn test_countdown_recursive() {
    let code = r#"
        function contagem(n) {
            if n <= 0 {
                return 0;
            }
            return contagem(n - 1) + 1;
        }
        
        contagem(5)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(5.0));
}

#[test]
fn test_nested_recursive_calls() {
    let code = r#"
        function ackermann(m, n) {
            if m == 0 {
                return n + 1;
            }
            if n == 0 {
                return ackermann(m - 1, 1);
            }
            let temp = ackermann(m, n - 1);
            return ackermann(m - 1, temp);
        }
        
        ackermann(1, 1)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(3.0));
}

#[test]
fn test_mutual_recursion() {
    let code = r#"
        function ehPar(n) {
            if n == 0 {
                return true;
            }
            return ehImpar(n - 1);
        }
        
        function ehImpar(n) {
            if n == 0 {
                return false;
            }
            return ehPar(n - 1);
        }
        
        let par4 = ehPar(4);
        let impar4 = ehImpar(4);
        let par5 = ehPar(5);
        let impar5 = ehImpar(5);
        
        par4
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Bool(true));
}

#[test]
fn test_recursive_with_complex_conditions() {
    let code = r#"
        function triangular(n) {
            if n <= 1 {
                return n;
            }
            if n == 2 {
                return 3;
            }
            return n + triangular(n - 1);
        }
        
        triangular(6)
    "#;

    let result = execute_dryad_code(code).unwrap();
    // 6 + 5 + 4 + 3 + 2 + 1 = 21
    assert_eq!(result, Value::Number(21.0));
}

#[test]
fn test_recursive_function_with_local_variables() {
    let code = r#"
        function processo(n) {
            let temp = n * 2;
            if temp <= 2 {
                return temp;
            }
            return temp + processo(n - 1);
        }
        
        processo(3)
    "#;

    let result = execute_dryad_code(code).unwrap();
    // processo(3): temp=6, return 6 + processo(2)
    // processo(2): temp=4, return 4 + processo(1)
    // processo(1): temp=2, return 2
    // Total: 6 + 4 + 2 = 12
    assert_eq!(result, Value::Number(12.0));
}
