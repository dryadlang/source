---
title: "Interface de Funções Estrangeiras (FFI)"
description: "Carregamento e chamada de bibliotecas nativas (.dll, .so, .dylib)."
category: "Bibliotecas Padrão"
subcategory: "Sistema"
order: 40
---

# Foreign Function Interface (FFI)

O módulo FFI do Dryad permite que scripts carreguem e executem funções de bibliotecas dinâmicas externas compiladas em linguagens como C, C++ ou Rust.

> [!WARNING]
> **Operação de Baixo Nível**.
> O uso de FFI ignora as garantias de segurança do runtime. Tipos de dados incorretos ou ponteiros inválidos podem causar crash imediato do processo.

## 🚀 Leitura Rápida

1.  **Load**: Carregue a biblioteca usando `ffi_load_library`.
2.  **Verify**: (Opcional) Verifique se o símbolo existe com `ffi_get_symbol`.
3.  **Call**: Execute a função especificando o tipo de retorno com `ffi_call`.
4.  **Unload**: Libere a biblioteca com `ffi_unload_library`.

---

## ⚙️ Visão Técnica

O motor FFI utiliza a crate **libloading** para carregar símbolos dinâmicos de forma segura em Rust. Atualmente, o Dryad suporta chamadas para funções que **não recebem argumentos**, mas que retornam diversos tipos de dados.

### Tipos de Retorno Suportados

| Tipo Dryad  | Tipo C Correspondente | Descrição                    |
| :---------- | :-------------------- | :--------------------------- |
| `"void"`    | `void`                | Nulo (Null)                  |
| `"i32"`     | `int32_t`             | Número inteiro 32 bits       |
| `"i64"`     | `int64_t`             | Número inteiro 64 bits       |
| `"f64"`     | `double`              | Ponto flutuante 64 bits      |
| `"string"`  | `char*`               | String C (terminada em null) |
| `"pointer"` | `void*`               | Endereço de memória (Número) |

---

## Referência de Funções

### `ffi_load_library(path: string, alias?: string): bool`

Carrega uma biblioteca dinâmica para a memória.

- **path**: Caminho absoluto ou relativo para o arquivo (`.dll` no Windows, `.so` no Linux).
- **alias**: Nome amigável para referenciar a biblioteca posteriormente. Se omitido, o próprio `path` será usado como alias.

### `ffi_call(alias: string, symbol: string, return_type: string): any`

Executa uma função da biblioteca carregada.

- **alias**: O identificador definido no carregamento.
- **symbol**: O nome da função (ex: `"get_version"`).
- **return_type**: Um dos tipos suportados listados acima.

### `ffi_get_symbol(alias: string, symbol: string): bool`

Verifica se um símbolo específico existe na biblioteca sem executá-lo.

### `ffi_list_libraries(): [string]`

Retorna um array com os aliases de todas as bibliotecas atualmente carregadas no runtime.

### `ffi_unload_library(alias: string): bool`

Remove a biblioteca da memória e libera os recursos associados.

---

## Exemplo de Uso

```dryad
#<ffi>

// Carrega a biblioteca dinâmica C
let lib_path = "libs/minha_biblioteca.dll";
if (ffi_load_library(lib_path, "minha_lib")) {

    // Verifica se a função de versão existe
    if (ffi_get_symbol("minha_lib", "get_api_version")) {

        // Chama a função nativa que retorna um i32
        let version = ffi_call("minha_lib", "get_api_version", "i32");
        println("Versão da API Nativa: " + version);
    }

    ffi_unload_library("minha_lib");
}
```
