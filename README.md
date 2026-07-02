# portfolio-2026-backend

[![CI](https://github.com/mathieusouflis/portfolio-2026-backend/actions/workflows/ci.yml/badge.svg)](https://github.com/mathieusouflis/portfolio-2026-backend/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

JSON API backing my [2026 portfolio](https://github.com/mathieusouflis) frontend — serves project/case-study content from Postgres. Built as a deliberate exercise in hexagonal (ports & adapters) architecture in Rust.

---

## Table of Contents

- [Overview](#overview)
- [Getting Started](#getting-started)
- [Architecture](#architecture)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)

---

## Overview

The public portfolio site is a separate frontend project; this repo is its content API — listing and fetching portfolio projects/case studies, backed by Postgres. It's also where I'm practicing hexagonal architecture properly in Rust: the domain logic has no knowledge of Axum or sqlx, so the HTTP layer and the database are both replaceable adapters around a stable core.

---

## Getting Started

```bash
git clone https://github.com/mathieusouflis/portfolio-2026-backend.git
cd portfolio-2026-backend
git config core.hooksPath .githooks   # activate the Git hooks — optional but recommended
cp .env.example .env                   # then fill in the values
```

Full setup guide (prerequisites, dependency install, dev server): [docs/product-code/tutorials/getting-started](docs/product-code/tutorials/getting-started.md).

---

## Architecture

Hexagonal (ports & adapters), split across four crates: `domain` (entities + repository traits, no framework dependencies), `application` (use cases orchestrating the domain), `infrastructure` (sqlx/Postgres — implements the domain's repository traits), and `web` (Axum — HTTP handlers, translates requests into use case calls). Dependencies point inward: `web` and `infrastructure` depend on `domain`/`application`, never the reverse.

See [docs/product-code/concept/architecture-overview](docs/product-code/concept/architecture-overview.md) for the full picture and [docs/product-code/decisions](docs/product-code/decisions/) for why.

```
.
├── crates/
│   ├── domain/          # Entities, repository traits — no framework deps
│   ├── application/     # Use cases
│   ├── infrastructure/  # sqlx/Postgres adapters
│   └── web/              # Axum HTTP adapters
├── src/                 # Binary entrypoint, wires the crates together
├── docs/                # Documentation — organized by domain × type, see docs/README.md
└── .github/              # GitHub configuration
```

---

## Documentation

All project documentation lives in [`docs/`](docs/README.md), organized by domain (what it's about) × type (what shape it takes). Start there for anything beyond this quickstart — architecture, environment variables, coding conventions, CI/CD, decisions, and more.

---

## Contributing

This is a personal, solo-maintained project, but issues and PRs are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

---

## License

Distributed under the [MIT License](LICENSE).
