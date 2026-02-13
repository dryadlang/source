---
title: "Tipos de Dados"
description: "Explicação dos tipos de dados dinâmicos e representação interna no Dryad."
category: "Linguagem"
order: 2
---

# Tipos de Dados

A linguagem Dryad é **dinamicamente tipada**, o que significa que os tipos são determinados em tempo de execução. Todos os valores em memória são representados pela enum `Value` do interpretador.

## 🚀 Leitura Rápida

- **Tipagem**: Dinâmica (runtime-checked).
- **Primitivos**: Number (f64), String (UTF-8), Boolean, Null.
- **Estruturas**: Array (listas), Object (dicionários), Tuple (imutáveis).
- **Nativos**: Threads e Mutexes são cidadãos de primeira classe.

---

## ⚙️ Visão Técnica

Internamente, o Dryad utiliza o sistema de tipos do Rust para garantir segurança e eficiência. Todo valor é uma instância da enum `Value`.

### 1. Representação do `Value`

Diferente de C (onde se usariam uniões perigosas), o Rust utiliza _Tagged Unions_ para enums.

```rust
pub enum Value {
    Number(f64),
    String(Arc<String>),
    Boolean(bool),
    Array(Arc<RwLock<Vec<Value>>>),
    // ...
}
```

- **Passagem por Valor**: Primitivos como `Number` e `Boolean` são pequenos e copiados na pilha (stack).
- **Passagem por Referência**: `String`, `Array` e `Object` utilizam contagem de referências (`Arc`) para evitar clonagens de memória pesadas (heap).

### 2. Especificidades dos Tipos

#### `Number` (IEEE 754)

Seguindo o padrão do JavaScript e Lua, o Dryad não diferencia inteiros de flutuantes no nível da linguagem. Todos são `f64`.

- **Vantagem**: Simplicidade matemática e compatibilidade total com FFI.
- **Referência**: [IEEE Standard for Floating-Point Arithmetic](https://en.wikipedia.org/wiki/IEEE_754).

#### `String` (UTF-8 Imutável)

Strings no Dryad são sequências de bytes UTF-8 válidos, garantidas pela segurança do tipo `String` do Rust.

- **Unicode**: Suporte total a emojis e caracteres internacionais out-of-the-box.

---

## 📚 Referências e Paralelos

- **Referência Rust**: [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html).
- **Gerenciamento de Memória**: [Stack vs Heap Allocation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap).
- **CS Fundamental**: "Types and Programming Languages" (Benjamin C. Pierce).

---

## Tipos Compostos (Ref)

### `Array`

- Baseado em `Vec<Value>`.
- **Thread-Safety**: Protegido por `RwLock` quando compartilhado entre fibras.

### `Object`

- Coleção chave-valor baseada em `HashMap<String, Value>`.
- Acesso otimizado via O(1) médio.

### `Tuple`

- Sequência imutável. Útil para retorno de múltiplos valores de funções.
