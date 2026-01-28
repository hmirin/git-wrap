use anyhow::{bail, Context, Result};

use super::runner;

/// Ensure we're inside a git repository
pub fn ensure_in_repo() -> Result<()> {
    if !runner::check(&["rev-parse", "--git-dir"]) {
        bail!("Not a git repository");
    }
    Ok(())
}

/// Get the upstream tracking branch (e.g., "origin/main")
pub fn get_upstream() -> Result<String> {
    let output = runner::run_output(&["rev-parse", "--abbrev-ref", "--symbolic-full-name", "@{u}"])?;
    if output.is_empty() {
        bail!("No upstream configured");
    }
    Ok(output)
}

/// Get (behind, ahead) counts relative to upstream
pub fn get_behind_ahead() -> Result<(u32, u32)> {
    let output = runner::run_output(&["rev-list", "--left-right", "--count", "@{u}...HEAD"])?;
    let parts: Vec<&str> = output.split_whitespace().collect();

    if parts.len() != 2 {
        bail!("Unexpected rev-list output: {}", output);
    }

    let behind: u32 = parts[0].parse().context("Failed to parse behind count")?;
    let ahead: u32 = parts[1].parse().context("Failed to parse ahead count")?;

    Ok((behind, ahead))
}

/// Get a git config value as a boolean
pub fn get_config_bool(key: &str) -> Option<bool> {
    let output = runner::run_output(&["config", "--get", key]).ok()?;
    match output.to_lowercase().as_str() {
        "true" | "1" | "yes" => Some(true),
        "false" | "0" | "no" => Some(false),
        _ => None,
    }
}
