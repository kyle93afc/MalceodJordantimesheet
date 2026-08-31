# 03 — Tracking Session: `tick() → Snapshot` with injected clock and title source (expand)

**What to build:** A `TrackingSession` module that owns today's time end-to-end — idle, pause/subtract, resume, project switching, manual entry, merge, notification-once, auto-close-due, sleep gaps — behind one `tick()` call, fully tested with a fake clock and fake titles. The existing `ProjectTracker` and the window are left untouched in this ticket (expand step); ticket 04 migrates the window and deletes the old tracker (contract).

**Blocked by:** 02 — Project Detection.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (section *Tracking Session*) and `CONTEXT.md` (Tracking Session, Snapshot, Idle).

Reference bugs the tests must cover: submodule commits `a6353cc` (double `update_project_time` per tick → duplicate near-zero history entries; `was_locked` OR-clause → phantom time) and `af8c3d6` (banner stuck on Paused after resume).

- [x] `TrackingSession(config, clock=time.time, title_source=<win32 reader>)` with interface: `tick() -> Snapshot`, `activity()`, `add_manual(project, seconds)`, `merge(source, target)`, `restore(totals, history)`, `totals`, `history`, `day`, `start_new_day(date)`
- [x] `Snapshot` frozen dataclass: `project`, `state` in {`tracking`, `paused`, `locked`}, `totals` (copy), `idle_since`, `tracking_since`, `notify_idle`, `auto_close_due`
- [x] Tests (fake clock advanced by hand, fake title source returning a scripted sequence):
  - [x] idle 6 min then `activity()` then tick: no time added during idle, run-up (min of threshold and total) subtracted once, state returns to `tracking`, next tick adds only time since resume
  - [x] two `tick()` calls in the same second add ≤ 1 s total and append at most one history entry per tick with a positive duration
  - [x] title changes → project switches; time before the switch stays on the old project
  - [x] `notify_idle` is True on exactly one tick per idle period
  - [x] `auto_close_due` becomes True after `AUTO_CLOSE_IDLE_MINUTES` of idle
  - [x] a gap of more than twice the idle threshold between ticks (sleep) is treated as idle since last activity: paused, subtracted, `session_resumed`-style event recorded on the next `activity()`
  - [x] `add_manual` and `merge` update `totals` and `history` (`manual: True` flag on manual entries)
  - [x] `restore` then `tick` continues from restored totals; `start_new_day` resets totals and sets `day`
- [x] Session emits events through a callable (`on_event(kind, project, idle_seconds)`) rather than writing files; tests capture them in a list
- [x] Full suite green; one commit on `arch/deepen-modules`
