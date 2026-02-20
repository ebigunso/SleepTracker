# Agent Harness Map (Repository Entry Point)

This repository is operated with an **agent harness**: a small set of stable entrypoints + a structured knowledge store that keeps agents reliable over time.

This file is intentionally short. It is a map, not a manual.

---

## Quick start (for agents)

Read in this order (stop early if the task is trivial):

1) `AGENTS.md` (this file)
2) `docs/coding-agent/index.md` (knowledge store index / system of record)
3) `docs/coding-agent/rules/index.md` → role-specific rules
4) `docs/coding-agent/design/*` if the task touches architecture/behavior
5) `docs/coding-agent/references/*` for how-to-run, validation, UI/E2E
6) Relevant plan under `docs/coding-agent/plans/active/` (or create one if non-trivial)

---

## Core invariants (high signal)

- **Non-trivial work requires an execution plan** under `docs/coding-agent/plans/active/`.
- Plans use **Task IDs `Task_X`** and include explicit **validation ownership**.
- Plans include **Task Waves**:
  - tasks in the same wave are intended to be **dispatched in parallel** (when `owns` are disjoint)
  - waves are executed **sequentially**
- **Do not claim “done” without evidence** of required validation (commands/manual/e2e/review), or an explicit user waiver.
- UI / flow / layout changes require **browser evidence**:
  - use `playwright-cli` (global skill) for automation and screenshots
  - store artifacts under **`.playwright-cli/`**
- When you discover missing knowledge (how to run / how to validate / architectural constraints), prefer:
  - adding/updating a doc under `docs/coding-agent/`, and/or
  - staging migration candidates under rule placeholders

---

## Where things live

- Knowledge store index: `docs/coding-agent/index.md`
- Repo rules: `docs/coding-agent/rules/`
- Active execution plans: `docs/coding-agent/plans/active/`
- Completed plans: `docs/coding-agent/plans/completed/`
- Tech debt tracker: `docs/coding-agent/plans/tech-debt-tracker.md`
- Architecture + “taste”: `docs/coding-agent/design/`
- How-to-run + validation mapping: `docs/coding-agent/references/`
- Troubleshooting notes (repo-specific): `docs/coding-agent/troubleshooting/`
- Quality rubric: `docs/coding-agent/quality/QUALITY_SCORE.md`
- Lessons log (runtime, repo-local): `docs/coding-agent/lessons.md` (if present)
- Skill candidates/drafts staging (repo-local): `docs/coding-agent/skill-candidates.md`, `docs/coding-agent/skill-drafts/`

---

## Updating this map

If you add a new “primary entrypoint” document that agents should read early, add it here and in:
- `docs/coding-agent/index.md`
- `docs/coding-agent/rules/common.md` → “Repository Reference Documents”
