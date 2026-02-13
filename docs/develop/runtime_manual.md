---
title: "Manual do Runtime"
description: "Detalhes técnicos sobre a execução da AST e gestão de memória."
category: "Desenvolvimento"
order: 4
---

# Manual Técnico: Runtime (Interpretador)

**Localização**: `crates/dryad_runtime/`
**Responsável**: Executar a Árvore Sintática Abstrata (AST), gerenciar o ciclo de vida da memória e orquestrar a concorrência nativa.

## 🚀 Leitura Rápida

- **Modelo**: Interpretador _Tree-Walking_ (percorre a AST diretamente).
- **Tipagem**: Dinâmica, baseada na enum `Value`.
- **Memória**: Gerenciada por contagem de referências atômicas (`Arc`).
- **Concorrência**: Suporte a Threads nativas do SO com isolamento de memória.

---

## ⚙️ Visão Técnica

O Runtime do Dryad atua como a ponte entre a semântica da linguagem e os recursos físicos do hardware. Ele é implementado em Rust para herdar performance próxima ao C sem os riscos de segurança de memória comuns em interpretadores legados.

### 1. Modelo de Execução (Tree-Walking)

Diferente de VMs baseadas em Bytecode (como a JVM ou Lua), o Dryad atualmente utiliza um padrão de projeto **Visitor** para percorrer os nós da AST.

> [!NOTE]
> **Trade-off**: O _Tree-Walking_ é mais fácil de implementar e depurar, mas sofre com o _overhead_ de chamadas recursivas do Rust e acessos dispersos na memória. No futuro, planejamos migrar para uma VM baseada em registradores.

### 2. Representação de Dados (`Value`)

Todos os dados no Dryad residem na enum `Value`.

- **Heap vs Stack**: Tipos grandes (Strings, Arrays, Objetos) são envolvidos em `Arc<T>` ou `Box<T>`, permitindo compartilhamento seguro entre threads sem cópias custosas.
- **Copy-on-Write (CoW)**: Otimizamos mutações em strings para evitar realocações desnecessárias.

### 3. Gerenciamento de Escopo Léxico

O runtime mantém uma pilha de ambientes (`Environment`). Cada ambiente é essencialmente um `HashMap` protegido por um `Arc<RwLock<...>>` se houver necessidade de compartilhamento.

| Camada             | Estrutura Interna     | Paralelo em Rust             |
| :----------------- | :-------------------- | :--------------------------- |
| **Global Scope**   | Static Map / Root Env | `lazy_static!` ou `OnceCell` |
| **Function Scope** | Novo Frame na Pilha   | `Box::new(LocalEnv)`         |
| **Closures**       | Variáveis Capturadas  | `Arc<Environment>` aninhado  |

### 4. Concorrência e Paralelismo Real

O Dryad diferencia **Fibras (Concorrência Lógica)** de **Threads (Paralelismo de Hardware)**.

- **Fibras**: Implementadas via Máquina de Estados (Async/await). Referência: [Tokio Tasks](https://tokio.rs/tokio/tutorial/spawning).
- **Threads Nativas**: Utilizam `std::thread` do Rust. O isolamento é garantido pela propriedade de _Ownership_ do Rust, onde o estado deve ser explicitamente clonado ou movido para a nova thread.

---

## 📚 Referências e Paralelos

- **Modelo de Memória**: [Rust `std::sync::Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).
- **Interpretador Estilo Lox**: Baseado na parte I do livro [Crafting Interpreters](https://craftinginterpreters.com/).
- **Teoria de Sistemas**: "Operating Systems: Three Easy Pieces" (Remzi Arpaci-Dusseau) - Seção sobre Concorrência.

---

## 5. Módulos Nativos (FFI Simplificado)

O Dryad permite que funções Rust sejam expostas ao script. Isso é feito através de uma ponte trait-based, onde o Rust registra funções que aceitam `&[Value]` e retornam `Result<Value, Error>`.
