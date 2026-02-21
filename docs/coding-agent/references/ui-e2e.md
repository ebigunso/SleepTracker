# UI / E2E Evidence (playwright-cli)

This repo uses `playwright-cli` (global skill) for targeted E2E and visual evidence collection.

This doc standardizes:
- what evidence to collect
- where artifacts go
- how to keep runs bounded and reproducible

---

## Artifact rules

- All browser artifacts must live under: `.playwright-cli/`
- Use descriptive filenames:
  - `<flow>__<state>__<viewport>.png`
  - Example: `login__after-submit__mobile.png`
- Do not commit artifacts unless the repo explicitly chooses to (recommended: gitignore `.playwright-cli/`).

---

## When UI evidence is required

Collect E2E/visual evidence when any of these are true:
- acceptance criteria include UI behavior, navigation flows, or layout correctness
- changes touch frontend/UI routes/forms/auth or UX-critical screens
- a regression was reported in UI behavior
- the plan explicitly requires E2E/visual evidence

---

## Evidence checklist (minimum)

For each required flow:
1) screenshot(s) at key states (before/after actions)
2) console scan (at least error level)
3) network scan:
   - record failed requests
   - record unexpected redirects (if applicable)
4) viewports:
   - at least 2 representative breakpoints (e.g., mobile + desktop)

---

## Recommended `playwright-cli` workflow (bounded)

Typical sequence:
- start app (if needed) using repo-documented commands (see `how-to-run.md`)
- open browser and navigate
- snapshot to find stable refs
- perform actions using snapshot refs
- resize for viewports and capture screenshots under `.playwright-cli/`
- collect console/network signals
- close and stop

Keep runs minimal: collect required evidence, then stop.

---

## Reporting evidence

When reporting results (review output or task report), include:
- base_url and readiness result
- flows executed and whether each passed
- viewports tested
- screenshots captured (paths under `.playwright-cli/`)
- console errors/warnings (if any)
- failed network requests/unexpected redirects (if any)

---

## When E2E goes sideways (treat as a deviation)

E2E failures are one example of a broader class of “deviations”:
unexpected outcomes that require course correction.

Rules:
- handle it via the improvement loop (pause → lessons → prevention)
- capture the recovery steps if they are repeatable:
  - lessons entry: `docs/coding-agent/lessons.md`
  - troubleshooting entry (if durable): `docs/coding-agent/troubleshooting/`

See: `docs/coding-agent/references/improvement-loop.md`
