# Contributing

Thank you for your interest in contributing! This is a solo-maintained personal project, but issues and pull requests from others are welcome. This document explains how to get involved, what to expect, and how changes get merged.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [How to Contribute](#how-to-contribute)
  - [Reporting Bugs](#reporting-bugs)
  - [Suggesting Features](#suggesting-features)
  - [Submitting Code Changes](#submitting-code-changes)
- [Commit Conventions](#commit-conventions)
- [Pull Request Process](#pull-request-process)
- [Code Style](#code-style)
- [Testing](#testing)

---

## Code of Conduct

This project follows the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you agree to uphold these standards. Please report unacceptable behavior to the maintainers.

---

## Getting Started

1. **Fork** the repository on GitHub
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/your-username/portfolio-2026-backend.git
   cd portfolio-2026-backend
   ```
3. **Add the upstream remote:**
   ```bash
   git remote add upstream https://github.com/mathieusouflis/portfolio-2026-backend.git
   ```
4. Follow the full setup guide: [docs/product-code/tutorials/getting-started](docs/product-code/tutorials/getting-started.md)
5. Activate the Git hooks (one-time, recommended — validates commit messages locally before you push): [docs/team-process/how-to/activate-git-hooks](docs/team-process/how-to/activate-git-hooks.md)

---

## How to Contribute

### Reporting Bugs

Before opening an issue:
- Search [existing issues](../../issues) to avoid duplicates
- Make sure you are on the latest version (`git pull upstream main`)

When opening a bug report, use the **Bug Report** template and include:
- Steps to reproduce
- Expected vs actual behavior
- Your environment (OS, runtime version, relevant tool versions)
- Relevant logs or screenshots

### Suggesting Features

Open a **Feature Request** issue with:
- A clear description of the problem the feature solves
- Your proposed solution
- Alternatives you considered

Features that align with the project's scope and architecture are more likely to be accepted.

### Submitting Code Changes

1. Create a branch from `main`:
   ```bash
   git checkout main
   git pull upstream main
   git checkout -b feat/your-feature-name
   ```
2. Make your changes following the [code style](#code-style) guidelines
3. Write or update tests
4. Run the validation suite locally (see [Getting Started](docs/product-code/tutorials/getting-started.md))
5. Commit following [commit conventions](#commit-conventions)
6. Push and open a Pull Request

---

## Commit Conventions

We follow **Conventional Commits**. Each commit message should be:

```
<type>(<scope>): <short description>

[optional body]

[optional footer]
```

**Types:**

| Type       | When to use                           |
|------------|----------------------------------------|
| `feat`     | New feature or behavior               |
| `fix`      | Bug fix                               |
| `refactor` | Code change with no behavior change   |
| `test`     | Adding or updating tests              |
| `docs`     | Documentation only                    |
| `chore`    | Tooling, dependencies, config         |
| `perf`     | Performance improvement               |
| `ci`       | CI/CD changes                         |

**Scope** (optional): the module or area affected, e.g. `auth`, `api`, `ui`, `docker`.

**Examples:**
```
feat(auth): add refresh token revocation on logout
fix(api): return 409 when resource already exists
refactor(core): extract validation to separate utility
test(auth): add unit tests for login use case
docs(setup): add environment variable reference
chore(deps): upgrade dependencies
```

**Rules:**
- Use the **imperative mood** ("add" not "adds" or "added")
- Keep the first line under **72 characters**
- Reference issues in the footer: `Closes #42`, `Fixes #17`

Enforced automatically both locally (if hooks are activated) and in CI — see [docs/operations/concept/ci-cd-pipeline](docs/operations/concept/ci-cd-pipeline.md) for how.

---

## Pull Request Process

1. **One PR per concern** — don't mix unrelated changes
2. **Fill the PR template** — describe what changed and why
3. **Keep diffs small** — large PRs are hard to review; split if needed. Aim for something reviewable in under 30 minutes
4. **All CI checks must pass** before merging
5. **Squash or rebase** before merge if history is messy

This project is solo-maintained — there's no second reviewer, so a green CI run is the merge gate rather than an approving review. External PRs are read and merged (or discussed) by Mathieu directly. See [docs/organizational/reference/tool-inventory](docs/organizational/reference/tool-inventory.md) for the tools involved.

---

## Code Style

See [docs/product-code/reference/code-style](docs/product-code/reference/code-style.md) for detailed conventions.

---

## Testing

Every PR should maintain or improve the existing test coverage.

---

## Questions?

Open an [issue](../../issues) — there's no separate Discussions space for this project.
