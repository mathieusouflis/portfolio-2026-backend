---
domain: product-code
type: decision-record
status: accepted
owner: Mathieu (solo maintainer)
supersedes:
---

# Docker on a VPS via Dokploy, infra provisioned with Terraform + Ansible

## Context

This is a low-traffic personal API with a single maintainer — no need for a managed platform's autoscaling or team-oriented features, and no existing cloud account to default into. But hand-running commands on a VPS over SSH doesn't leave a record of what the server looks like, which matters even solo once the setup is more than a few commands old.

Alternatives considered: a managed platform (Fly.io/Railway/Render) — less infra to own, but less control and a recurring cost tied to a vendor rather than a single VPS; hand-configured VPS with no IaC — fastest to start, but undocumented and unreproducible if the VPS is ever rebuilt.

## Decision

Deploy as a Docker container on a VPS, managed through **Dokploy** (self-hosted PaaS — handles container builds/deploys/rollbacks on top of the VPS without hand-writing a full orchestration layer). The underlying VPS and DNS are provisioned as code with **Terraform**; **Ansible** handles server configuration and the Dokploy install/deploy steps that follow provisioning. Terraform owns "what infrastructure exists," Ansible owns "what's configured/running on it."

## Consequences

Easier: the VPS can be destroyed and recreated from `terraform apply` + the Ansible playbooks rather than from memory; the actual deploy step (build → push → release) is Dokploy's job, not a hand-rolled script.

Harder: three tools to learn/maintain (Terraform, Ansible, Dokploy) instead of one managed-platform dashboard; no infra exists yet as of this decision, so [operations/runbooks/deployment-runbook](../../operations/runbooks/deployment-runbook.md) is provisional until it's been run for real — see that file's `last_exercised` field.

## Superseded by

