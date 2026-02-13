# Dryad Programming Language

![Dryad Logo](https://img.shields.io/badge/Dryad-v0.1.0-green)
![Oak Package Manager](https://img.shields.io/badge/Oak-Modular-blue)
![License](https://img.shields.io/badge/License-MIT-blue)
[![Rust](https://github.com/Dryad-lang/source/actions/workflows/rust.yml/badge.svg)](https://github.com/Dryad-lang/source/actions/workflows/rust.yml)

Dryad é uma linguagem de programação moderna, interpretada, com sintaxe expressiva inspirada em JavaScript/TypeScript e tipagem dinâmica. Desenvolvida em Rust com arquitetura modular e sistema de pacotes Oak integrado.

## 🎯 Pilares de Desenvolvimento

1. **Test-Driven Development (TDD)** - Cada funcionalidade possui testes abrangentes
2. **Sistema de Erros Padronizado** - Códigos de erro categorizados e documentados
3. **CLI Intuitivo** - Interface de linha de comando rica em funcionalidades
4. **Gestor de Pacotes (Oak)** - Ferramenta independente para gerenciamento de projetos
5. **Modularidade** - Componentes desacoplados para máxima testabilidade
6. **Completude sem Complexidade** - Código completo mas sem over-engineering

## ✨ Principais Características

- 🚀 **Sintaxe Familiar**: Inspirada em JavaScript/TypeScript
- 📦 **Sistema Oak**: Package manager modular integrado
- 🔗 **Imports/Exports**: Sistema modular avançado (`import`/`use`)
- 📚 **Common Libraries**: Bibliotecas nativas organizadas via diretivas `#`
- ⚡ **Performance**: Interpretador tree-walking otimizado em Rust
- 🛠️ **CLI Moderna**: Interface de linha de comando completa
- 🧵 **Concorrência Real**: Threads nativas, async/await, mutexes
- 🏗️ **Orientação a Objetos**: Classes com herança, métodos estáticos

## 🚀 Início Rápido

### 1. Instalação

```bash
# Clone o repositório
git clone https://github.com/dryad-lang/dryad.git
cd dryad

# Compile o projeto
cargo build --release

# Execute um script
./target/release/dryad run script.dryad
```

### 2. Primeiro Programa

```dryad
// hello.dryad
var nome = "Mundo";
print("Olá, " + nome + "!");

// Funções
function soma(a, b) {
    return a + b;
}

// Arrow functions
const dobro = (x) => x * 2;

// Classes com herança
class Animal {
    nome = "Sem nome";
    falar() {
        print("Som genérico");
    }
}

class Cachorro extends Animal {
    latir() {
        print("Au au!");
    }
}

// Async/Await
async function carregarDados() {
    await tarefa();
}

// Concorrência - Threads reais
thread function tarefaPesada() {
    // Executa em nova thread do SO
}
```

### 3. Sistema de Módulos

```dryad
// Importação estilo ES6
import { func1, func2 } from "modulo";
import * as utils from "utils";
import "init_script";

// Importação legada/simplificada
use "./utils/helper.dryad";

// Exportação
export function minhaFunc() { }
export class MinhaClasse { }

// Diretivas Nativas (Standard Library)
#<file_io>
#<http_client>
#<tcp>
```

## 📦 Estrutura do Projeto

```
dryad/
├── crates/
│   ├── dryad_lexer/        # Tokenização (análise léxica)
│   ├── dryad_parser/       # Parser e construção de AST
│   ├── dryad_runtime/      # Interpretador principal
│   ├── dryad_errors/       # Sistema de erros padronizados
│   ├── dryad_cli/          # CLI para executar código Dryad
│   ├── dryad_benchmark/    # Testes de performance
│   └── oak/                # Gestor de pacotes Oak
├── technical_docs/         # Documentação técnica completa
├── examples/               # Exemplos de código
├── tests/                  # Testes integrados
└── Cargo.toml              # Workspace principal
```

## 🏗️ Arquitetura

O interpretador Dryad segue uma arquitetura clássica de "tree-walking":

```
Código Fonte (.dryad)
    ↓
Lexer (dryad_lexer) → Tokens
    ↓  
Parser (dryad_parser) → AST (Abstract Syntax Tree)
    ↓
Runtime (dryad_runtime) → Resultado
```

### Componentes Principais

1. **Lexer**: Transforma código fonte em tokens, preservando localização (linha/coluna)
2. **Parser**: Análise recursiva descendente, gera AST fortemente tipada
3. **Runtime**: Interpretador tree-walking, gerencia escopos, memória e concorrência
4. **Errors**: Sistema unificado de erros com códigos e mensagens formatadas
5. **Oak**: Gerenciador de pacotes com resolução de módulos via trait `ModuleResolver`

## ✅ Funcionalidades Implementadas

### Core Language
- ✅ Variáveis (`var`) e Constantes (`const`)
- ✅ Tipos: Number, String, Bool, Null, Array, Tuple, Object
- ✅ Operadores: Aritméticos, Lógicos, Comparação, Bitwise, Atribuição
- ✅ Controle de Fluxo: if/else, while, do-while, for, for-in, break, continue
- ✅ Funções: Declaração, Expressão, Lambda (arrow functions), Async, Thread
- ✅ Classes: Declaração, Herança (`extends`), Métodos (instância/estáticos), Propriedades
- ✅ Módulos: import/export ES6, diretivas nativas (`#<modulo>`)
- ✅ Tratamento de Erros: try/catch/finally, throw
- ✅ Concorrência: async/await, threads nativas (`thread function`), mutexes

### Standard Library (Módulos Nativos)
- ✅ **#file_io**: read_file, write_file, append_file, file_exists, delete_file, mkdir, list_dir
- ✅ **#http_client**: http_get, http_post, http_download
- ✅ **#tcp**: Suporte a conexões TCP
- ✅ **#system_env**: Acesso a variáveis de ambiente
- ✅ **#console_io**: Entrada/saída no console
- ✅ **#terminal_ansi**: Cores e formatação ANSI
- ✅ **#binary_io**: Manipulação de dados binários
- ✅ **#time / #date_time**: Manipulação de tempo
- ✅ **#crypto**: Criptografia
- ✅ **#debug**: Ferramentas de debug
- ✅ **#encode_decode**: Codificação/decodificação
- ✅ **#utils**: Funções utilitárias

### CLI (dryad)
- ✅ `dryad run <arquivo>` - Executa código Dryad
- ✅ `dryad run <arquivo> --verbose` - Mostra tokens e AST
- ✅ `dryad check <arquivo>` - Valida sintaxe
- ✅ `dryad tokens <arquivo>` - Debug: mostra tokens
- ✅ `dryad repl` - Modo interativo
- ✅ `dryad version` - Informações da versão

### Gestor de Pacotes (Oak)
- ✅ `oak init <nome>` - Cria novo projeto
- ✅ `oak info` - Informações do projeto
- ✅ `oak list` - Lista dependências
- ✅ `oak install <pacote>` - Adiciona dependência
- ✅ `oak remove <pacote>` - Remove dependência
- ✅ `oak run <script>` - Executa scripts definidos
- ✅ `oak clean` - Limpa cache

## 📋 Sistema de Erros

Códigos de erro categorizados:

| Categoria | Range | Descrição |
|-----------|-------|-----------|
| Léxico | 1000-1999 | Caractere inesperado, string não terminada |
| Parser | 2000-2999 | Token inesperado, sintaxe inválida |
| Runtime | 3000-3999 | Variável não definida, divisão por zero |
| Tipos | 4000-4999 | Tipos incompatíveis, conversão inválida |
| I/O | 5000-5999 | Arquivo não encontrado, permissão negada |
| Módulos | 6000-6999 | Módulo desconhecido, importação circular |
| Sintaxe | 7000-7999 | Erros de estrutura sintática |
| Warnings | 8000-8999 | Variável não usada, função depreciada |
| Sistema | 9000-9999 | Memória insuficiente, stack overflow |

## 🗺️ Roadmap

### Curto Prazo (v0.2)
- Melhorias em mensagens de erro
- Expansão da stdlib
- Otimização de cache de variáveis

### Médio Prazo (v0.5)
- Garbage Collector real (substituir RC)
- Bytecode VM (migrar de Tree-Walking)
- Pattern matching e destructuring

### Longo Prazo (v1.0)
- JIT Compiler
- LSP Server completo para VS Code
- Debugger interativo

## 🛠️ Desenvolvimento

```bash
# Build do projeto
cargo build

# Executar todos os testes
cargo test

# Executar CLI
cargo run --bin dryad --help

# Executar Oak
cargo run --bin oak --help

# Modo REPL interativo
cargo run --bin dryad repl
```

## 📚 Documentação

A documentação técnica completa está disponível em `technical_docs/`:

- **Linguagem**: Sintaxe, tipos, operadores, funções, classes, controle de fluxo
- **Implementação**: Arquitetura detalhada, lexer, parser, runtime, erros
- **Manuais de Desenvolvimento**: Guias para contribuidores
- **Stdlib**: Documentação das bibliotecas nativas
- **Roadmap**: Planejamento estratégico do projeto

## 🤝 Contribuindo

Este projeto segue rigorosamente os princípios de TDD:

1. Escreva testes para a nova funcionalidade
2. Implemente a funcionalidade para passar nos testes
3. Refatore mantendo todos os testes passando
4. Adicione códigos de erro apropriados quando necessário

## 📄 Licença

MIT License - veja o arquivo LICENSE para detalhes.

---

*Documentação gerada a partir de technical_docs/*
