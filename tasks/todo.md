# Release-readiness check via Herdr (2026-09-09)

- [x] Inspect implementation status and start independent read-only Fable reviewers (`spec-audit`, `standards-audit`).
- [x] Reproduce material findings using temporary data; verify latest source and tests.
- [x] Collect standards/spec verdicts and report readiness, unresolved defects and real-machine test gaps.

Review: **HOLD release**. 62 tests passed, but 14 isolated evidence checks confirmed defects/policy gaps; 47 untracked day-record-looking files are not ignored (contents not read). Full standards/spec report: `C:\Users\k.greig\AppData\Local\Temp\release-readiness-20260909-123833.md`. Evidence scripts are beside it. Source blob `3d859ad`; no application edits by this review.

- [x] Send the verified blockers and safe implementation instructions to original Claude pane `wF:p1`, as requested by the user.

Handoff acknowledged: Claude is reading the report/evidence and checking untracked records before edits. Instructions preserve both real local/shared data copies, require regression tests and an isolated sandbox build only, and explicitly forbid release/push/tag. Fix completion remains unverified.

Scope: current v2.3 work versus released application `52d9ce5`; review only, no release or application edits by Pi. Original Claude pane's draft command left untouched.

---

# Architecture review (2026-09-09)

- [x] Trace recent hot spots against the domain glossary, ADRs, callers and tests.
- [x] Rank deepening opportunities; propose no concrete interfaces or application edits.
- [x] Write, validate and open a visual HTML report in the OS temp directory.
- [x] Collect Claude Fable 5.1's second opinion through Herdr (`wF:p1`; assessed against ongoing v2.3 work).

## Review
- Report: `C:\Users\k.greig\AppData\Local\Temp\architecture-review-20260909-121711.html` (opened with the Windows default browser).
- Three candidates: complete Tracking Session ownership of today (top recommendation), deepen Day Ledger save guarantees, preserve Project number through Today selection.
- Verification: 55 existing tests passed; six temporary evidence checks confirmed current defects. HTML structure, six diagrams, anchors, vocabulary and source paths checked; visual browser rendering not automatically inspected.
- Report and runnable evidence script remain outside the repo; no application edits or concrete interface proposals. Existing unrelated changes preserved.
- Concurrent v2.2.1 logo/bundling commits arrived during review; reran all tests and evidence checks successfully. Report line references remain pinned to application `a466b69`.
- Fable second opinion: all six defects reproduced before its latest fix; favour small changes within existing modules over a broad editable-day refactor. Multi-machine readers increase save-safety priority. Recommended order: Qt item identity, atomic per-file writes/export failure handling, day-scoped asks, multi-segment idle trim, isolated nested history.
- Fable independently fixed Copy/Merge as part of its ongoing v2.3 work despite the review-only request; it reported 62 tests passing and held the remaining review fixes pending user instruction. This Pi review made no application edits.
- Caveats on the second opinion: local atomic replacement does not guarantee atomic cloud propagation or a coherent two-file pair; expiring pending asks alone cannot invalidate an already-displayed prompt. Keep the report's delayed-answer regression check when implementing day-scoped validation.

---

# Timeline + Ask-when-unsure (v2.2)

## Phase A: keep the evidence (foundation, no UI)
- [x] History segments record `title` and `exe` (process name) alongside project/duration
- [x] Coalesce consecutive same-project ticks into one segment (start, end) so a day is ~50 rows not ~15,000
- [x] Record gaps as segments too: `state = idle | away | sleep`, with start/end
- [x] Stop auto-closing after 120 min idle; keep the app alive and record the gap instead
- [x] Tests: segment coalescing, gap recording, old-format history still loads

## Phase B: day timeline you can correct
- [x] Horizontal bar on the Today tab, one block per segment, colour per project, hover shows title + times
- [x] Click block -> reassign to a project (recent projects list + search); totals recompute from segments
- [x] Reassigning an idle/away block turns it into billable time (this is the site-visit path)
- [x] Tests: reassign updates totals; split/merge of segments

