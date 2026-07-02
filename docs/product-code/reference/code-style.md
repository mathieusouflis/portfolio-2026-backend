---
domain: product-code
type: reference
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# Code Style Guide

This document defines the code style and formatting conventions for this project. Consistency matters more than any individual rule — when in doubt, follow existing patterns in the codebase.

---

## Tooling

```bash
cargo fmt --check                        # Check formatting
cargo fmt                                # Auto-fix formatting
cargo clippy --workspace --all-targets   # Lint
```

All checks must pass before a PR can be merged. The CI pipeline enforces this automatically — see [operations/concept/ci-cd-pipeline](../../operations/concept/ci-cd-pipeline.md).

---

## General Principles

- **Clarity over cleverness** — write code for the next reader, not the compiler
- **Explicit over implicit** — avoid magic; name things for what they do
- **Small functions** — each function should do one thing
- **No dead code** — remove commented-out code before committing
- **Domain stays framework-agnostic** — `domain` and `application` must not depend on Axum or sqlx types; see [product-code/decisions/0001-hexagonal-architecture](../decisions/0001-hexagonal-architecture.md)

---

## Naming Conventions

Standard Rust conventions, enforced by `clippy`:

| Element | Convention | Example |
|---------|------------|---------|
| Files / modules | `snake_case` | `project_repository.rs` |
| Types (structs, enums, traits) | `PascalCase` | `ProjectRepository` |
| Functions / methods / variables | `snake_case` | `find_by_slug` |
| Constants / statics | `SCREAMING_SNAKE_CASE` | `MAX_RETRY_COUNT` |
| Crates | `kebab-case` on crates.io, `snake_case` internally | `application` |

---

## Formatting

Governed entirely by `rustfmt` defaults (`cargo fmt`) — no manual bikeshedding on indentation, line length, or trailing commas. If a formatting question isn't answered by running `cargo fmt`, it's not a formatting question.

---

## Import / Dependency Order

`rustfmt` groups and sorts `use` statements automatically. By convention, keep this order:

1. `std`
2. External crates (`axum`, `sqlx`, `serde`, ...)
3. Workspace crates (`domain`, `application`, `infrastructure`)
4. `crate::`/`self::`/`super::` (local module) imports

---

## Error Handling

- Never swallow errors silently — propagate with `?` or handle explicitly
- Use typed errors (`enum` implementing `std::error::Error`, e.g. via `thiserror`) in `domain`/`application`; reserve broader error types (e.g. `anyhow`) for the `web` binary boundary where errors just need to become an HTTP response
- Validate inputs at system boundaries (`web` handlers); trust internal code once past that boundary
- No `unwrap()`/`expect()` outside of tests and startup code (e.g. config parsing where a bad value should fail fast)

---

## Testing Conventions

- Unit tests live in `#[cfg(test)] mod tests` next to the code they test
- Integration tests (exercising a crate's public API, e.g. `web`'s HTTP routes) live in that crate's `tests/` directory
- Each test should be independent and not rely on shared mutable state (a fresh test database/transaction per test, not a shared one)
- Test names describe the expected behavior: `returns_404_when_project_not_found`

---

## Running the Full Validation Suite

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

All four must pass before pushing.

## See also

- [operations/concept/ci-cd-pipeline](../../operations/concept/ci-cd-pipeline.md) — how these checks run in CI
- [product-code/tutorials/getting-started](../tutorials/getting-started.md)
