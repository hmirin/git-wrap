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

    /// Generic hooks or custom commands
    #[serde(default)]
    pub commands: HashMap<String, CommandConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommitConfig {
    /// Automatically ensure pre-commit is installed and configured
    #[serde(default = "default_true")]
    pub ensure_pre_commit: bool,

    /// Commands to run before commit
    #[serde(default)]
    pub before: Vec<String>,

    /// Commands to run after commit
    #[serde(default)]
    pub after: Vec<String>,
}

impl Default for CommitConfig {
    fn default() -> Self {
        Self {
            ensure_pre_commit: true,
            before: vec![],
            after: vec![],
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

    /// Commands to run before push
    #[serde(default)]
    pub before: Vec<String>,

    /// Commands to run after push
    #[serde(default)]
    pub after: Vec<String>,
}

impl Default for PushConfig {
    fn default() -> Self {
        Self {
            pull_before_push: true,
            pull_strategy: None,
            before: vec![],
            after: vec![],
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CommandConfig {
    /// If set, this is a custom command (doesn't call git)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub run: Vec<String>,

    /// Description for custom commands
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Commands to run before the git command (ignored if `run` is set)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub before: Vec<String>,

    /// Commands to run after the git command (ignored if `run` is set)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub after: Vec<String>,
}

impl CommandConfig {
    /// Returns true if this is a custom command (has `run` steps)
    pub fn is_custom(&self) -> bool {
        !self.run.is_empty()
    }
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

    /// Get config for a specific command
    pub fn get_command(&self, command: &str) -> Option<&CommandConfig> {
        self.commands.get(command)
    }

    /// Generate a default config for create-config command
    pub fn generate_default() -> Self {
        let mut commands = HashMap::new();

        // Custom command: sync (fetch + pull + push)
        commands.insert(
            "sync".to_string(),
            CommandConfig {
                run: vec![
                    "git fetch".to_string(),
                    "git pull --rebase".to_string(),
                    "git push".to_string(),
                ],
                description: Some("Fetch, pull, and push in one command".to_string()),
                before: vec![],
                after: vec![],
            },
        );

        // Example: run npm install after checkout
        commands.insert(
            "checkout".to_string(),
            CommandConfig {
                run: vec![],
                description: None,
                before: vec![],
                after: vec!["# npm install".to_string()],
            },
        );

        Self {
            commit: CommitConfig {
                ensure_pre_commit: true,
                before: vec![],
                after: vec![],
            },
            push: PushConfig {
                pull_before_push: true,
                pull_strategy: Some("git-default".to_string()),
                before: vec![],
                after: vec![],
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
