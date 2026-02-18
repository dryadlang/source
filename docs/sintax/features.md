---
title: "Funcionalidades e Roadmap"
description: "Status de implementação dos recursos da linguagem e planos futuros."
category: "Linguagem"
order: 13
---

# Funcionalidades e Roadmap

Acompanhe o status oficial de implementação dos recursos da linguagem Dryad. O projeto transicionou para a arquitetura de **Bytecode VM** para performance estável.

## 🚀 Leitura Rápida

- **Runtime**: Mode híbrido (AST e VM). VM é recomendada para produção.
- **OOP**: Suporte completo a classes, visibilidade (public/private) e interfaces.
- **Segurança**: Sandbox configurável e limites de recursão (E3040) ativos.
- **Checker**: Motor de análise estática v0.1 operacional.

---

## ⚙️ Visão Técnica

### Status de Implementação

Legenda: ✅ 100% | 🚧 Em andamento | 📋 Planejado

#### 1. Core Runtime (Bytecode VM)

- [x] **Executor de Opcodes**: ✅ VM baseada em pilha com 69+ instruções.
- [x] **Gerenciamento de Frames**: ✅ Proteção contra Stack Overflow e limites de chamada.
- [x] **Garbage Collector**: ✅ Sistema Mark-and-Sweep integrado no Heap.
- [x] **Exception Handling**: ✅ `try/catch/finally` integrado ao fluxo do bytecode.

#### 2. Linguagem e OOP

- [x] **Classes**: ✅ Membros estáticos, propriedades e métodos.
- [x] **Visibilidade**: ✅ `public` e `private` funcionais (verificação em runtime).
- [x] **Interfaces**: ✅ Implementação de múltiplos contratos via `implements`.
- [x] **Pattern Matching**: ✅ Match avançado, bindings, guards e rest patterns (`...`).
- [x] **Destructuring & Spread**: ✅ Suporte total em let, const, arrays e funções.

#### 3. Ecossistema e Tooling

- [x] **Oak PM**: ✅ Instalação e execução de pacotes via CLI.
- [x] **Dryad Checker**: ✅ Verificação de tipos para variáveis e assinaturas.
- [w] **AOT Compilation**: 🚧 Protótipo funcional para Windows/Linux (55%).

---

## 📋 Roadmap Detalhado

### 🟡 Fase Atual: Estabilização e FFI (v1.2)

- [ ] **FFI Avançado**: Passagem de structs complexas para bibliotecas C.
- [ ] **Networking Real**: Substituir mocks de TCP/UDP por implementações reais em `tokio`.
- [ ] **Checker OOP**: Verificação estática de contratos de interfaces.

### 📋 Futuro: Performance Extrema (v2.0)

- **JIT Compilation**: Compilação dinâmica de hot paths para código nativo.
- **LSP Server**: Suporte nativo para autocompletion e refatoração em IDEs.

---

## 📚 Referências e Paralelos

- **Evolução Técnica**: Ver [roadmap.md](../implementation/roadmap.md).
- **Trabalho Concluído**: Ver [done.md](../implementation/done.md).
- **Dívida Técnica**: Ver [todo.md](../implementation/todo.md).
