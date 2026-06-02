# Guia de Integração do Bytecode VM

## Resumo

O modo bytecode foi integrado ao runtime do Dryad. Quando a flag `--compile` é usada na CLI, o código é compilado para bytecode e executado pela VM baseada em pilha.

## Como Usar

### Via CLI

```bash
# Modo normal (interpretador AST)
dryad run script.dryad

# Modo bytecode
dryad run script.dryad --compile

# Debug de bytecode
DRYAD_DEBUG_BYTECODE=1 dryad run script.dryad --compile

# Debug da VM (mostra execução passo a passo)
DRYAD_DEBUG_VM=1 dryad run script.dryad --compile
```

### Programaticamente

```rust
use dryad_runtime::Interpreter;
use dryad_parser::Parser;

let mut interpreter = Interpreter::new();
interpreter.set_compile_mode(true);

let program = Parser::new(tokens).parse().unwrap();
let result = interpreter.execute(&program);
```

## O que Funciona ✅

### Expressões
- [x] Literais: números, strings, booleanos, nil
- [x] Variáveis: leitura e escrita
- [x] Operações aritméticas: +, -, *, /, %
- [x] Operações de comparação: ==, !=, <, >, <=, >=
- [x] Operações lógicas: &&, ||, !
- [x] Operações bitwise: &, |, ^, ~, <<, >>
- [x] Expressões unárias: -, !, ~
- [x] Arrays (criação e indexação - parcial)
- [x] Tuples (criação e acesso - parcial)

### Statements
- [x] Expressões
- [x] Declaração de variáveis (let, const)
- [x] Atribuições
- [x] Blocos (escopos)
- [x] If/else
- [x] While
- [x] Do-while
- [x] For tradicional
- [x] Declaração de funções
- [x] Return
- [x] Print

### Funcionalidades de Funções
- [x] Declaração de funções com parâmetros
- [x] Chamadas de função
- [x] Return de valores
- [x] Variáveis locais em funções
- [x] Recursão
- [x] Verificação de aridade
- [ ] Closures (upvalues) - parcial

## Funcionalidades Implementadas ✅

### Alto Prioridade
- [x] Funções (declaração e chamada)
- [x] Return de valores
- [x] Parâmetros e argumentos
- [x] Escopo de função
- [x] Variáveis locais em funções

## Em Desenvolvimento 🚧

### Alto Prioridade
- [ ] Closures (upvalues)
- [ ] Classes e objetos
- [ ] Propriedades e métodos
- [ ] Construtores

### Média Prioridade
- [ ] ForEach
- [ ] Try/catch/throw
- [ ] Break/Continue (opcode existe, mas não implementado na VM)
- [ ] Incremento/decremento (++/--)

### Baixa Prioridade (JIT)
- [ ] Compilação JIT (não é prioridade)

## Limitações Atuais

1. **Classes**: Suporte parcial aos opcodes, mas não funcional
2. **Closures**: Upvalues implementados nos opcodes, mas não funcionais na VM
3. **Exceções**: Try/catch não implementado
4. **Módulos**: Import/use não suportado no bytecode
5. **Nativas**: Funções nativas precisam de adaptação

## Arquitetura de Integração

```
┌─────────────────────────────────────────────────────────┐
│                    CLI (dryad_cli)                       │
│                    Flag: --compile                       │
└────────────────────┬────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Interpreter (dryad_runtime)                 │
│  ┌─────────────────┐    ┌─────────────────────────────┐ │
│  │  compile_mode   │───►│  execute_bytecode()         │ │
│  │  = false        │    │                             │ │
│  └─────────────────┘    │  1. Compila AST → Bytecode  │ │
│                         │  2. Executa na VM           │ │
│  ┌─────────────────┐    └─────────────────────────────┘ │
│  │  Modo AST       │                                    │
│  │  (interpretador)│                                    │
│  │  tradicional    │                                    │ │
│  └─────────────────┘                                    │ │
└─────────────────────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────┐
│              Bytecode Compiler & VM                      │
│                   (dryad_bytecode)                       │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │  Compiler    │  │    Chunk     │  │     VM       │  │
│  │  AST →       │──►│  Bytecode    │──►│  Executor    │  │
│  │  Bytecode    │  │              │  │              │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
└─────────────────────────────────────────────────────────┘
```

## Métricas de Performance (Estimativas)

| Cenário | Interpretador AST | Bytecode VM | Ganho |
|---------|------------------|-------------|-------|
| Aritmética em loop | 100% | ~35% | ~2.8x |
| Variáveis e escopo | 100% | ~40% | ~2.5x |
| Controle de fluxo | 100% | ~45% | ~2.2x |
| Inicialização | 100% | ~85% | ~1.2x |

*Nota: Métricas baseadas em implementações similares (Lua, Python)*

## Debugging

### Ver bytecode gerado
```bash
DRYAD_DEBUG_BYTECODE=1 dryad run script.dryad --compile
```

Saída esperada:
```
=== BYTECODE ===
== script ==
Constants:
  [   0] '10'
  [   1] '20'

Bytecode:
0000    1 CONSTANT       0 '10'
0002    1 DEFINE_GLOBAL  2 'x'
...
```

### Ver execução passo a passo
```bash
DRYAD_DEBUG_VM=1 dryad run script.dryad --compile
```

Saída esperada:
```
          [ 10 ]
0000    1 CONSTANT 0
          [ 10 ][ 20 ]
0002    1 CONSTANT 1
...
```

## Próximos Passos Recomendados

1. **Implementar funções** - Prioridade máxima para tornar o bytecode útil
2. **Suite de testes** - Criar testes comparando AST vs bytecode
3. **Benchmarks** - Medir performance real em cenários reais
4. **Otimizações** - Constant folding, inline, peephole
5. **Serialização** - Salvar/carregar bytecode compilado

## Código de Teste

Arquivo: `test_bytecode.dryad`

```dryad
# Teste básico do bytecode
print "Iniciando teste...";

# Aritmética
let a = 10;
let b = 20;
print a + b;

# Condicional
if (a < b) {
    print "a é menor que b";
}

# Loop
let i = 0;
while (i < 3) {
    print i;
    i = i + 1;
}

print "Teste concluído!";
```

Execute:
```bash
dryad run test_bytecode.dryad --compile
```
