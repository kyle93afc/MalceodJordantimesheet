# Setup Matt Pocock's Skills

- [x] Explore repository metadata and existing agent configuration.
- [x] Confirm issue tracker and triage label choices with the user.
- [x] Draft the `CLAUDE.md` Agent skills block and `docs/agents/*.md` files.
- [x] Get user approval for the drafts.
- [x] Write and verify the approved configuration.

## Review

Configured local Markdown issues under `.scratch/<feature-slug>/`, the five default triage labels, and single-context domain docs. Added one `## Agent skills` block to `CLAUDE.md` and created the three approved `docs/agents/` files. Verified the block is unique, each file matches its seed template, and `git diff --check` passes.

# Deepen modules (architecture review 2026-08-31)

Spec: `.scratch/deepen-modules/spec.md` · tickets `.scratch/deepen-modules/issues/01–07` · branch `arch/deepen-modules` in both repos.

- [x] 01 Delete pass-throughs, import side effects → `main()`, CI runs tests
- [x] 02 Project Detection pure module + table tests (review fix: only remember numeric previous)
- [x] 03 TrackingSession + Snapshot with fake clock/title tests (expand)
- [x] 04 Window renders Snapshot; ProjectTracker deleted; quit path unified; move/scroll = activity (contract)
- [x] 05 DayLedger owns data dir; seconds column; reload bugs fixed; rollover (review fix: quit survives save failure)
- [x] 06 Settings: user fields only, explicit path, one alias map
- [x] 07 Updater/installer: asset-name constant, version.txt after swap, 3 update outcomes, logs under data root (review fix: installed name stays TimeSheet.exe → ADR 0001)
- [x] Final two-axis code review (Standards / Spec) — findings triaged → ticket 08 (done); PyInstaller build verified from fixed .spec
- [ ] User: decide merge + version bump/release (not done; nothing pushed)

## Review

Implemented by a pi agent (gpt-5.6-sol, high) in a Herdr pane, one ticket at a time, each reviewed by diff + independent test run before the next was sent. Suite: 1 → 39 tests, all stdlib unittest. Main file 2,227 → 2,224 lines but the window no longer opens files or reads tracker privates; `TrackingSession`, `DayLedger`, `detect_project`, `Settings` are each testable through their interface. Nothing pushed; version not bumped.
