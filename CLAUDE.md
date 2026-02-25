# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Workflow Orchestration

### 1. Plan Mode Default
- Enter plan mode for ANY non-trivial task (3+ steps or architectural decisions)
- If something goes sideways, STOP and re-plan immediately – don't keep pushing
- Use plan mode for verification steps, not just building
- Write detailed specs upfront to reduce ambiguity

### 2. Subagent Strategy
- Use subagents liberally to keep main context window clean
- Offload research, exploration, and parallel analysis to subagents
- For complex problems, throw more compute at it via subagents
- One task per subagent for focused execution

### 3. Self-Improvement Loop
- After ANY correction from the user: update `tasks/lessons.md` with the pattern
- Write rules for yourself that prevent the same mistake
- Ruthlessly iterate on these lessons until mistake rate drops
- Review lessons at session start for relevant project

### 4. Verification Before Done
- Never mark a task complete without proving it works
- Diff behavior between main and your changes when relevant
- Ask yourself: "Would a staff engineer approve this?"
- Run tests, check logs, demonstrate correctness

### 5. Demand Elegance (Balanced)
- For non-trivial changes: pause and ask "is there a more elegant way?"
- If a fix feels hacky: "Knowing everything I know now, implement the elegant solution"
- Skip this for simple, obvious fixes – don't over-engineer
- Challenge your own work before presenting it

### 6. Autonomous Bug Fixing
- When given a bug report: just fix it. Don't ask for hand-holding
- Point at logs, errors, failing tests – then resolve them
- Zero context switching required from the user
- Go fix failing CI tests without being told how

### Task Management

1. **Plan First**: Write plan to `tasks/todo.md` with checkable items
2. **Verify Plan**: Check in before starting implementation
3. **Track Progress**: Mark items complete as you go
4. **Explain Changes**: High-level summary at each step
5. **Document Results**: Add review section to `tasks/todo.md`
6. **Capture Lessons**: Update `tasks/lessons.md` after corrections

### Core Principles

- **Simplicity First**: Make every change as simple as possible. Impact minimal code.
- **No Laziness**: Find root causes. No temporary fixes. Senior developer standards.
- **Minimal Impact**: Changes should only touch what's necessary. Avoid introducing bugs.

---

## Project Overview

This is a Windows time tracking application for MacLeod Jordan. It automatically tracks time spent on projects based on window focus and detects project numbers from window titles.

**Desktop Application**: `timesheetorg/timesheet_tracker_New_v9.py` - PyQt5-based GUI

## Architecture

### Core Components
- **Main Application**: `timesheetorg/timesheet_tracker_New_v9.py` - The primary time tracking application using PyQt5
- **Data Storage**: `timesheetorg/timesheet_data/` - JSON files for project history and CSV exports
- **Update Checker**: `timesheetorg/update_checker.py` - Handles version checking

### Key Features
- Automatic project detection from window titles using regex patterns
- Real-time tracking with 2-second updates and 30-second auto-save
- Idle detection (5-minute threshold)
- System tray integration
- Week-by-week navigation with arrow buttons
- Data export to Excel/CSV

## Common Development Commands

### Building the Application
```bash
cd timesheetorg
build.bat
```

### Running the Application
```bash
python timesheetorg/timesheet_tracker_New_v9.py
```

### Environment Setup
```bash
pip install -r timesheetorg/requirements.txt
```

### Dependencies
PyQt5, pynput, pywin32, python-dateutil

## Data Flow

1. **Desktop App** tracks active windows and detects project numbers
2. **Project data** is stored in `timesheetorg/timesheet_data/` as JSON files
3. **CSV exports** are created in the same data directory for weekly views

## Important File Locations

- Main application: `timesheetorg/timesheet_tracker_New_v9.py`
- Data directory: `timesheetorg/timesheet_data/`
- Icons/assets: `timesheetorg/assets/`
- Build spec: `timesheetorg/timetracker.spec`

## Deployment

The application uses PyInstaller to create a single executable that includes:
- Python runtime
- All dependencies
- Application assets (icons, logos)
- Data directory structure

Network deployment is handled via batch scripts that copy the executable and create shortcuts.
