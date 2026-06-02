// crates/dryad_benchmark/src/lib.rs
use std::time::{Duration, Instant};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

pub mod test_cases;
pub mod reports;
pub mod profiler;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResult {
    pub name: String,
    pub duration: Duration,
    pub timestamp: DateTime<Utc>,
    pub memory_usage: Option<usize>,
    pub success: bool,
    pub error_message: Option<String>,
    pub iterations: u64,
    pub throughput: Option<f64>,
    pub metadata: HashMap<String, String>,
}

impl BenchmarkResult {
    pub fn new(name: String) -> Self {
        Self {
            name,
            duration: Duration::default(),
            timestamp: Utc::now(),
            memory_usage: None,
            success: true,
            error_message: None,
            iterations: 1,
            throughput: None,
            metadata: HashMap::new(),
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = duration;
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.success = false;
        self.error_message = Some(error);
        self
    }

    pub fn with_iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    pub fn with_throughput(mut self, throughput: f64) -> Self {
        self.throughput = Some(throughput);
        self
    }

    pub fn add_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: u64,
    pub warmup_iterations: u64,
    pub measure_memory: bool,
    pub output_format: OutputFormat,
    pub save_to_file: Option<String>,
}

#[derive(Debug, Clone)]
pub enum OutputFormat {
    Console,
    Json,
    Html,
    Csv,
    Markdown,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            iterations: 100,
            warmup_iterations: 10,
            measure_memory: false,
            output_format: OutputFormat::Console,
            save_to_file: None,
        }
    }
}

pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    pub fn new(config: BenchmarkConfig) -> Self {
        Self {
            config,
            results: Vec::new(),
        }
    }

    pub fn run<F>(&mut self, name: &str, mut benchmark_fn: F) -> BenchmarkResult
    where
        F: FnMut() -> Result<(), Box<dyn std::error::Error>>,
    {
        // Aquecimento
        for _ in 0..self.config.warmup_iterations {
            let _ = benchmark_fn();
        }

        let start = Instant::now();
        let mut errors = Vec::new();
        let mut successful_iterations = 0u64;

        for _ in 0..self.config.iterations {
            match benchmark_fn() {
                Ok(_) => successful_iterations += 1,
                Err(e) => errors.push(e.to_string()),
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / self.config.iterations as u32;

        let mut result = BenchmarkResult::new(name.to_string())
            .with_duration(avg_duration)
            .with_iterations(successful_iterations);

        if !errors.is_empty() {
            result = result.with_error(errors.join("; "));
        }

        if successful_iterations > 0 {
            let throughput = successful_iterations as f64 / duration.as_secs_f64();
            result = result.with_throughput(throughput);
        }

        self.results.push(result.clone());
        result
    }

    pub fn run_with_input<F, I>(&mut self, name: &str, input: I, mut benchmark_fn: F) -> BenchmarkResult
    where
        F: FnMut(&I) -> Result<(), Box<dyn std::error::Error>>,
        I: Clone,
    {
        // Aquecimento
        for _ in 0..self.config.warmup_iterations {
            let _ = benchmark_fn(&input);
        }

        let start = Instant::now();
        let mut errors = Vec::new();
        let mut successful_iterations = 0u64;

        for _ in 0..self.config.iterations {
            match benchmark_fn(&input) {
                Ok(_) => successful_iterations += 1,
                Err(e) => errors.push(e.to_string()),
            }
        }

        let duration = start.elapsed();
        let avg_duration = duration / self.config.iterations as u32;

        let mut result = BenchmarkResult::new(name.to_string())
            .with_duration(avg_duration)
            .with_iterations(successful_iterations);

        if !errors.is_empty() {
            result = result.with_error(errors.join("; "));
        }

        if successful_iterations > 0 {
            let throughput = successful_iterations as f64 / duration.as_secs_f64();
            result = result.with_throughput(throughput);
        }

        self.results.push(result.clone());
        result
    }

    pub fn get_results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    pub fn clear_results(&mut self) {
        self.results.clear();
    }

    pub fn save_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(filename) = &self.config.save_to_file {
            match self.config.output_format {
                OutputFormat::Json => {
                    let json = serde_json::to_string_pretty(&self.results)?;
                    std::fs::write(filename, json)?;
                }
                OutputFormat::Csv => {
                    let mut csv_content = String::new();
                    csv_content.push_str("name,duration_ms,success,iterations,throughput,timestamp\n");
                    
                    for result in &self.results {
                        csv_content.push_str(&format!(
                            "{},{},{},{},{},{}\n",
                            result.name,
                            result.duration.as_millis(),
                            result.success,
                            result.iterations,
                            result.throughput.unwrap_or(0.0),
                            result.timestamp.format("%Y-%m-%d %H:%M:%S")
                        ));
                    }
                    std::fs::write(filename, csv_content)?;
                }
                OutputFormat::Html => {
                    let html = reports::ReportGenerator::generate_html_report(&self.results);
                    std::fs::write(filename, html)?;
                }
                OutputFormat::Markdown => {
                    let markdown = reports::ReportGenerator::generate_markdown_report(&self.results);
                    std::fs::write(filename, markdown)?;
                }
                _ => {
                    return Err("Formato de saída não suportado para salvar em arquivo".into());
                }
            }
        }
        Ok(())
    }
}
