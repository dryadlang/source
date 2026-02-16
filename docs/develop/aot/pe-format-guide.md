# Formato PE/COFF (Portable Executable) - Guia Técnico para Windows

## Visão Geral

PE é o formato de executáveis, bibliotecas (DLL) e drivers no Windows. Este guia cobre os aspectos essenciais para gerar executáveis Windows (.exe) a partir do bytecode Dryad.

## Estrutura do PE

### Visão Geral

```
┌─────────────────────────────────────┐
│ DOS Header (64 bytes)               │
│ - e_magic: "MZ"                     │
│ - e_lfanew: offset do PE header     │
├─────────────────────────────────────┤
│ Stub DOS (opcional)                 │
│ "This program cannot be run..."     │
├─────────────────────────────────────┤
│ PE Signature "PE\0\0" (4 bytes)     │
├─────────────────────────────────────┤
│ COFF File Header (20 bytes)         │
├─────────────────────────────────────┤
│ Optional Header                     │
│ (224 bytes PE32+, 96 bytes PE32)    │
├─────────────────────────────────────┤
│ Section Table                       │
│ (40 bytes por seção)                │
├─────────────────────────────────────┤
│ Sections                            │
│ - .text (código)                    │
│ - .rdata (read-only data)           │
│ - .data (dados)                     │
│ - .idata (imports)                  │
│ - .reloc (relocações)               │
└─────────────────────────────────────┘
```

## 1. DOS Header

```c
typedef struct _IMAGE_DOS_HEADER {
    WORD  e_magic;      // Magic number: "MZ" (0x5A4D)
    WORD  e_cblp;       // Bytes on last page
    WORD  e_cp;         // Pages in file
    WORD  e_crlc;       // Relocations
    WORD  e_cparhdr;    // Size of header in paragraphs
    WORD  e_minalloc;   // Minimum extra paragraphs
    WORD  e_maxalloc;   // Maximum extra paragraphs
    WORD  e_ss;         // Initial SS value
    WORD  e_sp;         // Initial SP value
    WORD  e_csum;       // Checksum
    WORD  e_ip;         // Initial IP value
    WORD  e_cs;         // Initial CS value
    WORD  e_lfarlc;     // File address of relocation table
    WORD  e_ovno;       // Overlay number
    WORD  e_res[4];     // Reserved
    WORD  e_oemid;      // OEM identifier
    WORD  e_oeminfo;    // OEM information
    WORD  e_res2[10];   // Reserved
    LONG  e_lfanew;     // File address of new exe header (offset do PE)
} IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;
```

**Valores típicos:**
- `e_magic`: 0x5A4D ("MZ")
- `e_lfanew`: 0x40 ou 0x80 (offset do PE header, alinhado)

## 2. PE Signature

4 bytes: `PE\0\0` (0x50, 0x45, 0x00, 0x00)

## 3. COFF Header

```c
typedef struct _IMAGE_FILE_HEADER {
    WORD  Machine;              // Arquitetura
    WORD  NumberOfSections;     // Número de seções
    DWORD TimeDateStamp;        // Timestamp
    DWORD PointerToSymbolTable; // Offset da tabela de símbolos (geralmente 0)
    DWORD NumberOfSymbols;      // Número de símbolos (geralmente 0)
    WORD  SizeOfOptionalHeader; // Tamanho do optional header (224 para PE32+)
    WORD  Characteristics;      // Flags
} IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;
```

**Machine types:**
- 0x014c: i386 (x86)
- 0x8664: x86-64 (AMD64)
- 0xaa64: ARM64

**Characteristics:**
- 0x0002: Executable image
- 0x0100: 32-bit machine
- 0x0200: System (driver)
- 0x2000: DLL

## 4. Optional Header (PE32+)

