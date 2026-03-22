# git-wrap

**Git with guardrails. No more oops.**

A Rust CLI that wraps Git commands with safety checks and custom hooks.
Drop-in replacement for `git` — unknown commands pass through transparently.

```bash
# works exactly like git
git-wrap status
git-wrap log --oneline -10
git-wrap branch -a

# commit and push are just a bit smarter
git-wrap commit -m "feat: add new feature"
git-wrap push
```

---

## Why git-wrap?

**Opinionated Git.** Like black for Python or biome for JS — just use it and get safe defaults.

All features are **on by default**. Turn off what you don't need.

| Common Mistake | git-wrap Solution |
|----------------|-------------------|
| Forgot to install pre-commit, pushed bad code | Auto-detect → install → configure hooks |
| Push rejected because remote has new commits | Auto fetch → detect behind → pull → push |
| Force pushed and lost remote history | Blocked by default. Use `--yes` to override |
| Committed directly to main | Blocked by default. Use `--yes` to override |
| Submodules out of date after pull/checkout | Auto `submodule update --init --recursive` |
| Uncommitted changes conflict with pull | Auto-stash before pull |
| Need to run scripts before/after git commands | Custom hooks for any command |
| Learning curve for a new tool | Zero. Everything passes through to git |

---

## Install

```bash
cargo install --path .
```

Or drop the release binary into `~/.local/bin/`.

### Recommended: alias it

```bash
# ~/.bashrc or ~/.zshrc
alias git="git-wrap"
```

Now your regular `git` is safe by default.

---

## Quick Start

```bash
# Generate a config file with defaults
git-wrap create-config

# Edit .git-wrap.config.json to customize
```

---

## Features

### `git-wrap pull`

**Auto-stash + submodule update.**

```bash
$ git-wrap pull
→ Auto-stashing uncommitted changes
→ git pull --autostash
Already up to date.
→ git submodule update --init --recursive
```

### `git-wrap checkout` / `switch`

**Auto submodule update after switching branches.**

```bash
$ git-wrap checkout feature-branch
Switched to branch 'feature-branch'
→ git submodule update --init --recursive
```

### `git-wrap push`

**Pull-before-push + force push protection.**

```bash
$ git-wrap push --force
⚠ Force pushing can overwrite remote history.
  Run with --yes to confirm: git-wrap push --force --yes

$ git-wrap push --force --yes
→ git push --force
```

### `git-wrap commit`

**Ensures pre-commit hooks are installed. Blocks direct commits to main/master.**

```bash
$ git-wrap commit -m "oops"   # on main branch
⚠ You are committing directly to main.
  Run with --yes to confirm: git-wrap commit -m oops --yes
```

```
┌───────────────────────────────────────────────┐
│ .pre-commit-config.yaml exists?               │
│   ├─ No  → run git commit as usual            │
│   └─ Yes → pre-commit installed?              │
│              ├─ No  → install via uvx/pipx    │
│              └─ Yes → hooks configured?       │
│                         ├─ No  → run install  │
│                         └─ Yes → git commit   │
└───────────────────────────────────────────────┘
```

```bash
$ git-wrap commit -m "fix: typo"
→ pre-commit not found, installing...
→ Installing via pipx...
✓ pre-commit installed via pipx
→ Installing pre-commit hooks...
✓ pre-commit hooks installed
trim trailing whitespace.....Passed
[main abc1234] fix: typo
```

### `git-wrap push`

**Pull-before-push to prevent rejected pushes.**

```
┌────────────────────────────────────────┐
│ git fetch                              │
│ upstream configured?                   │
│   ├─ No  → warn and push anyway        │
│   └─ Yes → behind remote?              │
│              ├─ No  → push             │
│              └─ Yes → pull then push   │
└────────────────────────────────────────┘
```

```bash
$ git-wrap push
→ Behind by 3 commit(s), pulling...
Successfully rebased and updated refs/heads/main.
To github.com:you/repo.git
   abc1234..def5678  main -> main
```

### Custom Commands

**Define your own commands that run multiple steps.**

