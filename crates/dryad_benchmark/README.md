# Dryad Benchmark

Ferramenta completa de benchmark para a linguagem de programação Dryad, oferecendo análise de performance detalhada para todos os componentes do compilador/interpretador.

## 🚀 Características

- **Benchmarks Completos**: Testa lexer, parser, runtime e pipeline completo
- **Múltiplos Formatos de Relatório**: Console, JSON, HTML e CSV
- **Profiling Detalhado**: Análise de tempo e uso de memória
- **Casos de Teste Variados**: De simples operações a programas complexos
- **Análise Comparativa**: Compare performance entre versões
- **Stress Testing**: Testes de carga e estabilidade
- **Interface Amigável**: CLI intuitiva com barra de progresso e cores

## 📦 Instalação

### Como parte do workspace Dryad:

```bash
cargo build --release -p dryad_benchmark
```

### Executável independente:

```bash
cd crates/dryad_benchmark
cargo install --path .
```

## 🔧 Uso

### Benchmark Completo

Execute todos os testes de benchmark:

```bash
# Teste completo com 100 iterações
cargo run -p dryad_benchmark -- all

# Com profiling detalhado
cargo run -p dryad_benchmark -- all --profile

# Salvar relatório em HTML
cargo run -p dryad_benchmark -- all --format html --output relatorio.html

# Ajustar número de iterações
cargo run -p dryad_benchmark -- all --iterations 500 --warmup 20
```

### Benchmarks por Categoria

```bash
# Apenas lexer
cargo run -p dryad_benchmark -- lexer

# Apenas parser
cargo run -p dryad_benchmark -- parser

# Apenas runtime
cargo run -p dryad_benchmark -- runtime
```

### Teste de Arquivo Específico

```bash
# Testar um arquivo .dryad específico
cargo run -p dryad_benchmark -- file test_files/biblioteca.dryad

# Com profiling
cargo run -p dryad_benchmark -- file test_files/matematica.dryad --profile
```

### Benchmarks com Criterion

Para benchmarks mais precisos usando criterion:

```bash
# Benchmark do lexer
cargo bench --bench lexer_bench

# Benchmark do parser
cargo bench --bench parser_bench

# Benchmark do runtime
cargo bench --bench runtime_bench

# Benchmark end-to-end
cargo bench --bench end_to_end_bench

# Todos os benchmarks
cargo bench
```

### Análise Comparativa

```bash
# Comparar com baseline anterior
cargo run -p dryad_benchmark -- compare baseline_results/ --run-new

# Stress test
cargo run -p dryad_benchmark -- stress --duration 120 --threads 4
```

## 📊 Formatos de Relatório

### Console (Padrão)
Relatório colorido e formatado para terminal com:
- Tabela de resultados
- Estatísticas resumidas (média, min, max, percentis)
- Hotspots de performance
- Recomendações de otimização

### JSON
Dados estruturados para análise automatizada:
```bash
# Executar todos os benchmarks com saída em Markdown
cargo run -p dryad_benchmark -- all --format markdown --output benchmark.md --iterations 3

# Executar todos os benchmarks com saída em Markdown e 10000 iterações / para testes de produção
cargo run -p dryad_benchmark -- all --format markdown --output benchmark.md --iterations 10000

# Benchmarks específicos por categoria
cargo run -p dryad_benchmark -- lexer --format markdown --output lexer_bench.md
cargo run -p dryad_benchmark -- parser --format console
cargo run -p dryad_benchmark -- runtime --format json --output runtime.json
```

## 🧪 Casos de Teste

### Lexer
- Expressões aritméticas simples
- Strings com caracteres especiais
- Arrays e objetos literais
- Literais numéricos diversos
- Escalabilidade com código grande

### Parser
- Funções recursivas (Fibonacci)
- Estruturas de controle aninhadas
- Expressões matemáticas complexas
- Classes com herança
- Análise de complexidade algorítmica

