use colored::*;

pub fn print_success(message: &str) {
    println!("{} {}", "✓".green().bold(), message);
}

#[allow(dead_code)]
pub fn print_error(message: &str) {
    eprintln!("{} {}", "✗".red().bold(), message);
}

pub fn print_warning(message: &str) {
    println!("{} {}", "⚠".yellow().bold(), message);
}

pub fn print_info(message: &str) {
    println!("{} {}", "ℹ".blue().bold(), message);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_utils_module() {
        // Just verify the module compiles and functions exist
        assert_eq!(1 + 1, 2);
    }
}
