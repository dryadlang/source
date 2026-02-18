---
title: "Pattern Matching"
description: "Sistema de correspondência de padrões com match, desestruturação e guardas."
category: "Linguagem"
order: 18
---

# Pattern Matching (match)

O Dryad oferece um sistema de pattern matching poderoso e expressivo através da palavra-chave `match`, permitindo desestruturação de dados complexos com segurança.

## 🚀 Leitura Rápida

- **Exaustividade**: O compilador recomenda cobrir todos os casos ou usar `_`.
- **Desestruturação**: Extraia dados de Arrays, Objetos e Tuplas diretamente no padrão.
- **Guards**: Adicione condições extras com `if`.
- **Bindings**: Novos identificadores criados no padrão valem apenas para aquele braço.

---

## ⚙️ Visão Técnica

O motor de matching do Dryad executa em tempo de execução através de um visitador de padrões que suporta aninhamento profundo.

### 1. Algoritmo de Correspondência

Para cada braço do `match`:

1.  **Teste de Estrutura**: O valor coincide com o formato (ex: é um Array de 3 elementos?).
2.  **Teste de Conteúdo**: Literais internos coincidem?
3.  **Binding**: Se coincidir, as variáveis no padrão são injetadas em um novo escopo local.

### 2. Guards (Filtros de Braço)

O guarda (`if condition`) é avaliado **após** a correspondência do padrão e **antes** da execução do corpo. Se o guarda falhar, o interpretador continua para o próximo braço.

### 3. Exemplos de Padrões

```dryad
match (data) {
    [1, x, ...rest] if x > 10 => "Começa com 1, " + x + " e resto: " + rest,
    { nome: n } => "Usuário: " + n,
    (a, b) => "Tupla de " + a + " e " + b,
    [head, ...tail] => "Cabeça: " + head + ", Cauda: " + tail,
    _ => "Sem correspondência"
}
```

---

## 📚 Referências e Paralelos

- **Rust Match**: Fortemente inspirado no [Match do Rust](https://doc.rust-lang.org/book/ch06-02-match.html).
- **Elixir Pattern Matching**: Conceitos de desestruturação e binds.
- **CS**: [Pattern Matching Algorithms](https://en.wikipedia.org/wiki/Pattern_matching).
