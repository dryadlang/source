# Formato ELF (Executable and Linkable Format) - Guia Técnico

## Visão Geral

ELF é o formato padrão de executáveis, bibliotecas e objetos no Linux e sistemas Unix-like. Este guia cobre os aspectos essenciais para gerar executáveis ELF a partir do bytecode Dryad.

## Estrutura do ELF

### 1. ELF Header (64 bytes no x86_64)

```c
// elf64.h (simplificado)
#define ELFMAG0 0x7f
#define ELFMAG1 'E'
#define ELFMAG2 'L'
#define ELFMAG3 'F'

typedef struct {
    uint8_t  e_ident[16];     // Magic number e outras infos
    uint16_t e_type;          // Tipo: ET_EXEC (2) para executáveis
    uint16_t e_machine;       // Arquitetura: EM_X86_64 (62)
    uint32_t e_version;       // Versão: EV_CURRENT (1)
    uint64_t e_entry;         // Entry point (endereço virtual)
    uint64_t e_phoff;         // Offset da Program Header Table
    uint64_t e_shoff;         // Offset da Section Header Table
    uint32_t e_flags;         // Flags específicas do processador
    uint16_t e_ehsize;        // Tamanho do ELF header (64 bytes)
    uint16_t e_phentsize;     // Tamanho de cada program header (56 bytes)
    uint16_t e_phnum;         // Número de program headers
    uint16_t e_shentsize;     // Tamanho de cada section header (64 bytes)
    uint16_t e_shnum;         // Número de section headers
    uint16_t e_shstrndx;      // Índice da section de nomes de sections
} Elf64_Ehdr;
```

**Campos importantes:**
- `e_ident[0-3]`: Magic number `\x7fELF`
- `e_ident[4]`: Classe (32-bit=1, 64-bit=2)
- `e_ident[5]`: Endianness (1=little, 2=big)
- `e_type`: 2=EXEC (executável), 3=DYN (shared lib)
- `e_machine`: 62=x86_64, 183=ARM, 64=AArch64

### 2. Program Header Table

Define segmentos a serem carregados em memória:

```c
typedef struct {
    uint32_t p_type;          // Tipo: PT_LOAD (1), PT_INTERP (3), etc
    uint32_t p_flags;         // Permissões: PF_X (1), PF_W (2), PF_R (4)
    uint64_t p_offset;        // Offset no arquivo
    uint64_t p_vaddr;         // Endereço virtual
    uint64_t p_paddr;         // Endereço físico (geralmente ignora)
    uint64_t p_filesz;        // Tamanho no arquivo
    uint64_t p_memsz;         // Tamanho na memória
    uint64_t p_align;         // Alinhamento
} Elf64_Phdr;
```

**Tipos de segmentos:**
- `PT_NULL` (0): Ignorado
- `PT_LOAD` (1): Carregado em memória (código, dados)
- `PT_DYNAMIC` (2): Informações de linkagem dinâmica
- `PT_INTERP` (3): Path do interpretador (ex: /lib64/ld-linux-x86-64.so.2)
- `PT_NOTE` (4): Informações auxiliares
- `PT_TLS` (7): Thread-local storage

### 3. Sections Comuns

```
.text    - Código executável (read + execute)
.rodata  - Dados somente leitura (read)
.data    - Dados inicializados (read + write)
.bss     - Dados não inicializados (read + write, não ocupa espaço no arquivo)
.symtab  - Tabela de símbolos
.strtab  - Tabela de strings
.shstrtab- Nomes das sections
```

## Gerando um ELF Mínimo

### Exemplo em C

