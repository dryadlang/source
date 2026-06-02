---
title: "Gestão de Pacotes"
description: "Arquitetura do gerenciador Oak e resolução de módulos."
category: "Desenvolvimento"
order: 66
---

# Gerenciamento de Pacotes e Módulos

O ecossistema Dryad utiliza o **Oak** como seu gerenciador de pacotes oficial, baseado em um sistema de resolução de módulos desacoplado e plugável.

## 🚀 Leitura Rápida

- **CLI Tool**: `oak` (instalador e resolvedor).
- **Padrão**: Utiliza o design pattern **Adapter** para conectar o runtime a diferentes fontes de pacotes.
- **Lockfile**: `oaklock.json` garante reprodutibilidade das dependências.
- **Aliases**: Importações via nomes amigáveis em vez de caminhos relativos longos.

---

## ⚙️ Visão Técnica

### 1. Desacoplamento via Traits (Rust)

O Runtime do Dryad não sabe "onde" os arquivos estão. Ele consome o trait `ModuleResolver`. Isso permite que o mesmo código Dryad seja executado a partir de arquivos locais, de um Registry HTTP ou até embutido em um binário Rust.

```rust
pub trait ModuleResolver: Send + Sync {
    fn resolve(&self, module_path: &str, current_path: Option<&Path>) -> Result<PathBuf, DryadError>;
}
```

### 2. O Papel do Adaptador Oak (`OakAdapter`)

Quando você usa a CLI, o `OakAdapter` é injetado no interpretador.

- **Mapeamento de Grafo**: Ele carrega o `oaklock.json` e constrói um grafo de dependências em memória.
- **Prioridade de Busca**:
  1. Check em nomes nativos (Built-ins).
  2. Check em aliases do `oaklock.json`.
  3. Resolução via sistema de arquivos relativo.

### 3. Segurança de Módulos

Módulos importados são "congelados" em memória após o primeiro carregamento (Memoização). Isso evita ciclos de importação infinitos e garante que o estado de um módulo seja consistente em toda a aplicação.

---

## 📚 Referências e Paralelos

- **Patterns**: [Adapter Design Pattern](https://refactoring.guru/design-patterns/adapter).
- **Rust Echo**: Similar ao funcionamento do `cargo` com `Cargo.lock`.
- **Node.js**: Inspirado no `pnpm` pela sua estrutura de módulos plana e eficiente.

---

## Estrutura do Projeto Oak

```bash
projeto/
├── main.dryad
├── oaklock.json       # Árvore de dependências resolvida
└── oak_modules/       # Código fonte das bibliotecas instaladas
```
