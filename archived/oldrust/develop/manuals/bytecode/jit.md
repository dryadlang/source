---
title: "Bytecode Compiler"
description: "Sistema de compilação bytecode do Dryad - foco atual. JIT é futuro."
category: "Projeto"
order: 10
---

# Bytecode Compiler

> **Nota**: Este documento foca no **Bytecode** (`--compile`). O JIT está planejado para o futuro mas **NÃO é prioridade** atual.

## Visão Geral

O Dryad agora suporta múltiplos modos de execução:

1. **Interpretador** (padrão): Executa o código diretamente a partir do AST
2. **Compilador de Bytecode**: Compila o código para bytecode antes de executar (✅ Implementado)

## Uso

```bash
# Modo interpretador (padrão)
dryad run script.dryad

# Compila para bytecode antes de executar
dryad run script.dryad --compile
```

## 1. Bytecode Compiler (✅ Implementado)

### Descrição

O compilador de bytecode traduz o AST (Abstract Syntax Tree) para instruções de uma máquina virtual baseada em pilha. O bytecode é então executado por uma VM eficiente.

### Benefícios

- **Inicialização mais rápida**: O parsing e validação são feitos uma vez
- **Execução mais rápida**: Bytecode é mais eficiente que interpretar AST diretamente
- **Portabilidade**: Bytecode pode ser salvo e executado sem re-parsing
- **Cache-friendly**: Acesso sequencial às instruções

### Implementação

O compilador de bytecode foi implementado no crate `dryad_bytecode`:

```
crates/
  dryad_bytecode/
    src/
      lib.rs         # API pública
      vm.rs          # Máquina virtual baseada em pilha
      compiler.rs    # Compilador AST -> Bytecode
      opcode.rs      # Definição de opcodes
      chunk.rs       # Representação de bytecode em chunks
      value.rs       # Sistema de tipos dinâmicos
      debug.rs       # Disassembler para debug
```

### Opcodes Implementados

```rust
pub enum OpCode {
    // Constantes
    Constant(u8),        // Carrega constante do chunk
    ConstantLong(u16),   // Carrega constante grande
    Nil, True, False,
    
    // Operações aritméticas
    Add, Subtract, Multiply, Divide, Modulo, Negate,
    
    // Comparações
    Equal, Greater, Less, GreaterEqual, LessEqual,
    
    // Operações lógicas
    Not, And, Or,
    
    // Operações bitwise
    BitAnd, BitOr, BitXor, BitNot, ShiftLeft, ShiftRight,
    
    // Controle de fluxo
    Jump(u16),           // Pulo incondicional
    JumpIfFalse(u16),    // Pulo condicional
    JumpIfTrue(u16),     // Pulo condicional
    Loop(u16),           // Pulo para trás (loops)
    Break, Continue,
    
    // Variáveis
    DefineGlobal(u8),    // Define variável global
    GetGlobal(u8),       // Carrega variável global
    SetGlobal(u8),       // Define variável global
    GetLocal(u8),        // Carrega variável local
    SetLocal(u8),        // Define variável local
    
    // Funções (parcial)
    Call(u8),            // Chama função
    Return,              // Retorna de função
    Closure(u8),         // Cria closure
    GetUpvalue(u8),      // Carrega upvalue
    SetUpvalue(u8),      // Define upvalue
    CloseUpvalue,        // Fecha upvalues
    
    // Objetos (parcial)
    Class(u8),           // Cria classe
    Method(u8),          // Define método
    Invoke(u8),          // Chama método
    GetProperty(u8),     // Acessa propriedade
    SetProperty(u8),     // Define propriedade
    This,                // Referência 'this'
    Super(u8),           // Referência 'super'
    
    // Coleções (parcial)
    Array(u16),          // Cria array
    Index,               // Acessa índice
    SetIndex,            // Define índice
    Tuple(u8),           // Cria tuple
    TupleAccess(u8),     // Acessa elemento de tuple
    
    // Pilha
    Pop, PopN(u8), Dup, DupN(u8), Swap,
    
    // I/O e debug
    Print, PrintLn, Nop, Halt,
}
```

### Exemplo de Bytecode Gerado

**Código fonte:**
```dryad
let x = 10;
let y = 20;
print x + y;
```

