use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub workflows: WorkflowConfig,
    pub commits: CommitConfig,
    pub sync: SyncConfig,
    pub cleanup: CleanupConfig,
    pub ai: Option<AiConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowConfig {
    pub feature_branch_prefix: String,
    pub hotfix_branch_prefix: String,
    pub release_branch_prefix: String,
    pub main_branch: String,
    pub develop_branch: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitConfig {
    pub conventional: bool,
    pub sign_commits: bool,
    pub gpg_key: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncConfig {
    pub strategy: SyncStrategy,
    pub auto_stash: bool,
    pub prune_on_fetch: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SyncStrategy {
    Rebase,
    Merge,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CleanupConfig {
    pub delete_merged: bool,
    pub days_until_stale: u32,
    pub protect_branches: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AiConfig {
    pub enabled: bool,
    pub provider: String,
    pub model: String,
    pub api_key: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            workflows: WorkflowConfig {
                feature_branch_prefix: "feature/".to_string(),
                hotfix_branch_prefix: "hotfix/".to_string(),
                release_branch_prefix: "release/".to_string(),
                main_branch: "main".to_string(),
                develop_branch: Some("develop".to_string()),
            },
            commits: CommitConfig {
                conventional: true,
                sign_commits: false,
                gpg_key: None,
            },
            sync: SyncConfig {
                strategy: SyncStrategy::Rebase,
                auto_stash: true,
                prune_on_fetch: true,
            },
            cleanup: CleanupConfig {
                delete_merged: true,
                days_until_stale: 30,
                protect_branches: vec![
                    "main".to_string(),
                    "master".to_string(),
                    "develop".to_string(),
                    "production".to_string(),
                ],
            },
            ai: None,
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Path::new(".gwf.toml");

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content =
            fs::read_to_string(config_path).context("Failed to read configuration file")?;

        let config: Config =
            toml::from_str(&content).context("Failed to parse configuration file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Path::new(".gwf.toml");
        let content = toml::to_string_pretty(self).context("Failed to serialize configuration")?;

        fs::write(config_path, content).context("Failed to write configuration file")?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.workflows.feature_branch_prefix, "feature/");
        assert_eq!(config.workflows.hotfix_branch_prefix, "hotfix/");
        assert_eq!(config.workflows.release_branch_prefix, "release/");
        assert_eq!(config.workflows.main_branch, "main");
        assert_eq!(config.commits.conventional, true);
        assert_eq!(config.sync.auto_stash, true);
        assert_eq!(config.cleanup.delete_merged, true);
    }

    #[test]
    fn test_sync_strategy() {
        let config = Config::default();
        matches!(config.sync.strategy, SyncStrategy::Rebase);
    }
}
