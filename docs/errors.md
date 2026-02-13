---
title: "Guia de Erros"
description: "Como interpretar diagnósticos e resolver problemas de execução."
category: "Erros"
order: 1
---

# Códigos de Erro e Diagnósticos

O sistema de erros do Dryad é projetado para ser informativo, preciso e amigável ao desenvolvedor, fornecendo não apenas o que deu errado, mas **onde** e **como** corrigir.

## 🚀 Leitura Rápida

- **Identificável**: Cada erro possui um código único de 4 dígitos.
- **Informativo**: Mensagens incluem trechos do código fonte (Visual Snippets).
- **Categorizado**: Faixas numéricas indicam o componente responsável (Lexer, Parser, Runtime).

---

## ⚙️ Visão Técnica

O motor de diagnósticos reside na crate `dryad_errors`. Ele utiliza uma estrutura centralizada para garantir consistência em todo o pipeline.

### 1. A Crate `dryad_errors`

Diferente de simples strings de erro, o Dryad utiliza o tipo `DryadError` que captura metadados contextuais:

- **`code`**: O identificador único.
- **`span`**: A localização exata (offset) no arquivo.
- **`context`**: Linhas adjacentes para exibição visual.

### 2. Formatação Rica (Visual Diagnostics)

Inspirado por compiladores modernos como Rust e Elm, o Dryad utiliza bibliotecas como `miette` para renderizar erros no terminal:

```ansi
error[2003]: missing semicolon
  --> main.dryad:10:5
   |
10 |     let x = 10
   |               ^ esperado ';'
```

### 3. Categorias de Códigos

| Faixa    | Componente   | Descrição                                                  |
| :------- | :----------- | :--------------------------------------------------------- |
| **1xxx** | **Lexer**    | Erros de baixo nível (caracteres, strings não fechadas).   |
| **2xxx** | **Parser**   | Erros de estrutura gramatical.                             |
| **3xxx** | **Runtime**  | Erros durante a execução (divisão por zero, tipos).        |
| **8xxx** | **Warnings** | Alertas que não impedem a execução, mas sugerem melhorias. |

---

## 📚 Referências e Paralelos

- **Instração Visual**: [Miette Crate](https://github.com/zkat/miette).
- **Rust Patterns**: [Rust Error Handling Guidelines](https://rust-lang.github.io/api-guidelines/errors.html).
- **Glossário**: [Compiler Diagnostics (Wikipedia)](https://en.wikipedia.org/wiki/Compiler_diagnostic).

---

## Tabela de Códigos Implementados

_(A lista abaixo é a referência oficial para o desenvolvimento do interpretador)._

### 1xxx: Erros Léxicos

| Código   | Descrição                         |
| :------- | :-------------------------------- |
| **1001** | Caractere inesperado              |
| **1002** | Literal de string não terminado   |
| **1003** | Bloco de comentário não terminado |
| **1004** | Formato de número inválido        |
| **1005** | Sequência de escape inválida      |
| **1006** | Diretiva nativa inválida          |

### 2xxx: Erros de Parser (Sintaxe)

| Código   | Descrição                                  |
| :------- | :----------------------------------------- |
| **2001** | Token inesperado                           |
| **2003** | Ponto e vírgula hiante (missing semicolon) |
| **2005** | Parêntese de fechamento ausente            |
| **2011** | Declaração de variável inválida            |
| **2017** | Parâmetros de função ausentes              |
| **2018** | Condição de while ausente                  |
| **2019** | Componentes de for ausentes                |

### 3xxx: Erros de Runtime

| Código   | Descrição                          |
| :------- | :--------------------------------- |
| **3001** | Variável não definida              |
| **3005** | Operação aritmética inválida       |
| **3006** | Multiplicação inválida             |
| **3007** | Divisão por zero                   |
| **3009** | Comparação inválida                |
| **3010** | Break fora de loop                 |
| **3011** | Continue fora de loop              |
| **3020** | Exceção lançada (usuário)          |
| **3021** | Retorno de função inválido         |
| **3022** | Contexto `this` inválido           |
| **3023** | `super` não implementado           |
| **3034** | Atribuição de propriedade inválida |

### 5xxx: Erros de I/O

| Código   | Descrição              |
| :------- | :--------------------- |
| **5001** | Arquivo não encontrado |
| **5002** | Permissão negada       |

### 6xxx: Erros de Módulo

| Código   | Descrição                  |
| :------- | :------------------------- |
| **6001** | Módulo nativo desconhecido |
| **6002** | Importação circular        |

### 8xxx: Warnings

| Código   | Descrição              |
| :------- | :--------------------- |
| **8001** | Variável não utilizada |

### 9xxx: Erros de Sistema

| Código   | Descrição                          |
| :------- | :--------------------------------- |
| **9001** | Memória insuficiente               |
| **9002** | Estouros de pilha (Stack Overflow) |
