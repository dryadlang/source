# 🌳 Oak - Gestor de Pacotes para Dryad

**Versão:** 0.1.0  
**Status:** Em desenvolvimento  
**Linguagem:** Rust  

---

## 📋 Índice

1. [Visão Geral](#visão-geral)
2. [Arquitetura Atual](#arquitetura-atual)
3. [Funcionalidades Implementadas](#funcionalidades-implementadas)
4. [Sistema de Registry (Planejado)](#sistema-de-registry-planejado)
5. [Roadmap de Desenvolvimento](#roadmap-de-desenvolvimento)
6. [API Reference](#api-reference)
7. [Estrutura de Arquivos](#estrutura-de-arquivos)
8. [Contribuindo](#contribuindo)

---

## 🎯 Visão Geral

O **Oak** é o gestor de pacotes oficial da linguagem Dryad, projetado para ser simples, eficiente e robusto. Inspirado em ferramentas como npm, cargo e yarn, mas adaptado às necessidades específicas do ecossistema Dryad.

### Objetivos Principais
- 🚀 **Simplicidade**: Interface intuitiva e comandos claros
- 📦 **Modularidade**: Sistema robusto de resolução de dependências
- 🔒 **Confiabilidade**: Lock files para builds reproduzíveis
- 🌐 **Registry**: Sistema distribuído de repositórios
- ⚡ **Performance**: Downloads otimizados e cache local

---

## 🏗️ Arquitetura Atual

### Componentes Principais

```rust
// Estruturas de dados principais
struct OakConfig {          // oaklibs.json - Configuração do projeto
    name: String,
    version: String,
    dependencies: HashMap<String, String>,
    dev_dependencies: HashMap<String, String>,
    scripts: HashMap<String, String>,
    // ...
}

struct OakLock {           // oaklock.json - Lock file gerado
    modules: HashMap<String, ModuleConfig>,
}

struct ModuleConfig {      // Mapeamento de módulos para caminhos
    paths: HashMap<String, String>,
}
```

### Tipos de Projeto

1. **Project** - Aplicações executáveis
   - Ponto de entrada: `main.dryad`
   - Estrutura livre para desenvolvimento
   
2. **Library** - Bibliotecas reutilizáveis
   - Ponto de entrada: `src/main.dryad`
   - Módulos exportáveis em `lib/`

---

## ✅ Funcionalidades Implementadas

### 🛠️ Comandos Básicos

| Comando | Status | Descrição |
|---------|--------|-----------|
| `oak init <nome>` | ✅ Completo | Cria novo projeto Dryad |
| `oak info` | ✅ Completo | Exibe informações do projeto atual |
| `oak list` | ✅ Completo | Lista dependências instaladas |
| `oak run <script>` | ✅ Completo | Executa scripts definidos em `oaklibs.json` |
| `oak clean` | ✅ Completo | Remove cache e arquivos temporários |
| `oak lock` | ✅ Completo | Gera/atualiza `oaklock.json` |

### 📁 Gerenciamento de Projetos

#### ✅ Inicialização de Projetos
```bash
# Projeto simples
oak init meu-app --type project

# Biblioteca
oak init minha-lib --type library
```

**Estruturas Geradas:**

**Projeto:**
```
meu-app/
├── main.dryad          # Ponto de entrada
├── oaklibs.json        # Configuração
├── .gitignore          # Git ignore
└── README.md           # Documentação
```

**Biblioteca:**
```
minha-lib/
├── src/
│   └── main.dryad      # Ponto de entrada
├── lib/
│   ├── matematica.dryad
│   └── utilidades.dryad
├── oaklibs.json
├── oaklock.json
├── .gitignore
└── README.md
```

#### ✅ Sistema de Scripts
```json
{
  "scripts": {
    "start": "dryad run main.dryad",
    "test": "dryad test",
    "check": "dryad check main.dryad"
  }
}
```

### 🔧 Instalação de Pacotes (Simulado)

#### 🟡 Implementação Atual (Modo Simulado)
```bash
oak install matematica-utils    # Cria estrutura local simulada
oak install dryad-stdlib        # Biblioteca padrão simulada
```

**Pacotes Simulados Disponíveis:**
- `matematica-utils` - Funções matemáticas avançadas
- `dryad-stdlib` - Biblioteca padrão (math, string)
- `file-utils` - Utilitários de arquivo

---

## 🌐 Sistema de Registry (Planejado)

### Visão da Arquitetura

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Oak Client    │───▶│   Registry API   │───▶│  Git Registry   │
│   (Comando CLI) │    │ (REST JSON API)  │    │  (Repositório)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                ▲                        │
                                │                        │
                         ┌─────────────────┐    ┌─────────────────┐
                         │   Package Info   │    │ Package Repos   │
                         │   (Metadata)     │    │ (Código Fonte)  │
                         └─────────────────┘    └─────────────────┘
```

### 🏛️ Fase 1: Git Registry (Repositório Central)

**Repositório:** `dryad-lang/oak-registry`

**Estrutura:**
```
oak-registry/
├── packages/
│   ├── matematica-utils/
│   │   ├── metadata.json
│   │   └── versions/
│   │       ├── 0.1.0.json
│   │       ├── 0.1.1.json
│   │       └── 0.2.0.json
│   ├── dryad-stdlib/
│   │   ├── metadata.json
│   │   └── versions/
│   │       └── 1.0.0.json
│   └── file-utils/
│       ├── metadata.json
│       └── versions/
│           └── 0.3.0.json
├── index.json          # Índice geral de pacotes
└── README.md
```

**Exemplo - `packages/matematica-utils/metadata.json`:**
```json
{
  "name": "matematica-utils",
  "description": "Funções matemáticas avançadas para Dryad",
  "author": "Dryad Team",
  "license": "MIT",
  "repository": "https://github.com/dryad-lang/matematica-utils",
  "homepage": "https://dryadlang.org/packages/matematica-utils",
  "keywords": ["math", "mathematics", "utils"],
  "latest_version": "0.2.0",
  "created_at": "2024-01-15T10:00:00Z",
  "updated_at": "2024-06-20T15:30:00Z"
}
```

**Exemplo - `packages/matematica-utils/versions/0.2.0.json`:**
```json
{
  "name": "matematica-utils",
  "version": "0.2.0",
  "description": "Funções matemáticas avançadas para Dryad",
  "main": "src/main.dryad",
  "repository": {
    "type": "git",
    "url": "https://github.com/dryad-lang/matematica-utils",
    "tag": "v0.2.0",
    "commit": "abc123def456"
  },
  "dependencies": {
    "dryad-stdlib": "^1.0.0"
  },
  "dev_dependencies": {},
  "files": [
    "src/",
    "lib/",
    "oaklibs.json",
    "README.md"
  ],
  "size": 45678,
  "checksum": {
    "sha256": "a1b2c3d4e5f6..."
  }
}
```

### 🛡️ Fase 2: Registry API

**Endpoint Base:** `https://api.dryadlang.org/v1/`

#### Endpoints Planejados:

| Método | Endpoint | Descrição |
|--------|----------|-----------|
| `GET` | `/packages` | Lista todos os pacotes |
| `GET` | `/packages/{name}` | Informações do pacote |
| `GET` | `/packages/{name}/versions` | Lista versões |
| `GET` | `/packages/{name}/versions/{version}` | Info da versão específica |
| `GET` | `/search?q={query}` | Busca pacotes |
| `GET` | `/download/{name}/{version}` | URL de download |

**Exemplo - Resposta de `/packages/matematica-utils/versions/0.2.0`:**
```json
{
  "status": "success",
  "data": {
    "name": "matematica-utils",
    "version": "0.2.0",
    "description": "Funções matemáticas avançadas para Dryad",
    "download_url": "https://github.com/dryad-lang/matematica-utils/archive/v0.2.0.tar.gz",
    "repository": "https://github.com/dryad-lang/matematica-utils",
    "dependencies": {
      "dryad-stdlib": "^1.0.0"
    },
    "checksum": "a1b2c3d4e5f6...",
    "size": 45678,
    "published_at": "2024-06-20T15:30:00Z"
  }
}
```

### 📦 Fase 3: Download e Instalação

#### Fluxo de Instalação:

1. **Resolução de Dependência**
   ```bash
   oak install matematica-utils@^0.2.0
   ```

2. **Consulta à API**
   ```
   GET https://api.dryadlang.org/v1/packages/matematica-utils/versions/0.2.0
   ```

3. **Download do Repositório**
   ```
   git clone --branch v0.2.0 --depth 1 https://github.com/dryad-lang/matematica-utils
   ```

4. **Validação e Extração**
   - Verificar checksum SHA-256
   - Extrair apenas arquivos necessários
   - Validar `oaklibs.json` do pacote

5. **Instalação Local**
   ```
   oak_modules/
   ├── matematica-utils@0.2.0/
   │   ├── src/
   │   ├── lib/
   │   └── oaklibs.json
   └── dryad-stdlib@1.0.0/
       ├── math.dryad
       ├── string.dryad
       └── oaklibs.json
   ```

6. **Atualização de Lock File**
   ```json
   {
     "modules": {
       "matematica-utils": {
         "version": "0.2.0",
         "resolved": "https://github.com/dryad-lang/matematica-utils/archive/v0.2.0.tar.gz",
         "checksum": "a1b2c3d4e5f6...",
         "paths": {
           "matematica": "./oak_modules/matematica-utils@0.2.0/lib/matematica.dryad",
           "formas": "./oak_modules/matematica-utils@0.2.0/lib/formas.dryad"
         }
       }
     }
   }
   ```

---

## 🗓️ Roadmap de Desenvolvimento

### 🎯 Milestone 1: Registry Infrastructure (4-6 semanas)

#### ✅ Semana 1-2: Estrutura Base
- [x] ~~Estruturas de dados principais (`OakConfig`, `OakLock`)~~
- [x] ~~Comandos básicos (`init`, `info`, `list`, `clean`)~~
- [x] ~~Sistema de scripts~~
- [x] ~~Simulação de pacotes~~

#### 🔄 Semana 3-4: Sistema de Cache e Download
- [ ] **Implementar cache local** (`~/.oak/cache/`)
- [ ] **Sistema de download HTTP/HTTPS**
- [ ] **Validação de checksums**
- [ ] **Compressão/descompressão de pacotes**

#### 🔄 Semana 5-6: Resolução de Dependências
- [ ] **Parser de versões semânticas**
- [ ] **Algoritmo de resolução de dependências**
- [ ] **Detecção de conflitos**
- [ ] **Geração de lockfile otimizada**

### 🎯 Milestone 2: Registry API (3-4 semanas)

#### 🔄 Semana 7-8: API Backend
- [ ] **Servidor HTTP com actix-web**
- [ ] **Integração com Git registry**
- [ ] **Cache de metadados**
- [ ] **Rate limiting e autenticação**

#### 🔄 Semana 9-10: Integração Cliente
- [ ] **Cliente HTTP no Oak**
- [ ] **Tratamento de erros de rede**
- [ ] **Fallback para repositórios locais**
- [ ] **Logs detalhados de operações**

### 🎯 Milestone 3: Funcionalidades Avançadas (4-5 semanas)

#### 🔄 Semana 11-13: Comando Publish
- [ ] **Validação de pacotes**
- [ ] **Geração automática de metadados**
- [ ] **Upload para registry**
- [ ] **Versionamento automático**

#### 🔄 Semana 14-15: Tooling
- [ ] **Comando `oak search`**
- [ ] **Comando `oak outdated`**
- [ ] **Comando `oak audit`**
- [ ] **Migração de projetos**

---

## 📚 API Reference

### Estruturas de Dados

#### OakConfig (oaklibs.json)
```rust
#[derive(Serialize, Deserialize, Debug)]
struct OakConfig {
    name: String,                           // Nome do projeto
    version: String,                        // Versão do projeto
    description: Option<String>,            // Descrição
    author: Option<String>,                 // Autor
    license: Option<String>,                // Licença
    project_type: ProjectType,              // "project" ou "library"
    main: Option<String>,                   // Arquivo principal
    dependencies: HashMap<String, String>,  // Dependências de produção
    dev_dependencies: HashMap<String, String>, // Dependências de desenvolvimento
    scripts: HashMap<String, String>,       // Scripts personalizados
}
```

#### OakLock (oaklock.json)
```rust
#[derive(Serialize, Deserialize, Debug)]
struct OakLock {
    modules: HashMap<String, ModuleConfig>, // Mapeamento de módulos
}

#[derive(Serialize, Deserialize, Debug)]
struct ModuleConfig {
    version: String,                        // Versão instalada
    resolved: String,                       // URL de origem
    checksum: String,                       // Hash de validação
    paths: HashMap<String, String>,         // Caminhos dos arquivos
}
```

### Comandos CLI

#### oak init
```bash
oak init <nome> [OPTIONS]

OPTIONS:
    -p, --path <PATH>    Diretório para criar o projeto
    -t, --type <TYPE>    Tipo de projeto (project|library) [default: project]

EXAMPLES:
    oak init meu-app                    # Projeto no diretório atual
    oak init minha-lib --type library   # Biblioteca
    oak init projeto --path /tmp/test   # Projeto em diretório específico
```

#### oak install
```bash
oak install [PACKAGE] [OPTIONS]

OPTIONS:
    -v, --version <VERSION>    Versão específica do pacote

EXAMPLES:
    oak install                       # Instala todas as dependências
    oak install matematica-utils      # Instala pacote específico
    oak install lodash@^4.17.0        # Instala versão específica
```

#### oak run
```bash
oak run <SCRIPT>

EXAMPLES:
    oak run start    # Executa script "start"
    oak run test     # Executa script "test"
    oak run build    # Executa script "build"
```

---

## 📁 Estrutura de Arquivos

### Layout de Projeto Completo

```
projeto-dryad/
├── src/                    # Código fonte (opcional)
│   ├── components/
│   ├── utils/
│   └── config/
├── lib/                    # Módulos exportáveis (apenas libraries)
│   ├── matematica.dryad
│   └── utilidades.dryad
├── tests/                  # Testes (futuro)
│   ├── unit/
│   └── integration/
├── docs/                   # Documentação (opcional)
├── oak_modules/            # Dependências instaladas (gerado)
│   ├── matematica-utils@0.2.0/
│   └── dryad-stdlib@1.0.0/
├── .oak/                   # Cache local (gerado)
│   ├── cache/
│   └── logs/
├── main.dryad             # Ponto de entrada (projects)
├── oaklibs.json           # Configuração do projeto
├── oaklock.json           # Lock file (gerado)
├── .gitignore             # Git ignore
└── README.md              # Documentação
```

### Configuração de Exemplo

**oaklibs.json (Projeto):**
```json
{
  "name": "meu-webapp",
  "version": "1.0.0",
  "description": "Minha aplicação web em Dryad",
  "author": "João Silva <joao@email.com>",
  "license": "MIT",
  "type": "project",
  "main": "main.dryad",
  "dependencies": {
    "matematica-utils": "^0.2.0",
    "dryad-stdlib": "^1.0.0",
    "http-client": "^2.1.0"
  },
  "dev_dependencies": {
    "dryad-test": "^0.1.0"
  },
  "scripts": {
    "start": "dryad run main.dryad",
    "dev": "dryad run main.dryad --watch",
    "test": "dryad test",
    "build": "dryad build --release",
    "clean": "oak clean && rm -rf dist/",
    "lint": "dryad check src/ main.dryad"
  }
}
```

**oaklock.json:**
```json
{
  "modules": {
    "matematica-utils": {
      "version": "0.2.0",
      "resolved": "https://github.com/dryad-lang/matematica-utils/archive/v0.2.0.tar.gz",
      "checksum": "sha256:a1b2c3d4e5f6789...",
      "paths": {
        "matematica": "./oak_modules/matematica-utils@0.2.0/lib/matematica.dryad",
        "formas": "./oak_modules/matematica-utils@0.2.0/lib/formas.dryad",
        "estatistica": "./oak_modules/matematica-utils@0.2.0/lib/estatistica.dryad"
      }
    },
    "dryad-stdlib": {
      "version": "1.0.0",
      "resolved": "https://github.com/dryad-lang/stdlib/archive/v1.0.0.tar.gz",
      "checksum": "sha256:b2c3d4e5f6789...",
      "paths": {
        "math": "./oak_modules/dryad-stdlib@1.0.0/math.dryad",
        "string": "./oak_modules/dryad-stdlib@1.0.0/string.dryad",
        "array": "./oak_modules/dryad-stdlib@1.0.0/array.dryad"
      }
    }
  }
}
```

---

## 🤝 Contribuindo

### Setup de Desenvolvimento

```bash
# Clone o repositório
git clone https://github.com/dryad-lang/source.git
cd source/crates/oak

# Build
cargo build

# Testes
cargo test

# Executar oak local
cargo run -- init teste-app
```

### Tarefas Prioritárias

#### 🔥 Alta Prioridade
1. **Sistema de download HTTP** - Implementar cliente robusto
2. **Cache inteligente** - Otimizar downloads repetidos  
3. **Resolução de dependências** - Algoritmo completo
4. **Validação de integridade** - Checksums e assinaturas

#### 🟡 Média Prioridade
5. **Registry API** - Backend para metadados
6. **Comando publish** - Upload de pacotes
7. **Busca de pacotes** - Interface de pesquisa
8. **Migração de projetos** - Compatibilidade

#### 🔵 Baixa Prioridade
9. **Interface gráfica** - GUI opcional
10. **Plugins** - Sistema de extensões
11. **Analytics** - Métricas de uso
12. **Mirror support** - Repositórios alternativos

### Arquitetura de Testes

```rust
// Estrutura de testes planejada
tests/
├── unit/
│   ├── config_test.rs      # Testes de configuração
│   ├── lockfile_test.rs    # Testes de lock file
│   └── resolver_test.rs    # Testes de resolução
├── integration/
│   ├── install_test.rs     # Testes de instalação
│   ├── publish_test.rs     # Testes de publicação
│   └── registry_test.rs    # Testes de registry
└── fixtures/
    ├── sample_projects/    # Projetos de exemplo
    └── mock_packages/      # Pacotes simulados
```

---

## 📄 Licença

MIT License - veja [LICENSE](../../LICENSE) para mais detalhes.

---

## 📞 Contato

- **Repositório:** [https://github.com/dryad-lang/source](https://github.com/dryad-lang/source)
- **Issues:** [https://github.com/dryad-lang/source/issues](https://github.com/dryad-lang/source/issues)
- **Discussões:** [https://github.com/dryad-lang/source/discussions](https://github.com/dryad-lang/source/discussions)

---

**Atualizado em:** Setembro 2025  
**Próxima revisão:** Outubro 2025