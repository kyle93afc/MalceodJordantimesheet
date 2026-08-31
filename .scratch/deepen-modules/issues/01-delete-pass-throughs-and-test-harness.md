# 01 — Delete the pass-throughs; make the module importable cleanly; run tests in CI

**What to build:** The tracker's public surface shrinks to what is actually used, "total time" gets one meaning, and `python -m unittest discover -s tests` runs clean (no stray log files) locally and in CI. The app behaves exactly as before.

**Blocked by:** None — can start immediately.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (sections *Delete the pass-throughs*, *General*, *Testing Decisions*).

- [x] `ProjectState` deleted; the tracker holds the current project as a name plus a start timestamp; every former reader of `active_project.name` / `.start_time` / `.total_time` updated; `was_locked`, `is_active`, `last_update` gone
- [x] `get_project_summary`, `ProjectState.format_time`, `FileManager.get_latest_file`, `Config.get_alias`, unused `Config` fields (`CSV_DATETIME_FORMAT`, `LOGO_FILENAME`, `ICON_FILENAME`), the dead `"17749"` branch in `normalize_project_number`, and both no-op `self.tracker.config = self.config` assignments removed; callers that used the summary read the totals map directly
- [x] Import-time side effects that write files (root `logging.basicConfig(filename=...)`, logo copy) moved into `main()`; `import timesheet_tracker_New_v9` in a fresh directory creates no files
- [x] A smoke test that imports the module and constructs `Config()` and a tracker with the current-project fields, asserting the working directory is untouched
- [x] `tests/` passes locally with `python -m unittest discover -s tests` from `timesheetorg/`
- [x] Parent repo `.github/workflows/release.yml` runs that command (with deps installed) before the build step
- [x] App launches (`python timesheet_tracker_New_v9.py` starts without traceback; close it after a few seconds) — verify by running it briefly and checking the exit/log
- [x] One commit on `arch/deepen-modules` in the submodule; one commit on `arch/deepen-modules` in the parent for the workflow
