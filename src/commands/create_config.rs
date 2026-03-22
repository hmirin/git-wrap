use anyhow::{bail, Result};
use colored::Colorize;

use crate::config::{self, Config};

pub fn run(_args: &[String]) -> Result<()> {
    if config::exists() {
        bail!(
            "{} already exists. Delete it first to regenerate.",
            config::CONFIG_PATH
        );
    }

    let config = Config::generate_default();
    config::save(&config)?;

    println!(
        "{} Created {}",
        "✓".green(),
        config::CONFIG_PATH.cyan()
    );
    println!();
    println!("Default settings (all enabled, opt-out in config):");
    println!();
    println!("  {}", "safety:".bold());
    println!("    blockForcePush:  block force push (use --yes to override)");
    println!("    blockMainBranch: block commits to main/master (use --yes)");
    println!();
    println!("  {}", "auto:".bold());
    println!("    submoduleUpdate:    auto-update submodules after pull/checkout/switch");
    println!("    stashOnPull:        auto-stash uncommitted changes on pull");
    println!("    setUpstreamOnPush:  auto set-upstream on first push");
    println!();
    println!("  {}", "commit:".bold());
    println!("    ensurePreCommit: auto-install pre-commit hooks");
    println!();
    println!("  {}", "push:".bold());
    println!("    pullBeforePush:  pull before push (rebase by default)");
    println!();
    println!("Add custom hooks for any command in the 'commands' section.");

    Ok(())
}
