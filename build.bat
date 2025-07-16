@echo off
echo Building TimeSheet Application...

:: Activate virtual environment
call venv\Scripts\activate.bat

:: Install PyInstaller if not already installed
pip install pyinstaller

:: Create assets directory if it doesn't exist
if not exist "timesheetorg\assets" mkdir "timesheetorg\assets"

:: Copy icon to assets
copy "timesheetorg\MACLEOD_JORDAN_LOGO.ico" "timesheetorg\assets\icon.ico"

:: Build single executable
pyinstaller --noconfirm --onefile --windowed --icon=timesheetorg\MACLEOD_JORDAN_LOGO.ico --add-data "timesheetorg/assets;assets/" timesheetorg\timesheet_tracker_New_v9.py --name TimeSheet

echo Build complete! Executable is in the dist folder.
pause 