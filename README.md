# TimeSheet Tracker

A Windows desktop application that automatically tracks time spent on projects based on window focus. Built with PyQt5.

## Quick Installation

### From Network Share
1. Navigate to `\\YOUR_NETWORK_SHARE\TimeSheet`
2. Double-click `network_install.bat`
3. Wait for installation to complete
4. Launch TimeSheet from your desktop or start menu

### From GitHub Release
1. Go to [Releases](https://github.com/kyle93afc/MalceodJordantimesheet/releases/latest)
2. Download `TimeTracker.exe`
3. Place it in `%LOCALAPPDATA%\TimeSheet\`
4. Run the executable

## Location of Files
- Program files: `%LOCALAPPDATA%\TimeSheet`
- User data: `%LOCALAPPDATA%\TimeSheet\data`
- Update log: `%LOCALAPPDATA%\TimeSheet\update.log`

## Updates

The application automatically checks for updates on startup by querying GitHub releases. When a new version is available, you'll see a dialog with release notes and the option to update immediately. The update downloads and applies automatically — no manual steps required.

## Versioning

This project uses [Semantic Versioning](https://semver.org/). The current version is defined in `timesheetorg/version.py`.

### Creating a Release

1. Update the version in `timesheetorg/version.py`
2. Commit, tag (`vX.Y.Z`), and push
3. GitHub Actions builds the `.exe` and creates a release automatically

See [RELEASE_INSTRUCTIONS.md](RELEASE_INSTRUCTIONS.md) for full details.

## Development

### Running the Application
```bash
python timesheetorg/timesheet_tracker_New_v9.py
```

### Environment Setup
```bash
pip install -r timesheetorg/requirements.txt
```

### Building the Executable
```bash
cd timesheetorg
pyinstaller timetracker.spec
```

### Dependencies
PyQt5, pynput, pywin32, python-dateutil

## Troubleshooting

If the application won't start:
1. Check if antivirus is blocking the executable
2. Ensure you have write access to `%LOCALAPPDATA%\TimeSheet`
3. Contact IT support if issues persist

If auto-update fails:
1. Check `%LOCALAPPDATA%\TimeSheet\update.log` for details
2. Download the latest release manually from GitHub
3. Replace the exe in `%LOCALAPPDATA%\TimeSheet\`
