# Persistent terminal cwd mismatches during validation

- Symptom:
  - A required command like `cd sleep-ui && npm run test:unit` fails with `No such file or directory` in an agent run.

- Likely cause:
  - The terminal session is persistent and the current directory is already `sleep-ui`, so repeating `cd sleep-ui` points to a non-existent nested path.

- Confirmations (commands / checks):
  - Run `pwd` to confirm current working directory.
  - Run `ls` to confirm expected project files exist in the active directory.

- Fix:
  - If already in target directory, run command without `cd` (for example: `npm run test:unit`).
  - Or normalize explicitly with an absolute path in one step.

- Prevention (docs/rules/checks):
  - Before running chained `cd <dir> && ...` commands in persistent terminals, verify cwd first.
  - Prefer absolute-path command forms for required validations when commands are run sequentially.
