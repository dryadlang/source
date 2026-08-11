// crates/dryad_runtime/tests/functions_as_values_tests.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};

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
        match interpreter.execute_statement(&statement) {
            Ok(value) => last_value = value,
            Err(err) => return Err(err),
        }
    }
    
    Ok(last_value)
}

#[test]
fn test_function_assignment_basic() {
    let code = r#"
        function quadrado(x) {
            return x * x;
        }
        
        let funcao = quadrado;
        funcao(4)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(16.0));
}

#[test]
fn test_function_assignment_and_call() {
    let code = r#"
        function somar(a, b) {
            return a + b;
        }
        
        let minhaFuncao = somar;
        let resultado = minhaFuncao(10, 20);
        resultado
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(30.0));
}

#[test]
fn test_function_reassignment() {
    let code = r#"
        function multiplicar(a, b) {
            return a * b;
        }
        
        function dividir(a, b) {
            return a / b;
        }
        
        let operacao = multiplicar;
        let resultado1 = operacao(6, 4);
        
        operacao = dividir;
        let resultado2 = operacao(12, 3);
        
        resultado1 + resultado2
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(28.0)); // 24 + 4
}

#[test]
fn test_multiple_function_variables() {
    let code = r#"
        function dobrar(x) {
            return x * 2;
        }
        
        function triplicar(x) {
            return x * 3;
        }
        
        let op1 = dobrar;
        let op2 = triplicar;
        
        let a = op1(5);  // 10
        let b = op2(4);  // 12
        
        a + b
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(22.0));
}

#[test]
fn test_function_as_value_with_local_variables() {
    let code = r#"
        function calcular(base) {
            let fator = 3;
            return base * fator + 1;
        }
        
        let minhaConta = calcular;
        minhaConta(5)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(16.0)); // 5 * 3 + 1
}

#[test]
fn test_function_variable_with_conditionals() {
    let code = r#"
        function maiorQue(a, b) {
            if a > b {
                return a;
            } else {
                return b;
            }
        }
        
        let comparar = maiorQue;
        comparar(8, 3)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_function_variable_recursive() {
    let code = r#"
        function fatorial(n) {
            if n <= 1 {
                return 1;
            }
            return n * fatorial(n - 1);
        }
        
        let calcFatorial = fatorial;
        calcFatorial(4)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(24.0));
}

#[test]
fn test_function_variable_with_string_operations() {
    let code = r#"
        function cumprimentar(nome) {
            return "Olá, " + nome + "!";
        }
        
        let saudar = cumprimentar;
        saudar("Maria")
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("Olá, Maria!".to_string()));
}

#[test]
fn test_function_variable_no_parameters() {
    let code = r#"
        function obterPi() {
            return 3.14159;
        }
        
        let pi = obterPi;
        pi()
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(3.14159));
}

#[test]
fn test_function_variable_multiple_calls() {
    let code = r#"
        function incrementar(x) {
            return x + 1;
        }
        
        let inc = incrementar;
        let a = inc(5);
        let b = inc(a);
        let c = inc(b);
        
        c
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0)); // 5 + 1 + 1 + 1
}

#[test]
fn test_function_variable_with_complex_expressions() {
    let code = r#"
        function calcular(x, y) {
            return (x + y) * 2 - 1;
        }
        
        let formula = calcular;
        formula(3 + 2, 4 * 2)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(25.0)); // (5 + 8) * 2 - 1 = 25
}

#[test]
fn test_function_assignment_in_block() {
    let code = r#"
        function potencia(base, exp) {
            return base ** exp;
        }
        
        {
            let f = potencia;
            let resultado = f(2, 3);
            resultado
        }
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(8.0));
}

#[test]
fn test_function_variable_scope() {
    let code = r#"
        function externa() {
            let valor = 10;
            return valor * 2;
        }
        
        let funcao = externa;
        {
            let valor = 5;  // Esta variável não deve afetar a função
            funcao()
        }
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(20.0));
}

#[test]
fn test_error_calling_non_function_variable() {
    let code = r#"
        let naoFuncao = 42;
        naoFuncao(5)
    "#;
    
    let result = execute_dryad_code(code);
    assert!(result.is_err());
    
    let error = result.unwrap_err();
    assert_eq!(error.code(), 3003);
    assert!(error.message().contains("não é uma função"));
}

#[test]
fn test_function_with_return_early() {
    let code = r#"
        function testar(x) {
            if x > 10 {
                return "grande";
            }
            return "pequeno";
        }
        
        let teste = testar;
        teste(15)
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::String("grande".to_string()));
}
