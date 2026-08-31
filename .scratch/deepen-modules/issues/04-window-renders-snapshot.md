# 04 — The window renders a Snapshot; delete `ProjectTracker` (contract)

**What to build:** `TimeTrackerWindow` drives tracking with exactly `session.tick()` on the timer and `session.activity()` on the input signal, and paints only what the returned `Snapshot` says. Manual entry, merge, load and save go through the session interface. The old `ProjectTracker` is deleted. "Update Now" saves before exiting. Mouse movement and scroll count as activity.

**Blocked by:** 03 — Tracking Session.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (sections *Tracking Session*, *General*).

- [x] `track_and_update` is: `snap = self.session.tick()`, then render (banner, current-project label, status bar, progress, today tab, tray tooltip) from `snap`, then notification if `snap.notify_idle`, auto-close if `snap.auto_close_due`, autosave on cadence. No branch in the window inspects idle/pause/lock state except through `snap.state`
- [x] The window reads no attribute of the session other than the interface in ticket 03; grep for `_idle_paused`, `was_screen_locked`, `_idle_notified`, `last_activity`, `last_update_time`, `active_project` in the window returns nothing
- [x] `init_tracking` and `load_saved_times` become `session.restore(...)` from the file manager; `merge_project` and `show_manual_entry` call `session.merge` / `session.add_manual`
- [x] pynput listeners include `on_move` and `on_scroll` (throttled to at most one signal emission per second so the Qt queue isn't flooded)
- [x] `_on_update_available` "Update Now" path calls the same quit path as tray Quit / close (save, then exit)
- [x] `_closing` is set on every quit path so no tick runs during shutdown
- [x] `ProjectTracker` class deleted; the session's event callable is wired to the file manager's event log
- [x] Manual smoke run: launch the app, switch between two windows with different project numbers in the title, confirm the label and totals change; leave idle ≥ 5 min is not required — tests cover it
- [x] Full suite green; one commit on `arch/deepen-modules`
