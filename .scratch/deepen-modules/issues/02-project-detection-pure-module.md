# 02 — Project Detection as a pure, table-tested module

**What to build:** Which project a window title belongs to is decided by one pure function whose rules are pinned by a table of cases. The tracker calls it and remembers the previous project. Detection no longer logs every title.

**Blocked by:** 01 — Delete the pass-throughs.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (section *Project Detection*).

- [x] Characterisation first: a table-driven test of the *current* `extract_project_number` behaviour — Outlook → previous / fallback, Teams with and without `|`, `FILES` path with several numbers (deepest wins), Explorer-style path, each of the five regex patterns, the `17749` rule, non-matching title → previous, non-matching title with no previous → `Microsoft Teams`, `normalize_project_number` for 4/5/6/7-digit inputs and with spaces/dashes. Green before any refactor.
- [x] `detect_project(title, previous) -> str` is a module-level pure function (no instance, no logging, no mutation); `normalize_project_number` lives beside it; both are the only detection entry points
- [x] The `17749` rule matches the whole number, not a substring (`117749` and `177490` must not match it) — the one deliberate behaviour change, covered by a test
- [x] The tracker, the manual-entry normalisation and the file loader call the new functions; `extract_project_number` and the static method on the tracker are gone
- [x] Per-tick DEBUG logging of window titles removed
- [x] Full suite green; one commit on `arch/deepen-modules`
