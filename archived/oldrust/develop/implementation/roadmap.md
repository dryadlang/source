---
title: "Roadmap e Visão Estratégica"
description: "Planejamento estratégico de longo prazo do projeto Dryad"
category: "Desenvolvimento"
order: 1
---

# Roadmap do Projeto Dryad

## Visão Geral

Transformar a linguagem Dryad de um protótipo funcional para uma linguagem de nível industrial (v2.0+), com compilação AOT nativa e um ecossistema completo de ferramentas.

**Status Atual (Fevereiro 2026):**

- ✅ Bytecode VM Completo (v1.1.0)
- ✅ Standard Library & Lang Features (v1.2.0)
- ✅ Base de Segurança Estabilizada
- 🚧 Planejamento AOT em andamento (55%)
- 📋 Expansão do Oak Package Manager

---

## Épicos e Milestones

### 🟢 Épicos Ativos (Fase 2: Expansão e Performance)

#### [E2] Oak Package Manager

- [x] **Lockfile Determinístico**: Garantir builds idênticos em qualquer máquina.
- [x] **Semantic Versioning**: Resolução inteligente de dependências.
- [ ] **Registry Central**: Backend para publicação de pacotes (`oak publish`).

#### [E3] Evolução da Sintaxe e Linguagem

- [x] **Pattern Matching v2**: Guards complexos e desestruturação aninhada.
- [x] **Destructuring e Spread**: `let {x} = obj` e `[...rest]`.
- [ ] **Módulos**: Resolução de dependências circulares.

#### [E4] Standard Library 2.0

- [x] **Async I/O**: Integração profunda com `tokio` para rede e arquivos.
- [x] **Database Drivers**: Conectores nativos para PostgreSQL/SQLite.
- [x] **HTTP Server**: Handlers dinâmicos e middlewares.
- [x] **WebSockets**: Suporte nativo a servidores.

#### [E5] Dryad Checker (Static Analysis)

- [ ] **Type Inference Core**: Algoritmo de inferência básica para variáveis.
- [x] **OOP Safety**: Verificação de contratos de interfaces e herança.
- [ ] **LSP Integration**: Motor de análise para suporte em IDEs.

---

### 🚧 Compilação AOT (Ahead-of-Time)

**Timeline:** 12 meses | **Progresso:** 55%
Foco em gerar binários nativos de alta performance (10-50x mais rápidos que bytecode) sem dependência de runtime externo.

- **M1: Fundações e "Hello World"** (Meses 1-2): Geração de IR e estrutura ELF básica funcional.
- **M2: Linux ELF Completo** (Meses 2-4): Executáveis ELF com suporte a funções, strings e cross-compilation.
- **M3: Windows PE Completo** (Meses 4-6): Geração de arquivos PE/COFF e integração com APIs Windows (kernel32).
- **M4: OOP e GC Nativo** (Meses 6-8): Implementação de vtables, Garbage Collector compilado e exceções nativas.
- **M5: Otimizações SSA** (Meses 9-10): Constant folding, dead code elimination e performance competitiva com C.
- **M6: v1.0 Estável** (Meses 11-12): Debug info (DWARF/PDB) e lançamento oficial do comando `dryad build`.

---

### 📋 Épicos Futuros (v2.0+)

#### [E6] Otimizações de Runtime

- **JIT Compilation**: Compilação dinâmica de trechos quentes do bytecode.
- **Zero-copy Lexer**: Melhoria massiva na velocidade de parsing.
- **Generational GC**: Redução de pauses em programas grandes.

#### [E7] Ferramentas Enterprise

- **LSP (Language Server Protocol)**: Autocomplete e navegação em IDEs.
- **Debugger Interativo**: Suporte a breakpoints e inspeção de variáveis.
- **Profiler Nativo**: Análise de gargalos de CPU e Memória.

---

## Progresso por Fase

| Fase       | Objetivo                | Status  |
| ---------- | ----------------------- | ------- |
| **Fase 0** | Core da Linguagem (AST) | ✅ 100% |
| **Fase 1** | Bytecode e Segurança    | ✅ 100% |
| **Fase 2** | AOT e Static Checker    | 🚧 55%  |
| **Fase 3** | Otimizações e Tooling   | ✅ 100% |

---

## Visão de Longo Prazo

Dryad aspira ser uma linguagem que combina a simplicidade de scripting with a performance de sistemas, oferecendo:

1. **Performance nativa** via AOT/JIT.
2. **Segurança por design** com sandboxing configurável.
3. **User Experience** superior para desenvolvedores com ferramentas integradas.

---

## Referências

- **TODOs Detalhados**: Ver [todo.md](todo.md)
- **Histórico de Trabalho**: Ver [done.md](done.md)
- **Manuais Técnicos**: `docs/develop/`
