---
title: "Compilador de Bytecode e JIT"
description: "Sistema de compilação para bytecode e JIT do Dryad."
category: "Projeto"
order: 10
---

# Compilador de Bytecode e JIT

## Visão Geral

O Dryad agora suporta múltiplos modos de execução:

1. **Interpretador** (padrão): Executa o código diretamente a partir do AST
2. **Compilador de Bytecode**: Compila o código para bytecode antes de executar
3. **JIT Compiler**: Compila funções quentes para código nativo em tempo de execução (experimental)

## Uso

```bash
# Modo interpretador (padrão)
dryad run script.dryad

# Compila para bytecode antes de executar
dryad run script.dryad --compile

# Usa compilação JIT (experimental)
dryad run script.dryad --jit
```

## 1. Bytecode Compiler

### Descrição

O compilador de bytecode traduz o AST (Abstract Syntax Tree) para instruções de uma máquina virtual baseada em pilha. O bytecode é então executado por uma VM eficiente.

### Benefícios

- **Inicialização mais rápida**: O parsing e validação são feitos uma vez
- **Execução mais rápida**: Bytecode é mais eficiente que interpretar AST diretamente
- **Portabilidade**: Bytecode pode ser salvo e executado sem re-parsing

### Implementação

O compilador de bytecode será implementado como um novo crate `dryad_bytecode`:

```
crates/
  dryad_bytecode/
    src/
      vm.rs         # Máquina virtual baseada em pilha
      compiler.rs   # Compilador AST -> Bytecode
      opcode.rs     # Definição de opcodes
      chunk.rs      # Representação de bytecode em chunks
```

### Opcodes Planejados

```rust
enum OpCode {
    // Constantes
    Constant(u8),        // Carrega constante do chunk
    ConstantLong(u24),  // Carrega constante grande
    
    // Operações aritméticas
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    
    // Comparações
    Equal,
    Greater,
    Less,
    
    // Controle de fluxo
    Jump(u16),           // Pulo incondicional
    JumpIfFalse(u16),   // Pulo condicional
    Loop(u16),          // Pulo para trás (loops)
    
    // Funções
    Call(u8),           // Chama função
    Return,             // Retorna de função
    
    // Escopo
    DefineGlobal(u8),   // Define variável global
    GetGlobal(u8),      // Carrega variável global
    SetGlobal(u8),     // Define variável global
    GetLocal(u8),      // Carrega variável local
    SetLocal(u8),      // Define variável local
    
    // Objetos
    Class(u8),          // Cria classe
    Method(u8),         // Define método
    Invoke(u8),         // Chama método
    
    // Pilha
    Pop,                // Remove topo da pilha
    Dup,                // Duplica topo da pilha
    
    // Misc
    Null,
    True,
    False,
    Print,
}
```

## 2. JIT Compiler (Just-In-Time)

### Descrição

O compilador JIT compila funções frequentemente executadas ("quentes") para código de máquina nativo em tempo de execução.

### Benefícios

- **Performance nativa**: Código máquina é executado diretamente pela CPU
- **Otimizações específicas**: Pode aplicar otimizações baseadas em profiling em runtime
- **Speculative execution**: Pode fazer suposições otimistas e reverter se necessário

### Arquitetura

```
         +------------------+
         |   Interpreter    |
         +--------+---------+
                  |
         +--------v---------+
         |  Profile Hot     |
         |  Functions       |
         +--------+---------+
                  |
         +--------v---------+
         |  Bytecode Gen    |
         +--------+---------+
                  |
         +--------v---------+
         |  Codegen (LLVM/  |
         |  Cranelift)      |
         +--------+---------+
                  |
         +--------v---------+
         |  Native Code     |
         +------------------+
```

### Escolha de Backend

#### LLVM
- Maturidade e otimizações avançadas
- Suporte a múltiplas arquiteturas
- Overhead de compilação maior

#### Cranelift
- Mais leve e rápido para compilação incremental
- Excelente para JIT
- Em desenvolvimento ativo ( usado pelo Rust)

### Métricas de Performance

**Meta**: 10x mais rápido que o interpretador atual em benchmarks end-to-end.

## 3. Sistema Híbrido

### Estratégia

1. **Warm-up**: Inicia executando via interpretador
2. **Profiling**: Identifica funções executadas frequentemente
3. **Bytecode**: Compila para bytecode após N execuções
4. **JIT**: Promove funções quentes para código nativo após M execuções

### Limites Configuráveis

```dryad
// Configurar thresholds via CLI
dryad run script.dryad --compile --jit --bytecode-threshold=5 --jit-threshold=10
```

## 4. Roadmap de Implementação

### Fase 1: Bytecode VM (T5.1) ✅ Em progresso

- [ ] Definir opcode set
- [ ] Implementar VM baseada em pilha
- [ ] Implementar compilador AST -> Bytecode
- [ ] Serialização de bytecode
- [ ] Integração com CLI

### Fase 2: Otimizações de Bytecode

- [ ] Constant folding em tempo de compilação
- [ ] Inline de funções pequenas
- [ ] peephole optimization

### Fase 3: JIT Compiler (3.1)

- [ ] Sistema de profiling
- [ ] Integração com Cranelift
- [ ] Compilação de funções quentes
- [ ] Desotimização quando necessário

## 5. Documentação Técnica

### Formato de Bytecode

O bytecode é salvo em chunks com a seguinte estrutura:

```
+------------------+
|   Header        |  # Magic number, version
+------------------+
|   Constants     |  # Tabela de constantes
+------------------+
|   Functions     |  # Definições de funções
+------------------+
|   Code Chunks   |  # Opcodes por função
+------------------+
|   Debug Info    |  # Nomes de variáveis, linhas
+------------------+
```

### VM Stack-Based

A VM usa uma pilha de valores para execução:

```
[Stack]
  +---------+
  |   val   | <- Top
  +---------+
  |   val   |
  +---------+
  |   ...   |
  +---------+
```

### Chamada de Função

1. Empilha argumentos
2. Call opcode empilha frame
3. Executa corpo da função
4. Return desempilha frame e retorna valor

---

## Status Atual

| Feature | Status |
|---------|--------|
| CLI flags | ✅ Implementado |
| Bytecode VM | ⏳ Planejado |
| JIT Compiler | ⏳ Planejado |

**Meta de versão**: v0.4.0 para primeira versão funcional do compilador de bytecode.
