---
title: "Validação e LSP"
description: "Lógica de análise estática e integração com Language Server Protocol."
category: "Desenvolvimento"
order: 69
---

# Lógica para Validador Sintático (IDE)

O validador é o motor intelectual por trás do suporte a IDEs, permitindo feedback de erro em tempo real sem a necessidade de execução completa do script.

## 🚀 Leitura Rápida

- **Objetivo**: Detectar erros de digitação e lógica básica instantaneamente.
- **Fases**: Tokenização → Parsing Resiliente → Análise de Escopo.
- **Diferencial**: Não para no primeiro erro; tenta entender o resto do arquivo.
- **LSP**: Pronto para integração com o protocolo oficial de editores.

---

## ⚙️ Visão Técnica

Para que o validador seja útil, ele deve ser **Indulgente** e **Resiliente**. Um parser comum de compilador costuma abortar no primeiro erro, o que tornaria a experiência na IDE frustrante.

### 1. Parsing Resiliente (Error Recovery)

Utilizamos uma técnica de **Sincronização por Ponto de Parada**.

- **Estratégia**: Se o parser espera um `;` mas encontra um identificador, ele registra o erro, mas "salta" tokens até encontrar um marcador de sincronização seguro (ex: `}`, `function`, `class`).
- **Nós de Erro**: A AST resultante contém nós especiais `Expr::Error`, permitindo que o restante da árvore seja analisada para erros semânticos (como variáveis não definidas).

### 2. Análise de Escopo Estática

Diferente da execução, o validador percorre a árvore apenas para verificar a validade dos identificadores.

- **Symbol Table Stack**: Mantém uma pilha de escopos. Se um identificador não existe na pilha, um diagnóstico de "Undefined Variable" é emitido.
- **Linting**: O validador detecta variáveis declaradas mas nunca utilizadas através de flags de acesso durante a travessia.

### 3. Integração LSP (Language Server Protocol)

O validador é o coração do `dryad-lsp`. Ele converte os resultados da análise interna em mensagens que editores como VS Code e JetBrains entendem.

| Conceito Dryad   | Conceito LSP                |
| :--------------- | :-------------------------- |
| `DryadError`     | `Diagnostic`                |
| `SourceLocation` | `Range` (Line/Char mapping) |
| `Suggestion`     | `CodeAction` (Quick Fix)    |

---

## 📚 Referências e Paralelos

- **LSP Spec**: [Microsoft Language Server Protocol](https://microsoft.github.io/language-server-protocol/).
- **Rust Library**: [Tower LSP](https://github.com/ebakalov/tower-lsp) - A base recomendada para a implementação Rust do servidor.
- **Resilient Parsing**: [Matklad - Resilient LL Parsing](https://matklad.github.io/2023/05/21/resilient-ll-parsing.html).

---

## Estrutura do Validador (Exemplo Rust)

```rust
struct Diagnostic {
    range: Range, // Espaço ocupado no editor
    severity: Error | Warning,
    message: String,
}

fn validate(source: &str) -> Vec<Diagnostic> {
    let (tokens, lex_errors) = lex_resilient(source);
    let (ast, parse_errors) = parse_resilient(tokens);
    let semantic_errors = analyze_scopes(&ast);

    return collect_all(lex_errors, parse_errors, semantic_errors);
}
```
