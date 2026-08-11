---
title: "Guia de Erros"
description: "Como interpretar diagnósticos e resolver problemas de execução."
category: "Referências"
order: 102
---

# Guia de Erros e Diagnósticos

O sistema de erros do Dryad é projetado para ser informativo, preciso e amigável ao desenvolvedor, fornecendo não apenas o que deu errado, mas **onde** e **como** corrigir.

## 🚀 Leitura Rápida

- **Identificável**: Cada erro possui um código único de 4 dígitos (ex: E1001).
- **Informativo**: Mensagens incluem trechos do código fonte (Visual Snippets).
- **Categorizado**: Faixas numéricas indicam o componente responsável (1xxx: Lexer, 2xxx: Parser, 3xxx: Runtime, etc).
- **Formatado**: Diagnósticos ricos com cores e ponteiros visuais no terminal.

---

## ⚙️ Visão Técnica

O motor de diagnósticos reside na crate `dryad_errors`. Ele utiliza uma estrutura centralizada para garantir consistência em todo o pipeline de compilação e execução.

### 1. A Crate `dryad_errors`

Diferente de simples strings de erro, o Dryad utiliza o tipo `DryadError` que captura metadados contextuais:

- **`code`**: O identificador único para busca rápida na documentação.
- **`span`**: A localização exata (offset) no arquivo de origem.
- **`context`**: Linhas adjacentes para exibição visual clara.

### 2. Formatação Rica (Visual Diagnostics)

Inspirado por compiladores modernos como Rust e Elm, o Dryad utiliza bibliotecas como `miette` e `ariadne` para renderizar erros no terminal:

```ansi
error[2003]: missing semicolon
  --> main.dryad:10:5
   |
10 |     let x = 10
   |               ^ esperado ';' aqui
```

### 3. Categorias de Códigos

| Faixa    | Componente  | Descrição                                               |
| :------- | :---------- | :------------------------------------------------------ |
| **1xxx** | **Lexer**   | Erros de baixo nível (caracteres inválidos, strings).   |
| **2xxx** | **Parser**  | Erros de estrutura gramatical e sintaxe.                |
| **3xxx** | **Runtime** | Erros durante a execução (tipos, lógica, divisão zero). |
| **6xxx** | **Module**  | Problemas com importação e resolução de nomes.          |
| **9xxx** | **System**  | Erros graves do host (memória, stack overflow).         |

---

## 📚 Referências e Paralelos

- **Catálogo Completo**: Consulte a [Lista de Códigos de Erro](errors/error_codes.md).
- **Inspiração**: [Rust Error Index](https://doc.rust-lang.org/error-index.html).
- **Ferramentas**: [Crate Miette](https://github.com/zkat/miette).

---

> [!TIP]
> **Dica**: Use o comando `dryad check` para rodar o `dryad_checker` e identificar erros de tipo (3xxx) antes mesmo de rodar o código.
