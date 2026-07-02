# Project Name

<!-- Replace the title above, fill in each section, and delete these comments when done. -->

<!--
One-time repo settings (not controlled by files in this template):
- Enable GitHub Discussions under Settings → General → Features.
  CONTRIBUTING.md, SUPPORT.md, and the issue template config all link
  to Discussions as the place to ask questions; those links 404 until
  this is turned on.
-->

[![CI](https://github.com/your-org/your-repo/actions/workflows/ci.yml/badge.svg)](https://github.com/your-org/your-repo/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

One-paragraph description of what this project does and who it is for.

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

Describe the problem this project solves, its main features, and any key design decisions worth highlighting upfront.

---

## Getting Started

```bash
git clone https://github.com/your-org/your-repo.git
cd your-repo
git config core.hooksPath .githooks   # activate the Git hooks — optional but recommended
cp .env.example .env                   # then fill in the values
```

Full setup guide (prerequisites, dependency install, dev server): [docs/product-code/tutorials/getting-started](docs/product-code/tutorials/getting-started.md).

---

## Architecture

<!-- Once this project has a real shape, replace the pointer below with a one-paragraph summary and keep the link for detail. -->

See [docs/product-code/concept/todo_architecture-overview](docs/product-code/concept/todo_architecture-overview.md) — template placeholder, not yet filled in.

```
.
├── src/           # Source code
├── docs/          # Documentation — organized by domain × type, see docs/README.md
└── .github/       # GitHub configuration
```

---

## Documentation

All project documentation lives in [`docs/`](docs/README.md), organized by domain (what it's about) × type (what shape it takes). Start there for anything beyond this quickstart — architecture, environment variables, coding conventions, CI/CD, decisions, and more.

---

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) before opening a pull request.

---

## License

Distributed under the [MIT License](LICENSE).
