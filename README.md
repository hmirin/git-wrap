# git-wrap

**Git with guardrails. No more oops.**

A Rust CLI that wraps dangerous Git commands with safety checks.
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

### Passthrough

**Everything else goes straight to git.**

```bash
git-wrap stash pop         # → git stash pop
git-wrap rebase -i HEAD~3  # → git rebase -i HEAD~3
git-wrap whatever          # → git whatever
```

---

## Configuration

### `.git-wrap.config.json` (optional)

Place in repository root.

```json
{
  "push": {
    "pullBeforePush": true,
    "pullStrategy": "rebase"
  }
}
```

| Key | Value | Description |
|-----|-------|-------------|
| `pullBeforePush` | `true` / `false` | Enable auto-pull before push (default: `true`) |
| `pullStrategy` | `"git-default"` | Respect git's `pull.rebase` setting |
| | `"rebase"` | Always use `--rebase` (default) |
| | `"merge"` | Always merge |

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
