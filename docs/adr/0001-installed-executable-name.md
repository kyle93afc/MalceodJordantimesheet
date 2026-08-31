# ADR 0001 — The installed executable stays `TimeSheet.exe`

**Status:** accepted · 2026-08-31

## Context

The release asset built by CI and downloaded by the updater is `TimeTracker.exe` (`version.EXECUTABLE_NAME`). The self-installer copies it to `%LOCALAPPDATA%\TimeSheet\TimeSheet.exe` and points the Run key and Start-menu shortcut at that path. Every existing MacLeod Jordan install has `TimeSheet.exe` on disk.

The 2026-08-31 architecture review flagged the two names as an inconsistency and ticket 07 briefly unified them to `TimeTracker.exe`, which broke `is_already_installed()`, `_kill_running_instances()` and the upgrade path for every existing user.

## Decision

Two names, on purpose:

- `version.EXECUTABLE_NAME = "TimeTracker.exe"` — the **release asset** and PyInstaller output. Used by CI, `build.bat`, `update_checker.ASSET_NAME`.
- `self_installer.INSTALLED_EXE_NAME = "TimeSheet.exe"` — the **installed file**. Used for install target, already-installed detection, process kill, shortcut and Run key.

The updater swaps whatever exe is running in place, so installed users keep `TimeSheet.exe` across updates.

## Consequences

- Do not "fix" the naming difference; a guard test in `tests/test_update_installer.py` pins `INSTALLED_EXE_NAME`.
- Renaming the installed file would need a migration step (detect old name, rename, rewrite Run key + shortcut) — only worth it if there is a user-facing reason.