## Phase C: ask when unsure, learn the answer
- [x] Trigger 1: N minutes (default 10) on a window with no project number -> small non-modal toast bottom-right: "What's this for?" + last 5 projects + "Not work"
- [x] Trigger 2: back from an away/sleep gap longer than M minutes (default 30) inside working hours -> "Away 2h 40m. Site visit?" same picker
- [x] Answer creates a rule: title-contains -> project (stored in settings, editable in Projects tab)
- [x] Rules are checked before regex in detect_project
- [x] Tests: rule matching, prompt suppression (once per window per day), gap-prompt thresholds

## Phase D: picker + visual refresh (added after first hands-on test)
- [x] suggest_projects(): number in the block's titles > rule > before/after neighbours > today's projects
- [x] ProjectPicker: likely list + type-to-search over all CMAP aliases; shared by AssignDialog and the toast
- [x] Hero card replaces banner + progress row; tabs no longer clip; gridless table with colour chips
- [x] Timeline: themed track, rounded blocks, now marker; display names use "125103 · Name"

## Later (not now)
- Outlook calendar prefill for away gaps (pywin32 COM)
- Floating always-on-top pill

## Review (2026-09-09)
- Done: Phases A, B, C. 54 tests pass. GUI smoke-tested against a temp data dir; today's real
  1810-tick history upgrades to 58 segments.
- Skipped: drag-to-split blocks (reassign whole block covers it; add if a block regularly spans two jobs).
- Skipped: per-title segment splitting (segments coalesce on project + guessed/direct; titles kept, max 5).
- Behaviour change: app no longer auto-closes after 2h idle; the gap is recorded and offered as a site visit.
- Reassigning the live (last) block also switches what is being tracked now.
- Not committed, not version-bumped.

# v2.3: Not work, Outlook calendar, multi-machine sync

## Not work
- [x] session.exclude(index): block state "excluded", project None, totals reduced; live block => that window stays excluded until it changes
- [x] AssignDialog gets a "Not work" button; toast's Not work calls exclude
- [x] Timeline draws excluded blocks dim; tests

## Outlook calendar
- [x] fetch_outlook_events(day) via win32com, tolerant of Outlook absent; cached 5 min in session
- [x] While a meeting whose subject resolves to a project is running, inferred time goes to it (not guessed)
- [x] Gap that overlaps an event: gap segment title = subject; ask heading names it; suggestion reason "in calendar"
- [x] Tests on the pure parts with fake events

## Sync across machines
- [x] Settings.DATA_ROOT (empty = LOCALAPPDATA); Settings dialog folder picker; main() loads settings before choosing root
- [x] DayLedger writes <name>.<machine>.csv/.json, event log per machine; reads merge every machine's files; legacy unsuffixed files count as this machine and are renamed once
- [x] Today/week tabs show merged totals; timeline shows other machines' blocks (hover names the machine, not clickable)
- [x] Tests: two machines same day merge, legacy rename, no clobber

## Release-readiness fixes (2026-09-09, blockers from the two Fable audits)
- [x] Atomic temp+replace day writes; failed history write leaves previous pair intact
- [x] Export refuses after failed save and when any peer file is unreadable; export name per machine
- [x] Archive touches own files only; log stays in the local folder; legacy adoption only in the local root, shared-root legacy files shown read-only
- [x] Migration: same-day differing local copy preserved as .conflict-<stamp> and surfaced at startup, never skipped or overwritten; never resurrects archived days
- [x] Restart: timer stopped, one final save, stays running on save or launch failure, quits without re-saving
- [x] Asks carry their day; session refuses stale assign/Not work; toast hidden at midnight; modal dialog result checked against its opening day
- [x] Idle trim walks trailing blocks and adjusts each block's own project; excluded blocks lose duration only
- [x] History view deep-copied
- [x] Week tab overlays merged totals; Merge refuses remote-only and names the remote share that stays
- [x] Overlap between machines and unreadable peers shown under the timeline; no deduplication (policy open)
- [x] /data/ ignored; 47 untracked records left in place, unread
- [x] 77 tests; 13/13 reviewer evidence checks flip to fixed; 2.3.0 built and smoke-run in an isolated temp root, Outlook off
- [ ] OPEN with user: overlap policy; calendar attribution scope (any inferred window vs Teams/Outlook only)
- [ ] RELEASE GATES: two real PCs on one OneDrive account (offline, reconnect, overlap, restart mid-setup); real Outlook recurrence/timezone check; live-tracker migration once user decides on today's duplicate files
