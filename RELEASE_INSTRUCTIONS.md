# TimeTracker — Version & Release Instructions

Use this document when handling version bumps and publishing new releases to GitHub.

---

## Overview

- **Project**: TimeTracker (PyQt5 desktop app)
- **Repo**: https://github.com/kyle93afc/MalceodJordantimesheet
- **Update mechanism**: App checks GitHub releases on startup; users get a notification dialog
- **Version format**: Semantic versioning (e.g. `1.2.0`)
- **Build tool**: PyInstaller (single-file `.exe`)

---

## Files That Must Be Updated for a New Version

When releasing version `X.Y.Z`, update **one file**:

### `timesheetorg/version.py`

```python
VERSION = "X.Y.Z"
```

That's it. The window title and update checker read from this constant automatically.

---

## Release Process

### Option A: Automated (Recommended)

The GitHub Actions workflow builds and releases automatically when you push a version tag.

#### Step 1: Update the version

Edit `timesheetorg/version.py`:

```python
VERSION = "X.Y.Z"
```

#### Step 2: Commit and tag

```bash
git add timesheetorg/version.py
git commit -m "Bump version to X.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

#### Step 3: Done

GitHub Actions will:
1. Build `TimeTracker.exe` using PyInstaller
2. Create a GitHub release tagged `vX.Y.Z`
3. Attach the `.exe` as a release asset
4. Auto-generate release notes from commits

You can edit the release notes afterwards via the GitHub web UI or:

```bash
gh release edit vX.Y.Z --notes "Your custom release notes here"
```

### Option B: Manual Release

If GitHub Actions is unavailable or you need to release locally:

#### Step 1: Update version (same as above)

#### Step 2: Build locally

```bash
cd timesheetorg
pyinstaller timetracker.spec
```

#### Step 3: Create GitHub release manually

```bash
gh release create vX.Y.Z timesheetorg/dist/TimeTracker.exe --title "vX.Y.Z" --notes "Release notes here"
```

---

## Checklist for a Release

- [ ] Update `timesheetorg/version.py` with new version
- [ ] Commit the version change
- [ ] Create and push git tag `vX.Y.Z`
- [ ] Verify GitHub Actions build succeeds (or build manually)
- [ ] Verify release appears at https://github.com/kyle93afc/MalceodJordantimesheet/releases
- [ ] Verify `TimeTracker.exe` is attached as a release asset
- [ ] (Optional) Edit release notes on GitHub

---

## How the Update System Works

1. **On startup** (after 5-second delay), the app checks:
   `https://api.github.com/repos/kyle93afc/MalceodJordantimesheet/releases/latest`
2. Compares the release tag version with the built-in `VERSION` constant
3. If a newer version exists, shows an **Update Available** dialog with:
   - New version number
   - Release notes from GitHub
   - "Update Now" / "Later" buttons
4. If the user clicks **Update Now**:
   - Downloads `TimeTracker.exe` from the release assets
   - Shows download progress
   - Creates a helper batch script that:
     - Waits for the app to exit
     - Replaces the running `.exe` with the new one
     - Relaunches the app
     - Deletes itself
5. The app exits and the batch script handles the swap

### Important Notes

- Update checks only work in the **frozen** (PyInstaller) build, not during development
- The GitHub API allows 60 unauthenticated requests/hour — more than enough
- Update logs are written to `%LOCALAPPDATA%\TimeSheet\update.log`
- The asset name in GitHub releases must be `TimeTracker.exe` (matches `ASSET_NAME` in `update_checker.py`)

---

## Notes for AI Assistants

1. **Version consistency**: The version in `version.py` and the git tag must match (e.g. `1.2.0` in code, `v1.2.0` as tag).
2. **Git tag format**: Tags use a `v` prefix: `v1.2.0`, not `1.2.0`.
3. **Single version source**: Only `timesheetorg/version.py` needs updating. Everything else reads from it.
4. **Asset name**: The release must contain an asset named `TimeTracker.exe` for auto-update to find it.
5. **Release notes**: Write meaningful notes — they're shown to users in the update dialog.
6. **Network deployment**: The old network-share deployment (`build_and_prepare.bat`) still works independently of GitHub releases.
