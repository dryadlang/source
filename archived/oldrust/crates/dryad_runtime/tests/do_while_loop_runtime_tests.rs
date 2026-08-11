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

fn execute_code_with_interpreter(input: &str) -> Interpreter {
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
    interpreter.execute_and_return_value(&program).unwrap();
    interpreter
}

#[test]
fn test_simple_do_while_execution() {
    let input = r#"
    let i = 0;
    let sum = 0;
    do {
        sum = sum + i;
        i = i + 1;
    } while (i < 3);
    sum
    "#;
    
    let result = execute_code(input);
    // sum = 0 + 0 = 0, i = 1, 1 < 3 = true
    // sum = 0 + 1 = 1, i = 2, 2 < 3 = true  
    // sum = 1 + 2 = 3, i = 3, 3 < 3 = false
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_do_while_executes_at_least_once() {
    let input = r#"
    let count = 0;
    do {
        count = count + 1;
    } while (false);
    count
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(1.0));
}

#[test]
fn test_do_while_with_multiple_iterations() {
    let input = r#"
    let result = 1;
    let counter = 1;
    do {
        result = result * 2;
        counter = counter + 1;
    } while (counter <= 4);
    result
    "#;
    
    let output = execute_code(input);
    // result = 1 * 2 = 2, counter = 2, 2 <= 4 = true
    // result = 2 * 2 = 4, counter = 3, 3 <= 4 = true
    // result = 4 * 2 = 8, counter = 4, 4 <= 4 = true
    // result = 8 * 2 = 16, counter = 5, 5 <= 4 = false
    assert_eq!(output, dryad_runtime::Value::Number(16.0));
}

