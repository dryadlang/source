---
title: "Roadmap e Visão Estratégica"
description: "Planejamento estratégico de longo prazo do projeto Dryad"
category: "Desenvolvimento"
order: 1
---

# Roadmap do Projeto Dryad

## Visão Geral

Transformar a linguagem Dryad de um protótipo funcional (v1.0) para uma linguagem robusta, segura e performática (v2.0+), com ecossistema de ferramentas maduro e compilação AOT para binários nativos.

**Status Atual (Fevereiro 2026):**
- ✅ Bytecode VM completo (~95% da linguagem, performance 2-3x)
- ✅ Refatorações estruturais de segurança concluídas
- 🚧 Planejamento AOT em andamento (55%)
- 📋 Oak Package Manager em desenvolvimento

---

## Épicos Concluídos ✅

### [E0] Bytecode VM
**Status:** Concluído (Fevereiro 2026)

- ✅ 69+ opcodes implementados
- ✅ Performance 2-3x melhor que AST
- ✅ 100% portável (x86/ARM)
- ✅ Funções, classes, exceções, arrays

### [E1] Refatoração Estrutural e Segurança
**Status:** Concluído

- ✅ Sandbox de Execução Nativa (Remover `native_exec` inseguro)
- ✅ Refatoração do Monólito Oak (Dividir `main.rs`)
- ✅ Proteção contra Stack Overflow (Recursion Limit)
- ✅ Modularização do Interpretador (Environment/NativeRegistry)

---

## Épicos em Andamento 🚧

### [E2] Oak Package Manager
- [ ] Validação de Checksum/Integridade
- [ ] Semantic Versioning Real
- [ ] Lockfile Determinístico

### [E3] Evolução da Sintaxe
- [ ] Pattern Matching Avançado
- [ ] Destructuring e Spread Operator
- [ ] Template Strings (parcial)

### [E4] Standard Library
- [ ] Servidor HTTP/TCP Robusto
- [ ] Async File I/O (tokio::fs)
- [ ] Drivers de Banco de Dados

---

## Épicos Futuros 📋

### [E5] Compilação AOT (Ahead-of-Time)
**Timeline:** 12 meses | **Progresso:** 55%

- [ ] Fundações e IR (Meses 1-2)
- [ ] Linux ELF Completo (Meses 2-4)
- [ ] Windows PE Completo (Meses 4-6)
- [ ] Features Avançadas - OOP, GC (Meses 6-8)
- [ ] Otimizações (Meses 9-10)
- [ ] Debug e Ferramentas (Meses 11-12)

**Milestones:**
- M1 (Mês 2): "Hello World" compilado para Linux
- M2 (Mês 4): Executáveis ELF completos
- M3 (Mês 6): Executáveis Windows PE
- M4 (Mês 8): GC e exceções nativas
- M5 (Mês 10): Otimizações avançadas
- M6 (Mês 12): v1.0 estável

### [E6] Otimizações de Runtime
- [ ] Constant Folding
- [ ] Dead Code Elimination
- [ ] Lexer Zero-copy
- [ ] Generational GC

### [E7] Ecossistema Enterprise
- [ ] Central Package Registry
- [ ] Language Server Protocol (LSP)
- [ ] Debugger Interativo
- [ ] Profiler

---

## Features Planejadas

### Melhorias na Linguagem
- **Pattern Matching Avançado**: Guards complexos, matching em arrays/tuplas
- **Sistema de Tipos Opcional**: Gradual typing com `dryad check`
- **Garbage Collection Avançado**: Coletor incremental, gerações

### Ecossistema
- **Standard Library 2.0**: JSON Stream, Crypto (RSA/AES), WebSockets
- **FFI**: Carregar bibliotecas dinâmicas (.so/.dll)
- **Interface de Debug**: Protocolo para IDEs, breakpoints

### Performance
- **JIT Compilation**: Compilar bytecode quente para código nativo
- **Otimizações de Bytecode**: Constant folding, dead code elimination

---

## Progresso por Fase

| Fase | Status | Progresso |
|------|--------|-----------|
| Fase 0: Protótipo | ✅ Completo | 100% |
| Fase 1: Estabilização | ✅ Completo | 100% |
| Fase 2: Expansão | 🚧 Em Andamento | 40% |
| Fase 3: AOT e Ecossistema | 📋 Planejado | 0% |

---

## Visão de Longo Prazo (v2.0+)

**Objetivo Final:** Dryad como linguagem de produção completa

- ✅ Performance competitiva com linguagens compiladas
- ✅ Ecossistema de pacotes robusto
- ✅ Ferramentas maduras (LSP, Debugger, Profiler)
- ✅ Compilação AOT multiplataforma
- ✅ Comunidade ativa

**Estimativa:** 18-24 meses para v2.0 completo

---

## Documentação Técnica Relacionada

- **Bytecode**: `docs/develop/bytecode/`
- **AOT**: `docs/develop/aot/`
- **TODOs**: Ver `todo.md` para tarefas detalhadas
- **Concluído**: Ver `done.md` para histórico
