---
title: "Fila de Tarefas"
description: "Roadmap técnico e tarefas pendentes no desenvolvimento do Dryad."
category: "Projeto"
order: 1
---

# Task Queue

Lista linear de tarefas ordernadas por **prioridade técnica** e **dependências**. Siga esta ordem para evitar bloqueios.

---

## 🚀 Prioridade Imediata (Refactor Critical)

### 1. [T1.1] Sandbox: Remover `native_exec` Inseguro

- **Dependência**: Nenhuma
- **Descrição**: O comando `native_exec` permite RCE. Removê-lo ou protegê-lo com uma flag de permissão.
- **Ação**:
  1. Modificar `crates/dryad_runtime/src/native_modules/system_env.rs`: Adicionar flag `--allow-unsafe` no interpretador.
  2. Se flag não estiver ativa, `native_exec` deve lançar exceção.

### 2. [T1.3] Runtime: Limite de Recursão (Stack Overflow Fix)

- **Dependência**: Nenhuma
- **Descrição**: Evitar crashes rust-level em scripts recursivos.
- **Ação**:
  1. Implementar contador de profundidade (`call_depth`) em `Interpreter`.
  2. Adicionar `MAX_RECURSION_DEPTH` constante (ex: 1000).
  3. Lançar `RuntimeError::StackOverflow` se excedido.

### 3. [T1.2] Oak: Refatoração do `main.rs` (Monólito)

- **Dependência**: Nenhuma
- **Descrição**: O arquivo `crates/oak/src/main.rs` está inavegável.
- **Ação**:
  1. Criar pastas `src/commands`, `src/core`.
  2. Mover lógica de cada subcomando para `src/commands/<cmd>.rs`.
  3. Mover structs de config para `src/core/config.rs`.

---

## 🚧 Prioridade Alta (Features Essenciais)

### 4. [T3.1] Stdlib: Arrays Nativos v2

- **Dependência**: Nenhuma
- **Descrição**: Arrays precisam de métodos funcionais, utilitários e avançados para manipulação de dados.
- **Ação**:
  1. **Básicos:** `push(value)`, `pop()`, `shift()`, `unshift(value)`, `length()`.
  2. **Mapeamento e filtragem:** `map(fn)`, `filter(fn)`, `forEach(fn)`, `reduce(fn, initial)`, `reduceRight(fn, initial)`.
  3. **Busca e inspeção:** `includes(value)`, `indexOf(value)`, `lastIndexOf(value)`, `find(fn)`, `findIndex(fn)`, `every(fn)`, `some(fn)`.
  4. **Transformação e ordenação:** `sort(fn)`, `reverse()`, `slice(start, end)`, `concat(array)`, `join(separator)`.
  5. **Avançados / utilitários:**
     - `unique()` – retorna um array sem duplicatas.
     - `flatten(depth)` – achata arrays aninhados até a profundidade especificada.
     - `chunk(size)` – divide o array em subarrays de tamanho fixo.
     - `groupBy(fn)` – agrupa elementos baseado no retorno da função.
     - `zip(array2, ...)` – combina múltiplos arrays em pares de elementos.
     - `reverseMap(fn)` – aplica função e inverte o resultado.
     - `fill(value, start?, end?)` – preenche valores em intervalos.
     - `copyWithin(target, start, end)` – copia uma parte do array para outra posição.

  6. Expor **todos os métodos** como nativos no Runtime para o tipo `Value::Array` em Rust.

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
