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
        Some("cleanup") => commands::cleanup::run(&args[1..]),

        // Special handled commands with default safety features
        Some("commit") => commands::commit::run(&args[1..]),
        Some("push") => commands::push::run(&args[1..]),
        Some("pull") => commands::pull::run(&args[1..]),
        Some("checkout") => commands::checkout::run("checkout", &args[1..]),
        Some("switch") => commands::checkout::run("switch", &args[1..]),

        // Any other command: check for custom command or hooks, then passthrough
        Some(cmd) => run_command(cmd, &args),

        // No command
        None => git::runner::passthrough(&args),
    }
}

/// Run a command - either custom command, hooked git command, or passthrough
fn run_command(cmd: &str, args: &[String]) -> Result<()> {
    let config = config::load()?;

    if let Some(cmd_config) = config.get_command(cmd) {
        if cmd_config.is_custom() {
            // Custom command: run the steps
            commands::hooks::run_steps(&cmd_config.run)?;
            Ok(())
        } else {
            // Wrapped git command with before/after hooks
            commands::hooks::run_hooks(&cmd_config.before, "before")?;
            git::runner::run(&[cmd], &args[1..])?;
            commands::hooks::run_hooks(&cmd_config.after, "after")?;
            Ok(())
        }
    } else {
        // No config, passthrough to git
        git::runner::passthrough(args)
    }
}
