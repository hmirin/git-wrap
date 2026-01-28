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
    println!("Default settings:");
    println!("  commit: auto-install pre-commit hooks");
    println!("  push: pull before push (rebase by default)");
    println!();
    println!("Add custom hooks for any command in the 'commands' section.");

    Ok(())
}
