# Propostas Elegantes para Substituir o Sistema de Native Functions do Dryad

## Análise do Sistema Atual

### Problemas Identificados

O sistema atual de native functions do Dryad (`#<module_name>`) apresenta várias limitações:

1. **Poluição do Namespace Global**: Todas as funções nativas são injetadas no escopo global
2. **Sintaxe Não-Intuitiva**: `#<module_name>` é uma sintaxe especial que não se integra naturalmente
3. **Falta de Type Safety**: Nenhuma verificação de tipo em tempo de compilação
4. **Nomes Inconsistentes**: Funções como `file_read_string()` não seguem convenções modernas
5. **Sem Descoberta de API**: Impossível inspecionar o que um módulo oferece
6. **Limitações de FFI**: Tipos complexos passados como `void*` opaco
7. **Gerenciamento Manual**: Necessita carregar explicitamente cada módulo

## Proposta 1: Sistema de Módulos Nativos com Namespaces (Recomendado)

### Conceito

Integrar módulos nativos ao sistema de import/export padrão, usando namespaces e decorators para FFI.

### Sintaxe

```dryad
// Importação explícita com namespace
import io from "@native/io";
import crypto from "@native/crypto";

// Uso
let content = io.readFile("data.txt");
let hash = crypto.sha256(content);

// Ou importação seletiva
import { readFile, writeFile } from "@native/io";
let data = readFile("input.txt");

// Namespace alias
import * as fs from "@native/io";
fs.readFile("file.txt");
```

### Vantagens

✅ **Namespace Isolado**: Não polui escopo global  
✅ **Descoberta de API**: Autocomplete e documentação inline  
✅ **Type Safety**: Anotações de tipo para funções nativas  
✅ **Sintaxe Familiar**: Usa sistema de import padrão  
✅ **Lazy Loading**: Módulos carregados apenas quando importados  
✅ **Tree Shaking**: Apenas funções usadas são incluídas  

### Implementação Teórica

```dryad
// Definição de módulo nativo (conceitual)
@native("libdryad_io.so")
module io {
    export function readFile(path: string): string;
    export function writeFile(path: string, content: string): void;
    export function exists(path: string): bool;
    
    export class File {
        constructor(path: string);
        read(): string;
        write(content: string): void;
        close(): void;
    }
}
```

## Proposta 2: Foreign Function Interface (FFI) Declarativo

### Conceito

Permitir que usuários declarem suas próprias bindings FFI diretamente na linguagem, sem precisar escrever código C.

### Sintaxe

```dryad
// Declaração de biblioteca externa
@ffi("libm.so.6")
extern "C" {
    function sqrt(x: number): number;
    function pow(base: number, exp: number): number;
    function sin(x: number): number;
}

// Uso direto
let result = sqrt(16.0); // 4.0

// Para bibliotecas personalizadas
@ffi("./mylib.so")
extern "C" {
    function custom_hash(data: string): number;
    
    @struct
    class Point {
        x: number;
        y: number;
    }
    
    function distance(p1: Point, p2: Point): number;
}
```

### Vantagens

✅ **Flexibilidade Total**: Usuário pode integrar qualquer biblioteca C/C++  
✅ **Type Safety**: Declarações explícitas de tipos  
✅ **Documentação Inline**: FFI declarations são auto-documentadas  
✅ **Sem Intermediário**: Chamadas diretas, sem overhead  
✅ **Controle Fino**: Especificação de ABI, calling conventions  

### Características Avançadas

```dryad
// Especificação de ABI
@ffi("libcpp_lib.so", abi = "C++")
extern "C++" {
    class Vector {
        constructor(size: number);
        push(value: number): void;
        get(index: number): number;
    }
}

// Callbacks
@ffi("libuv.so")
extern "C" {
    type Callback = fn(result: number) -> void;
    function async_operation(callback: Callback): void;
}

// Uso
async_operation((result) => {
    print("Result: " + result);
});
```

## Proposta 3: Plugin System com Registros Dinâmicos

### Conceito

Sistema de plugins onde módulos nativos se auto-registram e expõem APIs através de um protocolo padronizado.

### Sintaxe

```dryad
// Carregar plugin
use plugin "io" from "@plugins/io.dryad.so";

// Plugin expõe objeto com API
let file = io.File.open("data.txt");
let content = file.readAll();
file.close();

// Ou com resource management automático
use io.File.open("data.txt") as file {
    let content = file.readAll();
    // file.close() chamado automaticamente
}
```

### Protocolo de Plugin (C++ side)

```cpp
// Plugin implementation (C++)
class IoPlugin : public DryadPlugin {
public:
    void Register(PluginRegistry& registry) override {
        registry.RegisterClass("File")
            .Constructor<string>()
            .Method("readAll", &File::readAll)
            .Method("write", &File::write)
            .Method("close", &File::close);
            
        registry.RegisterFunction("exists", &fs_exists);
        registry.RegisterFunction("remove", &fs_remove);
    }
};

DRYAD_EXPORT_PLUGIN(IoPlugin)
```

### Vantagens

