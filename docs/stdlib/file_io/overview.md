---
title: "Entrada e Saída (IO)"
description: "API de manipulação de arquivos e diretórios."
category: "Bibliotecas Padrão"
subcategory: "File I/O"
order: 34
---

# File I/O (Entrada e Saída de Arquivos)

A API de File I/O do Dryad fornece acesso direto e seguro ao sistema de arquivos do sistema operacional host.

## 🚀 Leitura Rápida

- **Simplicidade**: Funções globais para ler, escrever e listar arquivos.
- **Sincronia**: Por padrão, as operações são bloqueantes (aguardam o disco).
- **Segurança**: Caminhos são validados para evitar fugas de diretório (Directory Traversal).
- **Diretórios**: Suporte nativo a criação recursiva de pastas.

---

## ⚙️ Visão Técnica

As chamadas de File I/O no Dryad são implementadas através de funções nativas escritas em Rust que encapsulam o módulo `std::fs`.

### 1. Mapeamento para System Calls

Cada função Dryad tem um correspondente direto em baixo nível:

- `read_file` → `std::fs::read_to_string` (Open + Read + Close).
- `write_file` → `std::fs::File::create` + `write_all`.
- `mkdir` → `std::fs::create_dir_all`.

### 2. Tratamento de Encodings

Diferente de sistemas legados que lidam com bytes crus, o Dryad impõe **UTF-8** em todas as operações de texto. Se um arquivo contiver bytes inválidos, o interpretador retornará um erro técnico `5003 (InvalidEncoding)` em vez de corromper os dados.

### 3. Buffering e Performance

Para operações de escrita repetitivas (como em loops), as funções de alto nível como `append_file` realizam a abertura e fechamento do handle a cada chamada.

> [!TIP]
> **Performance Recommendation**: Para logs intensivos, recomenda-se o uso de fluxos bufferizados (Stream Writers) que mantêm o handle aberto, reduzindo o overhead de syscalls do Kernel.

---

## 📚 Referências e Paralelos

- **API Base**: [Rust `std::fs` Module](https://doc.rust-lang.org/std/fs/index.html).
- **Standard**: [POSIX System Interface - File and Directory](https://pubs.opengroup.org/onlinepubs/9699919799/functions/contents.html).
- **Teoria**: "Operating Systems: Three Easy Pieces" - File System Implementation.

---

## Funções e Exemplos

### `read_file(path: string): string`

Lê todo o conteúdo de um arquivo de texto para uma string.

### `list_dir(path: string): [string]`

Retorna um array com os nomes das entradas no diretório. Internamente utiliza o iterador `std::fs::read_dir`.

```dryad
// Exemplo: Scanner de Diretório
let itens = list_dir(".");
for (let item in itens) {
    if (file_exists(item)) {
        println("Encontrado: " + item);
    }
}
```
