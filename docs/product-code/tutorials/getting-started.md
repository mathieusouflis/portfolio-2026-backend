---
domain: product-code
type: tutorial
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# Getting Started

This guide walks a new contributor from zero to a running local environment. It assumes no prior context — for a specific task you already know how to do (e.g. "add an endpoint"), see [product-code/reference](../reference/) instead.

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable, edition 2024 (rustc ≥ 1.85) | [rustup.rs](https://rustup.rs) |
| Docker (+ Compose) | recent | [docker.com](https://docs.docker.com/get-docker/) |

---

## Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/mathieusouflis/portfolio-2026-backend.git
   cd portfolio-2026-backend
   ```

2. **Activate the Git hooks** (one-time, after cloning)

   ```bash
   git config core.hooksPath .githooks
   ```

   Details: [team-process/how-to/activate-git-hooks](../../team-process/how-to/activate-git-hooks.md).

3. **Copy the environment file**

   ```bash
   cp .env.example .env
   ```

   Open `.env` and fill in the required values — see [product-code/reference/environment-variables](../reference/environment-variables.md).

4. **Start local Postgres**

   ```bash
   docker compose up -d
   ```

   <!-- docker-compose.yml doesn't exist in the repo yet — add one pointing at a `postgres` image and matching the DATABASE_URL in .env before this step works. -->

5. **Build the workspace**

   ```bash
   cargo build
   ```

   Note: sqlx checks queries against a live database at compile time, so step 4 must succeed first (or `SQLX_OFFLINE=true` with a committed query cache, once one exists).

6. **Run the server**

   ```bash
   cargo run
   ```

---

## Common Commands

```bash
cargo run                                # Start the server
cargo test                               # Run tests
cargo clippy --workspace --all-targets   # Lint
cargo fmt                                # Format
cargo build --release                    # Build for production
```

---

## Troubleshooting

**Port already in use**

Find and stop the process occupying the port before starting the server.

**`cargo build` fails with sqlx compile-time query errors**

Make sure Postgres is running (`docker compose up -d`) and `DATABASE_URL` in `.env` matches it. sqlx needs a live connection to verify queries unless offline mode is configured.

**Dependencies not installing**

Make sure your Rust toolchain matches [Prerequisites](#prerequisites) (`rustup show`). Delete `Cargo.lock` and rerun `cargo build` to rule out local drift.

**Tests fail locally but pass in CI (or vice versa)**

Make sure your `.env` matches the values expected by the test suite, and that local Postgres is on the same major version as CI/production.

**Build fails with errors you didn't introduce**

Pull the latest `main` and rebuild — a dependency may have been updated since your last build.

For anything else, open an [issue](../../../../issues).

## See also

- [product-code/reference/environment-variables](../reference/environment-variables.md)
- [team-process/how-to/activate-git-hooks](../../team-process/how-to/activate-git-hooks.md)
