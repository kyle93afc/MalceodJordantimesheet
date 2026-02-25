@echo off
setlocal enabledelayedexpansion

:: Set title
title TimeTracker Launcher

:: Set the correct path
cd /d "%~dp0"
cd timesheetorg

:: Check if Python is installed and in PATH
python --version >nul 2>&1
if errorlevel 1 (
    echo Python is not found in PATH
    echo Please install Python and add it to your PATH
    pause
    exit /b 1
)

:: Check if pip is available
pip --version >nul 2>&1
if errorlevel 1 (
    echo pip is not found in PATH
    echo Please ensure pip is installed and in your PATH
    pause
    exit /b 1
)

:: Clean up corrupted packages
echo Cleaning up Python packages...
python -m pip cache purge
python -m pip install --upgrade pip setuptools wheel

:: Remove corrupted distributions if they exist
if exist "%LOCALAPPDATA%\Programs\Python\Python312\Lib\site-packages\~yinstaller" (
    rd /s /q "%LOCALAPPDATA%\Programs\Python\Python312\Lib\site-packages\~yinstaller"
)
if exist "%LOCALAPPDATA%\Programs\Python\Python312\Lib\site-packages\~ympy" (
    rd /s /q "%LOCALAPPDATA%\Programs\Python\Python312\Lib\site-packages\~ympy"
)

:: Install packages with specific versions to avoid conflicts
echo Installing/Updating required packages...
pip install --upgrade PyQt5
pip install --upgrade pynput
pip install --upgrade pywin32
pip install --upgrade python-dateutil

:: Check if the Python file exists
if not exist "timesheet_tracker_New_v9.py" (
    echo Error: timesheet_tracker_New_v9.py not found in %CD%
    echo Current directory contents:
    dir
    pause
    exit /b 1
)

:: Run the TimeTracker
echo Starting TimeTracker...
python timesheet_tracker_New_v9.py

:: If the program exits with an error, pause to show the error
if errorlevel 1 (
    echo TimeTracker exited with an error
    pause
    exit /b 1
)

exit /b 0

pyinstaller TimeSheet.spec 