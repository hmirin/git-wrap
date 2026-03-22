use anyhow::{bail, Result};
use colored::Colorize;
use std::path::Path;

use super::hooks;
use crate::config;
use crate::git;
use crate::precommit;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    let has_yes = args.iter().any(|a| a == "--yes");

    // Safety: block commits to main/master unless --yes
    if cfg.safety.block_main_branch {
        if let Ok(branch) = git::repo::get_current_branch() {
            if (branch == "main" || branch == "master") && !has_yes {
                eprintln!(
                    "{} You are committing directly to {}.",
                    "⚠".yellow(),
                    branch.bold()
                );
                eprintln!(
                    "  Run with --yes to confirm: git-wrap commit {} --yes",
                    args.join(" ")
                );
                bail!(
                    "Direct commit to {} blocked. Use --yes to override.",
                    branch
                );
            }
        }
    }

    // Strip --yes from args before passing to git
    let git_args: Vec<String> = args.iter().filter(|a| a.as_str() != "--yes").cloned().collect();

    // Run before hooks
    hooks::run_hooks(&cfg.commit.before, "before")?;

    // Special behavior: ensure pre-commit is set up
    if cfg.commit.ensure_pre_commit && Path::new(".pre-commit-config.yaml").exists() {
        precommit::ensure_installed()?;
        precommit::ensure_hooks_installed()?;
    }

    // Run git commit
    git::runner::run(&["commit"], &git_args)?;

    // Run after hooks
    hooks::run_hooks(&cfg.commit.after, "after")?;

    Ok(())
}
