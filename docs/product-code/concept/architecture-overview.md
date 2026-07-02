---
domain: product-code
type: concept
status: active
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# Architecture Overview

This is the content API for [mathieusouflis](https://github.com/mathieusouflis)'s 2026 portfolio — it serves project/case-study data to a separate frontend. It's built as a hexagonal (ports & adapters) system, primarily as a deliberate exercise in keeping business logic independent of frameworks and infrastructure. See [decisions/0001-hexagonal-architecture](../decisions/0001-hexagonal-architecture.md) for why.

## Components

Four crates, dependencies pointing inward:

```
        ┌─────────┐        ┌──────────────────┐
        │   web    │──────▶│   application     │
        │ (Axum)   │        │  (use cases)      │
        └─────────┘        └────────┬──────────┘
                                     │
                                     ▼
                             ┌───────────────┐
                             │    domain      │
                             │ (entities,     │
                             │  repo traits)  │
                             └───────▲───────┘
                                     │
                             ┌───────┴───────┐
                             │ infrastructure │
                             │ (sqlx/Postgres)│
                             └───────────────┘
```

- **`domain`** — the `Project` entity (and any related value objects) plus repository *traits* describing what persistence must support (e.g. `list_projects`, `find_by_slug`), with no implementation. No dependency on Axum, sqlx, or serialization frameworks.
- **`application`** — use cases that orchestrate `domain` types through its repository traits (e.g. "list all published projects"). This is where business rules that don't belong to a single entity live.
- **`infrastructure`** — implements `domain`'s repository traits against Postgres via sqlx. Depends on `domain`, not on `application` or `web`.
- **`web`** — Axum route handlers that deserialize requests, call an `application` use case, and serialize the response as JSON. Depends on `application` and `domain`.
- **`src/`** (binary crate) — the composition root: wires a concrete `infrastructure` implementation into `application`'s use cases and starts the Axum server.

## Core data flow

A request for the project list: `web` handler receives `GET /projects` → calls the `list_projects` use case in `application` → which calls the `ProjectRepository` trait it was given at startup → which is, at runtime, the `infrastructure` crate's sqlx-backed Postgres implementation → rows come back as `domain::Project` values → `application` returns them unchanged (no business logic needed for a plain listing) → `web` serializes them to JSON.

The same shape applies to fetching a single project by slug, and to any future write path (e.g. an admin endpoint), with `application` as the place validation/business rules would go before hitting `infrastructure`.

## Key constraints shaping this design

- **Solo maintainer, low traffic** — this is not an architecture chosen for scale; see [decisions/0001-hexagonal-architecture](../decisions/0001-hexagonal-architecture.md) for why it was chosen anyway.
- **Framework independence as the goal, not a side effect** — `domain` and `application` are deliberately kept ignorant of Axum and sqlx so the boundary is actually exercised, not just theoretical.
- **Deployment target is a single Docker container** — see [decisions/0003-deployment-infrastructure](../decisions/0003-deployment-infrastructure.md); the architecture doesn't currently need to account for multiple instances or horizontal scaling.

## See also

- [product-code/decisions](../decisions/) — individual architectural choices (crate layering, framework/DB pick, deployment infra)
- [product-code/reference](../reference/) — concrete environment variables and code-style conventions
