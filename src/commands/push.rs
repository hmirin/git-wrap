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

    let has_upstream = git::repo::get_upstream().is_ok();

    // Special behavior: pull before push
    if cfg.push.pull_before_push {
        let fetch_args: Vec<&str> = if cfg.auto.prune_on_fetch {
            println!("{} git fetch --prune", "→".cyan());
            vec!["fetch", "--prune"]
        } else {
            println!("{} git fetch", "→".cyan());
            vec!["fetch"]
        };
        git::runner::run_silent(&fetch_args)?;

        if has_upstream {
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
        }
    }

    // Auto set-upstream on first push
    let mut push_args = git_args;
    if !has_upstream && cfg.auto.set_upstream_on_push {
        // Only add -u if user didn't already specify -u/--set-upstream or a remote
        let already_has_upstream_flag = push_args
            .iter()
            .any(|a| a == "-u" || a == "--set-upstream");
        if !already_has_upstream_flag {
            if let (Ok(remote), Ok(branch)) = (
                git::repo::get_default_remote(),
                git::repo::get_current_branch(),
            ) {
                println!(
                    "{} No upstream configured, setting to {}/{}",
                    "→".cyan(),
                    remote,
                    branch
                );
                push_args = vec!["-u".to_string(), remote, branch]
                    .into_iter()
                    .chain(push_args)
                    .collect();
            }
        }
    }

    // Run git push
    println!("{} git push {}", "→".cyan(), push_args.join(" "));
    git::runner::run(&["push"], &push_args)?;

    // Run after hooks
    hooks::run_hooks(&cfg.push.after, "after")?;

    Ok(())
}
