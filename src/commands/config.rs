use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::config::Config;
use crate::utils::{print_info, print_success};

pub async fn execute(_show: bool, edit: bool, reset: bool) -> Result<()> {
    let config_path = Path::new(".gwf.toml");

    if reset {
        print_info("Resetting configuration to defaults...");
        let default_config = Config::default();
        default_config.save()?;
        print_success("Configuration reset to defaults");
        return Ok(());
    }

    if edit {
        let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        print_info(&format!("Opening configuration in {}...", editor));

        Command::new(editor)
            .arg(".gwf.toml")
            .status()
            .context("Failed to open editor")?;

        print_success("Configuration edited");
        return Ok(());
    }

    if !config_path.exists() {
        print_info("No configuration file found. Run 'gwf init' to create one.");
        return Ok(());
    }

    let content = fs::read_to_string(config_path).context("Failed to read configuration file")?;

    println!("{}", "Current configuration:".bright_white().underline());
    println!("{}", content);

    Ok(())
}
