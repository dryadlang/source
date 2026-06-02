---
title: "Guia Detalhado de Sintaxe"
description: "Referência completa da gramática, identificadores e estruturas da linguagem Dryad."
category: "Linguagem"
order: 12
---

# Guia de Sintaxe da Linguagem Dryad

Este documento serve como referência definitiva para a sintaxe da linguagem Dryad, detalhando regras gramaticais, estruturas de controle e convenções.

## 🚀 Leitura Rápida

- **Sintaxe C-Style**: Familiar para quem vem de JavaScript, C# ou Rust.
- **Variáveis**: Declaração com `let` (mutável) ou `const` (imutável).
- **Tipos**: Dinâmicos por padrão, anotações opcionais (`: number`, `: string`).
- **Comentários**: `//` linha, `/* */` bloco.

---

## ⚙️ Visão Técnica

### 1. Estrutura Léxica

- **Identificadores**: Devem começar com letras ou `_`. Case-sensitive.
- **Ponto e Vírgula**: Obrigatório após declarações e expressões.
- **Blocos**: Delimitados por `{}`.

### 2. Controle de Fluxo Avançado

Além dos tradicionais `if` e `for`, o Dryad oferece `match` com pattern matching:

```dryad
let resultado = match (valor) {
    1 => "um",
    2 => "dois",
    [a, b] => "array de dois: " + a + ", " + b,
    _ => "outro"
};
println(resultado);
```

### 3. Sistema de Erros (try/catch/finally)

```dryad
try {
    let dados = operacaoRisco();
    println(dados);
} catch (erro) {
    println("Erro: " + erro);
} finally {
    println("Finalizado");
}
```

O operador `?` propaga erros em expressões:

```dryad
let resultado = divide(10, 2)?;  // propaga se for erro
```

### 4. Sistema de Módulos e Diretivas

O Dryad integra-se ao sistema nativo via diretivas `#<module>`:

```dryad
#<console_io>
#<file_io>

println("Hello World");
let conteudo = read_file("dados.txt");
```

---

## 📚 Referências e Paralelos

- **Gramática de Referência**: Consulte a crate `dryad_parser` para a EBNF completa.
- **Sintaxe inspirada em**: JavaScript/TypeScript.

> Para a especificação completa de precedência de operadores, keywords e regras de crescimento, consulte `SYNTAX_MANIFEST.md` na raiz do projeto.
