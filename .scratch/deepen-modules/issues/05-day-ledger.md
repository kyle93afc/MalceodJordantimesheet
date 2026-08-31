# 05 — Day Ledger owns the data directory

**What to build:** All reading and writing of the data directory happens in one module, `DayLedger`, constructed with a root path. The week tab, export and the session event log go through it. Totals survive a restart exactly (idle-pause subtraction, merges, non-numeric names, no rounding inflation). The first save after midnight lands in the new day's files with fresh totals.

**Blocked by:** 04 — The window renders a Snapshot.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (section *Day Ledger*) and `CONTEXT.md` (Day Ledger).

- [x] `DayLedger(root, clock=time.time)` with interface `save_day(date, totals, history)`, `load_day(date) -> (totals, history)`, `load_week(monday) -> {project: [7 floats of hours]}`, `export_day(date, aliases) -> Path`, `log_event(kind, project, idle_seconds)`; `FileManager` is renamed/replaced by it
- [x] Per-day CSV gains a `Total Time (Seconds)` column; `load_day` prefers it and falls back to the hours column for files without it; a test writes an old-format file by hand and loads it
- [x] `load_day` derives totals from the CSV only; history is loaded as an audit trail and never changes totals — tests: save after an idle-pause subtraction and after a merge, reload, totals unchanged; `Microsoft Teams - Chat` and `Admin` round-trip as themselves; 2000 s round-trips as 2000 s
- [x] `save_day(date, ...)` always targets the file for `date`; backup to `backup/`, archive of files older than 30 days, and event-log rotation are inside the ledger and covered by a test each (use the injected clock for age)
- [x] Window: `refresh_week_tab` calls `load_week` and overlays today's live totals from the Snapshot; `export_to_csv` calls `export_day`; the window contains no `open(`, `csv.`, or `data_dir` path building
- [x] Rollover: on autosave/quit the window calls `save_day(session.day, ...)`; if the current date differs from `session.day` it then calls `session.start_new_day(today)` — a test at the session/ledger seam demonstrates yesterday's totals end up in yesterday's file and today's start at zero
- [x] The week tab's separate `>= 0.1 h` visibility filter is replaced by the same `MIN_TRACKING_MINUTES` rule as the today tab
- [x] README and CLAUDE.md say autosave is every 5 minutes
- [x] Full suite green; one commit on `arch/deepen-modules` (plus a parent commit for the CLAUDE.md line)
