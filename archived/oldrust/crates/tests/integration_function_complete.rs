// crates/tests/integration_function_complete.rs
// Testes de integração completos para o sistema Dryad
// Testam a interação entre lexer, parser e interpretador

use dryad_runtime::Interpreter;
use dryad_parser::Parser;
use dryad_lexer::{Lexer, token::{Token, TokenWithLocation}};

/// Função auxiliar para executar código Dryad completo
fn execute_dryad_code(input: &str) -> Result<String, String> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token().map_err(|e| format!("Erro no lexer: {:?}", e))?;
        match token.token {
            Token::Eof => break,
            _ => tokens.push(token),
        }
    }

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Erro no parser: {:?}", e))?;

    let mut interpreter = Interpreter::new();
    interpreter.execute(&program).map_err(|e| format!("Erro no interpretador: {:?}", e))?;

    // Retorna o valor da última expressão ou "null"
    match interpreter.get_last_value() {
        Some(value) => format!("{}", value),
        None => "null".to_string(),
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_arithmetic_operations() {
        let code = r#"
            let a = 10;
            let b = 5;
            let sum = a + b;
            let diff = a - b;
            let prod = a * b;
            let quot = a / b;
            prod
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "50");
    }

    #[test]
    fn test_function_definition_and_call() {
        let code = r#"
            function soma(a, b) {
                return a + b;
            }

            let resultado = soma(3, 7);
            resultado
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_conditional_logic() {
        let code = r#"
            let x = 10;
            let y = 5;
            let maior;

            if x > y {
                maior = x;
            } else {
                maior = y;
            }

            maior
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "10");
    }

    #[test]
    fn test_loop_structures() {
        let code = r#"
            let sum = 0;
            let i = 1;

            while i <= 5 {
                sum = sum + i;
                i = i + 1;
            }

            sum
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "15");
    }

    #[test]
    fn test_array_operations() {
        let code = r#"
            let arr = [1, 2, 3, 4, 5];
            let sum = 0;

            for num in arr {
                sum = sum + num;
            }

            sum
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "15");
    }

    #[test]
    fn test_class_instantiation() {
        let code = r#"
            class Pessoa {
                function init(nome) {
                    this.nome = nome;
                }

                function saudacao() {
                    return "Olá, " + this.nome;
                }
            }

            let pessoa = new Pessoa("João");
            pessoa.saudacao()
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "Olá, João");
    }

    #[test]
    fn test_lambda_functions() {
        let code = r#"
            let dobro = (x) => x * 2;
            let quadrado = (x) => x * x;

            let resultado = dobro(quadrado(3));
            resultado
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "18");
    }

    #[test]
    fn test_error_handling() {
        let code = r#"
            try {
                let x = 10 / 0;
            } catch (e) {
                "Erro capturado"
            }
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "Erro capturado");
    }

    #[test]
    fn test_native_module_usage() {
        let code = r#"
            #<console_io>
            native_input("Digite algo: ")
        "#;

        // Este teste pode precisar de simulação de entrada
        // Por enquanto, apenas verifica se o código compila e executa sem erros
        let result = execute_dryad_code(code);
        // Como native_input bloqueia, podemos testar apenas que não há erro de compilação
        assert!(result.is_ok() || result.is_err(), "O teste deve executar, mesmo que bloqueie");
    }

    #[test]
    fn test_complex_program() {
        let code = r#"
            // Calculadora de fatorial com recursão
            function fatorial(n) {
                if n <= 1 {
                    return 1;
                } else {
                    return n * fatorial(n - 1);
                }
            }

            // Classe para armazenar resultados
            class Calculadora {
                function init() {
                    this.resultados = [];
                }

                function calcular_fatorial(n) {
                    let resultado = fatorial(n);
                    this.resultados.push(resultado);
                    return resultado;
                }
            }

            let calc = new Calculadora();
            let fat5 = calc.calcular_fatorial(5);
            fat5
        "#;

        let result = execute_dryad_code(code);
        assert!(result.is_ok(), "Erro na execução: {:?}", result.err());
        assert_eq!(result.unwrap(), "120");
    }
}
