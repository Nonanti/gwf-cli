use anyhow::{Context, Result};
use chrono::{Duration, Local};
use colored::*;
use std::process::Command;

use crate::git::GitRepo;
use crate::utils::print_info;

pub async fn execute(days: u32, all: bool) -> Result<()> {
    let repo = GitRepo::open_current()?;

    println!(
        "{}",
        "Daily Standup Report".bright_white().bold().underline()
    );
    println!(
        "{}",
        Local::now()
            .format("%A, %B %d, %Y")
            .to_string()
            .bright_cyan()
    );
    println!();

    let since_date = Local::now() - Duration::days(days as i64);
    let since_str = since_date.format("%Y-%m-%d").to_string();

    let mut cmd = Command::new("git");
    cmd.arg("log")
        .arg("--since")
        .arg(&since_str)
        .arg("--author")
        .arg(get_git_user()?)
        .arg("--pretty=format:%h|%s|%cr|%b")
        .arg("--no-merges");

    if !all {
        cmd.arg("--branches");
    }

    let output = cmd.output().context("Failed to get git log")?;

    let log_output = String::from_utf8_lossy(&output.stdout);

    if log_output.is_empty() {
        print_info(&format!("No commits in the last {} day(s)", days));
    } else {
        println!("{}", "Recent commits:".bright_white().underline());
        for line in log_output.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 3 {
                let hash = parts[0];
                let message = parts[1];
                let time = parts[2];

                println!(
                    "  {} {} {}",
                    hash.bright_yellow(),
                    message.bright_white(),
                    format!("({})", time).bright_black()
                );
            }
        }
    }

    println!("\n{}", "Current status:".bright_white().underline());
    let current_branch = repo.current_branch()?;
    println!("  Branch: {}", current_branch.bright_cyan());

    if repo.has_uncommitted_changes()? {
        println!("  {} Uncommitted changes", "⚠".yellow());
    } else {
        println!("  {} Working directory clean", "✓".green());
    }

    let mut cmd = Command::new("git");
    cmd.arg("for-each-ref")
        .arg("--sort=-committerdate")
        .arg("--format=%(refname:short)|%(committerdate:relative)")
        .arg("refs/heads/")
        .arg("--count=5");

    let output = cmd.output().context("Failed to get branch list")?;

    let branches = String::from_utf8_lossy(&output.stdout);

    if !branches.is_empty() {
        println!("\n{}", "Recent branches:".bright_white().underline());
        for line in branches.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 2 {
                let branch = parts[0];
                let last_commit = parts[1];

                let marker = if branch == current_branch { "*" } else { " " };
                println!(
                    "  {} {} {}",
                    marker.bright_green(),
                    branch.bright_cyan(),
                    format!("({})", last_commit).bright_black()
                );
            }
        }
    }

    println!("\n{}", "Tips:".bright_white().underline());
    println!("  • Run 'gwf sync' to update your branches");
    println!("  • Run 'gwf cleanup' to remove merged branches");
    println!("  • Run 'gwf pr' to create a pull request");

    Ok(())
}

fn get_git_user() -> Result<String> {
    let output = Command::new("git")
        .args(["config", "user.email"])
        .output()
        .context("Failed to get git user email")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
