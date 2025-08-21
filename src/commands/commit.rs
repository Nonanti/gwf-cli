use anyhow::{bail, Context, Result};
use dialoguer::{Input, Select};
use std::process::Command;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success};

const COMMIT_TYPES: &[(&str, &str)] = &[
    ("feat", "A new feature"),
    ("fix", "A bug fix"),
    ("docs", "Documentation only changes"),
    (
        "style",
        "Changes that do not affect the meaning of the code",
    ),
    (
        "refactor",
        "A code change that neither fixes a bug nor adds a feature",
    ),
    ("perf", "A code change that improves performance"),
    ("test", "Adding missing tests or correcting existing tests"),
    ("chore", "Changes to the build process or auxiliary tools"),
];

pub async fn execute(message: Option<String>, ai: bool, amend: bool) -> Result<()> {
    let config = Config::load()?;
    let _repo = GitRepo::open_current()?;

    if ai {
        print_info("AI-powered commit messages are not yet implemented");
        return Ok(());
    }

    let final_message = if let Some(msg) = message {
        msg
    } else if config.commits.conventional {
        if !atty::is(atty::Stream::Stdin) {
            bail!("Interactive mode requires a terminal. Please provide a message with -m");
        }

        let type_index = Select::new()
            .with_prompt("Select commit type")
            .items(
                &COMMIT_TYPES
                    .iter()
                    .map(|(t, d)| format!("{}: {}", t, d))
                    .collect::<Vec<_>>(),
            )
            .default(0)
            .interact()?;

        let commit_type = COMMIT_TYPES[type_index].0;

        let scope: String = Input::new()
            .with_prompt("Scope (optional)")
            .allow_empty(true)
            .interact_text()?;

        let description: String = Input::new().with_prompt("Description").interact_text()?;

        let body: String = Input::new()
            .with_prompt("Body (optional)")
            .allow_empty(true)
            .interact_text()?;

        let breaking: String = Input::new()
            .with_prompt("Breaking change (optional)")
            .allow_empty(true)
            .interact_text()?;

        let mut commit_msg = if scope.is_empty() {
            format!("{}: {}", commit_type, description)
        } else {
            format!("{}({}): {}", commit_type, scope, description)
        };

        if !body.is_empty() {
            commit_msg.push_str(&format!("\n\n{}", body));
        }

        if !breaking.is_empty() {
            commit_msg.push_str(&format!("\n\nBREAKING CHANGE: {}", breaking));
        }

        commit_msg
    } else {
        if !atty::is(atty::Stream::Stdin) {
            bail!("Interactive mode requires a terminal. Please provide a message with -m");
        }

        Input::new().with_prompt("Commit message").interact_text()?
    };

    let mut cmd = Command::new("git");
    cmd.arg("commit");

    if amend {
        cmd.arg("--amend");
    }

    if config.commits.sign_commits {
        cmd.arg("-S");
        if let Some(key) = &config.commits.gpg_key {
            cmd.arg("--gpg-sign").arg(key);
        }
    }

    cmd.arg("-m").arg(&final_message);

    print_info("Creating commit...");

    let output = cmd.output().context("Failed to execute git commit")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        bail!("Git commit failed: {}", error);
    }

    print_success(&format!(
        "Commit created: {}",
        final_message.lines().next().unwrap_or("")
    ));

    Ok(())
}
