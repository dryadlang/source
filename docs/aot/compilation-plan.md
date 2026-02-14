# Plano de Compilação AOT (Ahead-of-Time) para Binários Nativos

## Visão Geral

Este documento descreve o plano para compilar código Dryad diretamente para **executáveis nativos** (Linux ELF e Windows PE/COFF), eliminando a necessidade do interpretador/runtime durante a execução.

## Objetivos

1. Gerar executáveis standalone (`.exe` no Windows, sem extensão no Linux)
2. Performance máxima (código de máquina nativo)
3. Distribuição simplificada (apenas o binário, sem runtime)
4. Suporte a x86_64 e ARM64

## Arquitetura Geral

```
Código Fonte (.dryad)
    ↓
Parser → AST
    ↓
Bytecode Compiler
    ↓
AOT Compiler (LLVM/Cranelift)
    ↓
Código de Máquina Nativo
    ↓
Linker + Runtime Embutido
    ↓
Executável Nativo (ELF/PE)
```

## Componentes Necessários

### 1. Backend de Geração de Código

#### Opção A: LLVM (Recomendada)
**Vantagens:**
- Maturidade e otimizações avançadas
- Suporte a múltiplas arquiteturas
- Eco-sistema vasto

**Desvantagens:**
- Overhead de compilação
- Complexidade alta

**Uso:**
```rust
// Compilar usando LLVM
dryad build script.dryad --target=x86_64-linux-gnu -o meu_programa
```

#### Opção B: Cranelift (Alternativa)
**Vantagens:**
- Mais rápido para compilar
- Escrito em Rust
- Usado pelo Wasmtime

**Desvantagens:**
- Menos otimizações que LLVM
- Mais novo (menos maduro)

### 2. Formato de Executável

#### Linux - ELF (Executable and Linkable Format)

**Estrutura do ELF:**
```
┌─────────────────────────────────────┐
│ ELF Header                          │
│ (64 bytes)                          │
├─────────────────────────────────────┤
│ Program Header Table                │
│ (Segmentos para carregar)           │
├─────────────────────────────────────┤
│ .text section                       │
│ (Código de máquina)                 │
├─────────────────────────────────────┤
│ .rodata section                     │
│ (Constantes, strings)               │
├─────────────────────────────────────┤
│ .data section                       │
│ (Variáveis globais inicializadas)   │
├─────────────────────────────────────┤
│ .bss section                        │
│ (Variáveis globais não inicializadas)│
├─────────────────────────────────────┤
│ .dynsym / .symtab                   │
│ (Tabela de símbolos)                │
├─────────────────────────────────────┤
│ Section Header Table                │
│ (Metadados)                         │
└─────────────────────────────────────┘
```

**Requisitos ELF:**
- Entry point: `_start` ou `main`
- Linkagem dinâmica: libc (para I/O)
- Ou linkagem estática (standalone)

#### Windows - PE (Portable Executable)

**Estrutura do PE:**
```
┌─────────────────────────────────────┐
│ DOS Header (64 bytes)               │
│ "MZ" magic                          │
├─────────────────────────────────────┤
│ PE Signature "PE\0\0"               │
├─────────────────────────────────────┤
│ COFF Header                         │
│ (Informações do arquivo)            │
├─────────────────────────────────────┤
│ Optional Header                     │
│ (Entry point, subsistema, etc)      │
├─────────────────────────────────────┤
│ Section Table                       │
│ (Headers das seções)                │
├─────────────────────────────────────┤
│ .text section                       │
│ (Código executável)                 │
├─────────────────────────────────────┤
│ .rdata section                      │
│ (Dados somente leitura)             │
├─────────────────────────────────────┤
│ .data section                       │
│ (Dados inicializados)               │
├─────────────────────────────────────┤
│ .idata section                      │
│ (Tabela de imports)                 │
├─────────────────────────────────────┤
│ .reloc section                      │
│ (Relocações)                        │
└─────────────────────────────────────┘
```

## Estratégia de Implementação

### Fase 1: Geração de Código (Mês 1-2)

#### 1.1 IR Intermediário
Criar uma representação intermediária próxima do código de máquina:

```rust
// Intermediate Representation
enum IrInstruction {
    // Movimentação
    LoadReg(u8, Value),           // reg = value
    StoreReg(u8, u8),             // reg1 = reg2
    LoadMemory(u8, u64),          // reg = memory[addr]
    StoreMemory(u64, u8),         // memory[addr] = reg
    
    // Aritmética
    Add(u8, u8, u8),              // reg3 = reg1 + reg2
    Sub(u8, u8, u8),
    Mul(u8, u8, u8),
    Div(u8, u8, u8),
    
    // Comparação e saltos
    Cmp(u8, u8),
    Jump(u64),
    JumpEqual(u64),
    JumpNotEqual(u64),
    JumpLess(u64),
    JumpGreater(u64),
    
    // Funções
    Call(u64),
    Ret,
    Push(u8),
    Pop(u8),
    
    // Syscalls (Linux)
    Syscall,                      // syscall para I/O
}
```

