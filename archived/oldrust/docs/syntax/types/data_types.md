---
title: "Tipos de Dados"
description: "Explicação dos tipos de dados dinâmicos e representação interna no Dryad."
category: "Linguagem"
order: 15
---

# Tipos de Dados

O Dryad é dinamicamente tipado, utilizando um motor de execução que prioriza a segurança de memória e a expressividade.

## 🚀 Leitura Rápida

- **Primitivos**: Number (f64), String (UTF-8), Boolean, Null.
- **Compostos**: Array (mutáveis), Object (chave-valor), Tuple (imutáveis).
- **Gestão**: Contagem de referências (`Arc`) para tipos pesados.
- **Interoperabilidade**: FFI-ready para integração com C/Rust.

---

## ⚙️ Visão Técnica

Internamente, cada valor é uma instância da enum `Value` da crate `dryad_runtime`.

### 1. Representação Interna (`Value`)

O motor utiliza o sistema de tipos do Rust para garantir que não ocorram crashes de acesso ilegal:

- **Stack-based**: Primitivos são armazenados diretamente na pilha.
- **Heap-based**: Arrays e Objetos usam ponteiros atômicos inteligentes (`Arc`) com travas de leitura/escrita (`RwLock`) para garantir thread-safety total.

### 2. O Onipresente `f64`

Seguindo o padrão de linguagens como JavaScript e Lua, o Dryad não diferencia `int` de `float` no nível da linguagem.

- **Vantagem**: Elimina erros de overflow de inteiros comuns em C e simplifica o interpretador.

### 3. Coleções (Arrays e Tuplas)

O Dryad oferece dois tipos principais para coleções sequenciais:

- **Arrays (`[]`)**: Mutáveis e dinâmicos. Suportam o operador **Spread** (`...`) para expansão e **Rest Patterns** para desestruturação.
- **Tuplas (`()`)**: Imutáveis após a criação. Ideais para retornar múltiplos valores de uma função de forma segura.

```dryad
let arr = [1, 2, 3];
arr[0] = 10; // Permitido

let tup = (1, "ok");
// tup[0] = 5; // Erro: Tuplas são imutáveis
```

### 4. Strings UTF-8

Strings são imutáveis e garantidamente UTF-8 válidas. O acesso a caracteres é O(n), mas operações de busca e comparação são otimizadas via Rust internal strings.

---

## 📚 Referências e Paralelos

- **Rust Enums**: [Tagged Unions in Rust](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html).
- **Floating Point**: [The Perils of Floating Point](https://docs.oracle.com/cd/E19957-01/806-3568/ncg_goldberg.html).
- **Types**: [Type Systems (Wikipedia)](https://en.wikipedia.org/wiki/Type_system).
