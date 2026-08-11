---
title: "Ambiente do Sistema"
description: "Interação com o SO, variáveis de ambiente e execução de comandos."
category: "Bibliotecas Padrão"
subcategory: "Sistema"
order: 38
---

# Ambiente do Sistema (System Env)

Este módulo fornece uma ponte segura entre o script Dryad e as APIs de baixo nível do sistema operacional hospedeiro.

## 🚀 Leitura Rápida

- **OS Introspection**: Identifique a plataforma (Windows, Linux, etc).
- **Processos**: Execute comandos shell e capture códigos de saída.
- **Variáveis**: Leia e defina chaves de ambiente (PATH, USER).
- **Controle**: Encerre o processo (`exit`) ou obtenha metadados como PID.

---

## ⚙️ Visão Técnica

As funções deste módulo são implementadas no runtime utilizando as crates nativas do Rust para portabilidade máxima entre sistemas POSIX e Windows.

### 1. Interação com Variáveis de Ambiente

O Dryad acessa o dicionário de ambiente do SO fornecido pelo Rust via `std::env::vars()`. Embora o Rust garanta segurança, alterações no ambiente (`native_set_env`) afetam globalmente o processo atual.

### 2. Execução de Comandos (Subprocesses)

A função `native_exec` utiliza o motor `std::process::Command`. O Dryad herda as permissões de segurança do usuário que iniciou o interpretador. Recomenda-se evitar a execução de strings não sanitizadas provenientes de fontes externas.

---

## 📚 Referências e Paralelos

- **Rust API**: [std::env Documentation](https://doc.rust-lang.org/std/env/index.html).
- **Subprocesses**: [Rust std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html).
- **Standards**: [POSIX Environment Variables](https://pubs.opengroup.org/onlinepubs/9699919799/basedefs/V1_chap08.html).

---

## Principais Funções

### `native_platform(): string`

Retorna o nome do sistema operacional atual: `"windows"`, `"linux"`, `"macos"`, etc.

### `native_arch(): string`

Retorna a arquitetura do processador: `"x86_64"`, `"aarch64"`, etc.

### `native_env(key: string): string | null`

Busca uma variável de ambiente. Retorna `null` se não existir.

### `native_set_env(key: string, value: string)`

Define uma variável de ambiente. Requer que o interpretador seja iniciado com `--allow-unsafe`.

### `native_exec(command: string): number`

Executa o comando no shell e retorna o código de saída. Requer `--allow-exec`.

### `native_exec_output(command: string): string`

Executa o comando e retorna sua saída padrão (stdout) como string. Requer `--allow-exec`.

### `native_pid(): number`

Retorna o ID do processo (PID) atual.

### `native_exit(code: number)`

Encerra o programa imediatamente com o código fornecido.

### `get_current_dir(): string`

Retorna o caminho absoluto do diretório de trabalho atual.

---

## Exemplo de Uso

```dryad
#<system_env>

println("S.O: " + native_platform());
println("Arquitetura: " + native_arch());
println("Diretório: " + get_current_dir());

let user = native_env("USER") || native_env("USERNAME");
println("Usuário atual: " + user);
```
