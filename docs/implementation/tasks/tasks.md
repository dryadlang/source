---
title: "Fila de Tarefas"
description: "Roadmap técnico e tarefas pendentes no desenvolvimento do Dryad."
category: "Projeto"
order: 1
---

# Task Queue

Lista linear de tarefas ordernadas por **prioridade técnica** e **dependências**. Siga esta ordem para evitar bloqueios.

---

## ✅ Concluídas (Previously Immediate Priority)

### 1. [T1.1] Sandbox: Remover `native_exec` Inseguro ✅

- **Status**: Concluído. Flags de segurança e sandbox implementados.

### 2. [T1.3] Runtime: Limite de Recursão ✅

- **Status**: Concluído. Erro `E3040` (StackOverflow) implementado em `Interpreter`.

### 3. [T1.2] Oak: Refatoração do `main.rs` ✅

- **Status**: Concluído. Código modularizado em `commands/` e `core/`.

### 4. [T1.4] Runtime: Modularização do Interpretador ✅

- **Status**: Concluído. Extração de `Environment` e `NativeRegistry`. Implementação de GC Automático.

---

## 🚧 Prioridade Alta (Features Essenciais)

### 4. [T3.1] Stdlib: Arrays Nativos v2 ✅

- **Status**: Concluído. Todos os métodos básicos, funcionais (map, filter, reduce), busca (find, includes) e utilitários (unique, zip, groupBy, flat) implementados em `interpreter.rs`.

### 5. [T2.1] Oak: Validação de Checksum

- **Dependência**: T1.2
- **Descrição**: Garantir integridade dos pacotes baixados.
- **Ação**:
  1. Calcular SHA-256 do arquivo baixado em `install_package`.
  2. Comparar com o hash fornecido pelo registry.
  3. Abortar se falhar.

---

## 📆 Prioridade Média (Expansão)

### 6. [T3.4] Sintaxe: Template Strings

- **Dependência**: Nenhuma (Alteração Lexer/Parser)
- **Descrição**: Suportar `${var}`.
- **Ação**:
  1. Lexer: Identificar backticks e interpolação.
  2. Parser: Transformar em concatenação de strings na AST.

### 7. [T4.2] Runtime: Async File I/O

- **Dependência**: Nenhuma
- **Descrição**: I/O bloqueante trava a thread principal.
- **Ação**:
  1. Substituir `std::fs` por `tokio::fs` em `native_modules/file_io.rs`.
  2. Atualizar assinaturas das funções nativas para `async`.

---

## 🔮 Prioridade Baixa (Longo Prazo/Complexo)

### 8. [T6.1] Oak: Publish Command

- **Dependência**: T1.2, T2.1
- **Descrição**: Envio de pacotes para servidor remoto.
- **Ação**:
  1. Implementar autenticação (Token).
  2. Empacotar diretório em `.tar.gz`.
  3. Upload via HTTP POST para API do Registry.

### 9. [T5.1] Bytecode VM (Spike)

- **Dependência**: Nenhuma (Projeto paralelo)
- **Descrição**: Prototipar uma VM baseada em pilha para substituir o interpretador atual no futuro.
