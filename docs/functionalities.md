---
title: "Funcionalidades Implementadas"
description: "Lista detalhada das funcionalidades implementadas na linguagem Dryad."
category: "Guia de Uso"
order: 1
---

# Funcionalidades Implementadas

Abaixo está a lista detalhada das funcionalidades implementadas na linguagem Dryad, com referência ao código fonte. Este documento serve como um guia para desenvolvedores e usuários que desejam entender as capacidades atuais da linguagem.

Legenda:

- **[implementado]**: Funcionalidade completa.
- **[parcialmente]**: Funcionalidade presente, mas com limitações conhecidas.
- **[planejado]**: Funcionalidade futura (não listada aqui, focando no código existente).

## Checklist

### Declarações e Variáveis

- **[implementado]**: Suporte para `let` e `const` com escopo de bloco.
- **[implementado]**: Inicialização de variáveis com valores padrão (`null` para não inicializadas).

### Controle de Fluxo

- **[implementado]**: Estruturas de controle como `if`, `else`, `while`, `for` e `for-in`.
- **[parcialmente]**: Suporte a `switch` ainda em desenvolvimento.

### Funções

- **[implementado]**: Funções padrão e anônimas (lambdas).
- **[implementado]**: Suporte a funções assíncronas com `async` e `await`.

### Tipos de Dados

- **[implementado]**: Tipos primitivos como `Number`, `String`, `Boolean` e `Null`.
- **[parcialmente]**: Suporte a arrays com métodos limitados.

### Operadores

- **[implementado]**: Operadores aritméticos básicos e avançados, como `**` (exponenciação) e `%%` (módulo positivo).

### Biblioteca Padrão

- **[implementado]**: Manipulação de arquivos com `read_file` e `write_file`.
- **[implementado]**: Comunicação HTTP com `http_get` e `http_post`.
- **[implementado]**: Funções utilitárias como `random` e `sleep`.
