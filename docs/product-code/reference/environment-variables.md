---
domain: product-code
type: reference
owner: Mathieu (solo maintainer)
last_reviewed: 2026-07-02
---

# Environment Variables

Every variable this project reads from its environment, generated from [`.env.example`](../../../.env.example) at the repository root. **This file must stay in sync with `.env.example`** — the reference is only useful if it matches the code, so update both in the same PR whenever a variable is added, renamed, or removed.

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `PORT` | No | `3000` | Port the Axum server listens on |
| `HOST` | No | `0.0.0.0` | Host/interface the server binds to |
| `DATABASE_URL` | Yes | — | Postgres connection string, e.g. `postgres://user:pass@localhost:5432/portfolio` |
| `JWT_SECRET` | Not currently used | — | Reserved for future auth — this API is currently public-read-only with no auth surface. Remove from `.env.example` or leave commented until an auth-requiring endpoint exists |

---

## Setup

```bash
cp .env.example .env
```

Then fill in every value marked `Required: Yes` above. Never commit the `.env` file — it is listed in `.gitignore`.

## See also

- [product-code/tutorials/getting-started](../tutorials/getting-started.md)
