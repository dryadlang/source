# Dryad Project Roadmap

Este documento apresenta o backlog estratégico do projeto, organizado por épicos. Ele funciona como um índice central para o planejamento de longo prazo.

## 🗺️ Visão Geral

O objetivo é transformar a linguagem Dryad de um protótipo funcional (v1.0) para uma linguagem robusta, segura e performática (v2.0), com um ecossistema de ferramentas maduro.

---

## 🟢 Épicos Ativos (Fase 1: Estabilização e Segurança)

Foco em resolver débitos técnicos críticos, melhorar a segurança e estabilizar a API existente.

### [E1] Refatoração Estrutural e Segurança
*Baseado em: `structural_refactor/danger.md` e `structural_refactor/refactor.md`*
* **Objetivo**: Eliminar riscos de RCE, Stack Overflow e melhorar a manutenibilidade do código.
* **Tasks Relacionadas**:
    - [ ] [T1.1] Sandbox de Execução Nativa (Remover `native_exec` inseguro)
    - [ ] [T1.2] Refatoração do Monólito Oak (Dividir `main.rs`)
    - [ ] [T1.3] Proteção contra Stack Overflow (Recursion Limit)
    - [ ] [T1.4] Thread Safety no Runtime (Migração `Rc` -> `Arc`)

### [E2] Oak Package Manager - Core
*Baseado em: `tracking/missing.md`*
* **Objetivo**: Tornar o gerenciamento de dependências confiável e seguro.
* **Tasks Relacionadas**:
    - [ ] [T2.1] Validação de Checksum/Integridade
    - [ ] [T2.2] Implementação de Semantic Versioning Real
    - [ ] [T2.3] Lockfile Determinístico (Correções)

---

## 🟡 Épicos Planejados (Fase 2: Expansão da Linguagem)

Introdução de features que faltam para paridade com linguagens modernas.

### [E3] Evolução da Sintaxe e Tipos
*Baseado em: `tracking/missing.md` e `tracking/features.md`*
* **Objetivo**: Melhorar a ergonomia e expressividade da linguagem.
* **Tasks Relacionadas**:
    - [ ] [T3.1] Arrays Nativos Completos (Métodos `.map`, `.filter`)
    - [ ] [T3.2] Pattern Matching (`match`)
    - [ ] [T3.3] Destructuring e Spread Operator
    - [ ] [T3.4] Template Strings

### [E4] Expansão da Standard Library
*Baseado em: `tracking/features.md`*
* **Objetivo**: Fornecer ferramentas essenciais para desenvolvimento backend.
* **Tasks Relacionadas**:
    - [ ] [T4.1] Servidor HTTP/TCP Robusto
    - [ ] [T4.2] Async File I/O (`tokio::fs`)
    - [ ] [T4.3] Driver de Banco de Dados (SQLite/Postgres)

---

## 🔴 Épicos Futuros (Fase 3: Performance e Ecossistema)

Features complexas que exigem mudanças arquiteturais profundas.

### [E5] Otimização e Runtime
*Baseado em: `structural_refactor/refactor.md`*
* **Objetivo**: Aumentar a performance de execução em 10x+.
* **Tasks Relacionadas**:
    - [ ] [T5.1] Bytecode VM (Substituir Tree-Walk Interpreter)
    - [ ] [T5.2] Lexer Otimizado (Zero-copy)
    - [ ] [T5.3] Garbage Collector (Mark-and-Sweep)

### [E6] Ecossistema Enterprise
*Baseado em: `tracking/features.md`*
* **Objetivo**: Ferramental para grandes times e projetos.
* **Tasks Relacionadas**:
    - [ ] [T6.1] Central Package Registry (Backend)
    - [ ] [T6.2] Language Server Protocol (LSP)
    - [ ] [T6.3] Debugger Interativo
