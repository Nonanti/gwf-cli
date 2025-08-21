use anyhow::{bail, Context, Result};
use colored::*;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::git::GitRepo;

pub async fn execute(force: bool) -> Result<()> {
    println!(
        "{}",
        "Initializing GWF in current repository...".bright_blue()
    );

    let repo = GitRepo::open_current().context("Not in a git repository. Run 'git init' first.")?;

    let config_path = Path::new(".gwf.toml");
    if config_path.exists() && !force {
        bail!("GWF is already initialized. Use --force to reinitialize.");
    }

    let config = Config::default();
    let config_content = toml::to_string_pretty(&config)?;

    fs::write(config_path, config_content).context("Failed to write configuration file")?;

    let gitignore_path = Path::new(".gitignore");
    if gitignore_path.exists() {
        let gitignore = fs::read_to_string(gitignore_path)?;
        if !gitignore.contains(".gwf.toml") {
            fs::write(
                gitignore_path,
                format!("{}\n# GWF configuration\n.gwf.toml\n", gitignore),
            )?;
            println!("  {} Added .gwf.toml to .gitignore", "✓".green());
        }
    }

    let current_branch = repo.current_branch()?;
    let remotes = repo.list_remotes()?;

    println!("\n{}", "Repository Information:".bright_white().underline());
    println!("  Current branch: {}", current_branch.bright_yellow());
    println!(
        "  Remotes: {}",
        if remotes.is_empty() {
            "none".red().to_string()
        } else {
            remotes.join(", ").bright_cyan().to_string()
        }
    );

    println!("\n{} GWF initialized successfully!", "✓".green().bold());
    println!("\n{}", "Next steps:".bright_white());
    println!("  1. Review and customize .gwf.toml");
    println!("  2. Run 'gwf feature <name>' to start a new feature");
    println!("  3. Run 'gwf --help' to see all available commands");

    Ok(())
}
