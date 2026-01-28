use anyhow::Result;
use colored::Colorize;

use super::hooks;
use crate::config;
use crate::git;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    // Run before hooks
    hooks::run_hooks(&cfg.push.before, "before")?;

    // Special behavior: pull before push
    if cfg.push.pull_before_push {
        // Fetch latest from remote
        git::runner::run_silent(&["fetch"])?;

        // Check if upstream is configured
        if git::repo::get_upstream().is_ok() {
            let (behind, _ahead) = git::repo::get_behind_ahead()?;

            if behind > 0 {
                println!(
                    "{} Behind by {} commit(s), pulling...",
                    "→".cyan(),
                    behind
                );
                let pull_args = cfg.get_pull_args();
                git::runner::run(&["pull"], &pull_args)?;
            }
        } else {
            println!("{} No upstream configured, skipping pull", "!".yellow());
        }
    }

    // Run git push
    git::runner::run(&["push"], args)?;

    // Run after hooks
    hooks::run_hooks(&cfg.push.after, "after")?;

    Ok(())
}
