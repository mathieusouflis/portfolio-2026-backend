---
domain: product-code
type: concept
status: todo
owner:
last_reviewed:
---

# Architecture Overview

<!--
TODO — this file has no derivable default content: a template has no real
architecture yet. Once this project has a real shape, replace this file's
content and rename it (drop the `todo_` prefix) to `architecture-overview.md`.

A Concept doc answers "why/how does this work", not "how do I use it"
(that's a Task/How-to) and not "what are the exact parameters" (that's a
Reference). Aim for the mental model a new engineer needs before the code
makes sense on its own.
-->

## What to cover here

- The main components/modules and how they talk to each other (a diagram earns its place here if there are 3+ interacting parts)
- The data flow for the system's core use case, end to end
- The key constraints that shaped the design (scale, latency, team size, compliance) — the "why", not just the "what"
- Links to the [Decision Records](../decisions/) that explain individual architectural choices in detail — this file is the map, the decisions are the reasoning

## See also

- [product-code/decisions](../decisions/) — individual architectural choices
- [product-code/reference](../reference/) — concrete API/schema/config lookup
