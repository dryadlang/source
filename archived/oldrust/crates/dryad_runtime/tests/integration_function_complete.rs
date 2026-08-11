// crates/tests/integration_function_complete.rs
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
    interpreter.execute_and_return_value(&program)
}

#[test]
fn test_integration_complete_function_system() {
    let code = r#"
        function saudacao(nome) {
            return "Olá, " + nome + "!";
        }
        
        function dobrar(x) {
            return x * 2;
        }
        
        function calcularCompleto(base, multiplicador) {
            let temp = dobrar(base);
            let resultado = temp + multiplicador;
            return resultado;
        }
        
        let nome = "Dryad";
        let msg = saudacao(nome);
        let valor = calcularCompleto(10, 5);
        
        valor
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(25.0)); // dobrar(10) = 20, 20 + 5 = 25
}

#[test]
#[ignore = "Stack overflow on Windows due to limited stack size"]
fn test_integration_recursive_fibonacci() {
    let code = r#"
        function fibonacci(n) {
            if (n <= 1) {
                return n;
            }
            return fibonacci(n - 1) + fibonacci(n - 2);
        }
        
        fibonacci(6)
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0)); // fibonacci(6) = 8
}

#[test]
fn test_integration_function_with_control_flow() {
    let code = r#"
        function categoria(idade) {
            if (idade < 13) {
                return "criança";
            }
            if (idade < 18) {
                return "adolescente";
            }
            if (idade < 60) {
                return "adulto";
            }
            return "idoso";
        }
        
        let idade1 = categoria(10);
        let idade2 = categoria(25);
        let idade3 = categoria(70);
        
        idade2
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("adulto".to_string()));
}

#[test]
fn test_integration_functions_with_scoping() {
    let code = r#"
        let global = 100;
        
        function testeEscopo() {
            let local = 50;
            
            function interna() {
                return local + 10;
            }
            
            return interna();
        }
        
        global + testeEscopo()
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(160.0)); // 100 + (50 + 10)
}

#[test]
fn test_integration_function_as_calculator() {
    let code = r#"
        function potencia(base, exp) {
            if (exp == 0) {
                return 1;
            }
            if (exp == 1) {
                return base;
            }
            return base * potencia(base, exp - 1);
        }
        
        function fatorial(n) {
            if (n <= 1) {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        let resultado = potencia(2, 4) + fatorial(4);
        resultado
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(40.0)); // 2^4 + 4! = 16 + 24 = 40
}

#[test]
fn test_integration_function_parameters_expressions() {
    let code = r#"
        function somar(a, b) {
            return a + b;
        }
        
        function multiplicar(x, y) {
            return x * y;
        }
        
        let resultado = somar(multiplicar(3, 4), somar(2, 3));
        resultado
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(17.0)); // (3*4) + (2+3) = 12 + 5 = 17
}

#[test]
fn test_integration_function_early_returns() {
    let code = r#"
        function buscarPrimeiroPar(lista) {
            let i = 0;
            while (i < 5) {
                if lista[i] % 2 == 0 {
                    return lista[i];
                }
                i = i + 1;
            }
            return -1;
        }
        
        // Simular busca sem arrays reais
        function simularBusca() {
            if (1 % 2 == 0) { return 1; }
            if (3 % 2 == 0) { return 3; }
            if (4 % 2 == 0) { return 4; }
            return -1;
        }
        
        simularBusca()
    "#;

    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(4.0));
}
