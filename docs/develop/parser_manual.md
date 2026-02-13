---
title: "Manual do Parser"
description: "Detalhes técnicos sobre o analisador sintático e Pratt Parsing."
category: "Desenvolvimento"
order: 3
---

# Manual Técnico: Parser (Analisador Sintático)

**Localização**: `crates/dryad_parser/`
**Responsável**: Transformar a sequência de tokens em uma Árvore Sintática Abstrata (AST) estruturada e semanticamente válida.

## 🚀 Destaques Práticos

- **Recursive Descent**: Lógica clara onde cada função mapeia uma regra da gramática.
- **Top-Down Parsing**: Começa da raiz (`Program`) e desce até as folhas (`Literals`).
- **Resiliência**: Projetado para identificar múltiplos erros sem abortar (Panic Recovery experimental).
- **Tipagem Forte**: AST baseada em Rust Enums, impossibilitando estados inválidos em tempo de compilação.

---

## ⚙️ Visão Técnica

O Parser do Dryad utiliza uma combinação de **Descida Recursiva** para declarações (statements) e **Pratt Parsing** (ou _Precedence Climbing_) para expressões.

### 1. Algoritmo de Pratt (Expressões)

Para evitar a explosão de métodos em gramáticas com muitos níveis de precedência (como matemática combinada com operadores lógicos), o Dryad utiliza uma tabela de precedência.

> [!NOTE]
> **Por que Pratt?**: Ele permite que um único loop processe operadores binários baseando-se apenas no "peso" (binding power) de cada operador, eliminando dezenas de funções recursivas.
> **Referência**: [Pratt Parsing em Rust](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).

### 2. Estrutura da AST (`ast.rs`)

A AST é o coração do compilador. Em Rust, utilizamos `Box<Expr>` para permitir tipos recursivos (já que o tamanho de um Enum deve ser conhecido em tempo de compilação).

```rust
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(Value),
    // ...
}
```

### 3. Recuperação de Erros (Panic Mode)

Ao encontrar um erro, o Parser entra em `synchronize()`. Ele descarta tokens até encontrar um delimitador de fim de sentença (como `;`) ou o início de uma nova declaração (`function`, `var`, `if`).

---

## 📚 Referências e Paralelos

- **Rustc Parser**: O compilador do Rust utiliza uma técnica similar de descida recursiva manual.
- **V8 Engine (JavaScript)**: O parser do V8 também utiliza Pratt Parsing para expressões para garantir velocidade de execução.
- **Livro Texto**: "Compilers: Principles, Techniques, and Tools" (Alfred Aho) - Seção sobre Gramáticas Livre de Contexto (CFG).

---

## 4. Detalhes de Implementação Críticos

### Sincronização e Recuperação de Erro

A estratégia de sincronização permite que ferramentas como o `Dryad LSP` mostrem todos os erros do arquivo de uma vez, em vez de apenas o primeiro.

### Parsing de Atribuição (`assignment`)

O parser realiza uma verificação de "L-Value". Se o lado esquerdo de um `=` não for algo onde possamos armazenar valor (como um literal `5 = 10`), o parser emite um erro semântico imediato.

---

## 5. Códigos de Erro Específicos

O Parser opera na faixa `2xxx`.

- **2001 (UnexpectedToken)**: Ocorreu um token fora do lugar esperado.
- **2005 (UnclosedDelimiter)**: Chaves `{` ou parênteses `(` sem o par correspondente.
- **2010 (ReservedKeyword)**: Uso de palavra reservada como nome de variável.
