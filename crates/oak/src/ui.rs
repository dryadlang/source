use colored::*;

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "✖".red().bold(), msg);
}

pub fn print_warning(msg: &str) {
    println!("{} {}", "⚠️".yellow().bold(), msg);
}

pub fn print_info(msg: &str) {
    println!("{} {}", "ℹ".blue().bold(), msg);
}