```c
// minimal_elf.c
// Gerar executável ELF mínimo (sem libc)

#include <stdint.h>
#include <stdio.h>
#include <string.h>

// ELF Header
uint8_t elf_header[] = {
    // e_ident[16]
    0x7f, 'E', 'L', 'F',        // Magic
    2,                          // 64-bit
    1,                          // Little endian
    1,                          // ELF version
    0,                          // OS/ABI (System V)
    0, 0, 0, 0, 0, 0, 0, 0,    // Padding
    
    // e_type: ET_EXEC (2)
    0x02, 0x00,
    
    // e_machine: x86_64 (62)
    0x3e, 0x00,
    
    // e_version: 1
    0x01, 0x00, 0x00, 0x00,
    
    // e_entry: 0x400000 + 0x80 (entry point)
    0x80, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // e_phoff: 64 (program header offset)
    0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // e_shoff: 0 (no section headers)
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // e_flags: 0
    0x00, 0x00, 0x00, 0x00,
    
    // e_ehsize: 64
    0x40, 0x00,
    
    // e_phentsize: 56
    0x38, 0x00,
    
    // e_phnum: 1
    0x01, 0x00,
    
    // e_shentsize: 64
    0x40, 0x00,
    
    // e_shnum: 0
    0x00, 0x00,
    
    // e_shstrndx: 0
    0x00, 0x00
};

// Program Header (PT_LOAD)
uint8_t phdr[] = {
    // p_type: PT_LOAD (1)
    0x01, 0x00, 0x00, 0x00,
    
    // p_flags: RWX (7)
    0x07, 0x00, 0x00, 0x00,
    
    // p_offset: 0
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // p_vaddr: 0x400000
    0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // p_paddr: 0x400000
    0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
    
    // p_filesz: (calculado em runtime)
    // p_memsz: (calculado em runtime)
    // p_align: 0x1000
    0x00, 0x10, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
};

// Código x86_64: exit(42)
// mov rdi, 42   (exit code)
// mov rax, 60   (sys_exit)
// syscall
uint8_t code[] = {
    0x48, 0xc7, 0xc7, 0x2a, 0x00, 0x00, 0x00,  // mov rdi, 42
    0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00,  // mov rax, 60
    0x0f, 0x05                                  // syscall
};

int main() {
    FILE* f = fopen("minimal", "wb");
    
    // Atualiza tamanhos no phdr
    uint64_t filesz = sizeof(elf_header) + sizeof(phdr) + sizeof(code);
    memcpy(phdr + 32, &filesz, 8);  // p_filesz
    memcpy(phdr + 40, &filesz, 8);  // p_memsz
    
    // Escreve ELF
    fwrite(elf_header, 1, sizeof(elf_header), f);
    fwrite(phdr, 1, sizeof(phdr), f);
    fwrite(code, 1, sizeof(code), f);
    
    fclose(f);
    
    // Torna executável
    chmod("minimal", 0755);
    
    printf("Executável 'minimal' criado!\n");
    return 0;
}
```

**Compilar e testar:**
```bash
gcc minimal_elf.c -o create_minimal
./create_minimal
./minimal
echo $?  # Saída: 42
```

## Seções Importantes para Dryad

### .text (Código)
```
Alinhamento: 0x1000 (4KB)
Flags: ALLOC, EXEC
Endereço típico: 0x400000+
```

### .rodata (Constantes)
```
Alinhamento: 0x1000
Flags: ALLOC
Strings, números constantes
```

### .data (Variáveis Globais)
```
Alinhamento: 0x1000
Flags: ALLOC, WRITE
Variáveis inicializadas
```

### .bss (Zeros)
```
Alinhamento: 0x1000
Flags: ALLOC, WRITE
Não ocupa espaço no arquivo (p_filesz = 0)
Apenas reserva espaço na memória (p_memsz > 0)
```

## Syscalls Linux x86_64

Para executáveis sem libc, usamos syscalls diretamente:

```asm
; sys_write(fd, buf, count)
mov rax, 1          ; syscall number
mov rdi, 1          ; stdout
lea rsi, [msg]      ; buffer
mov rdx, len        ; count
syscall

; sys_exit(code)
mov rax, 60
mov rdi, 0
syscall
```

**Syscalls comuns:**
- 0: read
- 1: write
- 2: open
- 3: close
- 9: mmap
- 10: mprotect
- 11: munmap
- 12: brk
- 60: exit

## Endereçamento

### Layout de Memória Típico (x86_64 Linux)

```
0x0000000000000000 - 0x00007fffffffffff: User space
  ┌─────────────────────────────────────┐
  │ Stack (cresce para baixo)           │
  │ Endereço alto (~0x7fff...)          │
  ├─────────────────────────────────────┤
  │ ...                                 │
  ├─────────────────────────────────────┤
  │ Heap (cresce para cima)             │
  ├─────────────────────────────────────┤
  │ BSS                                 │
  ├─────────────────────────────────────┤
  │ Data                                │
  ├─────────────────────────────────────┤
  │ Text (código)                       │
  │ 0x400000                            │
  └─────────────────────────────────────┘
0xffff800000000000 - 0xffffffffffffffff: Kernel space
```

## Ferramentas de Análise

