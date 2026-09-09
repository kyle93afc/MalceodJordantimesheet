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
