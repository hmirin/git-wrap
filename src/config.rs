use anyhow::Result;
use serde::Deserialize;
use std::fs;
use std::path::Path;

use crate::git;

#[derive(Debug, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub push: PushConfig,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PushConfig {
    #[serde(default = "default_true")]
    pub pull_before_push: bool,
    pub pull_strategy: Option<String>,
}

fn default_true() -> bool {
    true
}

impl Default for PushConfig {
    fn default() -> Self {
        Self {
            pull_before_push: true,
            pull_strategy: None,
        }
    }
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
}

pub fn load() -> Result<Config> {
    let path = Path::new(".git-wrap.config.json");
    if path.exists() {
        let content = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    } else {
        Ok(Config::default())
    }
}
