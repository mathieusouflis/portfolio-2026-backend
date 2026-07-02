---
domain: product-code
type: tutorial
owner: <!-- team/role that owns onboarding for this project -->
last_reviewed:
---

# Getting Started

<!-- Replace this file with your project's actual conventions. The sections below are a starting point. -->

This guide walks a new contributor from zero to a running local environment. It assumes no prior context — for a specific task you already know how to do (e.g. "add an endpoint"), see [product-code/reference](../reference/) instead.

---

## Prerequisites

<!-- List the tools required to run this project and link to their installation pages. -->

| Tool | Version | Install |
|------|---------|---------|
| <!-- e.g. Node --> | <!-- e.g. >= 22.x --> | <!-- link --> |
| <!-- e.g. Docker --> | <!-- e.g. >= 27.x --> | <!-- link --> |

---

## Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/your-org/your-repo.git
   cd your-repo
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

4. **Install dependencies**

   ```bash
   # Replace with your package manager / build tool
   # e.g. npm install / pip install -r requirements.txt / go mod download / mvn install
   ```

5. **Start the development server**

   ```bash
   # Replace with your start command
   # e.g. npm run dev / python manage.py runserver / go run ./cmd/server
   ```

---

## Common Commands

<!-- Replace with your actual commands. -->

```bash
# Start development server
# Run tests
# Run linter
# Build for production
```

---

## Troubleshooting

**Port already in use**

Find and stop the process occupying the port before starting the server.

**Dependencies not installing**

Make sure your runtime version matches the one listed in [Prerequisites](#prerequisites). Delete any lock files and retry a fresh install.

**Tests fail locally but pass in CI (or vice versa)**

Make sure your `.env` matches the values expected by the test suite, and reinstall dependencies from the lockfile to rule out local drift (e.g. `npm ci` / `pip install -r requirements.txt --no-deps` / `go mod tidy`).

**Build fails with errors you didn't introduce**

Pull the latest `main` and reinstall dependencies — a dependency may have been updated since your last build.

For anything else, open a [Discussion](../../../../discussions).

## See also

- [product-code/reference/environment-variables](../reference/environment-variables.md)
- [team-process/how-to/activate-git-hooks](../../team-process/how-to/activate-git-hooks.md)