### readelf
```bash
# Ler headers
readelf -h programa          # ELF header
readelf -l programa          # Program headers
readelf -S programa          # Section headers
readelf -s programa          # Símbolos

# Informações completas
readelf -a programa
```

### objdump
```bash
# Disassembly
objdump -d programa          # Código
objdump -D programa          # Tudo (inclui dados)
objdump -s programa          # Conteúdo das seções

# Headers
objdump -x programa          # Todos os headers
```

### hexdump
```bash
hexdump -C programa | head   # Ver bytes iniciais
```

## Exemplo Prático: "Hello World" em Assembly

```asm
; hello.asm
section .data
    msg: db "Hello, World!", 10
    len: equ $ - msg

section .text
    global _start

_start:
    ; sys_write(1, msg, len)
    mov rax, 1
    mov rdi, 1
    mov rsi, msg
    mov rdx, len
    syscall

    ; sys_exit(0)
    mov rax, 60
    xor rdi, rdi
    syscall
```

**Compilar:**
```bash
nasm -f elf64 hello.asm -o hello.o
ld hello.o -o hello
./hello
```

## Gerando ELF do Bytecode

### Algoritmo

```rust
fn generate_elf(bytecode: Chunk) -> Vec<u8> {
    let mut output = Vec::new();
    
    // 1. ELF Header
    let elf_header = Elf64_Ehdr {
        e_ident: [0x7f, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        e_type: ET_EXEC,
        e_machine: EM_X86_64,
        e_version: 1,
        e_entry: 0x400000 + 0x80,  // Base + ELF header + PHDR
        e_phoff: 64,               // PHDR vem depois do ELF header
        e_shoff: 0,                // Sem sections (opcional)
        e_flags: 0,
        e_ehsize: 64,
        e_phentsize: 56,
        e_phnum: 2,                // 2 segmentos: código e dados
        e_shentsize: 64,
        e_shnum: 0,
        e_shstrndx: 0,
    };
    output.extend_from_slice(&elf_header.to_bytes());
    
    // 2. Program Headers
    let code_phdr = Elf64_Phdr {
        p_type: PT_LOAD,
        p_flags: PF_R | PF_X,      // Read + Execute
        p_offset: 0,
        p_vaddr: 0x400000,
        p_paddr: 0x400000,
        p_filesz: code_size,
        p_memsz: code_size,
        p_align: 0x1000,
    };
    output.extend_from_slice(&code_phdr.to_bytes());
    
    let data_phdr = Elf64_Phdr {
        p_type: PT_LOAD,
        p_flags: PF_R | PF_W,      // Read + Write
        p_offset: data_offset,
        p_vaddr: 0x400000 + data_offset,
        p_paddr: 0x400000 + data_offset,
        p_filesz: data_size,
        p_memsz: data_size,
        p_align: 0x1000,
    };
    output.extend_from_slice(&data_phdr.to_bytes());
    
    // 3. Código
    let machine_code = compile_to_x86_64(bytecode);
    output.extend_from_slice(&machine_code);
    
    // 4. Dados (constantes, strings)
    let data = generate_data_section(bytecode);
    output.extend_from_slice(&data);
    
    output
}
```

## Considerações de Segurança

### NX Bit (No-Execute)
Modernos sistemas têm proteção NX. O executável precisa:
- Marcar .text como executável (PF_X)
- Marcar .data/.rodata como não-executável
- Ou usar mprotect() para alterar permissões

### ASLR (Address Space Layout Randomization)
Para compatibilidade com ASLR:
- Usar endereços relativos (RIP-relative)
- Position Independent Code (PIC)
- Ou desabilitar ASLR durante desenvolvimento: `setarch $(uname -m) -R ./programa`

### Stack Canaries
Para proteção contra stack smashing:
- Inserir canários na função
- Verificar antes do ret
- Requer runtime de suporte

## Referências

1. [ELF Format Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf)
2. [System V AMD64 ABI](https://gitlab.com/x86-psABIs/x86-64-ABI)
3. [Linux Kernel Syscalls](https://blog.rchapman.org/posts/Linux_System_Call_Table_for_x86_64/)
4. [Executable and Linkable Format](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format)
5. [ELF Tutorial](http://www.muppetlabs.com/~breadbox/software/elf.html)

## Próximos Passos

1. Implementar gerador de ELF básico
2. Criar runtime de syscalls
3. Suporte a bibliotecas dinâmicas (opcional)
4. Otimizações de tamanho
5. Debugging info (DWARF)
