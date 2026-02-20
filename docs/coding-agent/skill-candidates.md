# Skill Candidates (Repo-Local Staging)

- last_updated: 2026-02-20

This file is a staging area for Skill ideas discovered during work.
Runtime updates can only edit repo files, so candidates and drafts live here first.
Later, you (human) can migrate stable drafts into the global Skills library.

Single-writer policy:
- Only Orchestrator edits this file.

Sources of candidates:
- Worker reports: skill_candidates (structured)
- Research output: “Skill Candidate Suggestions” (optional)
- Reviewer output: “Skill Candidate Suggestions” (optional)
- Repeated user requests or repeated failure modes (e.g., validation misses)

---

## Active Candidates

| Name (kebab-case) | Action (create/update) | Status | Trigger (draft) | Rationale | Notes |
|---|---|---|---|---|---|
| post-correction-closure | create | idea | Any explicit user correction in a turn | Standardize mandatory follow-through steps (log lesson, restate behavior change, stage prevention guardrail) to avoid recurrence across correction types | Candidate draft path: docs/coding-agent/skill-drafts/post-correction-closure/ |
|  |  | idea |  |  |  |

Status values:
- idea
- approved-to-draft
- drafted
- migrated (to global skills)
- rejected
- archived

---

## Draft Locations

When a candidate is drafted, create it under:
- docs/coding-agent/skill-drafts/<skill-name>/SKILL.md
Optional:
- docs/coding-agent/skill-drafts/<skill-name>/scripts/
- docs/coding-agent/skill-drafts/<skill-name>/references/
- docs/coding-agent/skill-drafts/<skill-name>/assets/

Add a row here linking the draft location in Notes.

---

## Drafted Skills (Ready to Migrate)

| Name | Draft path | What’s included | Migration notes |
|---|---|---|---|
|  | docs/coding-agent/skill-drafts/<name>/ | SKILL.md (+ optional resources) |  |

---

## Rejected / Archived

| Name | Reason | Date |
|---|---|---|
|  |  |  |
