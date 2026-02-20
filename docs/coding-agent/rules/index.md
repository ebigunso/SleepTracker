# Agent Rules Index (Repository)

This directory contains **repo-specific** rules layered on top of global agent/skill definitions.

Primary entrypoint for the harness is the repo root `AGENTS.md`, which links into:
- `docs/coding-agent/index.md` (knowledge store index)
- this rules index

---

## Files

- `common.md`       — shared repo rules (validation, boundaries, reference docs)
- `worker.md`       — worker scope discipline + reporting expectations
- `orchestrator.md` — orchestration rules + plan lifecycle conventions

---

## Read order (recommended)

- Orchestrator:
  1) `AGENTS.md`
  2) `docs/coding-agent/index.md`
  3) `common.md` → `orchestrator.md`

- Worker:
  1) `AGENTS.md` (optional if already in context)
  2) `common.md` → `worker.md`

- Researcher / Reviewer:
  1) `AGENTS.md`
  2) `common.md`
  3) relevant docs under `docs/coding-agent/references/` and active plan

---

## Update policy

- Only the Orchestrator edits repo rule files (single responsibility).
- Workers propose rule candidates via their report contract.
- Rules that are cross-repo and stable should be staged under “Global Migration Candidates” and migrated into global definitions later.

---

## Relationship to other docs

- For repository entrypoints and navigation, see: `docs/coding-agent/index.md`
- For plan lifecycle, see: `docs/coding-agent/plans/README.md`
- For “must read” repo references, see `common.md` → “Repository Reference Documents”
