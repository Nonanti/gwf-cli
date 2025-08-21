use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::*;

mod commands;
mod config;
mod git;
mod utils;
mod workflows;

#[derive(Parser)]
#[command(
    name = "gwf",
    version,
    author,
    about = "Git Workflow Automator - Streamline your Git workflows",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Increase logging verbosity
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Suppress all output except errors
    #[arg(short, long)]
    quiet: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize GWF in the current repository
    Init {
        /// Force initialization even if already initialized
        #[arg(short, long)]
        force: bool,
    },

    /// Create and manage feature branches
    Feature {
        /// Name of the feature branch
        name: String,

        /// Base branch to create from
        #[arg(short, long)]
        from: Option<String>,

        /// Push to remote after creation
        #[arg(short, long)]
        push: bool,
    },

    /// Create and manage hotfix branches
    Hotfix {
        /// Name of the hotfix
        name: String,

        /// Target branch for the hotfix
        #[arg(short, long)]
        target: Option<String>,
    },

    /// Create and manage releases
    Release {
        /// Version number or increment (major/minor/patch)
        version: String,

        /// Generate changelog
        #[arg(short, long)]
        changelog: bool,

        /// Tag the release
        #[arg(short, long)]
        tag: bool,
    },

    /// Synchronize branches with upstream
    Sync {
        /// Sync all branches
        #[arg(short, long)]
        all: bool,

        /// Specific branch to sync
        #[arg(short, long)]
        branch: Option<String>,
    },

    /// Clean up merged and stale branches
    Cleanup {
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,

        /// Dry run - show what would be deleted
        #[arg(short, long)]
        dry_run: bool,

        /// Include remote branches
        #[arg(short, long)]
        remote: bool,
    },

    /// Create a conventional commit
    Commit {
        /// Commit message
        message: Option<String>,

        /// Use AI to generate commit message
        #[arg(short, long)]
        ai: bool,

        /// Amend the last commit
        #[arg(long)]
        amend: bool,
    },

    /// Create a pull request
    #[command(name = "pr")]
    PullRequest {
        /// PR title
        title: Option<String>,

        /// Target branch
        #[arg(short, long)]
        target: Option<String>,

        /// Mark as draft
        #[arg(short, long)]
        draft: bool,
    },

    /// Generate standup report
    Standup {
        /// Number of days to look back
        #[arg(short, long, default_value = "1")]
        days: u32,

        /// Include all branches
        #[arg(short, long)]
        all: bool,
    },

    /// Manage GWF configuration
    Config {
        /// Show current configuration
        #[arg(short, long)]
        show: bool,

        /// Edit configuration
        #[arg(short, long)]
        edit: bool,

        /// Reset to defaults
        #[arg(short, long)]
        reset: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        shell: clap_complete::Shell,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set up logging
    let log_level = match cli.verbose {
        0 => "warn",
        1 => "info",
        2 => "debug",
        _ => "trace",
    };

    if !cli.quiet {
        tracing_subscriber::fmt().with_env_filter(log_level).init();
    }

    // ASCII art banner
    if !cli.quiet && cli.verbose == 0 {
        println!(
            "{}",
            r#"
   _____ _       _________ 
  / ____| |     / ____/ __|
 | |  __| | /| / / /_ | |_ 
 | | |_ | |/ |/ / ___||  _|
 | |__| |__/|__/ /    | |  
  \_____\__/\__/_/     |_|  
                            
        "#
            .bright_cyan()
        );
        println!("{}\n", "Git Workflow Automator".bright_white().bold());
    }

    match cli.command {
        Commands::Init { force } => {
            commands::init::execute(force).await?;
        }
        Commands::Feature { name, from, push } => {
            commands::feature::execute(name, from, push).await?;
        }
        Commands::Hotfix { name, target } => {
            commands::hotfix::execute(name, target).await?;
        }
        Commands::Release {
            version,
            changelog,
            tag,
        } => {
            commands::release::execute(version, changelog, tag).await?;
        }
        Commands::Sync { all, branch } => {
            commands::sync::execute(all, branch).await?;
        }
        Commands::Cleanup {
            yes,
            dry_run,
            remote,
        } => {
            commands::cleanup::execute(yes, dry_run, remote).await?;
        }
        Commands::Commit { message, ai, amend } => {
            commands::commit::execute(message, ai, amend).await?;
        }
        Commands::PullRequest {
            title,
            target,
            draft,
        } => {
            commands::pr::execute(title, target, draft).await?;
        }
        Commands::Standup { days, all } => {
            commands::standup::execute(days, all).await?;
        }
        Commands::Config { show, edit, reset } => {
            commands::config::execute(show, edit, reset).await?;
        }
        Commands::Completions { shell } => {
            commands::completions::execute(shell);
        }
    }

    Ok(())
}