**Bytecode:**
```
== script ==
Constants:
  [   0] '10'
  [   1] '20'
  [   2] 'x'
  [   3] 'y'

Bytecode:
0000    1 CONSTANT       0 '10'
0002    1 DEFINE_GLOBAL  2 'x'
0004    2 CONSTANT       1 '20'
0006    2 DEFINE_GLOBAL  3 'y'
0008    3 GET_GLOBAL     2 'x'
0010    3 GET_GLOBAL     3 'y'
0012    3 ADD
0013    3 PRINT_LN
0014    3 NIL
0015    3 RETURN
```

### API de Uso

```rust
use dryad_bytecode::{Compiler, VM, InterpretResult};
use dryad_parser::Parser;

// Parse do código fonte
let program = Parser::new(tokens).parse()?;

// Compilação
let mut compiler = Compiler::new();
let chunk = compiler.compile(program)?;

// Debug
chunk.disassemble("script");

// Execução
let mut vm = VM::new();
match vm.interpret(chunk) {
    InterpretResult::Ok => println!("Sucesso!"),
    InterpretResult::RuntimeError => eprintln!("Erro em tempo de execução"),
    InterpretResult::CompileError => eprintln!("Erro de compilação"),
}
```

## 2. JIT Compiler (Just-In-Time) - Futuro

⚠️ **NOTA**: O JIT é uma feature futura e **NÃO** é prioridade atual. O foco deve ser em estabilizar e otimizar o bytecode primeiro.

### Descrição

O compilador JIT compila funções frequentemente executadas ("quentes") para código de máquina nativo em tempo de execução.

### Por que não implementar JIT agora?

1. **Complexidade muito alta**: Requer integração com Cranelift/LLVM
2. **Overhead de warm-up**: Para scripts pequenos, pode ser mais lento
3. **Manutenção difícil**: Bugs em JIT são difíceis de debugar
4. **Bytecode já é suficiente**: 2-3x mais rápido que AST, sem complexidade do JIT

### Quando implementar?

- Após bytecode estar 100% funcional
- Quando performance for crítica mesmo com bytecode
- Quando tivermos recursos para manter código JIT

### Arquitetura Planejada (Futuro)

```
         +------------------+
         |   Bytecode VM    |
         +--------+---------+
                  |
         +--------v---------+
         |  Profile Hot     |
         |  Functions       |
         +--------+---------+
                  |
         +--------v---------+
         |  Codegen (       |
         |  Cranelift)      |
         +--------+---------+
                  |
         +--------v---------+
         |  Native Code     |
         +------------------+
```

## 3. Roadmap de Implementação

### Fase 1: Bytecode VM (T5.1) ✅ Concluído

- [x] Definir opcode set
- [x] Implementar VM baseada em pilha
- [x] Implementar compilador AST -> Bytecode
- [x] Disassembler para debug
- [x] Integração com CLI (flags --compile)
- [ ] Serialização de bytecode

### Fase 2: Features Avançadas (T5.2) 🚧 Em Progresso

- [ ] Funções e closures completos
- [ ] Classes e objetos
- [ ] Try/catch/exceções
- [ ] ForEach
- [ ] Incremento/decremento

### Fase 3: Otimizações de Bytecode (T5.3)

- [ ] Constant folding em tempo de compilação
- [ ] Inline de funções pequenas
- [ ] Peephole optimization
- [ ] Dead code elimination

### Fase 4: JIT Compiler (T6)

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
| CLI flag (--compile) | ✅ Implementado |
| Sistema de opcodes | ✅ Implementado |
| VM baseada em pilha | ✅ Implementado |
| Compilador AST -> Bytecode | ✅ Implementado (parcial) |
| Variáveis locais e globais | ✅ Implementado |
| Controle de fluxo | ✅ Implementado |
| Disassembler | ✅ Implementado |
| Integração com runtime | ✅ Implementado |
| Funções | 🚧 Em progresso |
| Classes | ⏳ Planejado |
| JIT Compiler | ⏳ Futuro (não prioridade) |
| Serialização de bytecode | ⏳ Planejado |

**Meta de versão**: v0.4.0 para primeira versão funcional do compilador de bytecode.

## Recursos Adicionais

- [Guia de Integração](./BYTECODE_INTEGRATION.md) - Como usar o bytecode na prática
- [Documentação de Implementação](./BYTECODE_IMPLEMENTATION.md) - Detalhes técnicos
- [Guia do Desenvolvedor](../../manuals/DEVELOPER_MANUAL.md)
- [Especificação da Linguagem](../../manuals/SYNTAX.md)
