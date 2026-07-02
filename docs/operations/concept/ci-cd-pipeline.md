---
domain: operations
type: concept
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# CI/CD Pipeline

How this repository validates and ships changes, as actually configured under `.github/workflows/` and `.githooks/` today.

## Continuous Integration — `.github/workflows/ci.yml`

Runs on every push to `main` and every pull request targeting `main`. Three jobs, each on a fresh runner:

| Job | Purpose |
|---|---|
| `lint` | `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings` |
| `test` | `cargo test --workspace` |
| `build` | Runs after `lint` and `test` pass; `cargo build --workspace --release` |

Rust setup and dependency caching use `dtolnay/rust-toolchain@stable` and `Swatinem/rust-cache@v2`. The `test` job doesn't run against a live Postgres instance yet — nothing in the workspace uses sqlx queries yet either, so there's nothing to check against. Once the `infrastructure` crate adds real queries (see [decisions/0002-axum-postgres-sqlx](../../product-code/decisions/0002-axum-postgres-sqlx.md)), a Postgres service container will need to be added to `test` (and `lint`, since `cargo clippy` also compiles) for sqlx's compile-time query checking to succeed.

## Commit message validation — `.githooks/commit-msg` + `.github/workflows/commitlint.yml`

Commit messages are validated against Conventional Commits **twice**, both reusing the exact same shell script (`.githooks/commit-msg`) so there is one source of truth for the rule rather than two that can drift apart:

- **Locally**, if the contributor activated the hook (`git config core.hooksPath .githooks`) — fast feedback, but optional and easy to skip.
- **In CI**, on every pull request — the `commitlint.yml` workflow re-runs the same script against every commit in the PR, so a contributor who never activated the local hook is still caught before merge.

See [Commit Conventions](../../../CONTRIBUTING.md#commit-conventions) for the rule itself.

## Release — `.github/workflows/release.yml`

Triggered by pushing a tag matching `v*.*.*`. Creates a GitHub Release with auto-generated release notes from the commit history (`generate_release_notes: true`) — no manual changelog editing required, provided commits follow the Conventional Commits format enforced above.

## Dependency updates — `.github/dependabot.yml`

Three ecosystems are active: `cargo` (weekly, grouped minor/patch updates), `github-actions` (workflow action versions, monthly), and `docker` (monthly — relevant once a `Dockerfile` exists for the [Dokploy deployment](../runbooks/deployment-runbook.md)).

## See also

- [Commit Conventions](../../../CONTRIBUTING.md#commit-conventions)
- [team-process/how-to/activate-git-hooks](../../team-process/how-to/activate-git-hooks.md)
- [operations/runbooks](../runbooks/)
