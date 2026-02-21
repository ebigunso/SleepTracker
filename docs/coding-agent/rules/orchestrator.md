# Rules (Orchestrator)

- last_updated: 2026-02-21

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

## 4) Deviation-driven improvement loop (deviations → lessons → prevention)

A “deviation” is any time work does NOT go as expected and requires course correction, including:

- user correction or “do this differently going forward”
- plan delta (re-plan) due to significant new information
- subagent result: `blocked` / `failed`
- reviewer result: `NEEDS_REVISION` / `FAILED`
- required validation could not be executed, or required validation needed an explicit waiver
- environment/tooling recovery steps that were not already documented (install issues, port conflicts, Windows locks, etc.)
- “assumption mismatch” discoveries (we assumed X, reality is Y) that should change defaults

### Step 0 (mandatory): interrupt execution
When a deviation happens mid-task:
- stop dispatching Workers
- stop editing files
- run the improvement-loop checklist before continuing implementation

### Atomic lesson rule (required)
Write **one lesson per distinct failure category** (planning vs delegation vs validation vs environment vs review quality, etc.).
If a single narrative contains more than one independent root cause, split it.

### Immediate capture requirement (required)
Any deviation that required a course correction should produce lesson capture in the same turn:
- write atomic lesson entry(ies) to `docs/coding-agent/lessons.md`
- apply the smallest prevention update you can immediately:
  - update repo rules/docs, OR
  - add a troubleshooting entry, OR
  - stage a “Global Migration Candidate” bullet if cross-repo

### Subagent lesson candidates (required)
When dispatching subagents, require them to surface “Lesson Candidates” when they encounter deviations.
The Orchestrator remains the single writer of `docs/coding-agent/lessons.md`, but subagents must feed candidates.

Full checklist reference:
- `docs/coding-agent/references/improvement-loop.md`

---

## 5) Plan-first + subagent-first for non-trivial work (complexity-driven)

The decision to plan (and to dispatch subagents) depends on **task complexity**, not the “situation” (PR vs bug vs feature).

### Non-trivial (plan required) if ANY are true
- requires 3+ meaningful steps
- touches multiple files/components or cross-cutting concerns
- involves unknown patterns or unclear conventions
- introduces new dependencies/config/CI implications
- changes behavior (not just mechanical edits)
- impacts UI/user flows/layout correctness (requires evidence)
- requires significant validation beyond a quick sanity check

When in doubt, treat as non-trivial.

### Required non-trivial sequence (minimum)
1) plan or plan delta first (explicit tasks, validation ownership, waves)
2) dispatch Researcher first (unless the plan is purely mechanical and already obvious)
3) execute via Workers in waves
4) run Reviewer gate (and UI evidence if required)
5) capture deviations as lessons + prevention updates

### Complexity heuristics (examples, not special-cases)
- “Multiple unresolved PR comments” is a strong signal of non-trivial complexity.
- “Flaky CI test / environment-only failure” is a deviation event even if code change is small.
- “Validation mapping unclear” is non-trivial until documented or resolved.

---

## Global Migration Candidates

<!--
Add cross-repo / cross-task rules discovered at runtime here.
These will be migrated into global agent/skills definitions later.
-->

- Candidate for global Orchestrator guidance: “Deviation-driven improvement loop (deviations → lessons → prevention)”
- Candidate for global Orchestrator guidance: “Plan-first + subagent-first is complexity-driven (not situation-driven)”
