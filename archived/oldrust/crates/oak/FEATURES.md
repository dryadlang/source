# 🌳 Lista de Funcionalidades e Planejamento - Dryad Oak CLI

## 📋 Resumo

O **Oak CLI** representa o gestor de pacotes para a linguagem **Dryad**, centralizando o gerenciamento de projetos, dependências e registros de pacotes. Inspirado em ferramentas consolidadas, como npm e cargo, oferece uma interface simplificada e focada no ecossistema Dryad.

**Versão Atual:** 0.1.0

---

## ✅ Funcionalidades Existentes

### ⚙️ Comandos Básicos Implementados
| Comando | Status | Descrição |
|---------|--------|-----------|
| `oak init <nome>` | ✅ Completo | Cria um novo projeto ou biblioteca Dryad |
| `oak install` | ✅ Completo | Instala as dependências listadas no arquivo de configuração ou um pacote individual |
| `oak remove <pacote>` | ✅ Completo | Remove um pacote específico do projeto |
| `oak list` | ✅ Completo | Lista as dependências instaladas |
| `oak update` | ✅ Parcial | Atualiza dependências (função básica no momento) |
| `oak publish` | 🚧 Em Planejamento | Publica pacotes no Registry |
| `oak run <script>` | ✅ Completo | Executa scripts definidos no `oaklibs.json` |
| `oak exec <arquivo.dryad>` | ✅ Completo | Executa um arquivo Dryad, com suporte a parâmetros |
| `oak clean` | ✅ Completo | Limpa arquivos temporários e caches no projeto |
| `oak info` | ✅ Completo | Exibe informações detalhadas do projeto (nome, versão, dependências) |
| `oak lock` | ✅ Completo | Gera/atualiza o arquivo de lock `oaklock.json` |
| `oak registry` | ✅ Completo | Gerencia repositórios de pacotes, com subcomandos: |
| - `list` | ✅ Completo | Lista repositórios configurados |
| - `add` | ✅ Completo | Adiciona novo repositório |
| - `remove` | ✅ Completo | Remove um repositório da configuração |
| - `set-default` | ✅ Completo | Define um repositório como padrão |
| - `test` | ✅ Completo | Testa a conectividade com um repositório configurado |

### 🚀 Gerenciamento de Projetos

#### Inicialização de Projetos e Bibliotecas

```bash
# Criando um projeto-standard
oak init meu-app --type project

# Criando uma biblioteca
oak init minha-lib --type library
```

**Estruturas Automáticas Criadas:**

**Projeto:**
```
meu-app/
├── main.dryad          # Ponto de entrada
├── oaklibs.json        # Configuração do projeto
├── .gitignore          # Configurações do Git
└── README.md           # Documentação inicial
```

**Biblioteca:**
```
minha-lib/
├── src/
│   └── main.dryad      # Arquivo principal
├── lib/
│   ├── matematica.dryad
│   └── utilidades.dryad
├── oaklibs.json        # Configuração do projeto
├── .gitignore
└── README.md
```

#### Sistema de Scripts

Scripts customizados podem ser definidos no `oaklibs.json`:

```json
{
  "scripts": {
    "start": "oak exec main.dryad",
    "test": "oak exec tests/test.dryad",
    "check": "oak exec --validate main.dryad"
  }
}
```

Comandos para execução:

```bash
oak run start  # Executa o script "start"
oak run test   # Executa o script "test"
oak run check  # Valida sintaxe do projeto
```

#### Manipulação de Configuração

Os arquivos `oaklibs.json` e `oaklock.json` são centrais para a configuração e gerenciamento do projeto:

- **oaklibs.json**: Configuração geral do projeto, incluindo dependências, scripts e metadados.
- **oaklock.json**: Arquivo gerado automaticamente, contendo as dependências resolvidas e suas versões.

Funções associadas:
- `load_oaklock` / `save_oaklock`
- `load_config` / `save_config`

### 📦 Gerenciamento de Dependências

#### Instalação de Pacotes

Pacotes podem ser instalados individualmente ou em lote:
```bash
# Instala pacotes configurados em oaklibs.json
oak install

# Instala um pacote específico
oak install matematica-utils

# Instala uma versão específica
oak install matematica-utils@^0.2.0
```

Pacotes suportados atualmente (modo simulado):
- `dryad-stdlib`: Biblioteca padrão com módulos de matemática e strings.
- `matematica-utils`: Funções matemáticas avançadas e módulos úteis como estatísticas ou operações geométricas.

#### Remoção de Pacotes
```bash
oak remove matematica-utils
```

#### Atualização de Dependências
```bash
oak update
```
Nota: Atualização avançada ainda em planejamento.

#### Geração do Lock File
```bash
oak lock
```
- Analisa dependências configuradas e caminhos associados.
- Gera o arquivo `oaklock.json` atualizado automaticamente.

---

## 🛠️ Planejamento Futuro

### Sistema de Registry Planejado

1. **Registry API**
   - Suporte para repositórios distribuídos e centrais.
   - Estrutura baseada em metadados e lock files para integridade dos pacotes.
   - API RESTful para consulta, download e gerenciamento online.

2. **Comando Publish**
   - Implementação da publicação com geração automática de metadados.
   - Integração com o Registry para upload de pacotes.
   
3. **Resoluções Avançadas**
   - Algoritmo de dependência com suporte a resolução semântica e detecção de conflitos.

4. **Sistema de Cache e Performances**
   - Cache local inteligente (em `.oak/cache`).
   - Downloads eficientes com validação de checksum SHA-256.

---

Esse mapeamento abrange todas as funcionalidades existentes do gestor `oak` para a linguagem Dryad, com base na análise dos arquivos disponíveis e no roadmap presente no README. Próximo passo: investigar as pastas **runtime**, **parser**, **lexer** e **erros** e adicionar quaisquer funcionalidades documentadas ao mapeamento geral.