### Runtime
- Operações intensivas com arrays
- Recursão profunda
- Criação massiva de objetos
- Manipulação de strings
- Estruturas de dados complexas

### End-to-End
- Programas completos (sistema de biblioteca)
- Cálculos matemáticos intensivos
- Simulações do mundo real
- Testes de escalabilidade

## 📈 Análise de Performance

O benchmark fornece métricas detalhadas:

- **Duração**: Tempo médio de execução
- **Throughput**: Operações por segundo
- **Sucesso**: Taxa de execução bem-sucedida
- **Memória**: Uso de memória (quando disponível)
- **Percentis**: P50, P90, P95 para análise de distribuição
- **Hotspots**: Componentes que consomem mais tempo

### Categorização de Performance

- 🟢 **Excelente**: ≤ 10ms
- 🟡 **Bom**: 11-50ms
- 🟠 **Médio**: 51-200ms
- 🔴 **Lento**: > 200ms

## 🔬 Profiling Avançado

O profiler integrado oferece:

- Medição precisa por componente
- Análise de stack de chamadas
- Comparação entre execuções
- Relatórios em JSON para análise externa
- Detecção automática de hotspots

```rust
// Uso do profiler em código
use dryad_benchmark::{profile, Profiler};

let mut profiler = Profiler::new();

profile!(profiler, "lexer_phase", {
    // código do lexer
});

profile!(profiler, "parser_phase", {
    // código do parser
});

println!("{}", profiler.generate_report());
```

## 🎯 Casos de Uso

### Desenvolvimento
- Validar mudanças não prejudicam performance
- Identificar regressões de performance
- Otimizar componentes específicos

### CI/CD
- Testes automáticos de performance
- Alertas de degradação
- Comparação entre branches

### Pesquisa
- Análise acadêmica de performance
- Dados para papers e apresentações
- Comparação com outras linguagens

## 📝 Exemplos de Saída

### Console
```
🚀 Iniciando benchmark completo da linguagem Dryad...

═══════════════════════════════════════════════════════════════
                    RELATÓRIO DE BENCHMARK DRYAD
═══════════════════════════════════════════════════════════════

📊 ESTATÍSTICAS RESUMIDAS
──────────────────────────────────────────────────────
• Total de testes: 15
• Sucessos: 15 ✓
• Duração média: 23.45ms
• Duração mínima: 2.1ms
• Duração máxima: 156.7ms
• P50 (mediana): 15.2ms
• P90: 89.3ms
• P95: 134.2ms
• Throughput total: 1247.8 ops/s

💡 RECOMENDAÇÕES
──────────────────────────────────────────────────────
📈 Para melhorar o desempenho:
   • Execute com cargo build --release para testes de produção
   • Use ferramentas de profiling como 'perf' ou 'valgrind'
   • Considere otimizações algorítmicas para componentes lentos
   • Monitore o uso de memória durante a execução
```

## 🛠️ Configuração Avançada

### Configuração Personalizada

```rust
use dryad_benchmark::{BenchmarkConfig, OutputFormat};

let config = BenchmarkConfig {
    iterations: 1000,
    warmup_iterations: 50,
    measure_memory: true,
    output_format: OutputFormat::Html,
    save_to_file: Some("custom_report.html".to_string()),
};
```

### Extensibilidade

Adicione seus próprios casos de teste:

```rust
use dryad_benchmark::test_cases::{TestCase, TestCategory};

let custom_test = TestCase {
    name: "meu_teste".to_string(),
    code: "// seu código Dryad aqui".to_string(),
    description: "Descrição do teste".to_string(),
    expected_complexity: "O(n)".to_string(),
    category: TestCategory::Runtime,
};
```

## 🤝 Contribuição

Contribuições são bem-vindas! Especialmente:

- Novos casos de teste
- Otimizações de performance
- Novos formatos de relatório
- Melhorias na análise

## 📄 Licença

Este projeto segue a mesma licença do projeto principal Dryad.
