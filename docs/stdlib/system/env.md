---
title: "Ambiente do Sistema"
description: "Interação com o SO, variáveis de ambiente e execução de comandos."
category: "Bibliotecas Padrão"
subcategory: "Sistema"
order: 1
---

# Ambiente do Sistema (System Env)

O módulo `system_env` fornece uma ponte entre o script Dryad e as APIs de baixo nível do sistema operacional hospedeiro.

## 🚀 Leitura Rápida

- **OS Introspection**: Descubra em qual plataforma o código está rodando.
- **Processos**: Execute comandos do shell e capture saídas.
- **Variavéis**: Leia e defina chaves do ambiente (PATH, USER, etc).
- **Controle**: Encerre o processo (`exit`) ou obtenha o PID.

---

## ⚙️ Visão Técnica

As funções deste módulo são implementadas no runtime utilizando as crates nativas do Rust para portabilidade máxima.

### 1. Interação com Variáveis de Ambiente

O Dryad acessa o dicionário de ambiente do SO fornecido pelo Rust via `std::env::vars()`.

- **Thread-Safety**: Embora o Rust garanta segurança, alterações no ambiente (`native_set_env`) afetam todas as threads do processo atual, seguindo a semântica do SO.

### 2. Execução de Comandos (Subprocesses)

A função `native_exec` utiliza o motor `std::process::Command`.

- **Vulnerabilidade de Injeção**: O Dryad passa o comando inteiro para o shell (`sh -c` ou `cmd /C`).
- **Segurança**: Recomenda-se cautela ao concatenar entradas de usuários em comandos do sistema para evitar injeções de código maligno.

### 3. Portabilidade (Conditional Logic)

A função `native_platform` permite que desenvolvedores criem bibliotecas cross-platform:

```dryad
let sep = if (native_platform() == "windows") "\\" else "/";
```

---

## 📚 Referências e Paralelos

- **Rust API**: [std::env Documentation](https://doc.rust-lang.org/std/env/index.html).
- **Subprocesses**: [Rust std::process::Command](https://doc.rust-lang.org/std/process/struct.Command.html).
- **Standards**: [POSIX Environment Variables](https://pubs.opengroup.org/onlinepubs/009695399/basedefs/xbd_chap08.html).

---

## Principais Funções

### `native_exec(command: string): number`

Executa o comando e aguarda sua conclusão, retornando o código de saída do processo.

### `native_env(key: string): string | null`

Busca uma variável de ambiente. Retorna `null` (em vez de erro) se a chave não existir, facilitando verificações opcionais.
