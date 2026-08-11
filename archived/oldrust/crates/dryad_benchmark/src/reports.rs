// crates/dryad_benchmark/src/reports.rs
use crate::BenchmarkResult;
use colored::*;
use prettytable::{Table, Row, Cell, Attr, color};
use std::time::Duration;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate_console_report(results: &[BenchmarkResult]) {
        println!("\n{}", "═══════════════════════════════════════════════════════════════".blue().bold());
        println!("{}", "                    RELATÓRIO DE BENCHMARK DRYAD".blue().bold());
        println!("{}", "═══════════════════════════════════════════════════════════════".blue().bold());

        let mut table = Table::new();
        table.add_row(Row::new(vec![
            Cell::new("Nome").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Duração (ms)").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Sucesso").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Iterações").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Throughput (ops/s)").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
            Cell::new("Status").with_style(Attr::Bold).with_style(Attr::ForegroundColor(color::BLUE)),
        ]));

        for result in results {
            let duration_ms = result.duration.as_millis();
            let success_text = if result.success { "✓".green().to_string() } else { "✗".red().to_string() };
            let throughput = result.throughput
                .map(|t| format!("{:.2}", t))
                .unwrap_or_else(|| "N/A".to_string());
            
            let status = if result.success {
                match duration_ms {
                    0..=10 => "Excelente".green(),
                    11..=50 => "Bom".yellow(),
                    51..=200 => "Médio".bright_yellow(),
                    _ => "Lento".red(),
                }
            } else {
                "Erro".red()
            };

            table.add_row(Row::new(vec![
                Cell::new(&result.name),
                Cell::new(&duration_ms.to_string()),
                Cell::new(&success_text),
                Cell::new(&result.iterations.to_string()),
                Cell::new(&throughput),
                Cell::new(&status.to_string()),
            ]));
        }

        table.printstd();

        // Estatísticas resumidas
        Self::print_summary_statistics(results);

        // Recomendações
        Self::print_recommendations(results);
    }

    fn print_summary_statistics(results: &[BenchmarkResult]) {
        println!("\n{}", "📊 ESTATÍSTICAS RESUMIDAS".cyan().bold());
        println!("{}", "─".repeat(50).cyan());

        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
        let failed_count = results.len() - successful_results.len();

        if !successful_results.is_empty() {
            let durations: Vec<Duration> = successful_results.iter().map(|r| r.duration).collect();
            
            let total_duration: Duration = durations.iter().sum();
            let avg_duration = total_duration / durations.len() as u32;
            let min_duration = durations.iter().min().unwrap();
            let max_duration = durations.iter().max().unwrap();

            println!("• Total de testes: {}", results.len());
            println!("• Sucessos: {} {}", successful_results.len(), "✓".green());
            if failed_count > 0 {
                println!("• Falhas: {} {}", failed_count, "✗".red());
            }
            println!("• Duração média: {:.2}ms", avg_duration.as_millis());
            println!("• Duração mínima: {:.2}ms", min_duration.as_millis());
            println!("• Duração máxima: {:.2}ms", max_duration.as_millis());

            // Calcular percentis
            let mut sorted_durations = durations.clone();
            sorted_durations.sort();
            
            let p50_idx = sorted_durations.len() / 2;
            let p90_idx = (sorted_durations.len() as f64 * 0.9) as usize;
            let p95_idx = (sorted_durations.len() as f64 * 0.95) as usize;

            if sorted_durations.len() > 1 {
                println!("• P50 (mediana): {:.2}ms", sorted_durations[p50_idx].as_millis());
                println!("• P90: {:.2}ms", sorted_durations[p90_idx.min(sorted_durations.len() - 1)].as_millis());
                println!("• P95: {:.2}ms", sorted_durations[p95_idx.min(sorted_durations.len() - 1)].as_millis());
            }

            // Throughput total
            let total_throughput: f64 = successful_results
                .iter()
                .filter_map(|r| r.throughput)
                .sum();
            
            if total_throughput > 0.0 {
                println!("• Throughput total: {:.2} ops/s", total_throughput);
            }
        } else {
            println!("{}", "Nenhum teste foi executado com sucesso.".red());
        }
    }

    fn print_recommendations(results: &[BenchmarkResult]) {
        println!("\n{}", "💡 RECOMENDAÇÕES".yellow().bold());
        println!("{}", "─".repeat(50).yellow());

        let failed_results: Vec<_> = results.iter().filter(|r| !r.success).collect();
        let slow_results: Vec<_> = results.iter()
            .filter(|r| r.success && r.duration.as_millis() > 100)
            .collect();

        if !failed_results.is_empty() {
            println!("🔧 {}", "Testes falharam:".red().bold());
            for result in failed_results {
                println!("   • {}: {}", result.name, 
                    result.error_message.as_ref().unwrap_or(&"Erro desconhecido".to_string()));
            }
            println!();
        }

        if !slow_results.is_empty() {
            println!("⚠️  {}", "Testes lentos (>100ms):".yellow().bold());
            for result in slow_results {
                println!("   • {}: {:.2}ms", result.name, result.duration.as_millis());
            }
            println!("   💡 Considere otimizar estes componentes.");
            println!();
        }

        // Análise de categorias
        Self::analyze_by_category(results);

        println!("📈 {}", "Para melhorar o desempenho:".green().bold());
        println!("   • Execute com cargo build --release para testes de produção");
        println!("   • Use ferramentas de profiling como 'perf' ou 'valgrind'");
        println!("   • Considere otimizações algorítmicas para componentes lentos");
        println!("   • Monitore o uso de memória durante a execução");
    }

    fn analyze_by_category(results: &[BenchmarkResult]) {
        let categories = vec!["lexer", "parser", "runtime", "end_to_end"];
        
        for category in categories {
            let category_results: Vec<_> = results.iter()
                .filter(|r| r.name.to_lowercase().contains(category))
                .collect();
            
            if !category_results.is_empty() {
                let avg_duration: Duration = category_results
                    .iter()
                    .map(|r| r.duration)
                    .sum::<Duration>() / category_results.len() as u32;
                
                let success_rate = category_results.iter()
                    .filter(|r| r.success)
                    .count() as f64 / category_results.len() as f64 * 100.0;

                println!("📊 Categoria {}: {:.2}ms avg, {:.1}% sucesso", 
                    category.to_uppercase(), avg_duration.as_millis(), success_rate);
            }
        }
    }

    pub fn generate_json_report(results: &[BenchmarkResult]) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(results)
    }

    pub fn generate_html_report(results: &[BenchmarkResult]) -> String {
        let mut html = String::new();
        
        html.push_str(r#"
<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Relatório de Benchmark - Dryad</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; background-color: #f5f5f5; }
        .container { max-width: 1200px; margin: 0 auto; background: white; padding: 20px; border-radius: 8px; box-shadow: 0 2px 4px rgba(0,0,0,0.1); }
        h1 { color: #2c3e50; text-align: center; border-bottom: 3px solid #3498db; padding-bottom: 10px; }
        .summary { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 20px; margin: 20px 0; }
        .summary-card { background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 20px; border-radius: 8px; text-align: center; }
        .summary-card h3 { margin: 0 0 10px 0; }
        .summary-card .value { font-size: 2em; font-weight: bold; }
        table { width: 100%; border-collapse: collapse; margin: 20px 0; }
        th, td { padding: 12px; text-align: left; border-bottom: 1px solid #ddd; }
        th { background-color: #3498db; color: white; }
        tr:nth-child(even) { background-color: #f2f2f2; }
        .success { color: #27ae60; font-weight: bold; }
        .failure { color: #e74c3c; font-weight: bold; }
        .chart-container { margin: 20px 0; }
        .status-excellent { color: #27ae60; }
        .status-good { color: #f39c12; }
        .status-medium { color: #e67e22; }
        .status-slow { color: #e74c3c; }
    </style>
</head>
<body>
    <div class="container">
        <h1>🚀 Relatório de Benchmark - Linguagem Dryad</h1>
"#);

        // Estatísticas resumidas
        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
        let _failed_count = results.len() - successful_results.len();
        
        if !successful_results.is_empty() {
            let durations: Vec<Duration> = successful_results.iter().map(|r| r.duration).collect();
            let avg_duration = durations.iter().sum::<Duration>() / durations.len() as u32;
            let total_throughput: f64 = successful_results
                .iter()
                .filter_map(|r| r.throughput)
                .sum();

            html.push_str(&format!(r#"
        <div class="summary">
            <div class="summary-card">
                <h3>Total de Testes</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Sucessos</h3>
                <div class="value">{}</div>
            </div>
            <div class="summary-card">
                <h3>Duração Média</h3>
                <div class="value">{:.1}ms</div>
            </div>
            <div class="summary-card">
                <h3>Throughput Total</h3>
                <div class="value">{:.1} ops/s</div>
            </div>
        </div>
"#, results.len(), successful_results.len(), avg_duration.as_millis(), total_throughput));
        }

        // Tabela de resultados
        html.push_str(r#"
        <h2>📊 Resultados Detalhados</h2>
        <table>
            <thead>
                <tr>
                    <th>Nome do Teste</th>
                    <th>Duração (ms)</th>
                    <th>Status</th>
                    <th>Iterações</th>
                    <th>Throughput (ops/s)</th>
                    <th>Avaliação</th>
                </tr>
            </thead>
            <tbody>
"#);

        for result in results {
            let duration_ms = result.duration.as_millis();
            let status_class = if result.success { "success" } else { "failure" };
            let status_text = if result.success { "✓ Sucesso" } else { "✗ Falha" };
            let throughput = result.throughput
                .map(|t| format!("{:.2}", t))
                .unwrap_or_else(|| "N/A".to_string());
            
            let (evaluation, eval_class) = if result.success {
                match duration_ms {
                    0..=10 => ("Excelente", "status-excellent"),
                    11..=50 => ("Bom", "status-good"),
                    51..=200 => ("Médio", "status-medium"),
                    _ => ("Lento", "status-slow"),
                }
            } else {
                ("Erro", "failure")
            };

            html.push_str(&format!(r#"
                <tr>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="{}">{}</td>
                    <td>{}</td>
                    <td>{}</td>
                    <td class="{}">{}</td>
                </tr>
"#, result.name, duration_ms, status_class, status_text, result.iterations, throughput, eval_class, evaluation));
        }

        html.push_str(r#"
            </tbody>
        </table>
        
        <div style="margin-top: 40px; padding: 20px; background-color: #ecf0f1; border-radius: 8px;">
            <h3>💡 Informações do Relatório</h3>
            <p><strong>Gerado em:</strong> {}</p>
            <p><strong>Versão do Dryad:</strong> 0.1.0</p>
            <p><strong>Plataforma:</strong> Rust Benchmark Suite</p>
        </div>
    </div>
</body>
</html>
"#);

        html.replace("{}", &chrono::Utc::now().format("%d/%m/%Y %H:%M:%S UTC").to_string())
    }

    pub fn generate_csv_report(results: &[BenchmarkResult]) -> String {
        let mut csv = String::new();
        csv.push_str("nome,duracao_ms,sucesso,iteracoes,throughput,timestamp,categoria\n");
        
        for result in results {
            csv.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                result.name,
                result.duration.as_millis(),
                result.success,
                result.iterations,
                result.throughput.unwrap_or(0.0),
                result.timestamp.format("%Y-%m-%d %H:%M:%S"),
                result.metadata.get("category").unwrap_or(&"unknown".to_string())
            ));
        }
        
        csv
    }

    pub fn generate_markdown_report(results: &[BenchmarkResult]) -> String {
        let mut md = String::new();
        
        // Cabeçalho do relatório
        md.push_str("# 🚀 Relatório de Benchmark - Linguagem Dryad\n\n");
        md.push_str(&format!("**Data de Geração:** {}\n", chrono::Utc::now().format("%d/%m/%Y %H:%M:%S UTC")));
        md.push_str("**Versão:** 0.1.0\n");
        md.push_str("**Plataforma:** Rust Benchmark Suite\n\n");
        
        // Estatísticas resumidas
        let successful_results: Vec<_> = results.iter().filter(|r| r.success).collect();
        let failed_count = results.len() - successful_results.len();
        
        md.push_str("## 📊 Estatísticas Resumidas\n\n");
        
        if !successful_results.is_empty() {
            let durations: Vec<Duration> = successful_results.iter().map(|r| r.duration).collect();
            let total_duration: Duration = durations.iter().sum();
            let avg_duration = total_duration / durations.len() as u32;
            let min_duration = durations.iter().min().unwrap();
            let max_duration = durations.iter().max().unwrap();
            
            let total_throughput: f64 = successful_results
                .iter()
                .filter_map(|r| r.throughput)
                .sum();
            
            // Calcular percentis
            let mut sorted_durations = durations.clone();
            sorted_durations.sort();
            
            let p50_idx = sorted_durations.len() / 2;
            let p90_idx = (sorted_durations.len() as f64 * 0.9) as usize;
            let p95_idx = (sorted_durations.len() as f64 * 0.95) as usize;
            
            md.push_str("| Métrica | Valor |\n");
            md.push_str("|---------|-------|\n");
            md.push_str(&format!("| **Total de Testes** | {} |\n", results.len()));
            md.push_str(&format!("| **Sucessos** | {} ✅ |\n", successful_results.len()));
            if failed_count > 0 {
                md.push_str(&format!("| **Falhas** | {} ❌ |\n", failed_count));
            }
            md.push_str(&format!("| **Duração Média** | {} |\n", Self::format_duration_precise(avg_duration)));
            md.push_str(&format!("| **Duração Mínima** | {} |\n", Self::format_duration_precise(*min_duration)));
            md.push_str(&format!("| **Duração Máxima** | {} |\n", Self::format_duration_precise(*max_duration)));
            
            if sorted_durations.len() > 1 {
                md.push_str(&format!("| **P50 (Mediana)** | {} |\n", Self::format_duration_precise(sorted_durations[p50_idx])));
                md.push_str(&format!("| **P90** | {} |\n", Self::format_duration_precise(sorted_durations[p90_idx.min(sorted_durations.len() - 1)])));
                md.push_str(&format!("| **P95** | {} |\n", Self::format_duration_precise(sorted_durations[p95_idx.min(sorted_durations.len() - 1)])));
            }
            
            if total_throughput > 0.0 {
                md.push_str(&format!("| **Throughput Total** | {:.2} ops/s |\n", total_throughput));
            }
        } else {
            md.push_str("❌ **Nenhum teste foi executado com sucesso.**\n");
        }
        
        md.push_str("\n");
        
        // Análise por categoria
        md.push_str("## 📈 Análise por Categoria\n\n");
        let categories = vec!["lexer", "parser", "runtime", "end_to_end"];
        let mut found_categories = false;
        
        for category in categories {
            let category_results: Vec<_> = results.iter()
                .filter(|r| r.name.to_lowercase().contains(category))
                .collect();
            
            if !category_results.is_empty() {
                found_categories = true;
                let avg_duration: Duration = category_results
                    .iter()
                    .map(|r| r.duration)
                    .sum::<Duration>() / category_results.len() as u32;
                
                let success_rate = category_results.iter()
                    .filter(|r| r.success)
                    .count() as f64 / category_results.len() as f64 * 100.0;

                let status_icon = match avg_duration.as_nanos() {
                    0..=1_000_000 => "🟢",      // < 1ms
                    1_000_001..=50_000_000 => "🟡",   // 1-50ms  
                    50_000_001..=200_000_000 => "🟠", // 50-200ms
                    _ => "🔴",                         // > 200ms
                };

                md.push_str(&format!("- {} **{}**: {} (média), {:.1}% sucesso\n", 
                    status_icon, category.to_uppercase(), Self::format_duration_precise(avg_duration), success_rate));
            }
        }
        
        if !found_categories {
            md.push_str("Nenhuma categoria específica identificada.\n");
        }
        
        md.push_str("\n");
        
        // Tabela de resultados detalhados
        md.push_str("## 📋 Resultados Detalhados\n\n");
        md.push_str("| Nome do Teste | Duração | Status | Iterações | Throughput (ops/s) | Avaliação |\n");
        md.push_str("|---------------|---------|--------|-----------|-------------------|----------|\n");
        
        for result in results {
            let duration_formatted = Self::format_duration_for_table(result.duration);
            let status_icon = if result.success { "✅" } else { "❌" };
            let throughput = result.throughput
                .map(|t| format!("{:.2}", t))
                .unwrap_or_else(|| "N/A".to_string());
            
            let (evaluation, eval_icon) = if result.success {
                match result.duration.as_nanos() {
                    0..=1_000_000 => ("Excelente", "🟢"),  // < 1ms
                    1_000_001..=50_000_000 => ("Bom", "🟡"),       // 1-50ms
                    50_000_001..=200_000_000 => ("Médio", "🟠"),   // 50-200ms
                    _ => ("Lento", "🔴"),                            // > 200ms
                }
            } else {
                ("Erro", "❌")
            };

            md.push_str(&format!("| {} | {} | {} | {} | {} | {} {} |\n",
                result.name, duration_formatted, status_icon, result.iterations, throughput, eval_icon, evaluation));
        }
        
        md.push_str("\n");
        
        // Recomendações e análises
        let failed_results: Vec<_> = results.iter().filter(|r| !r.success).collect();
        let slow_results: Vec<_> = results.iter()
            .filter(|r| r.success && r.duration.as_millis() > 100)
            .collect();
        
        md.push_str("## 💡 Recomendações e Análises\n\n");
        
        if !failed_results.is_empty() {
            md.push_str("### 🔧 Testes que Falharam\n\n");
            for result in failed_results {
                md.push_str(&format!("- **{}**: {}\n", 
                    result.name, 
                    result.error_message.as_ref().unwrap_or(&"Erro desconhecido".to_string())
                ));
            }
            md.push_str("\n");
        }
        
        if !slow_results.is_empty() {
            md.push_str("### ⚠️ Testes Lentos (>100ms)\n\n");
            for result in slow_results {
                md.push_str(&format!("- **{}**: {:.2}ms\n", result.name, result.duration.as_millis()));
            }
            md.push_str("\n💡 **Sugestão**: Considere otimizar estes componentes.\n\n");
        }
        
        // Dicas de otimização
        md.push_str("## 🚀 Dicas para Melhorar o Desempenho\n\n");
        md.push_str("1. **Compilação Release**: Execute com `cargo build --release` para testes de produção\n");
        md.push_str("2. **Profiling**: Use ferramentas como `perf` ou `valgrind` para análise detalhada\n");
        md.push_str("3. **Otimizações Algorítmicas**: Revise algoritmos em componentes lentos\n");
        md.push_str("4. **Monitoramento de Memória**: Observe o uso de memória durante execução\n");
        md.push_str("5. **Benchmarks Criterion**: Use `cargo bench` para medições estatísticas precisas\n\n");
        
        // Rodapé
        md.push_str("---\n\n");
        md.push_str("*Relatório gerado automaticamente pela ferramenta de benchmark do Dryad*\n");
        
        md
    }

    // Helper function to format duration with appropriate precision
    fn format_duration_precise(duration: Duration) -> String {
        let nanos = duration.as_nanos();
        
        if nanos == 0 {
            return "< 0.001μs".to_string();
        } else if nanos < 1_000 {
            format!("{:.2}ns", nanos)
        } else if nanos < 1_000_000 {
            format!("{:.2}μs", nanos as f64 / 1_000.0)
        } else if nanos < 1_000_000_000 {
            format!("{:.2}ms", nanos as f64 / 1_000_000.0)
        } else {
            format!("{:.2}s", nanos as f64 / 1_000_000_000.0)
        }
    }

    fn format_duration_for_table(duration: Duration) -> String {
        let nanos = duration.as_nanos();
        
        if nanos == 0 {
            "< 0.001".to_string()
        } else if nanos < 1_000_000 {
            format!("{:.3}", nanos as f64 / 1_000_000.0)
        } else {
            format!("{:.2}", nanos as f64 / 1_000_000.0)
        }
    }
}
