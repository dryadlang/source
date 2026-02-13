use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn execute_code(input: &str) -> dryad_runtime::Value {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        let token = lexer.next_token().unwrap();
        if token.token == Token::Eof {
            break;
        }
        tokens.push(token);
    }
    
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    let mut interpreter = Interpreter::new();
    interpreter.execute_and_return_value(&program).unwrap()
}

#[test]
fn test_simple_if_true() {
    let input = r#"
    let resultado = "nao_alterado";
    if true {
        resultado = "alterado";
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("alterado".to_string()));
}

#[test]
fn test_simple_if_false() {
    let input = r#"
    let resultado = "nao_alterado";
    if false {
        resultado = "alterado";
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("nao_alterado".to_string()));
}

#[test]
fn test_if_with_condition() {
    let input = r#"
    let idade = 20;
    let status = "menor";
    if idade >= 18 {
        status = "maior";
    }
    status
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("maior".to_string()));
}

#[test]
fn test_if_with_condition_false() {
    let input = r#"
    let idade = 16;
    let status = "menor";
    if idade >= 18 {
        status = "maior";
    }
    status
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("menor".to_string()));
}

#[test]
fn test_if_else_true_branch() {
    let input = r#"
    let nota = 8.5;
    let resultado = "";
    if nota >= 7.0 {
        resultado = "aprovado";
    } else {
        resultado = "reprovado";
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("aprovado".to_string()));
}

#[test]
fn test_if_else_false_branch() {
    let input = r#"
    let nota = 5.5;
    let resultado = "";
    if nota >= 7.0 {
        resultado = "aprovado";
    } else {
        resultado = "reprovado";
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("reprovado".to_string()));
}

#[test]
fn test_nested_if_else_chain() {
    let input = r#"
    let pontuacao = 85;
    let classificacao = "";
    if pontuacao >= 90 {
        classificacao = "excelente";
    } else if pontuacao >= 80 {
        classificacao = "bom";
    } else if pontuacao >= 70 {
        classificacao = "regular";
    } else {
        classificacao = "insuficiente";
    }
    classificacao
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("bom".to_string()));
}

#[test]
fn test_nested_if_else_chain_different_branches() {
    // Teste da primeira condição
    let input1 = r#"
    let pontuacao = 95;
    let classificacao = "";
    if pontuacao >= 90 {
        classificacao = "excelente";
    } else if pontuacao >= 80 {
        classificacao = "bom";
    } else {
        classificacao = "regular";
    }
    classificacao
    "#;
    let result1 = execute_code(input1);
    assert_eq!(result1, dryad_runtime::Value::String("excelente".to_string()));
    
    // Teste da última condição
    let input2 = r#"
    let pontuacao = 60;
    let classificacao = "";
    if pontuacao >= 90 {
        classificacao = "excelente";
    } else if pontuacao >= 80 {
        classificacao = "bom";
    } else {
        classificacao = "regular";
    }
    classificacao
    "#;
    let result2 = execute_code(input2);
    assert_eq!(result2, dryad_runtime::Value::String("regular".to_string()));
}

#[test]
fn test_nested_if_statements() {
    let input = r#"
    let x = 5;
    let y = 3;
    let resultado = "nenhum";
    if x > 0 {
        if y > 0 {
            resultado = "ambos_positivos";
        }
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("ambos_positivos".to_string()));
}

#[test]
fn test_nested_if_statements_false() {
    let input = r#"
    let x = 5;
    let y = -3;
    let resultado = "nenhum";
    if x > 0 {
        if y > 0 {
            resultado = "ambos_positivos";
        }
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("nenhum".to_string()));
}

#[test]
fn test_if_with_complex_condition() {
    let input = r#"
    let idade = 25;
    let ativo = true;
    let elegivel = false;
    if idade >= 18 && ativo == true {
        elegivel = true;
    }
    elegivel
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Bool(true));
}

#[test]
fn test_if_with_complex_condition_false() {
    let input = r#"
    let idade = 25;
    let ativo = false;
    let elegivel = false;
    if idade >= 18 && ativo == true {
        elegivel = true;
    }
    elegivel
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Bool(false));
}

#[test]
fn test_if_with_multiple_statements() {
    let input = r#"
    let valor = 150;
    let total = 100;
    let bonus = 0;
    let aplicado = false;
    
    if valor > 100 {
        bonus = valor * 0.1;
        total = total + bonus;
        aplicado = true;
    }
    
    total
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(115.0)); // 100 + (150 * 0.1)
}

#[test]
fn test_if_scoping() {
    let input = r#"
    let x = 10;
    if true {
        let y = 20;
        x = x + y;
    }
    x
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(30.0));
}

#[test]
fn test_if_variable_shadowing() {
    let input = r#"
    let resultado = "externo";
    if true {
        let resultado = "interno";
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::String("externo".to_string()));
}

#[test]
fn test_if_else_scoping() {
    let input = r#"
    let escolha = true;
    let resultado = 0;
    if escolha {
        let temp = 10;
        resultado = temp * 2;
    } else {
        let temp = 5;
        resultado = temp * 3;
    }
    resultado
    "#;
    
    let result = execute_code(input);
    assert_eq!(result, dryad_runtime::Value::Number(20.0));
}

#[test]
fn test_syntax_md_examples() {
    // If simples
    let input1 = r#"
    let idade = 18;
    let status = "menor";
    if idade >= 18 {
        status = "maior_de_idade";
    }
    status
    "#;
    let result1 = execute_code(input1);
    assert_eq!(result1, dryad_runtime::Value::String("maior_de_idade".to_string()));
    
    // If-else
    let input2 = r#"
    let nota = 7.5;
    let resultado = "";
    if nota >= 7.0 {
        resultado = "aprovado";
    } else {
        resultado = "reprovado";
    }
    resultado
    "#;
    let result2 = execute_code(input2);
    assert_eq!(result2, dryad_runtime::Value::String("aprovado".to_string()));
    
    // If-else encadeado
    let input3 = r#"
    let pontuacao = 85;
    let classificacao = "";
    if pontuacao >= 90 {
        classificacao = "excelente";
    } else if pontuacao >= 80 {
        classificacao = "bom";
    } else if pontuacao >= 70 {
        classificacao = "regular";
    } else {
        classificacao = "insuficiente";
    }
    classificacao
    "#;
    let result3 = execute_code(input3);
    assert_eq!(result3, dryad_runtime::Value::String("bom".to_string()));
}
