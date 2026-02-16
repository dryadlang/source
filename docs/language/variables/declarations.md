---
title: "Variáveis e Escopo"
description: "Declaração de variáveis com let e const, escopo léxico e shadowing."
category: "Linguagem"
order: 14
---

# Variáveis e Escopo

O gerenciamento de estado no Dryad é feito através de declarações de variáveis com escopo léxico rigoroso.

## 🚀 Leitura Rápida

- **Mutável**: Use `let` para valores que podem mudar.
- **Imutável**: Use `const` para valores fixos (identificadores constantes).
- **Escopo**: Sempre limitado ao bloco `{ ... }` mais próximo.
- **Shadowing**: Permitido redefinir nomes em escopos internos.

---

## ⚙️ Visão Técnica

O interpretador gerencia variáveis através de uma pilha de **Tabelas de Símbolos**. Cada bloco de código cria um novo "Environment" que aponta para o seu pai.

### 1. Let vs Const (Semântica de Escrita)

Internamente, a tabela de símbolos armazena não apenas o `Value`, mas também uma flag `is_mutable`.

- **`let`**: Permite a instrução `Expr::Assign`.
- **`const`**: Lança o erro `3002 (ImmutableAssignment)` se uma atribuição for tentada após a inicialização.

### 2. Shadowing (Sombreamento)

O Dryad permite que uma variável em um escopo interno "esconda" uma variável com o mesmo nome em um escopo externo.

```dryad
let x = 10;
{
    let x = 20; // Shadowing de x externo
    console.log(x); // 20
}
console.log(x); // 10
```

> [!TIP]
> **Paralelo Rust**: O shadowing no Dryad é similar ao do Rust, mas ocorre apenas entre escopos diferentes (não é permitido redeclarar no mesmo nível de escopo).

### 3. Hoisting (Içamento)

Diferente do JavaScript, o Dryad **não possui hoisting** de variáveis. Tentar acessar uma variável antes de sua declaração resultará em um erro de runtime `3001 (UndefinedVariable)`.

---

## 📚 Referências e Paralelos

- **Escopo Léxico**: [Wikipedia: Scope (Computer Science)](<https://en.wikipedia.org/wiki/Scope_(computer_science)>).
- **Rust Shadowing**: [The Rust Programming Language - Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing).
- **Sistemas de Variáveis**: "Structure and Interpretation of Computer Programs" (SICP) - Seção sobre Modelos de Ambiente.
