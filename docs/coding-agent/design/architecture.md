# Architecture Overview

This document describes the repository architecture at a level useful for planning and review.

Keep it concrete:
- name modules
- describe boundaries
- define dependency directions

Update this doc when architecture changes or when repeated agent confusion indicates missing clarity.

---

## System overview

- Purpose:
- Primary runtime(s):
- Primary entrypoints (apps/services/CLIs):

---

## Modules and responsibilities

List the main modules/directories and what they own.

Example format:

- `src/<area>/` — <responsibility>
- `docs/` — <responsibility>
- `scripts/` — <responsibility>

---

## Dependency boundaries (rules)

Describe which modules may depend on which, and what is forbidden.

- Allowed dependencies:
- Forbidden dependencies:
- “Layer” direction (if any):

---

## Data contracts

- Key types/interfaces:
- API boundaries:
- Storage boundaries (DB/files/remote services):

---

## Testing boundaries

- Where unit tests live:
- Where integration/E2E tests live:
- What is considered “UI correctness” evidence:

---

## Common change patterns

Link to 1–3 representative examples (files or docs) that show how to extend the system correctly.

- Example 1:
- Example 2:
- Example 3:
