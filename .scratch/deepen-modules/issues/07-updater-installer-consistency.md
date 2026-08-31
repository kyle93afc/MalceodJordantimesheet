# 07 — Updater / installer consistency and logging hygiene

**What to build:** Build, updater and installer agree on one executable name; an auto-update leaves the installed-version marker truthful; logs are written once, to the data directory.

**Blocked by:** 05 — Day Ledger (log paths use its root). Independent of 06.

**Status:** done

Spec: `.scratch/deepen-modules/spec.md` (section *Updater / installer*).

- [x] Release/build name `TimeTracker.exe` is shared by `timetracker.spec`, `build.bat`, and `update_checker.ASSET_NAME`; the stable installed name remains `TimeSheet.exe` for existing installs
- [x] The updater batch script writes the new version to `version.txt` beside the exe after the swap; `get_installed_version` reflects it — covered by a test that runs the generated script logic or asserts its content
- [x] `UpdateCheckThread` distinguishes "up to date" / "check failed" from "update available" (three outcomes surfaced to the window; UI may just log the first two)
- [x] Module loggers do not propagate to root (no duplicate lines); all log files live under the Day Ledger root, not CWD; tests assert nothing is written to CWD
- [x] `add_to_startup` / installer registry writes share one function and set the `AUTO_START` setting consistently
- [x] Full suite green; one commit on `arch/deepen-modules`
