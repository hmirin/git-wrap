use anyhow::Result;

use super::hooks;
use super::pull::maybe_update_submodules;
use crate::config;
use crate::git;

/// Handles both `checkout` and `switch` commands
pub fn run(cmd: &str, args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;
    let cfg = config::load()?;

    // Run before hooks from commands.{cmd} config
    if let Some(cmd_config) = cfg.get_command(cmd) {
        hooks::run_hooks(&cmd_config.before, "before")?;
    }

    // Run git checkout/switch
    git::runner::run(&[cmd], args)?;

    // Auto submodule update
    maybe_update_submodules(&cfg)?;

    // Run after hooks from commands.{cmd} config
    if let Some(cmd_config) = cfg.get_command(cmd) {
        hooks::run_hooks(&cmd_config.after, "after")?;
    }

    Ok(())
}
