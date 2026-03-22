use anyhow::Result;
use colored::Colorize;

use super::hooks;
use crate::config::{self, Config};
use crate::git;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    // Run before hooks from commands.pull config
    if let Some(cmd_config) = cfg.get_command("pull") {
        hooks::run_hooks(&cmd_config.before, "before")?;
    }

    // Run pull with auto-stash and passthrough args
    pull_internal(&cfg, args)?;

    // Run after hooks from commands.pull config
    if let Some(cmd_config) = cfg.get_command("pull") {
        hooks::run_hooks(&cmd_config.after, "after")?;
    }

    Ok(())
}

/// Internal pull logic, reusable from push command
pub fn pull_internal(cfg: &Config, args: &[String]) -> Result<()> {
    // Auto set-upstream on pull if no upstream configured
    if cfg.auto.set_upstream_on_pull && git::repo::get_upstream().is_err() {
        if let (Ok(remote), Ok(branch)) = (
            git::repo::get_default_remote(),
            git::repo::get_current_branch(),
        ) {
            let upstream = format!("{}/{}", remote, branch);
            println!(
                "{} No upstream configured, setting to {}",
                "→".cyan(),
                upstream
            );
            git::runner::run(
                &["branch", &format!("--set-upstream-to={}", upstream)],
                &[],
            )?;
        }
    }

    let mut pull_args: Vec<String> = args.to_vec();

    // Auto-stash: add --autostash if there are uncommitted changes
    if cfg.auto.stash_on_pull
        && git::repo::has_uncommitted_changes()
        && !pull_args.iter().any(|a| a == "--autostash" || a == "--no-autostash")
    {
        println!("{} Auto-stashing uncommitted changes", "→".cyan());
        pull_args.insert(0, "--autostash".to_string());
    }

    // Run git pull
    println!("{} git pull {}", "→".cyan(), pull_args.join(" "));
    git::runner::run(&["pull"], &pull_args)?;

    // Auto submodule update
    maybe_update_submodules(cfg)?;

    Ok(())
}

/// Run submodule update if configured and .gitmodules exists
pub fn maybe_update_submodules(cfg: &Config) -> Result<()> {
    if cfg.auto.submodule_update && git::repo::has_submodules() {
        println!(
            "{} git submodule update --init --recursive",
            "→".cyan()
        );
        git::runner::run(&["submodule", "update", "--init", "--recursive"], &[])?;
    }
    Ok(())
}
