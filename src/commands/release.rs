use anyhow::{Context, Result};
use colored::*;
use semver::Version;
use std::process::Command;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success};

pub async fn execute(version: String, changelog: bool, tag: bool) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    let branch_name = format!("{}{}", config.workflows.release_branch_prefix, version);

    print_info(&format!("Creating release branch '{}'", branch_name));

    let new_version = match version.as_str() {
        "major" | "minor" | "patch" => {
            let output = Command::new("git")
                .args(["describe", "--tags", "--abbrev=0"])
                .output()
                .context("Failed to get last tag")?;

            let last_tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let current_version = last_tag.trim_start_matches('v');

            if let Ok(mut ver) = Version::parse(current_version) {
                match version.as_str() {
                    "major" => {
                        ver.major += 1;
                        ver.minor = 0;
                        ver.patch = 0;
                    }
                    "minor" => {
                        ver.minor += 1;
                        ver.patch = 0;
                    }
                    "patch" => {
                        ver.patch += 1;
                    }
                    _ => {}
                }
                ver.to_string()
            } else {
                "0.1.0".to_string()
            }
        }
        _ => version.clone(),
    };

    repo.create_branch(&branch_name, Some(&config.workflows.main_branch))?;
    repo.checkout(&branch_name)?;

    print_success(&format!("Created release branch '{}'", branch_name));

    if changelog {
        print_info("Generating changelog...");
        print_success("Changelog generated (not implemented)");
    }

    if tag {
        print_info(&format!("Creating tag v{}", new_version));

        Command::new("git")
            .args([
                "tag",
                "-a",
                &format!("v{}", new_version),
                "-m",
                &format!("Release v{}", new_version),
            ])
            .output()
            .context("Failed to create tag")?;

        print_success(&format!("Tagged release v{}", new_version));
    }

    println!("\n{}", "Next steps:".bright_white().underline());
    println!("  1. Update version files");
    println!("  2. Update CHANGELOG.md");
    println!("  3. Run 'gwf pr' to create a release PR");
    println!("  4. After merge, push tags with 'git push --tags'");

    Ok(())
}
