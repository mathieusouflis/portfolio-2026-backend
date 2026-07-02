---
domain: organizational
type: reference
status: active
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# Tool Inventory

What this project runs on. Every new external tool/dependency/vendor adopted should add a row here in the same PR that introduces it — this is the entry most commonly forgotten. See [product-code/decisions](../../product-code/decisions/) for *why* a given tool was chosen; this file is just the lookup table of *what* is in use.

| Tool | Used for | Alternatives considered |
|---|---|---|
| GitHub | Source control, CI/CD (Actions), issue tracking | — |
| Axum | HTTP framework (`web` crate) | Actix-web, Rocket — see [0002-axum-postgres-sqlx](../../product-code/decisions/0002-axum-postgres-sqlx.md) |
| sqlx + Postgres | Persistence (`infrastructure` crate) | Diesel/SeaORM, SQLite — see [0002-axum-postgres-sqlx](../../product-code/decisions/0002-axum-postgres-sqlx.md) |
| Docker | Container image for deployment; local Postgres via Compose | — |
| Dokploy | Self-hosted PaaS — builds/deploys/rolls back the container on the VPS | Fly.io, Railway, Render — see [0003-deployment-infrastructure](../../product-code/decisions/0003-deployment-infrastructure.md) |
| Terraform | Provisions the VPS and DNS as code | Hand-configured VPS — see [0003-deployment-infrastructure](../../product-code/decisions/0003-deployment-infrastructure.md) |
| Ansible | Configures the VPS and drives the Dokploy install/deploy steps | — |

## See also

- [product-code/decisions](../../product-code/decisions/) — why a given tool was chosen over alternatives
