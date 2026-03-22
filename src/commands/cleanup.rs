use anyhow::Result;
use colored::Colorize;

use crate::cli;
use crate::git;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;

    let has_yes = cli::has_yes(args);

    let current = git::repo::get_current_branch().unwrap_or_default();

    // Get merged branches
    let output = git::runner::run_output(&["branch", "--merged"])?;
    let branches: Vec<String> = output
        .lines()
        .map(|l| l.trim().trim_start_matches("* ").to_string())
        .filter(|b| {
            !b.is_empty()
                && b != "main"
                && b != "master"
                && *b != current
        })
        .collect();

    if branches.is_empty() {
        println!("{} Nothing to clean up. All branches are unmerged or protected.", "✓".green());
        return Ok(());
    }

    println!(
        "{} Found {} merged branch(es):",
        "→".cyan(),
        branches.len()
    );
    for branch in &branches {
        println!("  - {}", branch);
    }

    if !has_yes {
        eprintln!(
            "\n{} Run with --yes to delete: git-wrap cleanup --yes",
            "!".yellow()
        );
        return Ok(());
    }

    // Delete each branch
    let mut deleted = 0;
    for branch in &branches {
        println!("{} git branch -d {}", "→".cyan(), branch);
        match git::runner::run(&["branch", "-d"], &[branch.clone()]) {
            Ok(_) => deleted += 1,
            Err(e) => eprintln!("{} Failed to delete {}: {}", "✗".red(), branch, e),
        }
    }

    println!(
        "{} Deleted {} branch(es).",
        "✓".green(),
        deleted
    );

    Ok(())
}
