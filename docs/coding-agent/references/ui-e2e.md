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
2) console scan:
   - at least error level
3) network scan:
   - record failed requests
   - record unexpected redirects (if applicable)
4) viewports:
   - at least 2 representative breakpoints (e.g., mobile + desktop)

---

## Recommended `playwright-cli` workflow (bounded)

Typical sequence:

1) Start app (if needed) using repo-documented commands (see `how-to-run.md`)
2) Open browser and navigate:
   - `playwright-cli open`
   - `playwright-cli goto http://localhost:<port>/...`
3) Snapshot to find stable refs:
   - `playwright-cli snapshot`
4) Perform actions using snapshot refs:
   - `playwright-cli click e<id>`
   - `playwright-cli fill e<id> "<value>"`
5) Resize for viewports:
   - `playwright-cli resize 390 844`
   - `playwright-cli screenshot --filename=.playwright-cli/<name>.png`
   - `playwright-cli resize 1440 900`
   - `playwright-cli screenshot --filename=.playwright-cli/<name>.png`
6) Collect diagnostics:
   - `playwright-cli console` (or error-only if supported)
   - `playwright-cli network`
7) Close:
   - `playwright-cli close`

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
