# 08 — Code-review fixes (Standards + Spec axes, 2026-08-31)

**What to build:** Close the findings from the two-axis review of `85c73f3..HEAD`. Each item is small; land them as one commit (or a few) with a test where a behaviour is involved. No new scope.

**Blocked by:** 07 — Updater / installer consistency.

**Status:** done

## Hard defects

- [x] **Build break:** `timetracker.spec` does `from version import EXECUTABLE_NAME`, but the `pyinstaller` console entry point does not put the spec's directory on `sys.path` — CI's `pyinstaller timetracker.spec` fails with `ModuleNotFoundError`. Insert `SPECPATH` at the front of `sys.path` before the import (PyInstaller injects `SPECPATH` into the spec namespace). Verify by running `pyinstaller timetracker.spec --noconfirm` locally to the analysis stage (or at least `python -c "exec(open('timetracker.spec').read())"` with a stub) and by keeping the existing test.
- [x] **Installer deletes the user's Run key:** `perform_install` now calls `set_startup_registration(add_to_startup, …)` unconditionally; the dialog's checkbox defaults to unticked, so every upgrade removes an existing auto-start. Restore add-only semantics in the installer (`if add_to_startup:`), make `set_startup_registration(True, executable=None)` refuse (return False) rather than writing the string `"None"`, and put the exception text back into the warning shown to the user. Test both.
- [x] **AUTO_START consistency:** when an install with "Launch at startup" ticked succeeds, `main()` sets `settings.AUTO_START = True` and saves, so the Settings dialog agrees with the registry.

## Behaviour corrections

- [x] **Sleep-gap subtraction amount:** on a gap (> 2× idle threshold between ticks) subtract only the run-up that had already elapsed without input *before* the gap — `min(IDLE_THRESHOLD, last_tick − last_activity, total)` — not a flat threshold; the genuine-idle branch keeps its current amount (which equals the same formula). Test: work until 10 s before sleep, gap 1 h, resume → only ≤10 s subtracted.
- [x] **Sleep-gap when `activity()` arrives before the first post-wake `tick()`** (the real hardware order — the wake keypress is queued first): `activity()` must detect `now − last_tick > 2× threshold` while tracking and handle the gap itself — record `idle_since` = the *previous* `last_activity`, emit `idle_pause` and `session_resumed` with the true durations, subtract the run-up per the rule above, and resume tracking in the same call (no "Paused" state left behind, no idle toast after the user is already back). Test both orderings; `session_resumed` duration must be ≈ the gap.
- [x] **Midnight while running:** in `track_and_update`, if `datetime.now().date() != self.session.day`, call `_save_session()` immediately (it saves the old day then starts the new one) so post-midnight ticks land in today's files rather than waiting for the 5-minute autosave.
- [x] **Quit when the save fails:** keep quitting (an app that cannot quit is worse), but when the failure happens on the quit path show a `QMessageBox.warning` with the error text so the user knows time since the last autosave may be lost; autosave failures stay log-only.
- [x] **Test the other pause branch:** a test where the project total exceeds the threshold before idle — exactly `IDLE_THRESHOLD` is subtracted.

## Deletions (standards axis)

- [x] Remove the unreachable `locked` state everywhere (session, Snapshot docstring/contract, window branch); `state` is `tracking` or `paused`. Replace the four `in {"paused", "locked"}` checks accordingly.
- [x] Simplify logging to one file handler: drop the `propagate = False` + `NullHandler` lines in `update_checker.py` and `self_installer.py` and the handler surgery in `configure_logging`; root's file handler under the data root catches everything. Keep the "no duplicate lines" and "nothing written to CWD" tests passing.
- [x] Fold `add_to_startup` / `remove_from_startup` one-liners into `change_auto_start`.
- [x] Delete unused `CSV_DATE_FORMAT`.
- [x] `apply_update` writes `version.txt` only when the running exe lives in the install directory.

## Accepted as-is (record only)

- `self.config` naming for a `Settings` instance in the window/dialogs — cosmetic; not worth the diff.
- `MappingProxyType` on `totals` and the `activity_lock` — harmless.
- `FakeClock` duplicated in two test files; `sys.path.insert` boilerplate — test-only.
- `session.history` copied each tick — negligible at ≤ ~1,200 entries.
- `prune_history` public on the session — needed by the save step.

- [x] Full suite green; commit(s) in `timesheetorg` + parent pointer commit; Status done.
