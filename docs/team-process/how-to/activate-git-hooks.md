---
domain: team-process
type: how-to
owner: <!-- team/role that owns tooling -->
last_reviewed:
---

# How to Activate the Git Hooks

This repo ships two shell-based hooks under `.githooks/` — no runtime or dependencies required, so they work identically regardless of the project's language.

## Steps

1. From the repository root, run:

   ```bash
   git config core.hooksPath .githooks
   ```

2. That's it — this is a one-time, per-clone setting stored in your local Git config, not committed to the repo.

## What each hook does

| Hook | Runs on | Purpose |
|---|---|---|
| `commit-msg` | every `git commit` | Rejects commit messages that don't follow [Conventional Commits](../../../CONTRIBUTING.md#commit-conventions) |
| `pre-push` | every `git push` | Placeholder — uncomment the test/build checks inside `.githooks/pre-push` to run them before every push |

## Why this is optional but recommended

Activating the hook is not required to contribute — `commit-msg` validation also runs in CI on every pull request as a backstop, so a bad commit message is still caught before merge even if you skip this step. Activating it locally just gives you the same feedback earlier, before you push.

## See also

- [Commit Conventions](../../../CONTRIBUTING.md#commit-conventions)
- [operations/concept/ci-cd-pipeline](../../operations/concept/ci-cd-pipeline.md)
