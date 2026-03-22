use anyhow::{bail, Result};
use colored::Colorize;

use super::hooks;
use crate::config;
use crate::git;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    let has_yes = args.iter().any(|a| a == "--yes");
    let is_force = args
        .iter()
        .any(|a| a == "--force" || a == "-f" || a == "--force-with-lease");

    // Safety: block force push unless --yes
    if cfg.safety.block_force_push && is_force && !has_yes {
        eprintln!(
            "{} Force pushing can overwrite remote history.",
            "⚠".yellow()
        );
        eprintln!(
            "  Run with --yes to confirm: git-wrap push {} --yes",
            args.join(" ")
        );
        bail!("Force push blocked. Use --yes to override.");
    }

    // Strip --yes from args before passing to git
    let git_args: Vec<String> = args.iter().filter(|a| a.as_str() != "--yes").cloned().collect();

    // Run before hooks
    hooks::run_hooks(&cfg.push.before, "before")?;

    // Special behavior: pull before push
    if cfg.push.pull_before_push {
        println!("{} git fetch", "→".cyan());
        git::runner::run_silent(&["fetch"])?;

        if git::repo::get_upstream().is_ok() {
            let (behind, _ahead) = git::repo::get_behind_ahead()?;

            if behind > 0 {
                println!(
                    "{} Behind by {} commit(s), pulling...",
                    "→".cyan(),
                    behind
                );
                let pull_args = cfg.get_pull_args();
                println!("{} git pull {}", "→".cyan(), pull_args.join(" "));
                git::runner::run(&["pull"], &pull_args)?;
            }
        } else {
            println!("{} No upstream configured, skipping pull", "!".yellow());
        }
    }

    // Run git push
    println!("{} git push {}", "→".cyan(), git_args.join(" "));
    git::runner::run(&["push"], &git_args)?;

    // Run after hooks
    hooks::run_hooks(&cfg.push.after, "after")?;

    Ok(())
}
