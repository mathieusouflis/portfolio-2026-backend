---
domain: product-code
type: reference
owner: <!-- team/role that owns configuration -->
last_reviewed:
---

# Environment Variables

Every variable this project reads from its environment, generated from [`.env.example`](../../../.env.example) at the repository root. **This file must stay in sync with `.env.example`** — the reference is only useful if it matches the code, so update both in the same PR whenever a variable is added, renamed, or removed.

| Variable | Required | Default | Description |
|----------|----------|---------|-------------|
| `PORT` | No | `3000` | Port the server listens on — only applies if this project runs a network service; remove otherwise |
| `HOST` | No | `0.0.0.0` | Host/interface the server binds to — only applies if this project runs a network service; remove otherwise |
| `DATABASE_URL` | <!-- fill in --> | — | Connection string for the database |
| `JWT_SECRET` | <!-- fill in --> | `change-me-before-deploying` | Secret used to sign/verify auth tokens — **must** be overridden in every real environment |
| <!-- add rows for project-specific variables under "External services" in `.env.example` --> | | | |

---

## Setup

```bash
cp .env.example .env
```

Then fill in every value above marked `Required: <!-- fill in -->`. Never commit the `.env` file — it is listed in `.gitignore`.

## See also

- [product-code/tutorials/getting-started](../tutorials/getting-started.md)