#[test]
fn test_do_while_with_complex_condition() {
    let input = r#"
    let x = 10;
    let y = 5;
    let iterations = 0;
    do {
        x = x - 1;
        y = y + 1;
        iterations = iterations + 1;
    } while (x > y && iterations < 10);
    iterations
    "#;
    
    let result = execute_code(input);
    // x=10, y=5: x=9, y=6, iter=1, 9>6 && 1<10 = true
    // x=9, y=6: x=8, y=7, iter=2, 8>7 && 2<10 = true  
    // x=8, y=7: x=7, y=8, iter=3, 7>8 && 3<10 = false
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_nested_do_while_loops() {
    let input = r#"
    let outer_count = 0;
    let total = 0;
    do {
        outer_count = outer_count + 1;
        let inner_count = 0;
        do {
            inner_count = inner_count + 1;
            total = total + 1;
        } while (inner_count < 2);
    } while (outer_count < 3);
    total
    "#;
    
    let result = execute_code(input);
    // 3 iterações externas * 2 iterações internas = 6 incrementos totais
    assert_eq!(result, dryad_runtime::Value::Number(6.0));
}

#[test]
fn test_do_while_with_break() {
    let input = r#"
    let i = 0;
    let sum = 0;
    do {
        sum = sum + i;
        i = i + 1;
        if i == 3 {
            break;
        }
    } while (i < 10);
    sum
    "#;
    
    let result = execute_code(input);
    // sum = 0 + 0 = 0, i = 1, 1 != 3
    // sum = 0 + 1 = 1, i = 2, 2 != 3
    // sum = 1 + 2 = 3, i = 3, 3 == 3 -> break
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_do_while_with_continue() {
    let input = r#"
    let i = 0;
    let sum = 0;
    do {
        i = i + 1;
        if i == 2 {
            continue;
        }
        sum = sum + i;
    } while (i < 4);
    sum
    "#;
    
    let result = execute_code(input);
    // i=1, 1!=2, sum = 0 + 1 = 1, 1 < 4 = true
    // i=2, 2==2, continue (não soma), 2 < 4 = true  
    // i=3, 3!=2, sum = 1 + 3 = 4, 3 < 4 = true
    // i=4, 4!=2, sum = 4 + 4 = 8, 4 < 4 = false
    assert_eq!(result, dryad_runtime::Value::Number(8.0));
}

#[test]
fn test_do_while_boolean_literal_condition() {
    let input = r#"
    let count = 0;
    do {
        count = count + 1;
    } while (true && count < 3);
    count
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_do_while_variable_condition() {
    let input = r#"
    let running = true;
    let count = 0;
    do {
        count = count + 1;
        if count >= 3 {
            running = false;
        }
    } while (running);
    count
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(3.0));
}

#[test]
fn test_do_while_scope_variables() {
    let input = r#"
    let outer_var = 10;
    let total = 0;
    do {
        let inner_var = outer_var * 2;
        total = total + inner_var;
        outer_var = outer_var - 1;
    } while (outer_var > 8);
    total
    "#;
    
    let result = execute_code(input);
    // outer_var=10: inner_var=20, total=20, outer_var=9, 9>8=true
    // outer_var=9: inner_var=18, total=38, outer_var=8, 8>8=false
    assert_eq!(result, dryad_runtime::Value::Number(38.0));
}

#[test]
fn test_do_while_with_if_else_inside() {
    let input = r#"
    let i = 1;
    let even_count = 0;
    let odd_count = 0;
    do {
        if i % 2 == 0 {
            even_count = even_count + 1;
        } else {
            odd_count = odd_count + 1;
        }
        i = i + 1;
    } while (i <= 5);
    even_count + odd_count
    "#;
    
    let result = execute_code(input);
    // i vai de 1 a 5: 1(ímpar), 2(par), 3(ímpar), 4(par), 5(ímpar)
    // even_count = 2 (2, 4), odd_count = 3 (1, 3, 5)
    assert_eq!(result, dryad_runtime::Value::Number(5.0));
}

#[test]
fn test_exact_syntax_md_example() {
    let input = r#"
    let i = 0;
    let sum = 0;
    do {
        sum = sum + i;
        i = i + 1;
    } while (i < 5);
    sum
    "#;
    
    let result = execute_code(input);
    // sum = 0+0=0, i=1, 1<5=true
    // sum = 0+1=1, i=2, 2<5=true
    // sum = 1+2=3, i=3, 3<5=true
    // sum = 3+3=6, i=4, 4<5=true
    // sum = 6+4=10, i=5, 5<5=false
    assert_eq!(result, dryad_runtime::Value::Number(10.0));
}

#[test]
fn test_do_while_empty_body() {
    let input = r#"
    let counter = 0;
    do {
    } while (counter < 0);
    counter
    "#;
    
    let result = execute_code(input);
    // Corpo vazio mas deve executar pelo menos uma vez
    assert_eq!(result, dryad_runtime::Value::Number(0.0));
}

#[test]
fn test_do_while_single_statement() {
    let input = r#"
    let value = 5;
    do {
        value = value * 2;
    } while (value < 50);
    value
    "#;
    
    let result = execute_code(input);
    // value = 5 * 2 = 10, 10 < 50 = true
    // value = 10 * 2 = 20, 20 < 50 = true  
    // value = 20 * 2 = 40, 40 < 50 = true
    // value = 40 * 2 = 80, 80 < 50 = false
    assert_eq!(result, dryad_runtime::Value::Number(80.0));
}

#[test]
fn test_verify_variables_with_interpreter() {
    let input = r#"
    let x = 10;
    let y = 5;
    do {
        x = x - 1;
        y = y + 1;
    } while (x > y);
    x
    "#;
    
    let interpreter = execute_code_with_interpreter(input);
    
    // Verifica valores das variáveis
    let x_value = interpreter.get_variable("x").unwrap();
    let y_value = interpreter.get_variable("y").unwrap();
    
    // x=10, y=5: x=9, y=6, 9>6=true
    // x=9, y=6: x=8, y=7, 8>7=true  
    // x=8, y=7: x=7, y=8, 7>8=false
    assert_eq!(x_value, dryad_runtime::Value::Number(7.0));
    assert_eq!(y_value, dryad_runtime::Value::Number(8.0));
}


