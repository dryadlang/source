# Scripts de Build e Release do Dryad

## 📋 Scripts Disponíveis

### 🚀 `build_release.ps1` - Script Completo de Release
Script principal para criar releases completos e profissionais.

**Uso:**
```powershell
# Build completo em modo release
.\build_release.ps1 -Release

# Build em modo debug
.\build_release.ps1 -Debug

# Limpar e recriar tudo
.\build_release.ps1 -Clean -Release

# Personalizar diretório de saída
.\build_release.ps1 -Release -OutputDir "meu_release"
```

**Funcionalidades:**
- ✅ Build otimizado (release) ou debug
- ✅ Copia binários automaticamente
- ✅ Inclui documentação
- ✅ Cria arquivo BUILD_INFO.md
- ✅ Relatório detalhado
- ✅ Verificação de integridade

### ⚡ `quick_release.ps1` - Build Rápido para Desenvolvimento
Script simples para atualizações rápidas durante desenvolvimento.

**Uso:**
```powershell
.\quick_release.ps1
```

**Funcionalidades:**
- ⚡ Build rápido em modo debug
- 📦 Copia apenas binários essenciais
- 🔄 Atualiza pasta dryad_release

### 🔄 `update_release.ps1` - Atualização Completa
Script intermediário que faz build otimizado e copia todos os arquivos.

**Uso:**
```powershell
.\update_release.ps1
```

**Funcionalidades:**
- 🚀 Build otimizado (release)
- 📦 Copia binários e documentação
- 📝 Inclui arquivos de teste .dryad
- 📊 Relatório de tamanhos

## 🎯 Quando Usar Cada Script

| Situação | Script Recomendado | Motivo |
|----------|-------------------|--------|
| **Desenvolvimento ativo** | `quick_release.ps1` | Build rápido, sem otimizações |
| **Teste local** | `update_release.ps1` | Build otimizado com documentação |
| **Release oficial** | `build_release.ps1 -Release` | Build completo com relatórios |
| **Limpeza completa** | `build_release.ps1 -Clean -Release` | Recria tudo do zero |

## 📁 Estrutura de Saída

Todos os scripts criam/atualizam a pasta `dryad_release/` com:

```
dryad_release/
├── dryad.exe           # Interpretador principal
├── oak.exe             # Ferramenta Oak
├── benchmark.exe       # Tool de benchmark
├── README.md           # Documentação principal
├── benchmark.md        # Info sobre benchmarks
├── DRYAD_ERROR_GUIDE.md # Guia de erros (se disponível)
├── BUILD_INFO.md       # Info de build (build_release.ps1)
└── *.dryad            # Arquivos de exemplo
```

## ⚙️ Configuração e Personalização

### Modificar Lista de Binários
Edite a variável `$binaries` nos scripts:

```powershell
$binaries = @("dryad.exe", "oak.exe", "benchmark.exe", "meu_binario.exe")
```

### Adicionar Documentação
Edite a variável `$docs` em `update_release.ps1`:

```powershell
$docs = @("README.md", "CHANGELOG.md", "LICENSE.md")
```

### Personalizar Diretório de Saída
Use o parâmetro `-OutputDir`:

```powershell
.\build_release.ps1 -Release -OutputDir "dist"
```

## 🔧 Solução de Problemas

### "Cargo não encontrado"
Certifique-se de que o Rust está instalado e no PATH:
```powershell
cargo --version
rustc --version
```

### "Arquivo Cargo.toml não encontrado"
Execute os scripts no diretório raiz do projeto Dryad.

### "Falha na compilação"
Verifique os erros de compilação:
```powershell
cargo check
cargo test
```

### Problemas de Permissão
Execute o PowerShell como administrador se necessário.

## 📈 Exemplos de Uso

### Fluxo de Desenvolvimento Típico
```powershell
# Durante desenvolvimento
.\quick_release.ps1

# Antes de commit
.\update_release.ps1

# Para release oficial
.\build_release.ps1 -Clean -Release
```

### Automação com Tarefas Agendadas
```powershell
# Build noturno
.\build_release.ps1 -Release -OutputDir "nightly_$(Get-Date -Format 'yyyy-MM-dd')"
```

### Integração com CI/CD
```yaml
# Exemplo para GitHub Actions
- name: Build Dryad Release
  run: .\build_release.ps1 -Release
  shell: powershell
```

## ✨ Dicas e Truques

1. **Build Paralelo**: Use `cargo build -j` para builds mais rápidos
2. **Cache**: O diretório `target/` é preservado entre builds
3. **Tamanho**: Use `cargo build --release` para binários menores
4. **Debug**: Use `cargo build` para compilação mais rápida

---

**Criado por**: Sistema de Build Dryad  
**Versão**: 1.0