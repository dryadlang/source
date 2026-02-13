---
title: "Códigos de Erro"
description: "Catálogo completo de erros do compilador e do runtime."
category: "Erros"
order: 2
---

# Referência de Códigos de Erro

Lista completa dos códigos de erro emitidos pelo compilador e runtime da Dryad, com dicas de resolução para ajudar desenvolvedores a identificar e corrigir problemas rapidamente.

## 1xxx: Erros Léxicos

Erros encontrados durante a leitura dos caracteres do arquivo.

| Código   | Mensagem                   | Causa Provável                                                   | Solução                                           |
| :------- | :------------------------- | :--------------------------------------------------------------- | :------------------------------------------------ |
| **1001** | Caractere inesperado       | Uso de símbolos não reconhecidos (ex: `@`, `$`) fora de strings. | Remova o caractere ou coloque-o dentro de aspas.  |
| **1002** | String não terminada       | Esquecer de fechar aspas.                                        | Adicione `"` ou `'` ao final da string.           |
| **1004** | Formato de número inválido | Múltiplos pontos decimais ou letras em números.                  | Verifique a sintaxe numérica (`1.2.3` -> `1.23`). |

## 2xxx: Erros de Sintaxe (Parser)

Erros na estrutura gramatical do código.

| Código   | Mensagem                  | Causa Provável                                   | Solução                                         |
| :------- | :------------------------ | :----------------------------------------------- | :---------------------------------------------- |
| **2001** | Token inesperado          | Ordem incorreta de palavras (ex: `if (x) else`). | Verifique a estrutura do bloco ou comando.      |
| **2003** | Ponto e vírgula ausente   | Esquecer `;` após declaração.                    | Adicione `;`.                                   |
| **2005** | `)` ausente               | Parênteses desbalanceados.                       | Conte os parênteses abertos e fechados.         |
| **2011** | Nome de variável inválido | Usar palavras reservadas ou números no início.   | Use nomes válidos (ex: `let x`, não `let 123`). |

## 3xxx: Erros de Runtime

Erros que ocorrem durante a execução do programa.

| Código   | Mensagem              | Causa Provável                                            | Solução                                           |
| :------- | :-------------------- | :-------------------------------------------------------- | :------------------------------------------------ |
| **3001** | Variável não definida | Tentar usar variável antes de declarar ou fora de escopo. | Declare a variável com `let` ou verifique o nome. |
| **3005** | Operação inválida     | Somar número com objeto, etc.                             | Verifique os tipos dos operandos.                 |
| **3007** | Divisão por zero      | `x / 0`.                                                  | Adicione verificação `if (divisor != 0)`.         |
| **3022** | `this` inválido       | Usar `this` em função solta (fora de classe).             | Mova o código para um método de classe.           |

## 6xxx: Erros de Módulo

Erros relacionados a importação e carregamento de arquivos.

| Código   | Mensagem            | Causa Provável                 | Solução                                            |
| :------- | :------------------ | :----------------------------- | :------------------------------------------------- |
| **6001** | Módulo desconhecido | `#modulo` inexistente ou typo. | Verifique o nome do módulo nativo na documentação. |

## Exemplo de Tratamento de Erros

```dryad
try {
    let resultado = 10 / 0; // Gera erro 3007
} catch (erro) {
    println("Erro: " + erro.message);
}
```
