# Documentation

Organized by two axes: **domain** (what this document is about) and **type** (what shape it takes / what the reader is trying to do). A document always lives at the intersection of both — e.g. `product-code/reference/` is a lookup table about the code, `team-process/how-to/` is a step-by-step guide about how the team works.

| Type → <br> Domain ↓ | Concept | How-to | Reference | Decisions | Tutorials | Runbooks |
|---|---|---|---|---|---|---|
| **[product-code](product-code/)** | [Architecture overview](product-code/concept/todo_architecture-overview.md) 🚧 | — | [Code style](product-code/reference/code-style.md), [Environment variables](product-code/reference/environment-variables.md) | [First decision](product-code/decisions/todo_first-architecture-decision.md) 🚧 | [Getting started](product-code/tutorials/getting-started.md) | — |
| **[operations](operations/)** | [CI/CD pipeline](operations/concept/ci-cd-pipeline.md) | — | — | — | — | [Deployment](operations/runbooks/todo_deployment-runbook.md) 🚧 |
| **[team-process](team-process/)** | — | [Activate Git hooks](team-process/how-to/activate-git-hooks.md) | [Commit conventions & PR process](../CONTRIBUTING.md) ⧉ | — | — | — |
| **[organizational](organizational/)** | [Role charter](organizational/concept/todo_role-charter.md) 🚧 | — | [Tool inventory](organizational/reference/todo_tool-inventory.md) 🚧, [Code owners](../.github/CODEOWNERS) ⧉ | [Tooling choice](organizational/decisions/todo_tooling-choice.md) 🚧 | — | — |

🚧 = template placeholder (`todo_` prefix), no real content yet — see the note inside each file for what to fill in and how to rename it once done.
⧉ = lives outside `docs/` at a fixed path, see below.

A few things intentionally live outside this grid, at fixed paths GitHub itself expects: [`README.md`](../README.md), [`CONTRIBUTING.md`](../CONTRIBUTING.md), [`CODE_OF_CONDUCT.md`](../CODE_OF_CONDUCT.md), [`SECURITY.md`](../SECURITY.md), [`LICENSE`](../LICENSE), and [`.github/CODEOWNERS`](../.github/CODEOWNERS). Moving them would break the platform features tied to those paths (Security tab, community profile checklist, the PR-open banner) — so unlike the rest of the grid, these stay canonical at their GitHub-required location, and the grid links *to* them rather than absorbing their content. `CONTRIBUTING.md` in particular is intentionally kept complete (commit conventions, bug/feature reporting, PR process included inline) rather than split into `docs/team-process/`, since GitHub surfaces it directly at the moment a contributor opens a PR or issue — splitting it would add a click at exactly the point someone needs the answer fastest.