```json
{
  "commands": {
    "sync": {
      "run": ["git fetch", "git pull --rebase", "git push"],
      "description": "Fetch, pull, and push in one command"
    },
    "wip": {
      "run": ["git add -A", "git commit -m 'WIP'"]
    }
  }
}
```

```bash
$ git-wrap sync
→ git fetch
→ git pull --rebase
Already up to date.
→ git push
Everything up-to-date
```

### Custom Hooks

**Run commands before/after any git command.**

```json
{
  "commands": {
    "checkout": {
      "after": ["npm install"]
    },
    "pull": {
      "after": ["npm install", "npm run build"]
    }
  }
}
```

```bash
$ git-wrap checkout feature-branch
Switched to branch 'feature-branch'
→ [after] npm install
added 42 packages in 2s
```

### Passthrough

**Everything else goes straight to git.**

```bash
git-wrap stash pop         # → git stash pop
git-wrap rebase -i HEAD~3  # → git rebase -i HEAD~3
git-wrap whatever          # → git whatever
```

---

## Configuration

### Generate config

```bash
git-wrap create-config
```

Creates `.git-wrap.config.json` with sensible defaults.

### `.git-wrap.config.json`

```json
{
  "safety": {
    "blockForcePush": true,
    "blockMainBranch": true
  },
  "auto": {
    "submoduleUpdate": true,
    "stashOnPull": true
  },
  "commit": {
    "ensurePreCommit": true,
    "before": [],
    "after": []
  },
  "push": {
    "pullBeforePush": true,
    "pullStrategy": "git-default",
    "before": [],
    "after": []
  },
  "commands": {
    "sync": {
      "run": ["git fetch", "git pull --rebase", "git push"],
      "description": "Fetch, pull, and push in one command"
    },
    "checkout": {
      "after": ["npm install"]
    }
  }
}
```

### Options

#### `safety`

Safety guards — block dangerous operations unless `--yes` is passed. All default to `true`.

| Key | Type | Description |
|-----|------|-------------|
| `blockForcePush` | `bool` | Block `push --force` unless `--yes` (default: `true`) |
| `blockMainBranch` | `bool` | Block commits to main/master unless `--yes` (default: `true`) |

#### `auto`

Automation — do the right thing without thinking. All default to `true`.

| Key | Type | Description |
|-----|------|-------------|
| `submoduleUpdate` | `bool` | Auto `submodule update` after pull/checkout/switch (default: `true`) |
| `stashOnPull` | `bool` | Auto `--autostash` on pull if uncommitted changes (default: `true`) |

#### `commit`

| Key | Type | Description |
|-----|------|-------------|
| `ensurePreCommit` | `bool` | Auto-install pre-commit if config exists (default: `true`) |
| `before` | `string[]` | Commands to run before commit |
| `after` | `string[]` | Commands to run after commit |

#### `push`

| Key | Type | Description |
|-----|------|-------------|
| `pullBeforePush` | `bool` | Auto-pull before push if behind (default: `true`) |
| `pullStrategy` | `string` | `"git-default"`, `"rebase"`, or `"merge"` |
| `before` | `string[]` | Commands to run before push |
| `after` | `string[]` | Commands to run after push |

#### `commands`

Custom commands or hooks for git commands.

| Key | Type | Description |
|-----|------|-------------|
| `run` | `string[]` | Commands to execute (makes this a custom command) |
| `description` | `string` | Optional description for the command |
| `before` | `string[]` | Commands to run before git command (ignored if `run` is set) |
| `after` | `string[]` | Commands to run after git command (ignored if `run` is set) |

**Custom command** (has `run`):

```json
{
  "commands": {
    "sync": {
      "run": ["git fetch", "git pull --rebase", "git push"],
      "description": "Sync with remote"
    }
  }
}
```

**Hooked git command** (no `run`):

```json
{
  "commands": {
    "checkout": {
      "after": ["npm install"]
    }
  }
}
```

Use `#` prefix to comment out a command:

```json
{
  "commands": {
    "checkout": {
      "after": ["# npm install"]
    }
  }
}
```

---

## Build

```bash
cargo build --release
./target/release/git-wrap status
```

---

## License

MIT

---

<p align="center">
<b>git-wrap</b> — because <code>git push -f</code> should be a choice, not an accident.
</p>
