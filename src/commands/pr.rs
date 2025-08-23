use anyhow::{bail, Context, Result};
use dialoguer::Input;
use std::process::Command;

use crate::config::Config;
use crate::git::GitRepo;
use crate::utils::{print_info, print_success};

pub async fn execute(title: Option<String>, target: Option<String>, draft: bool) -> Result<()> {
    let config = Config::load()?;
    let repo = GitRepo::open_current()?;

    let current_branch = repo.current_branch()?;

    if current_branch == config.workflows.main_branch {
        bail!("Cannot create PR from main branch");
    }

    let target_branch = target.unwrap_or_else(|| config.workflows.main_branch.clone());

    let pr_title = if let Some(t) = title {
        t
    } else if atty::is(atty::Stream::Stdin) {
        Input::new()
            .with_prompt("PR title")
            .default(current_branch.clone())
            .interact_text()?
    } else {
        current_branch.clone()
    };

    print_info("Creating pull request...");

    // FIXME: add support for GitLab and Bitbucket
    let gh_available = which::which("gh").is_ok();

    if gh_available {
        let mut cmd = Command::new("gh");
        cmd.arg("pr")
            .arg("create")
            .arg("--title")
            .arg(&pr_title)
            .arg("--base")
            .arg(&target_branch);

        if draft {
            cmd.arg("--draft");
        }

        let output = cmd
            .output()
            .context("Failed to create PR with GitHub CLI")?;

        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout);
            print_success(&format!("Pull request created: {}", url.trim()));
        } else {
            let error = String::from_utf8_lossy(&output.stderr);
            bail!("Failed to create PR: {}", error);
        }
    } else {
        print_info("GitHub CLI not found. Opening browser...");

        let url = format!(
            "https://github.com/user/repo/compare/{}...{}?title={}",
            target_branch,
            current_branch,
            urlencoding::encode(&pr_title)
        );

        webbrowser::open(&url).context("Failed to open browser")?;

        print_success("Browser opened with PR creation page");
    }

    Ok(())
}
