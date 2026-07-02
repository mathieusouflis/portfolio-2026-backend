---
domain: operations
type: runbook
status: provisional
owner: Mathieu (solo maintainer)
last_exercised:
---

# Deployment runbook

<!--
PROVISIONAL — the tooling is decided (see decisions/0003-deployment-infrastructure)
but no infrastructure has actually been provisioned yet, so the steps below
are the intended procedure, not a verified one. Run it for real, fix whatever
is wrong, then fill in `last_exercised` above. A runbook that's never been
exercised is a hypothesis, not a procedure.
-->

## When to use this runbook

- First-time provisioning of the VPS this API runs on
- Deploying a new version after a merge to `main`
- Rebuilding the VPS from scratch after a failure

## Prerequisites

- SSH access to the target VPS
- Terraform CLI + credentials for the infra provider, and access to the Terraform state
- Ansible installed locally, with the inventory pointed at the provisioned VPS
- Dokploy admin access on the VPS
- `DATABASE_URL` and any other required secrets (see [product-code/reference/environment-variables](../../product-code/reference/environment-variables.md)) available to inject into Dokploy's environment config — never committed to the repo

## Steps

### First-time provisioning

1. `terraform apply` from the infra directory — provisions the VPS and DNS records.
2. Run the Ansible playbook against the new host — installs Docker, Dokploy, and any OS-level dependencies.
3. In the Dokploy UI, create the application, point it at this repo, and set the environment variables from `.env.example` (real values, not the placeholders).
4. Provision Postgres (either a Dokploy-managed Postgres service or a separate managed instance) and set `DATABASE_URL` accordingly.

### Routine deploy

1. Merge to `main` — CI (`ci.yml`) runs lint/test/build.
2. Trigger a Dokploy deploy (manually, or via a webhook once one is set up) — Dokploy builds the Docker image from the repo and releases it.
3. Confirm the new container is healthy (see Verification below) before considering the deploy done.

## Verification

- The API responds on its health/root endpoint with a 2xx status
- `GET /projects` (or the equivalent listing endpoint) returns data, confirming the `DATABASE_URL` connection is live
- Dokploy shows the new deployment as the active release

## Rollback

- Use Dokploy's built-in rollback to redeploy the previous image/release if the new one fails verification
- If the failure is infra-level (not app-level), `terraform plan` to check for drift before re-applying

## Last exercised

Not yet — infrastructure has not been provisioned as of this writing. Update this section (and the frontmatter `last_exercised`/`status` fields) the first time this runbook is actually run.

## See also

- [product-code/decisions/0003-deployment-infrastructure](../../product-code/decisions/0003-deployment-infrastructure.md)
- [operations/concept/ci-cd-pipeline](../concept/ci-cd-pipeline.md)
