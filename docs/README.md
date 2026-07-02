# Documentation

Organized by two axes: **domain** (what this document is about) and **type** (what shape it takes / what the reader is trying to do). A document always lives at the intersection of both — e.g. `product-code/reference/` is a lookup table about the code, `team-process/how-to/` is a step-by-step guide about how the team works.

| Type → <br> Domain ↓ | Concept | How-to | Reference | Decisions | Tutorials | Runbooks |
|---|---|---|---|---|---|---|
| **[product-code](product-code/)** | [Architecture overview](product-code/concept/architecture-overview.md) | — | [Code style](product-code/reference/code-style.md), [Environment variables](product-code/reference/environment-variables.md) | [0001 — Hexagonal architecture](product-code/decisions/0001-hexagonal-architecture.md), [0002 — Axum + Postgres/sqlx](product-code/decisions/0002-axum-postgres-sqlx.md), [0003 — Deployment infrastructure](product-code/decisions/0003-deployment-infrastructure.md) | [Getting started](product-code/tutorials/getting-started.md) | — |
| **[operations](operations/)** | [CI/CD pipeline](operations/concept/ci-cd-pipeline.md) | — | — | — | — | [Deployment](operations/runbooks/deployment-runbook.md) 🚧 |
| **[team-process](team-process/)** | — | [Activate Git hooks](team-process/how-to/activate-git-hooks.md) | [Commit conventions & PR process](../CONTRIBUTING.md) ⧉ | — | — | — |
| **[organizational](organizational/)** | — | — | [Tool inventory](organizational/reference/tool-inventory.md) | — | — | — |

🚧 = filled in but provisional — the deployment runbook describes an intended procedure that hasn't been run for real yet (no infrastructure provisioned as of this writing); see its `last_exercised` field.

⧉ = lives outside `docs/` at a fixed path, see below.

This project is solo-maintained, so the `organizational` domain is intentionally thin — no role charter (no roles to charter with one person) and no separate tooling-choice decision record (infra tooling rationale lives in the product-code ADRs above instead, alongside the code-level choices it was made with).

A few things intentionally live outside this grid, at fixed paths GitHub itself expects: [`README.md`](../README.md), [`CONTRIBUTING.md`](../CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md), [`SECURITY.md`](../SECURITY.md), and [`LICENSE`](../LICENSE). Moving them would break the platform features tied to those paths (Security tab, community profile checklist, the PR-open banner) — so unlike the rest of the grid, these stay canonical at their GitHub-required location, and the grid links *to* them rather than absorbing their content. `CONTRIBUTING.md` in particular is intentionally kept complete (commit conventions, bug/feature reporting, PR process included inline) rather than split into `docs/team-process/`, since GitHub surfaces it directly at the moment a contributor opens a PR or issue — splitting it would add a click at exactly the point someone needs the answer fastest.
