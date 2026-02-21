# Improvement Loop (Deviations → Lessons → Prevention)

This reference defines what the Orchestrator must do when there is a **deviation**:
any moment where reality diverges from expectations and course correction is required.

Goal: reduce repeated mistakes by converting deviations into durable artifacts.

---

## What counts as a deviation (trigger conditions)

Treat any of the following as a deviation event:

### A) Human course correction
- user says “do X going forward”, “prefer Y by default”, “this should be accounted for”
- user corrects process (planning, delegation, validation, reporting, scope)

### B) Plan instability / re-planning
- new information materially changes approach, scope, validation, or risk
- “we assumed X, but it’s actually Y” impacts decisions

### C) Execution failures
- subagent returns `blocked` or `failed`
- reviewer returns `NEEDS_REVISION` or `FAILED`
- required validation fails unexpectedly or cannot be executed

### D) Waivers / skips
- required validation is skipped (only allowed with explicit waiver)
- required evidence cannot be produced as planned

### E) Environment/tooling recovery
- any durable recovery steps not already documented (install issues, ports, file locks, container restarts, etc.)

---

## Step 0 (mandatory): pause active execution

If a deviation appears mid-task:
- stop dispatching Workers
- stop editing files
- complete the checklist below before continuing

This prevents “momentum” from postponing harness improvements.

---

## Core checklist (complete before continuing)

1) Classify the deviation category
Pick one primary category per lesson entry:
- planning
- delegation/subagents
- validation/evidence
- environment/tooling
- review quality
- docs/communication
- other (be specific)

2) Capture atomic lesson entry(ies)
- Update `docs/coding-agent/lessons.md`
- One lesson per failure category
- Each lesson names exactly one primary promotion target

3) Apply prevention immediately (when cheap)
Choose the smallest high-impact action:
- update repo rule files (`docs/coding-agent/rules/*`)
- update reference docs (`docs/coding-agent/references/*`)
- add troubleshooting entry (`docs/coding-agent/troubleshooting/*`)
- stage “Global Migration Candidates” bullets when cross-repo

4) Require subagent “Lesson Candidates” when relevant
- If deviation involved subagents, ensure their outputs include lesson candidates.
- The Orchestrator is the single writer of the lessons log, but subagents must feed candidates.

5) State persistent behavior change
- In the same turn: state what will change going forward,
  OR update the plan Decision Log if mid-execution.

---

## Micro-check: entry atomization (mandatory)

Before finalizing a lesson entry:
- Does it include more than one independent root cause?
  - If yes, split into multiple entries.

Atomic entries make promotion to global rules/skills predictable.
