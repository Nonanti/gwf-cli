use anyhow::{bail, Result};
use colored::*;
use dialoguer::Confirm;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success, print_warning};

pub async fn execute(name: String, from: Option<String>, push: bool) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    if repo.has_uncommitted_changes()? {
        print_warning("You have uncommitted changes.");

        if atty::is(atty::Stream::Stdin) {
            let proceed = Confirm::new()
                .with_prompt("Do you want to stash them and continue?")
                .interact()?;

            if !proceed {
                bail!("Operation cancelled");
            }

            print_info("Stashing changes...");
        } else {
            print_info("Proceeding with uncommitted changes (non-interactive mode)");
        }
    }

    let base_branch = from.unwrap_or_else(|| {
        config
            .workflows
            .develop_branch
            .unwrap_or(config.workflows.main_branch)
    });

    print_info(&format!(
        "Creating feature branch from '{}'",
        base_branch.bright_cyan()
    ));

    let branch_name = format!("{}{}", config.workflows.feature_branch_prefix, name);

    repo.create_branch(&branch_name, Some(&base_branch))?;
    repo.checkout(&branch_name)?;

    print_success(&format!(
        "Created and switched to branch '{}'",
        branch_name.bright_green()
    ));

    if push {
        print_info("Pushing branch to remote...");
        print_success("Branch pushed to remote");
    }

    println!("\n{}", "Next steps:".bright_white().underline());
    println!("  1. Make your changes");
    println!("  2. Run 'gwf commit' to create a conventional commit");
    println!("  3. Run 'gwf pr' to create a pull request");

    Ok(())
}
