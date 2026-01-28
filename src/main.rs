mod cli;
mod commands;
mod config;
mod git;
mod precommit;

use anyhow::Result;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match args.first().map(|s| s.as_str()) {
        Some("commit") => commands::commit::run(&args[1..]),
        Some("push") => commands::push::run(&args[1..]),
        _ => git::runner::passthrough(&args),
    }
}