```c
typedef struct _IMAGE_OPTIONAL_HEADER64 {
    WORD                 Magic;                    // 0x20b = PE32+
    BYTE                 MajorLinkerVersion;       // 1
    BYTE                 MinorLinkerVersion;       // 0
    DWORD                SizeOfCode;               // Tamanho da seção .text
    DWORD                SizeOfInitializedData;    // Tamanho de .data
    DWORD                SizeOfUninitializedData;  // Tamanho de .bss
    DWORD                AddressOfEntryPoint;      // RVA do entry point
    DWORD                BaseOfCode;               // RVA do início do código
    ULONGLONG            ImageBase;                // Endereço base preferido
    DWORD                SectionAlignment;         // Alinhamento na memória (0x1000)
    DWORD                FileAlignment;            // Alinhamento no arquivo (0x200)
    WORD                 MajorOperatingSystemVersion;  // 6
    WORD                 MinorOperatingSystemVersion;  // 0
    WORD                 MajorImageVersion;        // 0
    WORD                 MinorImageVersion;        // 0
    WORD                 MajorSubsystemVersion;    // 6
    WORD                 MinorSubsystemVersion;    // 0
    DWORD                Win32VersionValue;        // 0
    DWORD                SizeOfImage;              // Tamanho total na memória
    DWORD                SizeOfHeaders;            // Tamanho dos headers
    DWORD                CheckSum;                 // Checksum (0 para .exe)
    WORD                 Subsystem;                // 1=native, 2=GUI, 3=console
    WORD                 DllCharacteristics;       // Flags
    ULONGLONG            SizeOfStackReserve;       // Tamanho reservado da stack
    ULONGLONG            SizeOfStackCommit;        // Tamanho inicial da stack
    ULONGLONG            SizeOfHeapReserve;        // Tamanho reservado do heap
    ULONGLONG            SizeOfHeapCommit;         // Tamanho inicial do heap
    DWORD                LoaderFlags;              // 0
    DWORD                NumberOfRvaAndSizes;      // 16 (tamanho da data directory)
    IMAGE_DATA_DIRECTORY DataDirectory[16];        // Tabelas importantes
} IMAGE_OPTIONAL_HEADER64, *PIMAGE_OPTIONAL_HEADER64;
```

### Data Directories

```c
typedef struct _IMAGE_DATA_DIRECTORY {
    DWORD VirtualAddress;   // RVA
    DWORD Size;
} IMAGE_DATA_DIRECTORY, *PIMAGE_DATA_DIRECTORY;
```

**Índices:**
- 0: Export table
- 1: Import table
- 2: Resource table
- 5: Base relocation table
- 12: Import address table (IAT)

## 5. Section Table

```c
#define IMAGE_SIZEOF_SHORT_NAME 8

typedef struct _IMAGE_SECTION_HEADER {
    BYTE  Name[IMAGE_SIZEOF_SHORT_NAME];  // Nome da seção
    union {
        DWORD PhysicalAddress;
        DWORD VirtualSize;                // Tamanho na memória
    } Misc;
    DWORD VirtualAddress;                 // RVA
    DWORD SizeOfRawData;                  // Tamanho no arquivo
    DWORD PointerToRawData;               // Offset no arquivo
    DWORD PointerToRelocations;           // 0 para executáveis
    DWORD PointerToLinenumbers;           // 0
    WORD  NumberOfRelocations;            // 0
    WORD  NumberOfLinenumbers;            // 0
    DWORD Characteristics;                // Flags
} IMAGE_SECTION_HEADER, *PIMAGE_SECTION_HEADER;
```

### Characterísticas de Seção

```c
#define IMAGE_SCN_CNT_CODE              0x00000020  // Section contains code
#define IMAGE_SCN_CNT_INITIALIZED_DATA  0x00000040  // Section contains initialized data
#define IMAGE_SCN_CNT_UNINITIALIZED_DATA 0x00000080 // Section contains uninitialized data
#define IMAGE_SCN_MEM_EXECUTE           0x20000000  // Section is executable
#define IMAGE_SCN_MEM_READ              0x40000000  // Section is readable
#define IMAGE_SCN_MEM_WRITE             0x80000000  // Section is writable
```

## Seções Comuns

### .text (Código)
```
Name: ".text\0\0\0"
VirtualSize: <tamanho do código>
VirtualAddress: 0x1000 (após headers)
SizeOfRawData: <alinhado em 0x200>
PointerToRawData: <offset no arquivo>
Characteristics: 0x60000020 (CODE | EXECUTE | READ)
```

### .rdata (Dados somente leitura)
```
Name: ".rdata\0\0"
VirtualSize: <tamanho>
VirtualAddress: <após .text>
SizeOfRawData: <alinhado>
PointerToRawData: <offset>
Characteristics: 0x40000040 (INITIALIZED_DATA | READ)
```

