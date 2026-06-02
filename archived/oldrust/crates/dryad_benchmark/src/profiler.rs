// crates/dryad_benchmark/src/profiler.rs
use std::time::{Duration, Instant};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ProfilePoint {
    pub name: String,
    pub start_time: Instant,
    pub duration: Option<Duration>,
    pub memory_before: Option<usize>,
    pub memory_after: Option<usize>,
}

pub struct Profiler {
    points: HashMap<String, ProfilePoint>,
    stack: Vec<String>,
    measurements: Vec<ProfilePoint>,
}

impl Profiler {
    pub fn new() -> Self {
        Self {
            points: HashMap::new(),
            stack: Vec::new(),
            measurements: Vec::new(),
        }
    }

    pub fn start(&mut self, name: &str) {
        let point = ProfilePoint {
            name: name.to_string(),
            start_time: Instant::now(),
            duration: None,
            memory_before: Self::get_memory_usage(),
            memory_after: None,
        };

        self.points.insert(name.to_string(), point);
        self.stack.push(name.to_string());
    }

    pub fn end(&mut self, name: &str) -> Option<Duration> {
        if let Some(mut point) = self.points.remove(name) {
            let end_time = Instant::now();
            let duration = end_time.duration_since(point.start_time);
            
            point.duration = Some(duration);
            point.memory_after = Self::get_memory_usage();
            
            self.measurements.push(point);
            
            // Remove do stack se for o último item
            if let Some(last) = self.stack.last() {
                if last == name {
                    self.stack.pop();
                }
            }
            
            Some(duration)
        } else {
            None
        }
    }

    pub fn measure<F, R>(&mut self, name: &str, mut func: F) -> R 
    where 
        F: FnMut() -> R 
    {
        self.start(name);
        let result = func();
        self.end(name);
        result
    }

    pub fn get_measurements(&self) -> &[ProfilePoint] {
        &self.measurements
    }

    pub fn clear(&mut self) {
        self.points.clear();
        self.stack.clear();
        self.measurements.clear();
    }

    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== RELATÓRIO DE PROFILING ===\n\n");

        if self.measurements.is_empty() {
            report.push_str("Nenhuma medição disponível.\n");
            return report;
        }

        // Medições por nome (agregadas)
        let mut aggregated: HashMap<String, Vec<&ProfilePoint>> = HashMap::new();
        for measurement in &self.measurements {
            aggregated.entry(measurement.name.clone())
                .or_insert_with(Vec::new)
                .push(measurement);
        }

        report.push_str("📊 MEDIÇÕES AGREGADAS:\n");
        report.push_str(&format!("{:<30} {:<15} {:<15} {:<15} {:<15}\n", 
            "Nome", "Chamadas", "Total (ms)", "Média (ms)", "Memória (KB)"));
        report.push_str(&"-".repeat(90));
        report.push('\n');

        let mut total_time = Duration::new(0, 0);

        for (name, points) in &aggregated {
            let call_count = points.len();
            let total_duration: Duration = points.iter()
                .filter_map(|p| p.duration)
                .sum();
            let avg_duration = if call_count > 0 {
                total_duration / call_count as u32
            } else {
                Duration::new(0, 0)
            };

            let memory_usage = points.last()
                .and_then(|p| {
                    if let (Some(before), Some(after)) = (p.memory_before, p.memory_after) {
                        Some((after as i64 - before as i64) / 1024) // KB
                    } else {
                        None
                    }
                })
                .unwrap_or(0);

            report.push_str(&format!("{:<30} {:<15} {:<15.2} {:<15.2} {:<15}\n",
                name,
                call_count,
                total_duration.as_millis(),
                avg_duration.as_millis(),
                memory_usage
            ));

            total_time += total_duration;
        }

        report.push_str(&"-".repeat(90));
        report.push('\n');
        report.push_str(&format!("TEMPO TOTAL: {:.2}ms\n\n", total_time.as_millis()));

