// crates/dryad_runtime/tests/integration_functions_as_values.rs
use dryad_runtime::interpreter::{Interpreter, Value};
use dryad_parser::Parser;
use dryad_lexer::{Lexer, Token};

fn execute_dryad_code(input: &str) -> Result<Value, dryad_errors::DryadError> {
    let mut lexer = Lexer::new(input);
    let mut tokens = Vec::new();
    
    loop {
        match lexer.next_token().unwrap() {
            token if token.token == Token::Eof => break,
            token => tokens.push(token),
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
fn test_integration_function_calculator() {
    let code = r#"
        // Definindo operações matemáticas
        function somar(a, b) {
            return a + b;
        }
        
        function multiplicar(a, b) {
            return a * b;
        }
        
        function elevarAoQuadrado(x) {
            return x * x;
        }
        
        // Atribuindo funções a variáveis
        let operacao1 = somar;
        let operacao2 = multiplicar;
        let potencia = elevarAoQuadrado;
        
        // Usando as funções através das variáveis
        let resultado1 = operacao1(5, 3);        // 8
        let resultado2 = operacao2(4, 6);        // 24
        let resultado3 = potencia(resultado1);   // 64
        
        // Combinando resultados
        operacao1(resultado2, resultado3)        // 24 + 64 = 88
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(88.0));
}

#[test]
fn test_integration_function_selection_system() {
    let code = r#"
        // Sistema de seleção de operações
        function dobrar(x) {
            return x * 2;
        }
        
        function triplicar(x) {
            return x * 3;
        }
        
        function elevarAoCubo(x) {
            return x * x * x;
        }
        
        // Seletor baseado em condições
        let numero = 4;
        let operacao;
        
        if numero < 3 {
            operacao = dobrar;
        } else if numero < 6 {
            operacao = triplicar;
        } else {
            operacao = elevarAoCubo;
        }
        
        operacao(numero)  // numero = 4, então usa triplicar: 4 * 3 = 12
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(12.0));
}

#[test]
fn test_integration_function_chain_operations() {
    let code = r#"
        // Funções para encadeamento
        function incrementar(x) {
            return x + 1;
        }
        
        function dobrarEAdicionar(x) {
            return x * 2 + 5;
        }
        
        function calcularPercentual(x) {
            return x * 0.1;  // 10%
        }
        
        // Encadeamento de operações usando variáveis
        let passo1 = incrementar;
        let passo2 = dobrarEAdicionar;
        let passo3 = calcularPercentual;
        
        let valor = 10;
        let resultado = passo1(valor);         // 11
        resultado = passo2(resultado);        // 27 (11 * 2 + 5)
        resultado = passo3(resultado);        // 2.7 (27 * 0.1)
        
        resultado
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(2.7));
}

#[test]
fn test_integration_recursive_function_as_value() {
    let code = r#"
        // Função recursiva para calcular potência
        function potencia(base, expoente) {
            if expoente == 0 {
                return 1;
            }
            if expoente == 1 {
                return base;
            }
            return base * potencia(base, expoente - 1);
        }
        
        // Atribuindo função recursiva a variável
        let calcPotencia = potencia;
        
        // Testando diferentes casos
        let resultado1 = calcPotencia(2, 3);  // 8
        let resultado2 = calcPotencia(3, 2);  // 9
        let resultado3 = calcPotencia(5, 0);  // 1
        
        resultado1 + resultado2 + resultado3  // 8 + 9 + 1 = 18
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(18.0));
}

#[test]
fn test_integration_complex_function_composition() {
    let code = r#"
        // Sistema de processamento de dados
        function validarPositivo(x) {
            if x > 0 {
                return x;
            }
            return 0;
        }
        
        function aplicarDesconto(valor) {
            return valor * 0.9;  // 10% de desconto
        }
        
        function adicionarTaxa(valor) {
            return valor + 5;  // Taxa fixa de 5
        }
        
        function arredondar(valor) {
            // Simular arredondamento simples
            if valor > 54.0 {
                return 55;
            }
            return 54;
        }
        
        // Pipeline de processamento
        let validar = validarPositivo;
        let desconto = aplicarDesconto;
        let taxa = adicionarTaxa;
        let finalizar = arredondar;
        
        let preco = 55.0;
        
        // Aplicando pipeline: validar -> desconto -> taxa -> arredondar
        let resultado = validar(preco);       // 55.0
        resultado = desconto(resultado);      // 49.5
        resultado = taxa(resultado);          // 54.5
        resultado = finalizar(resultado);     // 55.0 (baseado na lógica de arredondamento)
        
        resultado
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(55.0));
}

#[test]
fn test_integration_function_array_simulation() {
    let code = r#"
        // Simulando um "array" de funções usando variáveis
        function operacao1(x) {
            return x + 10;
        }
        
        function operacao2(x) {
            return x * 2;
        }
        
        function operacao3(x) {
            return x - 5;
        }
        
        // "Array" de funções simulado
        let funcoes1 = operacao1;
        let funcoes2 = operacao2;
        let funcoes3 = operacao3;
        
        let valor = 8;
        
        // Aplicando todas as operações sequencialmente
        let resultado1 = funcoes1(valor);     // 18
        let resultado2 = funcoes2(valor);     // 16
        let resultado3 = funcoes3(valor);     // 3
        
        // Combinando todos os resultados
        resultado1 + resultado2 + resultado3  // 18 + 16 + 3 = 37
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(37.0));
}

#[test]
fn test_integration_conditional_function_assignment() {
    let code = r#"
        // Diferentes estratégias de cálculo
        function calculoSimples(x) {
            return x * 1.1;
        }
        
        function calculoComplexo(x) {
            return x * 1.5 + 10;
        }
        
        function calculoEspecial(x) {
            return x * x / 2;
        }
        
        // Seleção dinâmica de estratégia
        let estrategia;
        let tipo = 2;  // Simulando entrada
        
        if tipo == 1 {
            estrategia = calculoSimples;
        } else if tipo == 2 {
            estrategia = calculoComplexo;
        } else {
            estrategia = calculoEspecial;
        }
        
        // Usando a estratégia selecionada
        let base = 20;
        let resultado = estrategia(base);
        
        resultado  // 20 * 1.5 + 10 = 40
    "#;
    
    let result = execute_dryad_code(code).unwrap();
    assert_eq!(result, Value::Number(40.0));
}
