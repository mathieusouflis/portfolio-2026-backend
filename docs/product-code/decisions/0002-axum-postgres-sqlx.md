---
domain: product-code
type: decision-record
status: accepted
owner: Mathieu (solo maintainer)
supersedes:
---

# Axum for the web adapter, Postgres via sqlx for persistence

## Context

`Cargo.toml` already pulled in `tokio` and `serde` at the workspace level, but no web framework or database layer had been chosen yet. The `web` crate needed an HTTP framework; the `infrastructure` crate needed a way to talk to a database for portfolio project/case-study content.

Alternatives considered for the framework: Actix-web (mature, high performance, but a heavier/older API surface and its own runtime conventions) and Rocket (batteries-included, but historically slower to adopt new Rust/async idioms).

Alternatives considered for persistence: an ORM like Diesel or SeaORM (more abstraction, less control over queries); SQLite (simpler ops, no server to run, but doesn't match the intended production deployment).

## Decision

`web` uses **Axum**: it's tokio-native (already a workspace dependency), built on `tower` so middleware composes with the wider tower/hyper ecosystem, and its handler signatures stay close to plain async functions — a good fit for a hexagonal `web` crate that should stay a thin adapter over `application`'s use cases.

`infrastructure` uses **Postgres via sqlx**: compile-time-checked queries without a full ORM's abstraction layer, async-native, and Postgres is the target for the real (Dockerized) deployment — see [0003-deployment-infrastructure](0003-deployment-infrastructure.md).

## Consequences

Easier: handlers stay small and composable; query correctness is checked at compile time against the real schema (via `sqlx::query!`/`sqlx::migrate!`) rather than caught at runtime; no ORM-specific query DSL to learn on top of SQL.

Harder: sqlx's compile-time checks require a reachable database (or an offline query cache) during `cargo build`, which the local dev setup needs to account for — see [product-code/tutorials/getting-started](../tutorials/getting-started.md). Axum's ecosystem, while solid, is younger than Actix-web's, so some middleware may need to be hand-rolled.

## Superseded by

