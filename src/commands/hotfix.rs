use anyhow::Result;
use colored::*;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success};

pub async fn execute(name: String, target: Option<String>) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    let target_branch = target.unwrap_or_else(|| config.workflows.main_branch.clone());

    print_info(&format!(
        "Creating hotfix from '{}'",
        target_branch.bright_cyan()
    ));

    let branch_name = format!("{}{}", config.workflows.hotfix_branch_prefix, name);

    repo.create_branch(&branch_name, Some(&target_branch))?;
    repo.checkout(&branch_name)?;

    print_success(&format!(
        "Created and switched to hotfix branch '{}'",
        branch_name.bright_green()
    ));

    println!("\n{}", "Hotfix workflow:".bright_white().underline());
    println!("  1. Make your emergency fixes");
    println!("  2. Test thoroughly");
    println!("  3. Run 'gwf commit' to commit changes");
    println!(
        "  4. Run 'gwf pr --target {}' to create a pull request",
        target_branch
    );
    println!("  5. After merge, run 'gwf release patch' to tag the release");

    Ok(())
}
