# AST Optimizer

Módulo de otimização de AST (Abstract Syntax Tree) para o interpretador Dryad.

## Visão Geral

O otimizador de AST executa otimizações em tempo de compilação para melhorar o desempenho do código gerado.

## Otimizações Implementadas

### Constant Folding

O Constant Folding avalia expressões constantes em tempo de compilação, substituindo-as por seu valor resultante.

**Exemplos:**
- `2 + 2` → `4`
- `"Hello" + " World"` → `"Hello World"`
- `10 > 5` → `true`
- `!false` → `true`

### Short-Circuit Evaluation

Otimização de expressões booleanas que podem ser avaliadas sem executar todas as condições:

- `false && x` → `false`
- `true || x` → `true`
- `true && x` → `x`
- `false || x` → `x`

### Operadores Suportados

**Aritméticos:**
- `+` (adição)
- `-` (subtração)
- `*` (multiplicação)
- `/` (divisão)
- `%` (módulo)

**Comparação:**
- `==` (igual)
- `!=` (diferente)
- `<` (menor)
- `<=` (menor ou igual)
- `>` (maior)
- `>=` (maior ou igual)

**Lógicos:**
- `&&` (and)
- `||` (or)
- `!` (not)

**Unário:**
- `-n` (negação)
- `!b` (not booleano)

## Uso

```rust
use dryad_parser::{Parser, AstOptimizer};

let mut parser = Parser::new(tokens);
let mut program = parser.parse().unwrap();

let mut optimizer = AstOptimizer::new();
optimizer.optimize(&mut program);

println!("Otimizações aplicadas: {}", optimizer.optimizations_count());
```

## Exemplo

```dryad
// Antes da otimização
let x = 2 + 2;
let y = 10 > 5;
let z = "Hello" + " World";

// Depois da otimização (em tempo de compilação)
let x = 4;
let y = true;
let z = "Hello World";
```

## Integração

O otimizador é automaticamente integrado ao processo de parsing quando usado através do `Parser`. Para usar manualmente:

```rust
use dryad_parser::{Parser, AstOptimizer};

fn optimize_code(source: &str) -> Result<Program, Error> {
    let mut parser = Parser::new(lexer.tokenize(source)?);
    let mut program = parser.parse()?;
    
    let mut optimizer = AstOptimizer::new();
    optimizer.optimize(&mut program);
    
    Ok(program)
}
```
