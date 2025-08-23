use anyhow::Result;
use colored::Colorize;
use dialoguer::{Input, Select, Confirm};
use semver::Version;
use std::process::Command;
use git2::Repository;

pub async fn run() -> Result<()> {
    let repo = Repository::open(".")?;
    
    let mut latest_version = Version::new(0, 1, 0);
    let mut tags = Vec::new();
    
    repo.tag_foreach(|_oid, name| {
        if let Ok(name_str) = std::str::from_utf8(name) {
            let tag_name = name_str.trim_start_matches("refs/tags/");
            tags.push(tag_name.to_string());
            
            let version_str = tag_name.trim_start_matches('v');
            if let Ok(v) = Version::parse(version_str) {
                if v > latest_version {
                    latest_version = v;
                }
            }
        }
        true
    })?;
    
    if !tags.is_empty() {
        println!("{} v{}", "Latest version:".cyan(), latest_version.to_string().yellow());
    }
    
    let choices = vec![
        "Major version (breaking changes)",
        "Minor version (new features)",
        "Patch version (bug fixes)",
        "Custom version",
        "List tags",
        "Cancel"
    ];
    
    let selection = Select::new()
        .with_prompt("What kind of release?")
        .items(&choices)
        .default(2)
        .interact()?;
    
    let new_version = match selection {
        0 => {
            let mut v = latest_version.clone();
            v.major += 1;
            v.minor = 0;
            v.patch = 0;
            v
        },
        1 => {
            let mut v = latest_version.clone();
            v.minor += 1;
            v.patch = 0;
            v
        },
        2 => {
            let mut v = latest_version.clone();
            v.patch += 1;
            v
        },
        3 => {
            let input: String = Input::new()
                .with_prompt("Enter version (without v prefix)")
                .interact_text()?;
            Version::parse(&input)?
        },
        4 => {
            if tags.is_empty() {
                println!("{}", "No tags found".yellow());
            } else {
                println!("\n{}", "Existing tags:".bright_blue());
                for tag in tags.iter().rev().take(10) {
                    println!("  {}", tag.green());
                }
            }
            return Ok(());
        },
        _ => return Ok(())
    };
    
    let tag_name = format!("v{}", new_version);
    println!("\n{} {}", "Creating tag:".cyan(), tag_name.yellow());
    
    let message: String = Input::new()
        .with_prompt("Tag message")
        .default(format!("Release {}", new_version))
        .interact_text()?;
    
    let push = Confirm::new()
        .with_prompt("Push tag to remote?")
        .default(true)
        .interact()?;
    
    Command::new("git")
        .args(&["tag", "-a", &tag_name, "-m", &message])
        .status()?;
    
    println!("{} {}", "Created tag:".green(), tag_name);
    
    if push {
        Command::new("git")
            .args(&["push", "origin", &tag_name])
            .status()?;
        println!("{}", "Tag pushed to remote".green());
    }
    
    Ok(())
}