### .data (Dados inicializados)
```
Name: ".data\0\0\0"
VirtualSize: <tamanho>
VirtualAddress: <após .rdata>
SizeOfRawData: <alinhado>
PointerToRawData: <offset>
Characteristics: 0xC0000040 (INITIALIZED_DATA | READ | WRITE)
```

### .idata (Imports)
```
Name: ".idata\0\0"
VirtualSize: <tamanho da tabela de imports>
VirtualAddress: <RVA>
Characteristics: 0x40000040 (INITIALIZED_DATA | READ)
```

## Import Table (Idata)

Para usar funções do Windows (printf, exit, etc.):

```c
// Import Directory Entry
typedef struct _IMAGE_IMPORT_DESCRIPTOR {
    union {
        DWORD Characteristics;
        DWORD OriginalFirstThunk;  // RVA da ILT (Import Lookup Table)
    };
    DWORD TimeDateStamp;           // 0
    DWORD ForwarderChain;          // 0
    DWORD Name;                    // RVA do nome da DLL
    DWORD FirstThunk;              // RVA da IAT (Import Address Table)
} IMAGE_IMPORT_DESCRIPTOR, *PIMAGE_IMPORT_DESCRIPTOR;
```

**Exemplo: Importar printf da msvcrt.dll**

```
.idata section:
┌─────────────────────────────────────┐
│ IMAGE_IMPORT_DESCRIPTOR[0]          │
│ - OriginalFirstThunk → ILT          │
│ - Name → "msvcrt.dll"               │
│ - FirstThunk → IAT                  │
├─────────────────────────────────────┤
│ ILT (Import Lookup Table)           │
│ - RVA para "printf" (hint + nome)   │
│ - 0 (terminador)                    │
├─────────────────────────────────────┤
│ IAT (Import Address Table)          │
│ - Será preenchido pelo loader       │
│ - 0 (terminador)                    │
├─────────────────────────────────────┤
│ Nome da DLL: "msvcrt.dll\0"         │
├─────────────────────────────────────┤
│ Hint/Name: printf                   │
│ - WORD: Hint (ordinal)              │
│ - ASCIIZ: "printf"                  │
└─────────────────────────────────────┘
```

## Entry Point

No Windows, o entry point é diferente do Linux:

```asm
; Entry point Windows x64
; RCX = argc
; RDX = argv
; R8  = envp

start:
    ; Chamar main
    call main
    
    ; ExitProcess(code)
    mov rcx, rax        ; exit code
    call ExitProcess    ; Importado da kernel32.dll
```

**Syscalls Windows:**
- NÃO usar syscalls diretos (instáveis entre versões)
- SEMPRE usar APIs do Windows via imports
- kernel32.dll: ExitProcess, GetStdHandle, WriteConsoleA, etc.

## Exemplo Mínimo: "Hello World" Windows

```asm
; hello_win.asm
; Assemble: nasm -f win64 hello_win.asm -o hello_win.o
; Link:     gcc hello_win.o -o hello_win.exe

extern ExitProcess
extern GetStdHandle
extern WriteConsoleA

section .data
    msg: db "Hello, World!", 13, 10
    len: equ $ - msg
    written: dq 0

section .text
global start

start:
    ; GetStdHandle(STD_OUTPUT_HANDLE = -11)
    mov rcx, -11
    call GetStdHandle
    mov r12, rax          ; Salvar handle

    ; WriteConsoleA(handle, msg, len, &written, NULL)
    mov rcx, r12          ; hConsoleOutput
    lea rdx, [msg]        ; lpBuffer
    mov r8d, len          ; nNumberOfCharsToWrite
    lea r9, [written]     ; lpNumberOfCharsWritten
    push 0                ; lpReserved (shadow space alinhado)
    sub rsp, 32           ; Shadow space (4 registros * 8 bytes)
    call WriteConsoleA
    add rsp, 40           ; Desalocar shadow space + align

    ; ExitProcess(0)
    xor ecx, ecx
    call ExitProcess
```

**Compilar:**
```bash
nasm -f win64 hello_win.asm -o hello_win.o
gcc hello_win.o -o hello_win.exe -lkernel32
# ou linkar estaticamente (maior):
# gcc -static hello_win.o -o hello_win.exe
```

## Diferenças Linux vs Windows

