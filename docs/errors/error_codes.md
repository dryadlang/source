---
title: "Códigos de Erro"
description: "Catálogo completo de erros do compilador e do runtime."
category: "Erros"
order: 101
---

# Catálogo de Códigos de Erro

Este documento fornece a especificação técnica detalhada para cada erro emitido pelo ecossistema Dryad.

## 🚀 Leitura Rápida

- **1xxx**: Léxico (Tokens, Caracteres).
- **2xxx**: Sintaxe (Parser, Gramática).
- **3xxx**: Runtime (Execução, Tipos).
- **5xxx**: I/O (Arquivos, Rede).
- **9xxx**: Críticos (Sistema, Memória).

---

## ⚙️ Visão Técnica

Cada código é mapeado internamente na crate `dryad_errors` e possui uma URL de ajuda associada que aponta para este manual.

### 1xxx: Erros Léxicos

| Código   | Mensagem             | Causa Provável                                   | Solução                              |
| :------- | :------------------- | :----------------------------------------------- | :----------------------------------- |
| **1001** | Caractere inesperado | Uso de símbolos como `@` ou `$` fora de strings. | Remova o caractere ou use aspas.     |
| **1002** | String não terminada | Falta fechar `"` ou `'`.                         | Adicione o fechamento da string.     |
| **1004** | Número inválido      | Múltiplos pontos decimais (ex: `1.2.3`).         | Verifique a sintaxe do número.       |
| **1006** | Diretiva inválida    | `#modulo` inexistente no runtime.                | Verifique se o módulo nativo existe. |

### 2xxx: Erros de Sintaxe (Parser)

| Código   | Mensagem               | Causa Provável                               | Solução                                 |
| :------- | :--------------------- | :------------------------------------------- | :-------------------------------------- |
| **2001** | Token inesperado       | Ordem gramatical incorreta.                  | Verifique a sintaxe no Guia de Sintaxe. |
| **2003** | `;` esperado           | Esquecimento do ponto e vírgula obligatório. | Adicione `;` ao final da linha.         |
| **2005** | `)` ou `}` ausente     | Blocos ou parênteses desbalanceados.         | Feche todos os blocos abertos.          |
| **2011** | Identificador inválido | Nome de variável começando com número.       | Use nomes válidos (letras/\_).          |

### 3xxx: Erros de Runtime e Tipo

> [!NOTE]
> No Dryad, erros de tipo (Type Mismatch) durante a checagem dinâmica são reportados na faixa 3xxx para consistência com o motor de execução.

| Código   | Mensagem          | Causa Provável                                         | Solução                              |
| :------- | :---------------- | :----------------------------------------------------- | :----------------------------------- |
| **3001** | Não definido      | Variável usada antes de ser declarada.                 | Use `let` ou `const` para declarar.  |
| **3002** | Atribuição Const  | Tentar mudar valor de uma `const`.                     | Use `let` se precisar mutar o valor. |
| **3003** | Tipo incompatível | Operação entre tipos diferentes (ex: `bool + number`). | Garanta que os tipos coincidam.      |
| **3007** | Divisão por zero  | O denominador na operação `/` é zero.                  | Adicione proteção contra zero.       |
| **3022** | `this` inválido   | Uso de `this` fora de uma classe.                      | Use `this` apenas dentro de métodos. |
| **3101** | Result Inválido   | Uso do operador `?` em um tipo que não é `Result`.     | Use `?` apenas em valores `Result`.  |
| **3102** | Erro Propagado    | Um erro `Result(false, ...)` foi propagado via `?`.    | Trate o erro no nível superior.      |

---

## 📚 Referências e Paralelos

- **Guia de Diagnósticos**: Voltar para [Guia de Erros](../errors.md).
- **Linguagem**: [Guia de Sintaxe](../sintax/syntax.md).
- **Implementação**: Consulte `crates/dryad_errors/src/lib.rs`.
