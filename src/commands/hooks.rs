use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::process::Command;

/// Run a list of shell commands as hooks (with phase label)
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

/// Run a list of commands as steps (for custom commands)
pub fn run_steps(steps: &[String]) -> Result<()> {
    for cmd in steps {
        // Skip comments
        if cmd.trim().starts_with('#') {
            continue;
        }

        println!("{} {}", "→".cyan(), cmd);

        let status = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .status()
            .with_context(|| format!("Failed to execute: {}", cmd))?;

        if !status.success() {
            bail!("Command failed: {}", cmd);
        }
    }
    Ok(())
}
