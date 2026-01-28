use anyhow::Result;
use std::path::Path;

use super::hooks;
use crate::config;
use crate::git;
use crate::precommit;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    // Run before hooks
    hooks::run_hooks(&cfg.commit.before, "before")?;

    // Special behavior: ensure pre-commit is set up
    if cfg.commit.ensure_pre_commit && Path::new(".pre-commit-config.yaml").exists() {
        precommit::ensure_installed()?;
        precommit::ensure_hooks_installed()?;
    }

    // Run git commit
    git::runner::run(&["commit"], args)?;

    // Run after hooks
    hooks::run_hooks(&cfg.commit.after, "after")?;

    Ok(())
}
