use anyhow::Result;
use colored::Colorize;
use dialoguer::{Select, Input, Confirm};
use std::process::{Command, Stdio};

pub async fn run() -> Result<()> {
    let choices = vec![
        "Start bisect",
        "Mark as good",
        "Mark as bad",
        "Skip current",
        "Reset bisect",
        "Automated bisect with script"
    ];
    
    let selection = Select::new()
        .with_prompt("Bisect action")
        .items(&choices)
        .default(0)
        .interact()?;
    
    match selection {
        0 => start_bisect()?,
        1 => mark_good()?,
        2 => mark_bad()?,
        3 => skip_current()?,
        4 => reset_bisect()?,
        5 => automated_bisect()?,
        _ => {}
    }
    
    Ok(())
}

fn start_bisect() -> Result<()> {
    Command::new("git")
        .args(&["bisect", "start"])
        .status()?;
    
    let bad_commit: String = Input::new()
        .with_prompt("Bad commit (or press enter for HEAD)")
        .default("HEAD".to_string())
        .interact_text()?;
    
    Command::new("git")
        .args(&["bisect", "bad", &bad_commit])
        .status()?;
    
    let good_commit: String = Input::new()
        .with_prompt("Good commit")
        .interact_text()?;
    
    Command::new("git")
        .args(&["bisect", "good", &good_commit])
        .status()?;
    
    println!("{}", "Bisect started. Test and mark commits as good/bad.".green());
    show_status()?;
    
    Ok(())
}

fn mark_good() -> Result<()> {
    Command::new("git")
        .args(&["bisect", "good"])
        .status()?;
    show_status()?;
    Ok(())
}

fn mark_bad() -> Result<()> {
    Command::new("git")
        .args(&["bisect", "bad"])
        .status()?;
    show_status()?;
    Ok(())
}

fn skip_current() -> Result<()> {
    Command::new("git")
        .args(&["bisect", "skip"])
        .status()?;
    show_status()?;
    Ok(())
}

fn reset_bisect() -> Result<()> {
    Command::new("git")
        .args(&["bisect", "reset"])
        .status()?;
    println!("{}", "Bisect reset".green());
    Ok(())
}

fn automated_bisect() -> Result<()> {
    let script_path: String = Input::new()
        .with_prompt("Test script path")
        .interact_text()?;
    
    let confirm = Confirm::new()
        .with_prompt("Start automated bisect?")
        .default(true)
        .interact()?;
    
    if !confirm {
        return Ok(());
    }
    
    println!("{}", "Starting automated bisect...".cyan());
    
    Command::new("git")
        .args(&["bisect", "start"])
        .status()?;
    
    let bad_commit: String = Input::new()
        .with_prompt("Bad commit")
        .default("HEAD".to_string())
        .interact_text()?;
    
    Command::new("git")
        .args(&["bisect", "bad", &bad_commit])
        .status()?;
    
    let good_commit: String = Input::new()
        .with_prompt("Good commit")
        .interact_text()?;
    
    Command::new("git")
        .args(&["bisect", "good", &good_commit])
        .status()?;
    
    let output = Command::new("git")
        .args(&["bisect", "run", &script_path])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    
    if output.status.success() {
        println!("{}", "Bisect completed!".green().bold());
        
        let output = Command::new("git")
            .args(&["bisect", "view"])
            .output()?;
        
        if let Ok(commit) = String::from_utf8(output.stdout) {
            println!("{} {}", "First bad commit:".yellow(), commit.trim());
        }
    }
    
    Ok(())
}

fn show_status() -> Result<()> {
    let output = Command::new("git")
        .args(&["bisect", "log"])
        .output()?;
    
    if output.status.success() {
        let log = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = log.lines().collect();
        
        if let Some(last_line) = lines.last() {
            if last_line.contains("bisecting") {
                let output = Command::new("git")
                    .args(&["rev-list", "--count", "--bisect-all"])
                    .output()?;
                
                if let Ok(count_str) = String::from_utf8(output.stdout) {
                    if let Some(count) = count_str.trim().split_whitespace().next() {
                        println!("{} {} steps remaining", 
                            "Bisecting:".cyan(),
                            count.yellow()
                        );
                    }
                }
            }
        }
    }
    
    Ok(())
}