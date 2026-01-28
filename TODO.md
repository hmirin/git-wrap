# git-wrap TODO

## Safety Features

- [ ] **`reset --hard` confirmation** - Prompt before destructive reset
- [ ] **`clean -f` confirmation** - Prompt before removing untracked files
- [ ] **`branch -D` warning** - Warn when deleting unmerged branches
- [ ] **`stash drop` confirmation** - Prompt before dropping stashes
- [ ] **`push --force` confirmation** - Extra confirmation for force push
- [ ] **Branch protection** - Block direct commits to main/master
- [ ] **`amend` warning** - Warn when amending already-pushed commits
- [ ] **`rebase` warning** - Warn when rebasing published branches

## Convenience Features

- [ ] **`git-wrap sync`** - fetch + pull + push (DONE - via custom commands)
- [ ] **`git-wrap status` enhanced** - Show behind/ahead count with remote
- [ ] **`git-wrap init` enhanced** - Initialize with pre-commit template
- [ ] **`git-wrap undo`** - Undo last commit (soft reset)
- [ ] **`git-wrap unstage`** - Unstage all files
- [ ] **`git-wrap wip`** - Quick WIP commit
- [ ] **`git-wrap unwip`** - Undo WIP commit if it's the last one
- [ ] **`git-wrap squash N`** - Interactive squash last N commits
- [ ] **`git-wrap pr`** - Create PR from current branch (gh wrapper)
- [ ] **`git-wrap browse`** - Open repo in browser

## Automation Features

- [ ] **`checkout` auto-stash** - Stash before checkout, pop after
- [ ] **`rebase` auto-stash** - Stash before rebase, pop after
- [ ] **`pull` auto-stash** - Stash before pull, pop after
- [ ] **Auto-fetch on status** - Fetch in background when running status
- [ ] **Dependency install after checkout/pull** - Auto-run npm/pip install

## Configuration

- [ ] **Global config** - `~/.git-wrap.config.json` for defaults
- [ ] **Config inheritance** - Local config extends global config
- [ ] **Per-branch config** - Different settings for different branches
- [ ] **`git-wrap config`** - CLI for viewing/editing config
- [ ] **`git-wrap config --global`** - Edit global config

## Output & UX

- [ ] **Verbose mode** - `--verbose` flag for detailed output
- [ ] **Quiet mode** - `--quiet` flag for minimal output
- [ ] **Dry-run mode** - `--dry-run` to show what would happen
- [ ] **Color themes** - Configurable colors
- [ ] **`git-wrap help <command>`** - Show help for specific command

## Integration

- [ ] **Shell completions** - Bash/Zsh/Fish completions
- [ ] **Git alias integration** - Auto-generate git aliases
- [ ] **CI detection** - Disable interactive prompts in CI
- [ ] **Editor integration** - VS Code extension

## Built-in Custom Commands (defaults)

```json
{
  "commands": {
    "sync": {
      "run": ["git fetch", "git pull --rebase", "git push"]
    },
    "wip": {
      "run": ["git add -A", "git commit -m 'WIP'"]
    },
    "unwip": {
      "run": ["git log -1 --format=%s | grep -q '^WIP$' && git reset HEAD~1"]
    },
    "undo": {
      "run": ["git reset --soft HEAD~1"]
    },
    "unstage": {
      "run": ["git reset HEAD"]
    }
  }
}
```

## Technical Debt

- [ ] Add unit tests
- [ ] Add integration tests
- [ ] CI/CD pipeline
- [ ] Release automation
- [ ] Homebrew formula
- [ ] Publish to crates.io
