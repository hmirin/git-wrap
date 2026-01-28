use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::process::Command;

use crate::config::CommandHooks;

/// Run a list of shell commands
pub fn run_hooks(hooks: &[String], phase: &str) -> Result<()> {
    for cmd in hooks {
        // Skip comments
        if cmd.trim().starts_with('#') {
            continue;
        }

        println!("{} [{}] {}", "→".cyan(), phase, cmd);

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .with_context(|| format!("Failed to execute: {}", cmd))?;

        if !status.success() {
            bail!("Hook failed: {}", cmd);
        }
    }
    Ok(())
}

/// Run before hooks
pub fn run_before(hooks: &CommandHooks) -> Result<()> {
    if !hooks.before.is_empty() {
        run_hooks(&hooks.before, "before")?;
    }
    Ok(())
}

/// Run after hooks
pub fn run_after(hooks: &CommandHooks) -> Result<()> {
    if !hooks.after.is_empty() {
        run_hooks(&hooks.after, "after")?;
    }
    Ok(())
}
