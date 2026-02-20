# Rules (Orchestrator)

- last_updated: 2026-02-20

Repo-specific orchestration conventions that layer on top of global Orchestrator instructions.

---

## 1) Plan lifecycle (repo convention)

- Plans are created in: `docs/coding-agent/plans/active/`
- Plans are execution plans and may evolve during work:
  - update `status`
  - append to `Progress Log`
  - append to `Decision Log` when re-planning
- When finished (and validated), move the plan to: `docs/coding-agent/plans/completed/`

---

## 2) Waves (parallel dispatch semantics)

- A plan MUST include a “Task Waves” section.
- Within a wave, tasks are intended to be dispatched in parallel by default when `owns` are disjoint.
- Waves are executed sequentially.

---

## 3) Documentation gardening (when docs are missing)

If planning or execution reveals missing operational knowledge (how to run, how to validate, UI evidence expectations):
- add/update the relevant doc under `docs/coding-agent/references/`
- update `docs/coding-agent/index.md` or `rules/common.md` reference table if needed

---

## Global Migration Candidates

<!--
Add cross-repo / cross-task rules discovered at runtime here.
These will be migrated into global agent/skills definitions later.
-->

- (none yet)
