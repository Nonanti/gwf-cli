use anyhow::Result;
use colored::Colorize;
use dialoguer::{Select, Confirm};
use std::process::Command;

pub async fn run() -> Result<()> {
    // TODO: add reflog support for more undo options
    let choices = vec![
        "Undo last commit (keep changes)",
        "Undo last commit (discard changes)",
        "Undo last merge",
        "Abort current merge",
        "Cancel"
    ];
    
    let selection = Select::new()
        .with_prompt("What do you want to undo?")
        .items(&choices)
        .default(0)
        .interact()?;
    
    match selection {
        0 => {
            Command::new("git")
                .args(&["reset", "--soft", "HEAD~1"])
                .status()?;
            println!("{}", "Last commit undone, changes kept".green());
        },
        1 => {
            let confirm = Confirm::new()
                .with_prompt("This will discard all changes. Are you sure?")
                .default(false)
                .interact()?;
            
            if confirm {
                Command::new("git")
                    .args(&["reset", "--hard", "HEAD~1"])
                    .status()?;
                println!("{}", "Last commit undone, changes discarded".yellow());
            }
        },
        2 => {
            Command::new("git")
                .args(&["reset", "--hard", "ORIG_HEAD"])
                .status()?;
            println!("{}", "Last merge undone".green());
        },
        3 => {
            Command::new("git")
                .args(&["merge", "--abort"])
                .status()?;
            println!("{}", "Merge aborted".green());
        },
        _ => {
            println!("{}", "Cancelled".yellow());
        }
    }
    
    Ok(())
}