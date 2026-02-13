---
title: "Manual do Lexer"
description: "Detalhes técnicos sobre o analisador léxico e tokens."
category: "Desenvolvimento"
order: 2
---

# Manual Técnico: Lexer (Analisador Léxico)

**Localização**: `crates/dryad_lexer/`
**Responsável**: Transformar o código fonte (texto) em uma sequência de tokens de forma performática e resiliente.

## 🚀 Destaques Práticos

- **Unicode-Native**: Suporta caracteres especiais e emojis nativamente.
- **Lazy Evaluation**: Gera tokens apenas quando solicitado pelo parser.
- **Zero-Copy (Draft)**: Otimizado para minimizar alocações de string durante o scan.
- **Erros Precisos**: Rastreia `line`, `column` e `span` para feedback visual imediato.

---

## ⚙️ Visão Técnica

O Lexer do Dryad é implementado como uma **Máquina de Estados de Transição Direta**. Diferente de abordagens baseadas em geradores de código (como Flex), ele é escrito manualmente em Rust para garantir controle total sobre o tratamento de erros e performance.

### 1. Manipulação de String em Rust

O Rust trata strings como sequências de bytes UTF-8 válidos. O Lexer converte o `source` em um `Vec<char>` para facilitar o acesso via índice, garantindo que não quebraremos caracteres multi-byte (como emojis) durante o espiamento (`peek`).

> [!NOTE]
> **Performance Tip**: Em uma implementação de produção, converter para `Vec<char>` pode ser caro. O Lexer utiliza iteradores de caracteres (`Chars`) para manter a performance próxima de C++.

### 2. Estrutura de Dados do Lexer

```rust
pub struct Lexer {
    source: Vec<char>,          // Fonte Unicode-aware
    source_lines: Vec<String>,  // Snapshot para contextos de erro
    position: usize,            // Cursor atual (offset de caracteres)
    line: usize,                // Contador de linha para o erro
    column: usize,              // Contador de coluna para o erro
}
```

### 3. Lógica de Reconhecimento (DFA)

O Lexer utiliza o sistema de `match` do Rust, que é compilado para tabelas de salto altamente eficientes.

| Caractere | Lógica de Reconhecimento                                   | Referência Rust        |
| :-------- | :--------------------------------------------------------- | :--------------------- |
| `0..=9`   | Consumo guloso (greedy) até delimitador.                   | `char::is_ascii_digit` |
| `a..=z`   | Leitura de identificadores e check em HashMap de Keywords. | `HashMap::get` (O(1))  |
| `/`       | Diferencia divisor `/` de comentários `//` e `/*`.         | Pattern Matching       |

### 4. Gestão de Memória e Tokens

Cada token é um `Enum` em Rust. Isso é extremamente eficiente, pois o tamanho do enum é determinado pelo maior variante, permitindo que tokens sejam passados pela pilha (stack) de forma barata.

---

## 📚 Referências e Paralelos

- **Biblioteca Logos**: O Dryad inspira-se no design de [Logos](https://github.com/maciejhirsz/logos) para a definição de tokens.
- **Dragon Book**: Seção sobre DFA (Deterministic Finite Automata) e NFA.
- **Rust Docs**: [Module `std::str`](https://doc.rust-lang.org/std/str/index.html) para entendimento de UTF-8.

---

## 5. Rastreamento de Localização

Cada token emitido é envolvido em um `TokenWithLocation`:

```rust
pub struct TokenWithLocation {
    pub token: Token,
    pub location: SourceLocation, // { file, line, column, position }
}
```

## 6. Integração com Sistema de Erros

O Lexer gera erros do tipo `DryadError::Lexer` (código 1xxx).

- **Recuperação**: Utiliza pontos de sincronização para não travar o linter da IDE em caso de strings não fechadas.
