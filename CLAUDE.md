# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Windows time tracking application with two main components:
1. **Desktop Application** (`timesheetorg/timesheet_tracker_New_v9.py`) - PyQt5-based GUI that automatically tracks time spent on projects based on window focus
2. **Web Viewer** (`timesheet_viewer.py`) - Streamlit-based web interface for viewing and analyzing timesheet data

The application is designed for MacLeod Jordan's workflow and automatically detects project numbers from window titles.

## Architecture

### Core Components
- **Main Application**: `timesheetorg/timesheet_tracker_New_v9.py` - The primary time tracking application using PyQt5
- **Streamlit Viewer**: `timesheet_viewer.py` - Web-based data visualization and analysis tool
- **Data Storage**: `timesheetorg/timesheet_data/` - JSON files for project history and CSV exports
- **Portable Python**: `portable_python/` - Self-contained Python environment for distribution

### Key Features
- Automatic project detection from window titles using regex patterns
- Real-time tracking with 2-second updates and 30-second auto-saves
- Idle detection (5-minute threshold)
- System tray integration
- Data export to Excel/CSV
- Interactive charts and visualizations

## Common Development Commands

### Building the Application
```bash
# Build desktop executable (from root directory)
build.bat

# Build from timesheetorg directory
cd timesheetorg
build.bat
```

### Running the Application
```bash
# Run desktop application directly
python timesheetorg/timesheet_tracker_New_v9.py

# Run Streamlit viewer
launch_streamlit.bat
# or
portable_python/python.exe -m streamlit run timesheet_viewer.py
```

### Environment Setup
```bash
# Install dependencies for desktop app
pip install -r timesheetorg/requirements.txt

# Install dependencies for Streamlit viewer
pip install -r requirements.txt
```

### Development Dependencies
- **Desktop App**: PyQt5, pynput, pywin32, python-dateutil
- **Streamlit App**: streamlit, plotly, pandas

## Data Flow

1. **Desktop App** tracks active windows and detects project numbers
2. **Project data** is stored in `timesheetorg/timesheet_data/` as JSON files
3. **Streamlit viewer** reads the same data files for visualization
4. **Export functionality** creates CSV files in the same data directory

## Important File Locations

- Main application: `timesheetorg/timesheet_tracker_New_v9.py`
- Streamlit viewer: `timesheet_viewer.py`
- Data directory: `timesheetorg/timesheet_data/`
- Icons/assets: `timesheetorg/assets/` and `timesheetorg/MACLEOD_JORDAN_LOGO.*`
- Build specs: `timesheetorg/timetracker.spec`

## Deployment

The application uses PyInstaller to create a single executable that includes:
- Python runtime
- All dependencies
- Application assets (icons, logos)
- Data directory structure

Network deployment is handled via batch scripts that copy the executable and create shortcuts.