#### 1.2 Mapeamento Bytecode → IR
Converter cada opcode do bytecode para instruções IR:

```rust
fn bytecode_to_ir(opcode: OpCode) -> Vec<IrInstruction> {
    match opcode {
        OpCode::Add => vec![
            IrInstruction::Pop(0),           // Pop b para reg0
            IrInstruction::Pop(1),           // Pop a para reg1
            IrInstruction::Add(2, 1, 0),     // reg2 = reg1 + reg0
            IrInstruction::Push(2),          // Push resultado
        ],
        OpCode::Constant(idx) => vec![
            IrInstruction::LoadConst(0, idx),
            IrInstruction::Push(0),
        ],
        // ... outros opcodes
    }
}
```

### Fase 2: Backend x86_64 (Mês 2-3)

#### 2.1 Instruções x86_64
Gerar código assembly x86_64:

```asm
; Exemplo: add(a, b)
; Stack-based calling convention

section .text
global _start

_start:
    ; Setup
    xor rbp, rbp
    mov rdi, [rsp]      ; argc
    lea rsi, [rsp+8]    ; argv
    
    ; Call main
    call main
    
    ; Exit
    mov rdi, rax        ; exit code
    mov rax, 60         ; sys_exit
    syscall

main:
    push rbp
    mov rbp, rsp
    
    ; var x = 10
    mov rax, 10
    push rax
    
    ; var y = 20
    mov rax, 20
    push rax
    
    ; x + y
    pop rbx             ; y
    pop rax             ; x
    add rax, rbx
    push rax            ; resultado
    
    ; print(resultado)
    call print_number
    
    ; return 0
    xor rax, rax
    mov rsp, rbp
    pop rbp
    ret

print_number:
    ; Converte número para string e imprime
    ; usando syscall write
    push rbp
    mov rbp, rsp
    
    mov rax, 1          ; sys_write
    mov rdi, 1          ; stdout
    lea rsi, [number_str]
    mov rdx, number_len
    syscall
    
    mov rsp, rbp
    pop rbp
    ret

section .data
number_str: db "42", 10
number_len: equ $ - number_str
```

#### 2.2 Uso de Registradores
**Convenção de chamada System V AMD64 ABI (Linux):**
- Argumentos: RDI, RSI, RDX, RCX, R8, R9
- Retorno: RAX
- Preservados: RBX, RBP, R12-R15
- Voláteis: RAX, RCX, RDX, RSI, RDI, R8-R11

**Estratégia:**
- Usar stack-based VM no início
- Otimizar para registradores depois

### Fase 3: Runtime Embutido (Mês 3-4)

#### 3.1 Funções Necessárias
O executável precisa incluir:

```rust
// Runtime mínimo em C/Rust

// Memory management
void* dryad_alloc(size_t size);
void dryad_free(void* ptr);

// I/O
void dryad_print(const char* str);
void dryad_print_number(double n);
char* dryad_read_line();

// Strings
char* dryad_string_concat(const char* a, const char* b);
int dryad_string_length(const char* s);

// Arrays
void* dryad_array_new(size_t capacity);
void dryad_array_push(void* arr, void* item);
void* dryad_array_get(void* arr, size_t index);

// Exceptions
void dryad_throw(const char* msg);
void dryad_catch(void (*handler)(const char*));
```

#### 3.2 Implementação do Runtime

**Opção 1: Runtime em C (Recomendada)**
```c
// dryad_runtime.c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void dryad_print(const char* str) {
    printf("%s", str);
}

void dryad_print_number(double n) {
    printf("%f\n", n);
}

void* dryad_alloc(size_t size) {
    return malloc(size);
}

// ... outras funções
```

Compilar como biblioteca estática:
```bash
gcc -c -O2 -fPIC dryad_runtime.c -o dryad_runtime.o
ar rcs libdryad_runtime.a dryad_runtime.o
```

**Opção 2: Runtime em Rust**
```rust
// runtime/lib.rs
#[no_mangle]
pub extern "C" fn dryad_print(s: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(s) };
    print!("{}", c_str.to_string_lossy());
}

#[no_mangle]
pub extern "C" fn dryad_print_number(n: f64) {
    println!("{}", n);
}
```

### Fase 4: Linkagem (Mês 4-5)

#### 4.1 Gerar Objeto
```bash
# Compilar código Dryad para objeto
dryac compile script.dryad -o script.o --target=x86_64-linux-gnu

# Linkar com runtime
ld -o script script.o -L. -ldryad_runtime -lc
# ou
gcc -o script script.o -L. -ldryad_runtime
```

#### 4.2 Linkagem Estática vs Dinâmica

**Estática (Recomendada para distribuição):**
```bash
gcc -static -o script script.o libdryad_runtime.a -lc
```
- Executável standalone
- Maior tamanho
- Sem dependências externas

**Dinâmica:**
```bash
gcc -o script script.o -ldryad_runtime
```
- Menor tamanho
- Requer libdryad_runtime.so

## Interface de Linha de Comando

### Comando `build`

