# Lessons

- When extracting stateful logic into a pure function, characterise sequences and state-update policy as well as independent outputs; a detected display bucket is not necessarily the last real project number.
- Persistence failures on shutdown must be logged without blocking the quit path; test the failure path, not only successful round trips.
- Keep release artifact names separate from stable installed filenames; trace upgrades through disk paths, running processes, shortcuts, and Run keys before unifying identity constants.
- When deleting a whole method with a regex, anchor the end to the next `def` (lookahead), never to the first blank line; a blank line inside the body leaves an orphaned half-method that still parses.
- Store segment timestamps as epoch floats, not ISO strings: round-tripping ISO through `fromisoformat().timestamp()` raises OSError on Windows for pre-1970 test clocks and costs a parse per comparison.
- Adding a Qt module (QtSvg etc.) needs three things: the import, `hiddenimports` in timetracker.spec, and removing it from the spec's `excludes`. Verify with `CArchiveReader` that the `.pyd` is in the exe, then run the exe from a sandbox `LOCALAPPDATA\TimeSheet\` so the install dialog is skipped and the main window renders.
- Never let an optional-asset code path swallow the fallback: catch per attempt, not around the whole function.
- In pytest, hold the QApplication (and any TimeTrackerWindow) in a module-level list. If the only reference is a local, Qt aborts the whole process with "Must construct a QApplication before a QWidget" (exit 0xC0000409) and pytest prints no summary.
- Reviewer evidence scripts assert defects. Re-run them after fixing and expect every assertion to fail; write the desired-behaviour test separately.
- PyInstaller 6 onefile hands the unpack folder to children via _PYI_APPLICATION_HOME_DIR / _PYI_ARCHIVE_FILE / _PYI_PARENT_PROCESS_LEVEL (older: _MEIPASS2). Any self-relaunch must strip all of them from the child's environment, or the child shares the parent's temp folder and loses it when the parent exits. Reproduce with psutil: read the python child's environ, launch a second copy with those vars, compare the "Loaded QSS from" _MEI paths.
