---
domain: product-code
type: decision-record
status: accepted
owner: Mathieu (solo maintainer)
supersedes:
---

# Hexagonal architecture (ports & adapters) across four crates

## Context

This is a small, solo-maintained content API — traffic and team size don't demand a layered architecture on their own. The driving force is deliberate practice: applying hexagonal/clean architecture properly in Rust, where the domain logic should have zero knowledge of the HTTP framework or the database driver.

Alternative considered: a single flat crate with modules (`handlers/`, `models/`, `db/`). Simpler to navigate for a project this size, but doesn't enforce the dependency-direction discipline that's the actual point of this exercise — nothing stops a handler from reaching into sqlx directly, and the boundary erodes the first time it's convenient to skip it.

## Decision

The workspace is split into four crates with dependencies pointing inward:

- `domain` — entities (e.g. `Project`) and repository traits (ports). No dependency on Axum, sqlx, or tokio beyond what's unavoidable for async trait signatures.
- `application` — use cases (e.g. "list projects", "get project by slug") that orchestrate `domain` types through its repository traits. Depends on `domain` only.
- `infrastructure` — adapters implementing `domain`'s repository traits against Postgres via sqlx. Depends on `domain`.
- `web` — Axum HTTP handlers that call into `application`'s use cases and serialize the results. Depends on `application` and `domain`; wires a concrete `infrastructure` implementation at startup.

`domain` and `application` never depend on `infrastructure` or `web`.

## Consequences

Easier: swapping the database (a different store implementing the same repository trait) or the delivery mechanism (a CLI or a different framework alongside/instead of Axum) without touching business logic; unit-testing use cases against an in-memory fake repository instead of a real database.

Harder: more ceremony for a project this size — four `Cargo.toml`s, trait indirection for what could be a direct function call, and the discipline to keep `domain` genuinely framework-agnostic as the project grows. Accepted as the cost of the practice goal; if this stops being a useful exercise and starts being pure friction, that's worth revisiting (see Superseded by).

## Superseded by

