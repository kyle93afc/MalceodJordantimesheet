# TimeTracker — domain glossary

Single-context repo. The application lives in the `timesheetorg/` submodule.

| Term | Meaning |
|---|---|
| **Project number** | A 5–6 digit MacLeod Jordan job number, stored with a trailing dash (`124774-`). Non-numeric buckets also exist: `Microsoft Teams`, `Microsoft Teams - <channel>`, `Unknown Project`, and free-text manual-entry names. |
| **Alias** | A human-readable name for a project number (`124774-` → `Riverside Bridge`). Stored in Settings; imported from CMAP exports. |
| **Project Detection** | The pure rules that map a foreground window title (plus the previously detected project) to a project number. |
| **Tracking Session** | The module that owns today's time: current project, idle state, pause/resume, manual additions, merges. Driven by `tick(now, title)`; returns a **Snapshot**. |
| **Snapshot** | An immutable read-only view of the Tracking Session after a tick: current project, state (`tracking` / `paused`), per-project totals, idle-since, and whether auto-close is due. The window only ever paints a Snapshot. |
| **Idle** | No user input for `IDLE_THRESHOLD` seconds (default 5 min). On entering idle the session **pauses** and subtracts the idle run-up from the current project. |
| **Day Ledger** | The module that owns the on-disk data directory: per-day totals + history files, backups, archive, week view, export, session event log. The only code that reads or writes the data directory. |
| **Settings** | User-editable configuration persisted to JSON (idle threshold, daily target, aliases, theme…). Distinct from app constants (window size, logo file names), which are never persisted. |
| **Manual entry** | Time added by the user through the dialog rather than detected; flagged `manual` in history. |
| **Merge** | Moving all of one project's time for the day into another project. |