| Aspecto | Linux (ELF) | Windows (PE) |
|---------|-------------|--------------|
| **Entry** | `_start` | `start` (qualquer nome) |
| **Syscalls** | Direto (syscall) | Via APIs (kernel32) |
| **Linkagem** | ld ou gcc | link.exe ou gcc |
| **Bibliotecas** | libc.so | kernel32.dll, msvcrt.dll |
| **Calling Conv** | System V AMD64 | Windows x64 |
| **Shadow Space** | Não | Sim (32 bytes) |
| **Stack Align** | 16 bytes | 16 bytes |

## Calling Convention Windows x64

### Registradores
```
Argumentos: RCX, RDX, R8, R9 (inteiros/ponteiros)
            XMM0-XMM3 (floats)
Retorno:    RAX (inteiros), XMM0 (floats)
Voláteis:   RAX, RCX, RDX, R8-R11, XMM0-XMM5
Preservados: RBX, RBP, RDI, RSI, R12-R15, XMM6-XMM15
```

### Shadow Space
Antes de chamar uma função, deve alocar 32 bytes (4 registros) na stack:
```asm
sub rsp, 32    ; Shadow space
call func
add rsp, 32    ; Liberar
```

### Alinhamento
- Stack deve estar alinhada em 16 bytes antes do CALL
- O CALL empurra 8 bytes (return address)
- Então na função: RSP + 8 deve ser múltiplo de 16

## Gerando PE do Bytecode

### Estrutura do Gerador

```rust
struct PEGenerator {
    dos_header: IMAGE_DOS_HEADER,
    pe_signature: [u8; 4],
    coff_header: IMAGE_FILE_HEADER,
    optional_header: IMAGE_OPTIONAL_HEADER64,
    section_headers: Vec<IMAGE_SECTION_HEADER>,
    sections: Vec<Vec<u8>>,
}

impl PEGenerator {
    fn generate(bytecode: Chunk) -> Vec<u8> {
        let mut pe = PEGenerator::new();
        
        // 1. Configurar headers
        pe.setup_headers();
        
        // 2. Compilar bytecode para x64
        let code = compile_to_x64_windows(bytecode);
        pe.add_section(".text", code, 0x60000020);
        
        // 3. Adicionar dados
        let data = generate_data(bytecode);
        pe.add_section(".rdata", data, 0x40000040);
        
        // 4. Adicionar imports (se necessário)
        let imports = generate_import_table();
        pe.add_section(".idata", imports, 0x40000040);
        
        // 5. Serializar
        pe.serialize()
    }
}
```

## Ferramentas de Análise

### dumpbin (Visual Studio)
```cmd
dumpbin /headers programa.exe
dumpbin /imports programa.exe
dumpbin /disasm programa.exe
```

### objdump (MinGW)
```bash
objdump -x programa.exe       # Headers
objdump -d programa.exe       # Disassembly
objdump -p programa.exe       # Imports
```

### CFF Explorer
- GUI para visualizar estrutura PE
- Editar imports, resources, etc.

### PE-Bear
- Analisador PE open source
- Visualização gráfica

## Problemas Comuns

### 1. "not a valid Win32 application"
- Subsystem incorreto (deve ser CONSOLE ou WINDOWS)
- Arquitetura errada (x86 vs x64)

### 2. "Entry point not found"
- RVA do entry point incorreta
- Seção .text não carregada no endereço correto

### 3. Crash na inicialização
- Stack não alinhada
- Shadow space não alocada
- Imports não resolvidas

### 4. DEP (Data Execution Prevention)
- Seção .text deve ter flag EXECUTE
- Outras seções NÃO devem ter EXECUTE

## Referências

1. [PE Format Specification](https://docs.microsoft.com/en-us/windows/win32/debug/pe-format)
2. [Windows x64 Calling Convention](https://docs.microsoft.com/en-us/cpp/build/x64-calling-convention)
3. [PE Internals](http://www.csn.ul.ie/~caolan/publink/winresdump/winresdump/doc/pefile.html)
4. [An In-Depth Look into the Win32 Portable Executable File Format](https://docs.microsoft.com/en-us/archive/msdn-magazine/2002/february/inside-windows-win32-portable-executable-file-format-in-detail)
5. [OSDev PE](https://wiki.osdev.org/PE)

## Próximos Passos

1. Implementar gerador de PE básico
2. Criar runtime Windows (kernel32 imports)
3. Suporte a msvcrt.dll para printf
4. Testar em Windows 10/11
5. Suporte a DLLs (opcional)
