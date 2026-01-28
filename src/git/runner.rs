use anyhow::{Context, Result};
use std::process::{Command, ExitStatus, Stdio};

/// Pass all arguments directly to git
pub fn passthrough(args: &[String]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .status()
        .context("Failed to execute git")?;

    std::process::exit(status.code().unwrap_or(1));
}

/// Run git with a subcommand and additional arguments
pub fn run(subcommand: &[&str], args: &[String]) -> Result<()> {
    let status = Command::new("git")
        .args(subcommand)
        .args(args)
        .status()
        .context("Failed to execute git")?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }
    Ok(())
}

/// Run git silently (no output) and return the status
pub fn run_silent(subcommand: &[&str]) -> Result<ExitStatus> {
    Command::new("git")
        .args(subcommand)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .context("Failed to execute git")
}

/// Run git and capture output
pub fn run_output(args: &[&str]) -> Result<String> {
    let output = Command::new("git")
        .args(args)
        .output()
        .context("Failed to execute git")?;

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Check if a git command succeeds (for checking conditions)
pub fn check(args: &[&str]) -> bool {
    Command::new("git")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}
