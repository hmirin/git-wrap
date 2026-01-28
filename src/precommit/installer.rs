use anyhow::{bail, Context, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use which::which;

/// Ensure pre-commit is installed on the system
pub fn ensure_installed() -> Result<()> {
    if which("pre-commit").is_ok() {
        return Ok(());
    }

    println!("{} pre-commit not found, installing...", "→".cyan());

    // Try uvx first (preferred)
    if which("uvx").is_ok() {
        println!("{} Installing via uvx...", "→".cyan());
        let status = Command::new("uvx")
            .args(["install", "pre-commit"])
            .status()
            .context("Failed to run uvx")?;

        if status.success() {
            println!("{} pre-commit installed via uvx", "✓".green());
            return Ok(());
        }
    }

    // Try pipx
    if which("pipx").is_ok() {
        println!("{} Installing via pipx...", "→".cyan());
        let status = Command::new("pipx")
            .args(["install", "pre-commit"])
            .status()
            .context("Failed to run pipx")?;

        if status.success() {
            println!("{} pre-commit installed via pipx", "✓".green());
            return Ok(());
        }
    }

    // Try pip as fallback
    if which("pip").is_ok() || which("pip3").is_ok() {
        let pip_cmd = if which("pip3").is_ok() { "pip3" } else { "pip" };
        println!("{} Installing via {}...", "→".cyan(), pip_cmd);
        let status = Command::new(pip_cmd)
            .args(["install", "--user", "pre-commit"])
            .status()
            .context("Failed to run pip")?;

        if status.success() {
            println!("{} pre-commit installed via {}", "✓".green(), pip_cmd);
            return Ok(());
        }
    }

    bail!("Could not install pre-commit. Please install it manually: https://pre-commit.com/#install");
}

/// Ensure pre-commit hooks are installed in this repository
pub fn ensure_hooks_installed() -> Result<()> {
    let hook_path = Path::new(".git/hooks/pre-commit");

    if hook_path.exists() {
        return Ok(());
    }

    println!("{} Installing pre-commit hooks...", "→".cyan());

    let status = Command::new("pre-commit")
        .args(["install"])
        .status()
        .context("Failed to run pre-commit install")?;

    if !status.success() {
        bail!("pre-commit install failed");
    }

    println!("{} pre-commit hooks installed", "✓".green());
    Ok(())
}