✅ **Auto-Registro**: Plugins se registram automaticamente  
✅ **Reflexão**: Possível inspecionar plugins em runtime  
✅ **Versionamento**: Plugins podem ter versões  
✅ **Hot Reload**: Plugins podem ser recarregados sem restart  
✅ **Sandboxing**: Plugins isolados com permissões  

## Proposta 4: Trait-Based Foreign Interfaces

### Conceito

Usar traits/interfaces para definir contratos entre Dryad e código nativo, com implementações plugáveis.

### Sintaxe

```dryad
// Definição de trait para I/O
trait FileSystem {
    function readFile(path: string): string;
    function writeFile(path: string, content: string): void;
    function listDir(path: string): string[];
}

// Implementação nativa (registrada em runtime)
@native
class PosixFileSystem implements FileSystem {
    // Implementação em C/C++
}

// Uso
let fs: FileSystem = new PosixFileSystem();
let files = fs.listDir("/home");

// Ou via injeção de dependência
@inject(FileSystem)
function processFiles(fs: FileSystem) {
    let content = fs.readFile("data.txt");
    // ...
}
```

### Vantagens

✅ **Testabilidade**: Fácil criar mocks para testes  
✅ **Polimorfismo**: Múltiplas implementações (POSIX, Windows, WASM)  
✅ **Type Safety**: Contratos explícitos via traits  
✅ **Dependency Injection**: Facilita arquitetura limpa  

## Proposta 5: Embedded DSL para FFI (Domain-Specific Language)

### Conceito

Uma mini-linguagem dentro de Dryad especificamente para definir bindings FFI de forma declarativa e type-safe.

### Sintaxe

```dryad
ffi module io {
    library "libdryad_io" version "1.0";
    
    binding {
        fn readFile(path: str) -> str {
            extern "dryad_io_read_file";
            throws IOError;
        }
        
        fn writeFile(path: str, content: str) -> void {
            extern "dryad_io_write_file";
            throws IOError, PermissionError;
        }
        
        class File {
            constructor(path: str) {
                extern "dryad_io_file_new";
            }
            
            fn read() -> str {
                extern "dryad_io_file_read";
                method;
            }
            
            fn close() {
                extern "dryad_io_file_close";
                method;
                destructor;
            }
        }
    }
    
    types {
        struct FileInfo {
            size: u64;
            modified: u64;
            isDirectory: bool;
        }
    }
}

// Uso
import io;
let content = io.readFile("data.txt");
```

### Vantagens

✅ **Clareza**: Sintaxe declarativa específica para FFI  
✅ **Type Safety**: Tipos explícitos com validação  
✅ **Error Handling**: Exceções mapeadas explicitamente  
✅ **Metadados**: Versionamento, dependências  
✅ **Geração de Código**: Pode gerar bindings automaticamente  

## Proposta 6: WebAssembly-First com WASI

### Conceito

Priorizar WebAssembly como plataforma FFI primária, usando WASI (WebAssembly System Interface) para I/O.

### Sintaxe

```dryad
// Importar módulo WASM
import wasm from "@wasm/io.wasm";

// Uso direto
let content = wasm.exports.readFile("data.txt");

// Ou com wrapper type-safe
@wasm("@wasm/io.wasm")
module io {
    export function readFile(path: string): string;
    export function writeFile(path: string, content: string): void;
}

// Para código nativo não-WASM, usar adapter
@wasm_adapter(native = "libio.so")
module io_native {
    // Mesmo interface que versão WASM
}
```

### Vantagens

✅ **Portabilidade**: WASM roda em qualquer plataforma  
✅ **Segurança**: Sandbox automático  
✅ **Performance**: JIT compilation  
✅ **Ecossistema**: Acesso a bibliotecas Rust, C++, etc via WASM  
✅ **Futuro-Proof**: WASM é padrão da web  

## Proposta 7: Reactive Extensions (Rx-style) para I/O

### Conceito

Usar padrão Observable/Stream para operações assíncronas e I/O, inspirado em ReactiveX.

### Sintaxe

```dryad
import { Observable } from "@native/rx";
import io from "@native/io";

// Leitura reativa de arquivo
let fileStream = io.readFileStream("large_file.txt");

fileStream
    .map(chunk => chunk.toUpperCase())
    .filter(chunk => chunk.contains("ERROR"))
    .subscribe({
        next: (chunk) => print(chunk),
        error: (err) => print("Error: " + err),
        complete: () => print("Done")
    });

// Combinação de streams
let file1 = io.readFileStream("a.txt");
let file2 = io.readFileStream("b.txt");

Observable.merge(file1, file2)
    .subscribe(chunk => process(chunk));

// Watch de arquivos
io.watchFile("config.json")
    .debounce(1000)
    .subscribe(content => reloadConfig(content));
```

### Vantagens

✅ **Composabilidade**: Operadores para transformar streams  
✅ **Backpressure**: Controle de fluxo automático  
✅ **Cancelamento**: Fácil cancelar operações  
✅ **Async First**: Naturalmente assíncrono  
✅ **Padrão Conhecido**: Familiar para desenvolvedores modernos  

## Proposta 8: Capabilities-Based Security Model