        // Análise de hotspots
        report.push_str("🔥 HOTSPOTS (>10% do tempo total):\n");
        let total_ms = total_time.as_millis() as f64;
        
        for (name, points) in &aggregated {
            let duration: Duration = points.iter()
                .filter_map(|p| p.duration)
                .sum();
            let percentage = (duration.as_millis() as f64 / total_ms) * 100.0;
            
            if percentage > 10.0 {
                report.push_str(&format!("  • {}: {:.1}% ({:.2}ms)\n", 
                    name, percentage, duration.as_millis()));
            }
        }

        // Cronologia detalhada (últimas 10 medições)
        report.push_str("\n⏱️  CRONOLOGIA (últimas 10 medições):\n");
        let recent: Vec<_> = self.measurements.iter().rev().take(10).collect();
        
        for point in recent.iter().rev() {
            if let Some(duration) = point.duration {
                report.push_str(&format!("  {} -> {:.2}ms\n", 
                    point.name, duration.as_millis()));
            }
        }

        report
    }

    // Função auxiliar para obter uso de memória (simulada)
    fn get_memory_usage() -> Option<usize> {
        // Em um ambiente real, isso poderia usar APIs específicas do sistema
        // Por simplicidade, retornamos None aqui
        None
    }

    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        use serde_json::json;
        
        let measurements: Vec<_> = self.measurements.iter().map(|m| {
            json!({
                "name": m.name,
                "duration_ms": m.duration.map(|d| d.as_millis()).unwrap_or(0),
                "memory_before": m.memory_before,
                "memory_after": m.memory_after
            })
        }).collect();

        serde_json::to_string_pretty(&json!({
            "measurements": measurements,
            "total_measurements": self.measurements.len(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

// Macro para facilitar o profiling
#[macro_export]
macro_rules! profile {
    ($profiler:expr, $name:expr, $block:block) => {
        $profiler.measure($name, || $block)
    };
}

// Struct para análise comparativa
#[derive(Debug)]
pub struct ComparisonReport {
    pub before: Vec<ProfilePoint>,
    pub after: Vec<ProfilePoint>,
}

impl ComparisonReport {
    pub fn new(before: Vec<ProfilePoint>, after: Vec<ProfilePoint>) -> Self {
        Self { before, after }
    }

    pub fn generate_comparison(&self) -> String {
        let mut report = String::new();
        report.push_str("=== RELATÓRIO COMPARATIVO ===\n\n");

        // Agregar medições por nome
        let before_agg = Self::aggregate_by_name(&self.before);
        let after_agg = Self::aggregate_by_name(&self.after);

        report.push_str(&format!("{:<30} {:<15} {:<15} {:<15}\n", 
            "Nome", "Antes (ms)", "Depois (ms)", "Diferença (%)"));
        report.push_str(&"-".repeat(75));
        report.push('\n');

        for name in before_agg.keys() {
            if let (Some(before_duration), Some(after_duration)) = 
                (before_agg.get(name), after_agg.get(name)) {
                
                let before_ms = before_duration.as_millis() as f64;
                let after_ms = after_duration.as_millis() as f64;
                let change_percent = if before_ms > 0.0 {
                    ((after_ms - before_ms) / before_ms) * 100.0
                } else {
                    0.0
                };

                let change_indicator = if change_percent > 5.0 {
                    "🔴"
                } else if change_percent < -5.0 {
                    "🟢"
                } else {
                    "🟡"
                };

                report.push_str(&format!("{} {:<25} {:<15.2} {:<15.2} {:<15.1}%\n",
                    change_indicator, name, before_ms, after_ms, change_percent));
            }
        }

        report
    }

    fn aggregate_by_name(points: &[ProfilePoint]) -> HashMap<String, Duration> {
        let mut aggregated = HashMap::new();
        
        for point in points {
            if let Some(duration) = point.duration {
                *aggregated.entry(point.name.clone()).or_insert(Duration::new(0, 0)) += duration;
            }
        }
        
        aggregated
    }
}
