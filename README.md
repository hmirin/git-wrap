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

| Common Mistake | git-wrap Solution |
|----------------|-------------------|
| Forgot to install pre-commit, pushed bad code | Auto-detect → install → configure hooks |
| Push rejected because remote has new commits | Auto fetch → detect behind → pull → push |
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

### `git-wrap commit`

**Ensures pre-commit hooks are installed before committing.**

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

### Custom Hooks

**Run any command before/after any git command.**

```json
{
  "commands": {
    "checkout": {
      "before": [],
      "after": ["npm install"]
    },
    "pull": {
      "before": [],
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
  "commit": {
    "ensurePreCommit": true
  },
  "push": {
    "pullBeforePush": true,
    "pullStrategy": "git-default"
  },
  "commands": {
    "checkout": {
      "before": [],
      "after": ["npm install"]
    }
  }
}
```

### Options

#### `commit`

| Key | Value | Description |
|-----|-------|-------------|
| `ensurePreCommit` | `true` / `false` | Auto-install pre-commit if config exists (default: `true`) |

#### `push`

| Key | Value | Description |
|-----|-------|-------------|
| `pullBeforePush` | `true` / `false` | Auto-pull before push if behind (default: `true`) |
| `pullStrategy` | `"git-default"` | Respect git's `pull.rebase` setting |
| | `"rebase"` | Always use `--rebase` |
| | `"merge"` | Always merge |

#### `commands`

Generic hooks for any git command.

| Key | Type | Description |
|-----|------|-------------|
| `before` | `string[]` | Shell commands to run before git command |
| `after` | `string[]` | Shell commands to run after git command |

Use `#` prefix to comment out a hook:

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