```bash
# Compilar para Linux x86_64
dryad build script.dryad --target=x86_64-linux-gnu -o meu_app

# Compilar para Windows x86_64
dryad build script.dryad --target=x86_64-windows-gnu -o meu_app.exe

# Compilar para ARM64 Linux
dryad build script.dryad --target=aarch64-linux-gnu -o meu_app

# Compilar estaticamente
dryad build script.dryad --target=x86_64-linux-gnu --static -o meu_app

# Otimizações
dryad build script.dryad -O2 -o meu_app          # Otimização nível 2
dryad build script.dryad -Os -o meu_app          # Otimizar tamanho
```

### Flags

```
--target=<triple>     Target architecture (x86_64-linux-gnu, etc.)
-o, --output=<file>   Output file name
--static              Static linking
-O<level>             Optimization level (0, 1, 2, 3, s, z)
-g                    Include debug symbols
--strip               Strip symbols (smaller binary)
-L<path>              Library search path
-l<lib>               Link library
```

## Implementação Passo a Passo

### Mês 1: Fundações
- [ ] Criar crate `dryad_aot`
- [ ] Definir IR intermediário
- [ ] Implementar conversor Bytecode → IR
- [ ] Criar runtime básico em C

### Mês 2: Backend x86_64
- [ ] Implementar gerador de assembly x86_64
- [ ] Suporte a instruções aritméticas
- [ ] Suporte a controle de fluxo
- [ ] Suporte a funções

### Mês 3: Runtime e Memória
- [ ] Implementar alocador de memória
- [ ] Implementar gerenciamento de strings
- [ ] Implementar arrays dinâmicos
- [ ] Suporte a classes/objetos

### Mês 4: Linkagem e ELF
- [ ] Gerar arquivos objeto ELF
- [ ] Integrar com linker
- [ ] Criar executáveis Linux
- [ ] Testar em múltiplas distros

### Mês 5: Windows PE
- [ ] Suporte a formato PE/COFF
- [ ] Gerar executáveis Windows
- [ ] Testar no Windows 10/11
- [ ] Resolver dependências Windows

### Mês 6: Otimizações
- [ ] Register allocation
- [ ] Inlining de funções
- [ ] Constant folding
- [ ] Dead code elimination

## Exemplo Completo

### Código Fonte
```dryad
// hello.dryad
fn main() {
    print "Hello, World!";
    var x = 10;
    var y = 20;
    print x + y;
}
```

### Compilação
```bash
dryad build hello.dryad -o hello
```

### Resultado
```bash
$ ./hello
Hello, World!
30
$ file hello
hello: ELF 64-bit LSB executable, x86-64, version 1 (SYSV), 
       statically linked, not stripped
```

## Desafios Técnicos

### 1. Garbage Collection
**Problema:** Dryad usa GC, mas executáveis nativos precisam de gerenciamento manual ou RC.

**Soluções:**
- Usar ARC (Atomic Reference Counting)
- Implementar GC simples (mark-and-sweep)
- Deixar para o usuário gerenciar (manual)

### 2. Runtime Size
**Problema:** Runtime completo pode adicionar 500KB-1MB ao executável.

**Soluções:**
- Linkagem dinâmica (shared library)
- Tree shaking (remover código não usado)
- Runtime minimalista

### 3. Debugabilidade
**Problema:** Difícil debugar código gerado.

**Soluções:**
- Gerar DWARF debug info
- Mapeamento bytecode → código nativo
- Stack traces simbólicos

### 4. FFI
**Problema:** Chamar código C/bibliotecas externas.

**Soluções:**
- Suporte a extern functions
- Bindings automáticos (como bindgen)
- ABI compatível com C

## Roadmap

### Fase 1: Protótipo (3 meses)
- Suporte básico a Linux x86_64
- Programas simples (sem GC, sem classes)
- Runtime mínimo

### Fase 2: Completo (6 meses)
- Todas as features da linguagem
- Linux e Windows
- Otimizações básicas

### Fase 3: Produção (12 meses)
- Todas as arquiteturas
- Otimizações avançadas
- Debugging completo
- FFI estável

## Referências

### Formats
- [ELF Format](https://refspecs.linuxfoundation.org/elf/elf.pdf)
- [PE/COFF Format](https://learn.microsoft.com/en-us/windows/win32/debug/pe-format)

### Code Generation
- [LLVM](https://llvm.org/)
- [Cranelift](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)
- [GNU Assembler](https://sourceware.org/binutils/docs/as/)

### Calling Conventions
- [System V AMD64 ABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
- [Windows x64 Calling Convention](https://learn.microsoft.com/en-us/cpp/build/x64-calling-convention)

## Conclusão

A compilação AOT para binários nativos é um projeto ambicioso que levará ~12 meses para estar completo. O resultado será:

- ✅ Executáveis standalone
- ✅ Performance máxima
- ✅ Distribuição fácil
- ✅ Suporte a Linux e Windows

**Próximos passos:**
1. Decidir entre LLVM vs Cranelift
2. Implementar IR intermediário
3. Criar runtime mínimo
4. Protótipo x86_64 Linux
