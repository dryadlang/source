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
    let mut last_value = dryad_runtime::Value::Null;
    
    for stmt in &program.statements {
        last_value = interpreter.execute_statement(stmt).unwrap();
    }
    
    last_value
}

#[test]
fn test_simple_for_loop_execution() {
    let input = r#"
    let count = 0;
    let i = 0;
    for (i = 1; i <= 3; i = i + 1) {
        count = count + i;
    }
    count
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 6.0).abs() < 0.0001); // 1 + 2 + 3 = 6
    } else {
        panic!("Resultado deveria ser um número, mas foi: {:?}", result);
    }
}

#[test]
fn test_for_loop_with_empty_init() {
    let input = r#"
    let i = 0;
    let sum = 0;
    for (; i < 3; i = i + 1) {
        sum = sum + 1;
    }
    sum
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 3.0).abs() < 0.0001);
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_with_variable_condition() {
    let input = r#"
    let running = true;
    let counter = 0;
    let i = 0;
    for (i = 0; running; i = i + 1) {
        counter = counter + 1;
        if i >= 2 {
            running = false;
        }
    }
    counter
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 3.0).abs() < 0.0001); // Loop executa 3 vezes (i=0,1,2)
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_with_break() {
    let input = r#"
    let total = 0;
    let i = 0;
    for (i = 0; i < 10; i = i + 1) {
        if i == 3 {
            break;
        }
        total = total + i;
    }
    total
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 3.0).abs() < 0.0001); // 0 + 1 + 2 = 3 (para antes de i=3)
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_with_continue() {
    let input = r#"
    let total = 0;
    let i = 0;
    for (i = 0; i < 5; i = i + 1) {
        if i == 2 {
            continue;
        }
        total = total + i;
    }
    total
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 8.0).abs() < 0.0001); // 0 + 1 + 3 + 4 = 8 (pula i=2)
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_nested_for_loops() {
    let input = r#"
    let result = 0;
    let i = 0;
    let j = 0;
    for (i = 1; i <= 2; i = i + 1) {
        for (j = 1; j <= 2; j = j + 1) {
            result = result + (i * j);
        }
    }
    result
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        // i=1: j=1 -> 1*1=1, j=2 -> 1*2=2
        // i=2: j=1 -> 2*1=2, j=2 -> 2*2=4
        // Total: 1 + 2 + 2 + 4 = 9
        assert!((value - 9.0).abs() < 0.0001);
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_empty_body() {
    let input = r#"
    let count = 0;
    let i = 0;
    for (i = 0; i < 3; i = i + 1) {
        // Corpo vazio - lógica simples
    }
    i
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 3.0).abs() < 0.0001); // i termina como 3 (primeira vez que i < 3 é falso)
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_all_empty_components() {
    let input = r#"
    let x = 0;
    for (; ;) {
        x = x + 1;
        if x >= 3 {
            break;
        }
    }
    x
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        assert!((value - 3.0).abs() < 0.0001);
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_variable_scope() {
    let input = r#"
    let outer = 1;
    let i = 0;
    for (i = 0; i < 2; i = i + 1) {
        let inner = 10;
        outer = outer + inner;
    }
    outer
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        // outer inicia com 1
        // Loop executa 2 vezes, cada vez adiciona 10
        // Resultado: 1 + 10 + 10 = 21
        assert!((value - 21.0).abs() < 0.0001);
    } else {
        panic!("Resultado deveria ser um número");
    }
}

#[test]
fn test_for_loop_complex_condition() {
    let input = r#"
    let total = 0;
    let active = true;
    let count = 0;
    for (count = 1; count <= 5 && active; count = count + 1) {
        total = total + count;
        if count == 3 {
            active = false;
        }
    }
    total
    "#;
    
    let result = execute_code(input);
    if let dryad_runtime::Value::Number(value) = result {
        // count=1: total=1, count=2: total=3, count=3: total=6, active=false
        // count=4: condição falha (active é false)
        assert!((value - 6.0).abs() < 0.0001);
    } else {
        panic!("Resultado deveria ser um número");
    }
}

