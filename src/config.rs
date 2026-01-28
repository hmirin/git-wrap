use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::git;

pub const CONFIG_PATH: &str = ".git-wrap.config.json";

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    /// Special settings for commit command
    #[serde(default)]
    pub commit: CommitConfig,

    /// Special settings for push command
    #[serde(default)]
    pub push: PushConfig,

    /// Generic before/after hooks for any command
    #[serde(default)]
    pub commands: HashMap<String, CommandHooks>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitConfig {
    /// Automatically ensure pre-commit is installed and configured
    #[serde(default = "default_true")]
    pub ensure_pre_commit: bool,
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            ensure_pre_commit: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PushConfig {
    /// Automatically pull before push if behind
    #[serde(default = "default_true")]
    pub pull_before_push: bool,

    /// Pull strategy: "git-default", "rebase", or "merge"
    #[serde(default)]
    pub pull_strategy: Option<String>,
}

impl Default for PushConfig {
    fn default() -> Self {
        Self {
            pull_before_push: true,
            pull_strategy: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CommandHooks {
    /// Commands to run before the git command
    #[serde(default)]
    pub before: Vec<String>,

    /// Commands to run after the git command
    #[serde(default)]
    pub after: Vec<String>,
}

fn default_true() -> bool {
    true
}

impl Config {
    pub fn get_pull_args(&self) -> Vec<String> {
        match self.push.pull_strategy.as_deref() {
            Some("rebase") => vec!["--rebase".into()],
            Some("merge") => vec![],
            Some("git-default") | None => {
                // Check git config pull.rebase
                if git::repo::get_config_bool("pull.rebase").unwrap_or(false) {
                    vec!["--rebase".into()]
                } else {
                    // Default to rebase
                    vec!["--rebase".into()]
                }
            }
            Some(_) => vec!["--rebase".into()], // Unknown strategy, default to rebase
        }
    }

    /// Get hooks for a specific command
    pub fn get_hooks(&self, command: &str) -> Option<&CommandHooks> {
        self.commands.get(command)
    }

    /// Generate a default config for create-config command
    pub fn generate_default() -> Self {
        let mut commands = HashMap::new();

        // Example: run npm install after checkout
        commands.insert(
            "checkout".to_string(),
            CommandHooks {
                before: vec![],
                after: vec!["# npm install".to_string()],
            },
        );

        Self {
            commit: CommitConfig {
                ensure_pre_commit: true,
            },
            push: PushConfig {
                pull_before_push: true,
                pull_strategy: Some("git-default".to_string()),
            },
            commands,
        }
    }
}

pub fn load() -> Result<Config> {
    let path = Path::new(CONFIG_PATH);
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Config::default())
    }
}

pub fn save(config: &Config) -> Result<()> {
    let content = serde_json::to_string_pretty(config)?;
    fs::write(CONFIG_PATH, content)?;
    Ok(())
}

pub fn exists() -> bool {
    Path::new(CONFIG_PATH).exists()
}
