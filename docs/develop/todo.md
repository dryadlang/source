---
title: "Tarefas Pendentes"
description: "Lista de tarefas e TODOs para implementação"
category: "Desenvolvimento"
order: 2
---

# Tarefas Pendentes (TODO)

## Prioridade Alta 🔴

### Oak Package Manager
- [ ] **T2.1** Validação de Checksum/Integridade
  - Calcular SHA-256 do arquivo baixado
  - Comparar com hash do registry
  - Abortar se falhar
  
- [ ] **T2.2** Semantic Versioning Real
  - Implementar semver na resolução de dependências
  
- [ ] **T2.3** Lockfile Determinístico
  - Corrigir e melhorar arquivo de lock

### Linguagem e Sintaxe
- [ ] **T3.2** Pattern Matching Avançado
  - Guards mais complexos
  - Pattern matching em arrays/tuplas aninhadas
  
- [ ] **T3.3** Destructuring e Spread
  - `let {a, b} = obj`
  - `let [x, y] = arr`
  - `...args` em funções e arrays

### Standard Library
- [ ] **T4.1** Servidor HTTP/TCP Robusto
  - Completar API pública do http_server
  
- [ ] **T4.2** Async File I/O
  - Substituir `std::fs` por `tokio::fs`
  - Atualizar assinaturas para async

---

## Prioridade Média 🟡

### OOP e Classes
- [ ] Modificadores de Acesso (`private`, `protected`, `public`)
  - Parser aceita keywords, mas falta verificação em runtime
  
- [ ] Getters/Setters (`get prop()`, `set prop(v)`)

- [ ] Propriedades Estáticas (`static prop = val`)

### Funções
- [ ] Parâmetros Default (`function foo(x = 10)`)
- [ ] Parâmetros Nomeados (`foo(x: 10)`)
- [ ] Generators (`function*` e `yield`)

### Objetos (Maps)
- [ ] Métodos `.keys()`, `.values()`, `.entries()`

### Sintaxe Adicional
- [ ] Optional Chaining (`obj?.prop`)
- [ ] Nullish Coalescing (`??`)

---

## Prioridade Baixa 🟢

### I/O e Rede
- [ ] TCP Server (`bind`, `listen`, `accept`)
- [ ] UDP Support (completar bindings)
- [ ] File Streams (leitura/escrita bufferizada)

### Sistema
- [ ] Process Management (`fork`, `kill`, sinais)

### Tooling
- [ ] Oak Publish Command
  - Autenticação com Token
  - Empacotar em `.tar.gz`
  - Upload via HTTP POST
  
- [ ] Language Server Protocol (LSP)
- [ ] Debugger Interativo (breakpoints)
- [ ] Profiler

### AOT (Compilação Nativa)

**Status:** 🚧 Em desenvolvimento — consulte `aot/status.md` para progresso e tarefas detalhadas

- [ ] Gerador PE/COFF (Windows)
- [ ] Runtime nativo completo
- [ ] Otimizações SSA
- [ ] Debug info (DWARF/PDB)

---

## Refatorações Técnicas

### Parser & AST
- [ ] Interning de strings (`SmolStr`) para reduzir clones
- [ ] Boxear variantes grandes do enum `Expr`
- [ ] Implementar Pratt Parser para expressões

### Lexer
- [ ] Migrar para `Chars` iterator ou biblioteca `logos`
- [ ] Melhorar performance e segurança UTF-8

### Native Modules
- [ ] Unificar HTTP em módulo único
- [ ] Migrar file_io para tokio::fs (async)

### CLI
- [ ] Criar `Runner` struct para evitar duplicação de setup

---

## Notas

- Siga a ordem de prioridade para evitar bloqueios
- Marque como concluído movendo para `done.md`
- Atualize este documento conforme novas tarefas surgirem
