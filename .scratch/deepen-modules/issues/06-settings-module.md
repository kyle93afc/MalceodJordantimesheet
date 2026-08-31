# 06 — Settings: user settings only, one path, one alias map

**What to build:** User settings live in a `Settings` module loaded once from an explicit path; app constants are module constants and never persisted; aliases have one mutable map that the Projects tab, the Settings dialog and CMAP import all edit through the same interface.

**Blocked by:** 05 — Day Ledger.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (section *Settings*).

- [x] `Settings` dataclass holds only user-settable fields (idle threshold, update interval if user-facing, daily target, min tracking minutes, auto-start, notifications, auto-close minutes, theme, last seen version, aliases); `Settings.load(path)` ignores unknown keys, `save()` writes back to the same path
- [x] Window geometry, logo/icon names, date formats, app name, config file name become module-level constants; a test loads an old config JSON containing those keys and asserts they are ignored and not re-written on save
- [x] `Settings.load` is called once in `main()` and the instance is passed to the window; the two other load sites are gone
- [x] `import_cmap_project_aliases` takes a `Settings` (or its path) and mutates `aliases` then `save()`; the existing CMAP test passes with a temp path and no longer inspects raw JSON except to assert the saved file
- [x] Projects tab alias edit, Settings dialog and CMAP import all result in one `save()` and the in-memory map is what the window renders — no reload
- [x] Full suite green; one commit on `arch/deepen-modules`
