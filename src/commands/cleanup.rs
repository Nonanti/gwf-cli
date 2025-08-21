use anyhow::Result;
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success, print_warning};

pub async fn execute(yes: bool, dry_run: bool, remote: bool) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    print_info("Scanning for branches to clean up...");

    let current_branch = repo.current_branch()?;
    let branches = repo.list_branches(false)?;

    let mut branches_to_delete = Vec::new();

    for branch in &branches {
        if config.cleanup.protect_branches.contains(branch) || branch == &current_branch {
            continue;
        }

        if repo.is_branch_merged(branch, &config.workflows.main_branch)? {
            branches_to_delete.push(branch.clone());
        }
    }

    if branches_to_delete.is_empty() {
        print_success("No branches to clean up!");
        return Ok(());
    }

    println!("\n{}", "Branches to delete:".bright_white().underline());
    for branch in &branches_to_delete {
        println!("  - {}", branch.bright_red());
    }

    if dry_run {
        print_info("Dry run mode - no branches will be deleted");
        return Ok(());
    }

    let proceed = if yes {
        true
    } else if atty::is(atty::Stream::Stdin) {
        Confirm::new()
            .with_prompt("Do you want to delete these branches?")
            .default(false)
            .interact()?
    } else {
        print_warning("Non-interactive mode - skipping deletion");
        false
    };

    if !proceed {
        print_info("Cleanup cancelled");
        return Ok(());
    }

    for branch in branches_to_delete {
        match repo.delete_branch(&branch) {
            Ok(_) => print_success(&format!("Deleted branch '{}'", branch)),
            Err(e) => print_warning(&format!("Failed to delete '{}': {}", branch, e)),
        }
    }

    if remote {
        print_info("Pruning remote branches...");
        repo.fetch("origin")?;
        print_success("Remote branches pruned");
    }

    Ok(())
}
