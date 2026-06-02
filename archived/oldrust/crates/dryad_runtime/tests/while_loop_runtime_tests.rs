use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::Token};

fn execute_code(input: &str) -> dryad_runtime::Value {
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
    interpreter.execute_and_return_value(&program).unwrap()
}

#[test]
fn test_simple_while_true() {
    let input = r#"
    let contador = 0;
    while (contador < 3) {
        contador = contador + 1;
    }
    contador
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_simple_while_false() {
    let input = r#"
    let contador = 5;
    while (contador < 3) {
        contador = contador + 1;
    }
    contador
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(5.0)); // Loop não executa
}

#[test]
fn test_while_with_accumulator() {
    let input = r#"
    let i = 0;
    let soma = 0;
    while (i < 5) {
        soma = soma + i;
        i = i + 1;
    }
    soma
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(10.0)); // 0+1+2+3+4 = 10
}

#[test]
fn test_while_with_decreasing_counter() {
    let input = r#"
    let contador = 5;
    let resultado = 0;
    while (contador > 0) {
        resultado = resultado + contador;
        contador = contador - 1;
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(15.0)); // 5+4+3+2+1 = 15
}

#[test]
fn test_while_with_complex_condition() {
    let input = r#"
    let x = 0;
    let y = 10;
    while (x < 5 && y > 5) {
        x = x + 1;
        y = y - 1;
    }
    x + y
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(10.0)); // x=5, y=5, total=10
}

#[test]
fn test_while_boolean_condition() {
    let input = r#"
    let ativo = true;
    let contador = 0;
    while (ativo) {
        contador = contador + 1;
        if contador >= 3 {
            ativo = false;
        }
    }
    contador
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_nested_while_loops() {
    let input = r#"
    let outer = 0;
    let resultado = 0;
    while (outer < 3) {
        let inner = 0;
        while (inner < 2) {
            resultado = resultado + 1;
            inner = inner + 1;
        }
        outer = outer + 1;
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(6.0)); // 3 * 2 = 6
}

#[test]
fn test_while_with_if_inside() {
    let input = r#"
    let i = 0;
    let pares = 0;
    let impares = 0;
    while (i < 10) {
        if i % 2 == 0 {
            pares = pares + 1;
        } else {
            impares = impares + 1;
        }
        i = i + 1;
    }
    pares + impares
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(10.0)); // Todos os números de 0 a 9
}

#[test]
fn test_while_variable_scoping() {
    let input = r#"
    let externa = 10;
    let i = 0;
    while (i < 3) {
        let interna = externa + i;
        externa = externa + 1;
        i = i + 1;
    }
    externa
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(13.0)); // 10 + 3 iterações
}

#[test]
fn test_while_variable_shadowing() {
    let input = r#"
    let valor = 100;
    let i = 0;
    while (i < 2) {
        let valor = i;
        i = i + 1;
    }
    valor
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(100.0)); // Variável externa preservada
}

#[test]
fn test_while_zero_iterations() {
    let input = r#"
    let resultado = 42;
    while (false) {
        resultado = 0;
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(42.0)); // Loop nunca executa
}

#[test]
fn test_while_string_condition() {
    let input = r#"
    let texto = "ativo";
    let contador = 0;
    while (texto == "ativo") {
        contador = contador + 1;
        if contador >= 2 {
            texto = "inativo";
        }
    }
    contador
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(2.0));
}

#[test]
fn test_while_null_condition() {
    let input = r#"
    let valor = null;
    let executou = false;
    while (valor) {
        executou = true;
    }
    executou
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Bool(false)); // null é falsy
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    while (i < 5) {
        let temp = i;
        i = i + 1;
    }
    i
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(5.0));
}

#[test]
fn test_while_with_multiple_variables() {
    let input = r#"
    let a = 1;
    let b = 1;
    let c = 0;
    let count = 0;
    while (count < 5) {
        c = a + b;
        a = b;
        b = c;
        count = count + 1;
    }
    c
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(13.0)); // Sequência de Fibonacci
}

#[test]
fn test_while_with_floating_point() {
    let input = r#"
    let valor = 0.5;
    let contador = 0;
    while (valor < 2.0) {
        valor = valor + 0.5;
        contador = contador + 1;
    }
    contador
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(3.0)); // 0.5 -> 1.0 -> 1.5 -> 2.0
}

