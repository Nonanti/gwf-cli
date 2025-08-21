use anyhow::{Context, Result};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::process::Command;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success, print_warning};

pub async fn execute(all: bool, branch: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    print_info("Synchronizing with remote repository...");

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Fetching from origin...");

    repo.fetch("origin")?;
    pb.finish_with_message("Fetch complete");

    let branches_to_sync = if all {
        repo.list_branches(false)?
    } else if let Some(branch_name) = branch {
        vec![branch_name]
    } else {
        vec![repo.current_branch()?]
    };

    for branch_name in branches_to_sync {
        println!(
            "\n{} {}",
            "Syncing branch:".bright_white(),
            branch_name.bright_cyan()
        );

        let has_changes = repo.has_uncommitted_changes()?;
        if has_changes && config.sync.auto_stash {
            print_info("Stashing uncommitted changes...");
            Command::new("git")
                .args(["stash", "push", "-m", "gwf-sync-autostash"])
                .output()
                .context("Failed to stash changes")?;
        }

        repo.checkout(&branch_name)?;

        let result = match config.sync.strategy {
            crate::config::SyncStrategy::Rebase => {
                print_info("Rebasing...");
                Command::new("git")
                    .args(["rebase", &format!("origin/{}", branch_name)])
                    .output()
            }
            crate::config::SyncStrategy::Merge => {
                print_info("Merging...");
                Command::new("git")
                    .args(["merge", &format!("origin/{}", branch_name)])
                    .output()
            }
        };

        match result {
            Ok(output) if output.status.success() => {
                print_success(&format!("Branch '{}' synchronized", branch_name));
            }
            Ok(output) => {
                let error = String::from_utf8_lossy(&output.stderr);
                print_warning(&format!("Failed to sync '{}': {}", branch_name, error));

                Command::new("git")
                    .args(["rebase", "--abort"])
                    .output()
                    .ok();
                Command::new("git")
                    .args(["merge", "--abort"])
                    .output()
                    .ok();
            }
            Err(e) => {
                print_warning(&format!("Failed to sync '{}': {}", branch_name, e));
            }
        }

        if has_changes && config.sync.auto_stash {
            print_info("Restoring stashed changes...");
            Command::new("git").args(["stash", "pop"]).output().ok();
        }
    }

    if config.sync.prune_on_fetch {
        print_info("Pruning remote branches...");
        Command::new("git")
            .args(["remote", "prune", "origin"])
            .output()
            .context("Failed to prune remote branches")?;
        print_success("Remote branches pruned");
    }

    Ok(())
}
