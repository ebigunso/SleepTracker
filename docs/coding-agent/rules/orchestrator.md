# Rules (Orchestrator)

- last_updated: 2026-03-10

Repo-specific orchestration conventions that layer on top of global Orchestrator instructions.

---

## 1) Repo policy

- Plans live in `docs/coding-agent/plans/active/` while in progress and move to `docs/coding-agent/plans/completed/` after validation closes.
- Keep repo plans explicit about `Task Waves`; disjoint `owns` should run in parallel by default, with waves executed sequentially.
- If work reveals missing repository operational knowledge, update `docs/coding-agent/references/` and refresh the pointer table in `rules/common.md` or `docs/coding-agent/index.md`.

---

## 2) Routing to long-term homes

- Use `improvement-loop` for deviation handling, lesson capture thresholds, and same-turn persistent-default reporting; keep only repo-specific lesson destinations or overrides here.
- Use `git-workflow` for branch-safety gates, commit chunking, and shared-state Git mutation procedure; keep only repo-specific branch or release policy in local rules.
- Use `skills-maintenance` when changing first-party skill definitions or deciding whether governance belongs in a skill, `references/*`, or repo rules.

---

## 3) Repo-specific ownership

- The Orchestrator remains the single writer for `docs/coding-agent/lessons.md` and `docs/coding-agent/rules/*.md`.
- Worker lesson or rule suggestions should be normalized into repo rules only when they stay repository-specific; cross-repo procedure should move to the owning first-party skill instead.

---

## Global Migration Candidates

<!--
Add cross-repo / cross-task rules discovered at runtime here.
These will be migrated into global agent/skills definitions later.
-->

- Finalized (promoted globally): “Deviation-driven improvement loop (deviations → lessons → prevention)”
- Finalized (promoted globally): “Plan-first + subagent-first is complexity-driven (not situation-driven)”
- Finalized (promoted globally): “Required-evidence completeness check is fail-fast and blocks done state until evidence exists or is explicitly waived.”
