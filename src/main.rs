mod cli;
mod commands;
mod config;
mod git;
mod precommit;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        // git-wrap specific commands
        Some("create-config") => commands::create_config::run(&args[1..]),

        // Special handled commands with default safety features
        Some("commit") => commands::commit::run(&args[1..]),
        Some("push") => commands::push::run(&args[1..]),

        // Any other command: check for hooks, then passthrough
        Some(cmd) => run_with_hooks(cmd, &args),

        // No command
        None => git::runner::passthrough(&args),
    }
}

/// Run a git command with optional before/after hooks from config
fn run_with_hooks(cmd: &str, args: &[String]) -> Result<()> {
    let config = config::load()?;

    if let Some(hooks) = config.get_hooks(cmd) {
        // Run before hooks
        commands::hooks::run_before(hooks)?;

        // Run git command
        git::runner::run(&[cmd], &args[1..])?;

        // Run after hooks
        commands::hooks::run_after(hooks)?;

        Ok(())
    } else {
        // No hooks configured, passthrough
        git::runner::passthrough(args)
    }
}
