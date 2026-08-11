# 🚀 Relatório de Benchmark - Linguagem Dryad

**Data de Geração:** 21/09/2025 01:56:09 UTC
**Versão:** 0.1.0
**Plataforma:** Rust Benchmark Suite

## 📊 Estatísticas Resumidas

| Métrica | Valor |
|---------|-------|
| **Total de Testes** | 15 |
| **Sucessos** | 15 ✅ |
| **Duração Média** | 3.59ms |
| **Duração Mínima** | 6.90μs |
| **Duração Máxima** | 19.48ms |
| **P50 (Mediana)** | 307.89μs |
| **P90** | 16.48ms |
| **P95** | 19.48ms |
| **Throughput Total** | 366946.49 ops/s |

## 📈 Análise por Categoria

Nenhuma categoria específica identificada.

## 📋 Resultados Detalhados

| Nome do Teste | Duração | Status | Iterações | Throughput (ops/s) | Avaliação |
|---------------|---------|--------|-----------|-------------------|----------|
| simple_arithmetic | 0.009 | ✅ | 100000 | 111412.38 | 🟢 Excelente |
| string_literals | 0.007 | ✅ | 100000 | 144934.93 | 🟢 Excelente |
| variable_assignment | 0.239 | ✅ | 100000 | 4181.49 | 🟢 Excelente |
| complete_program | 0.291 | ✅ | 100000 | 3431.04 | 🟢 Excelente |
| while_loop | 0.687 | ✅ | 100000 | 1456.65 | 🟢 Excelente |
| if_statement | 0.262 | ✅ | 100000 | 3822.69 | 🟢 Excelente |
| for_loop | 19.48 | ✅ | 100000 | 51.34 | 🟡 Bom |
| nested_loops | 16.48 | ✅ | 100000 | 60.68 | 🟡 Bom |
| variable_operations | 0.276 | ✅ | 100000 | 3619.95 | 🟢 Excelente |
| exception_handling | 0.444 | ✅ | 100000 | 2253.68 | 🟢 Excelente |
| class_basic | 0.308 | ✅ | 100000 | 3247.91 | 🟢 Excelente |
| complex_function | 0.310 | ✅ | 100000 | 3227.09 | 🟢 Excelente |
| stress_loop_1 | 0.603 | ✅ | 100000 | 1658.85 | 🟢 Excelente |
| stress_loop_2 | 14.42 | ✅ | 100000 | 69.33 | 🟡 Bom |
| function_definition | 0.012 | ✅ | 100000 | 83518.49 | 🟢 Excelente |

## 💡 Recomendações e Análises

## 🚀 Dicas para Melhorar o Desempenho

1. **Compilação Release**: Execute com `cargo build --release` para testes de produção
2. **Profiling**: Use ferramentas como `perf` ou `valgrind` para análise detalhada
3. **Otimizações Algorítmicas**: Revise algoritmos em componentes lentos
4. **Monitoramento de Memória**: Observe o uso de memória durante execução
5. **Benchmarks Criterion**: Use `cargo bench` para medições estatísticas precisas

---

*Relatório gerado automaticamente pela ferramenta de benchmark do Dryad*
