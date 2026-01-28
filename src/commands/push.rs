use anyhow::Result;
use colored::Colorize;

use crate::config;
use crate::git;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let config = config::load()?;

    if !config.push.pull_before_push {
        return git::runner::run(&["push"], args);
    }

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
            let pull_args = config.get_pull_args();
            git::runner::run(&["pull"], &pull_args)?;
        }
    } else {
        println!(
            "{} No upstream configured, skipping pull",
            "!".yellow()
        );
    }

    git::runner::run(&["push"], args)
}
