use anyhow::Result;
use std::path::Path;

use crate::git;
use crate::precommit;

pub fn run(args: &[String]) -> Result<()> {
    git::repo::ensure_in_repo()?;

    // If .pre-commit-config.yaml exists, ensure pre-commit is set up
    if Path::new(".pre-commit-config.yaml").exists() {
        precommit::ensure_installed()?;
        precommit::ensure_hooks_installed()?;
    }

    git::runner::run(&["commit"], args)
}
