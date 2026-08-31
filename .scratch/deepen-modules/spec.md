# Spec — Deepen the TimeTracker modules

**Status:** ready-for-agent
**Source:** architecture review 2026-08-31 (candidates #1–#5 + defects found en route)
**Code lives in:** `timesheetorg/` submodule, single file `timesheet_tracker_New_v9.py`. Work on branch `arch/deepen-modules` in the submodule (and the same branch name in the parent repo for CI changes).

## Problem Statement

Three consecutive releases (v1.3.0 → v2.0.1 → v2.0.2) were fixes to idle tracking, and none of the bugs were in the tracking arithmetic — they were in the *order* the window called the tracker. The window reads five private tracker fields and writes one; it also opens the data directory's CSV files itself and mutates the tracker's totals directly. Nothing about tracking, detection or persistence can be tested without constructing the whole Qt window, so the only test in the repo is for CMAP alias import — which was testable only because it bypassed `Config`.

Users see this as: phantom time after idle, a banner stuck on "Paused", idle-pause subtraction and merges silently undone after a same-day restart, non-numeric project names duplicated as "Unknown Project" after reload, and up to five minutes of time lost when clicking "Update Now".

## Solution

Deepen five modules so that each has a small interface and owns its own behaviour, with tests crossing those interfaces and nothing else:

1. **Tracking Session** — one `tick(now, title) → Snapshot` interface with an injected clock and title source; the window becomes a renderer.
2. **Day Ledger** — the only reader/writer of the data directory; week view, export and the event log go behind it.
3. **Project Detection** — a pure `detect(title, previous) → project` function with table-driven tests.
4. **Delete the pass-throughs** — remove modules that fail the deletion test.
5. **Settings** — split app constants from user settings; accept a path.

Behaviour visible to the user stays the same except where the review found defects, which are fixed as part of the module that owns them.

## User Stories

1. As a user, I want time to stop accruing the moment I go idle, so that my timesheet doesn't contain phantom time.
2. As a user, I want the five minutes before idle detection subtracted from the project I was on, so that reading-then-walking-away isn't billed.
3. As a user, I want the banner and status bar to show "Tracking" again immediately after I come back from idle, so that I trust what the tracker is doing.
4. As a user, I want an idle notification once per idle period, not every tick.
5. As a user, I want the app to auto-close after the configured idle period, saving first.
6. As a user, I want moving the mouse or scrolling to count as activity, so that reading a long document or being on a call isn't treated as idle.
7. As a user, I want a laptop sleep/hibernate to be treated as idle from the last input, so that the pre-sleep run-up is subtracted and a resume event is logged.
8. As a user, I want the current project to switch when the foreground window's title shows a different project number.
9. As a user, I want Outlook time attributed to the project I was last working on.
10. As a user, I want Teams time attributed to `Microsoft Teams` or `Microsoft Teams - <channel>`.
11. As a user, I want the deepest project number in a file path (`...\FILES\124774\...`) to win.
12. As a user, I want to add manual time to any project and see it in today's totals and history.
13. As a user, I want to merge one project's time into another and have the merge survive a restart.
14. As a user, I want idle-pause subtractions to survive a restart.
15. As a user, I want project names like `Microsoft Teams - Chat` or `Admin` to reload as themselves, not as `Unknown Project`.
16. As a user, I want restarting the app mid-day to restore today's totals to within a second of what they were.
17. As a user, I want the first save after midnight to land in the new day's files with fresh totals, not yesterday's totals.
18. As a user, I want the week tab to show the same numbers as the day files, with today's column live.
19. As a user, I want to export today's timesheet as CSV with aliases.
20. As a user, I want "Update Now" to save my time before the app exits.
21. As a user, I want alias edits from the Projects tab, the Settings dialog and CMAP import to all land in the same place and be visible immediately without reloading config three times.
22. As a user, I want changing a default in a new release (e.g. window size) to take effect even though I have an existing config file.
23. As a maintainer, I want to write a test for "idle 6 min then resume adds no phantom time" in a handful of lines without a QApplication.
24. As a maintainer, I want the project-detection rules expressed as a table of `(title, previous, expected)` cases that doubles as the spec.
25. As a maintainer, I want the on-disk format known to exactly one module.
26. As a maintainer, I want the tests to run in CI on every push.
27. As a maintainer, I want tests to leave no log files or data files in the working directory.
28. As a maintainer, I want the tracker to stop writing every foreground window title to a DEBUG log every two seconds.

## Implementation Decisions

Vocabulary: see `CONTEXT.md` at the repo root (Project number, Alias, Project Detection, Tracking Session, Snapshot, Idle, Day Ledger, Settings, Manual entry, Merge). Architecture vocabulary: module, interface, seam, adapter, depth, leverage, locality.

**General**
- Everything stays in the single application file unless a module is pure and test-heavy (Project Detection may become its own small module file if that reads better); the entry point, build spec and PyInstaller layout do not change.
- Standard library `unittest` only. Tests live in `tests/` and run with `python -m unittest discover -s tests` from the submodule root. No new dependencies.
- Import-time side effects that write files (root logging to `timetracker.log`, logo copy) move into `main()` so importing the module in a test is clean.
- Version is not bumped and nothing is tagged; releasing is a separate decision.
- One commit per ticket on `arch/deepen-modules`.

**Delete the pass-throughs (ticket 01)**
- `ProjectState` is deleted. The tracker keeps the current project as a name and a start time.
- `get_project_summary`, `ProjectState.format_time`, `FileManager.get_latest_file`, `Config.get_alias`, the unused `Config` fields, the dead `"17749"` branch in `normalize_project_number`, and the no-op `tracker.config = config` assignments are deleted.
- "Total time" for a project has exactly one meaning: today's seconds in the totals map.

**Project Detection (ticket 02)**
- A module-level pure function `detect_project(title, previous) -> str` replaces `ProjectTracker.extract_project_number`. `previous` is the last detected project number (or `None`); the caller stores what comes back. `normalize_project_number` folds into the same module.
- Existing rules are preserved exactly (Outlook → previous, Teams split on `|`, `FILES` path, deepest path number, the ordered regex list, fallback to previous or `Microsoft Teams`) — the tests pin them before any change. The `"17749" in title` substring rule is kept but must be a whole-number match, not a substring match.
- No logging inside detection.

**Tracking Session (tickets 03–04)**
- `TrackingSession(config, clock, title_source)`; `clock` defaults to `time.time`, `title_source` defaults to the win32 foreground-title reader. Two adapters for each (real / fake in tests) justify the seam.
- Interface: `tick() -> Snapshot`, `activity()`, `add_manual(project, seconds)`, `merge(source, target)`, `restore(totals, history)`, `totals` and `history` (read-only views), `day` (the date the totals belong to), `start_new_day(date)`.
- `Snapshot` is a frozen dataclass: `project`, `state` (`tracking` / `paused` / `locked`), `totals`, `idle_since`, `tracking_since`, `notify_idle` (True on exactly one tick per idle period), `auto_close_due`.
- `tick()` does, in order, internally: read clock; read title; detect project (via Project Detection); switch if changed; compute elapsed since last tick; if not idle add elapsed to the current project and append history; if just became idle pause and subtract the run-up; if the gap since the last tick exceeds twice the idle threshold (sleep/hibernate) treat it as idle since last activity: pause, subtract, log. The window never calls anything but `tick()` and `activity()` on the timer/signal path.
- `activity()` is the only way out of `paused`. Idle detection is no input for `IDLE_THRESHOLD`; input includes mouse movement and scroll as well as clicks and keys.
- The old `ProjectTracker` is deleted once the window is migrated (expand in 03, contract in 04).
- Ticket 04 also routes "Update Now" through the normal quit path so a save happens first.

**Day Ledger (ticket 05)**
- `DayLedger(root, clock)`; `root` is the data directory (real `%LOCALAPPDATA%\TimeSheet\timesheet_data` in production, a temp dir in tests).
- Interface: `save_day(date, totals, history)`, `load_day(date) -> (totals, history)`, `load_week(monday) -> {project: [7 hours]}`, `export_day(date, aliases) -> Path`, `log_event(kind, project, idle_seconds)`.
- The per-day CSV gains a seconds column; `load_day` prefers it and falls back to hours for old files. Totals come from the CSV only; the history JSON is an audit trail and is never used to reconstruct totals. This fixes idle-pause subtractions and merges being undone on restart, the `Unknown Project` duplication, and the 36-second rounding inflation.
- `save_day` writes to the file for the date it is given, never "today's file"; rollover is handled by the caller saving the session's `day` then starting a new day. Backup, archive after 30 days and event-log rotation stay inside the ledger.
- The window never opens a file. The week tab and export call the ledger. The event log has one owner.
- Autosave cadence stays 300 s; README and CLAUDE.md are corrected to say so.

**Settings (ticket 06)**
- `Settings.load(path)` / `save()`; only user-settable fields are persisted, and unknown keys are ignored on load. App constants become module-level constants.
- `PROJECT_ALIASES` is the single mutable alias map; the Projects tab, the Settings dialog and CMAP import all mutate it and call `save()`. CMAP import no longer does its own JSON read-modify-write; its existing test keeps passing through `Settings` with a path argument.
- `Settings` is loaded once per process.

**Updater / installer (ticket 07)**
- Release asset / build name (`TimeTracker.exe`) and installed file name (`TimeSheet.exe`) are deliberately different — see `docs/adr/0001-installed-executable-name.md`. Build, CI and updater share one constant for the asset; the installer keeps its own for the installed file.
- The updater batch script writes `version.txt` after swapping the exe so the installer's "already installed" check stays truthful.
- Loggers stop propagating duplicates; log paths are anchored to the data directory, not CWD.

## Testing Decisions

- A good test crosses a module's interface and asserts on returned values or on files in a temp directory — never on private fields, never through Qt.
- Tested modules: Project Detection (table-driven), Tracking Session (fake clock + fake title source), Day Ledger (temp root, round-trips, week view, export, old-format fallback), Settings (temp path), CMAP import (existing test, re-pointed at Settings).
- The Tracking Session tests must include: idle 6 min then resume adds no phantom time; two ticks in the same second don't double count; switching projects moves subsequent time; pause subtracts min(threshold, total); `notify_idle` fires once per idle period; auto-close flag after the configured period; sleep gap handled; manual add and merge reflected in totals and history.
- Prior art: `tests/test_cmap_alias_import.py` — unittest, temp directories, explicit path arguments.
- Tests must not write into the working directory (assert this in a test if convenient).
- CI: the parent repo's release workflow gets a test step (Windows runner already used) that runs the suite before building.

## Out of Scope

- Any GUI redesign; Qt widgets and QSS untouched except where the window stops reaching into modules.
- Version bump, tagging, releasing.
- The self-installer's PowerShell shortcut logic beyond the naming/version fixes above.
- Multi-user or network storage.
- Migrating existing on-disk files (old CSVs remain readable via the hours fallback).

## Further Notes

- The parent repo carries `CLAUDE.md`, `CONTEXT.md`, `.scratch/` and the CI workflow; the submodule carries the code. Commit code in the submodule; commit the workflow change in the parent.
- The v2.0.1 diff (`a6353cc`) and v2.0.2 diff (`af8c3d6`) in the submodule are the reference for the two idle bugs the Tracking Session tests must cover.