### Conceito

Sistema baseado em capabilities onde acesso a recursos nativos requer capabilities explícitas.

### Sintaxe

```dryad
// Declarar capabilities necessárias
@requires(capability.filesystem.read)
@requires(capability.network.http)
function processData() {
    // Código tem acesso apenas ao declarado
    let data = readFile("data.txt");  // OK
    let response = http.get("api.com"); // OK
    // crypto.randomBytes()  // ERRO: capability não declarada
}

// Runtime fornece capabilities
runtime.run(processData, {
    capabilities: [
        capability.filesystem.read("/data/*"),
        capability.network.http("*.example.com")
    ]
});

// Ou via grants
grant filesystem.read to processData {
    paths: ["/data", "/tmp"]
}

grant network.http to processData {
    domains: ["api.example.com"]
}
```

### Vantagens

✅ **Segurança**: Princípio de menor privilégio  
✅ **Auditabilidade**: Clara declaração de permissões  
✅ **Sandboxing**: Isolamento por capability  
✅ **Granularidade**: Controle fino de acesso  

## Comparação das Propostas

| Proposta | Type Safety | Performance | Segurança | Facilidade | Flexibilidade |
|----------|-------------|-------------|-----------|------------|---------------|
| 1. Namespaces Nativos | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 2. FFI Declarativo | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 3. Plugin System | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 4. Trait-Based | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 5. Embedded DSL | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| 6. WASM-First | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| 7. Reactive Ext. | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |
| 8. Capabilities | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |

## Recomendação: Abordagem Híbrida

A melhor solução combina múltiplas propostas:

### Camadas do Sistema

```
┌─────────────────────────────────────────────────────┐
│  Dryad Application Code                             │
├─────────────────────────────────────────────────────┤
│  Standard Library (@std)                            │
│  - Namespaced Native Modules (Proposta 1)           │
├─────────────────────────────────────────────────────┤
│  FFI Layer                                          │
│  - Declarative FFI (Proposta 2)                     │
│  - Plugin System (Proposta 3)                       │
├─────────────────────────────────────────────────────┤
│  Security Layer                                     │
│  - Capabilities (Proposta 8)                        │
├─────────────────────────────────────────────────────┤
│  Runtime Adapters                                   │
│  - Native (C/C++/Rust)                              │
│  - WASM (Proposta 6)                                │
└─────────────────────────────────────────────────────┘
```

### Exemplo Completo

```dryad
// stdlib fornece módulos nativos via namespaces
import io from "@std/io";
import crypto from "@std/crypto";
import http from "@std/http";

// FFI declarativo para bibliotecas customizadas
@ffi("./mylib.so")
extern "C" {
    function custom_hash(data: string): number;
}

// Plugins para extensibilidade
use plugin "database" from "@plugins/postgres.dryad.so";

// Capabilities para segurança
@requires(capability.filesystem.read)
@requires(capability.network.http)
async function processData() {
    // Namespace nativo
    let data = await io.readFile("data.txt");
    
    // FFI direto
    let hash = custom_hash(data);
    
    // Plugin
    let db = database.connect("postgres://localhost");
    await db.query("INSERT INTO hashes VALUES ($1)", hash);
    
    // HTTP com reactive streams
    http.get("https://api.com/notify")
        .map(response => response.json())
        .subscribe(result => print(result));
}

// Executar com capabilities restritas
runtime.run(processData, {
    capabilities: [
        capability.filesystem.read("/data/*"),
        capability.network.http("api.com")
    ]
});
```

## Implementação Gradual

### Fase 1: Namespaces Nativos (3 meses)
- Migrar módulos existentes para namespaces
- Sistema de import para `@std/*`
- Type definitions para módulos nativos

### Fase 2: FFI Declarativo (3 meses)
- Parser para blocos `extern "C"`
- Code generation para bindings
- Documentação e exemplos

### Fase 3: Plugin System (2 meses)
- Plugin registry
- Auto-discovery de plugins
- Hot reload capabilities

### Fase 4: Security (2 meses)
- Capability system
- Permission checks em runtime
- Audit logging

### Fase 5: WASM Integration (3 meses)
- WASM runtime integration
- WASI implementation
- Adapter layer para código nativo

## Benefícios da Nova Arquitetura

1. **Developer Experience**: Sintaxe moderna e familiar
2. **Type Safety**: Verificação de tipos em compile-time
3. **Performance**: Sem overhead desnecessário
4. **Security**: Modelo de capabilities
5. **Portability**: Suporte a WASM
6. **Extensibility**: Sistema de plugins
7. **Maintainability**: Código mais organizado e testável
8. **Ecosystem**: Compatibilidade com bibliotecas C/C++/Rust

## Conclusão

O sistema atual de native functions (`#<module>`) pode ser substituído por uma arquitetura em camadas que combina:

- **Namespaces** para módulos standard
- **FFI declarativo** para flexibilidade
- **Plugins** para extensibilidade
- **Capabilities** para segurança
- **WASM** para portabilidade

Esta abordagem oferece melhor type safety, developer experience, e segurança, mantendo compatibilidade e performance.
