@echo off
setlocal enabledelayedexpansion

:: Terminate any running instances of the application
echo Terminating any running TimeSheet/TimeTracker instances...
taskkill /F /IM TimeSheet.exe 2>nul
taskkill /F /IM TimeTracker.exe 2>nul
if %ERRORLEVEL% EQU 0 (
    echo Successfully terminated application processes.
    timeout /t 2 >nul
) else (
    echo No application processes were running.
)

:: Set installation directories
set "INSTALL_DIR=%LOCALAPPDATA%\TimeSheet"
set "DESKTOP_SHORTCUT=%USERPROFILE%\Desktop\TimeSheet.lnk"
set "START_MENU_DIR=%APPDATA%\Microsoft\Windows\Start Menu\Programs\TimeSheet"
set "START_MENU_SHORTCUT=%START_MENU_DIR%\TimeSheet.lnk"

:: Create directories if they don't exist
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"
if not exist "%START_MENU_DIR%" mkdir "%START_MENU_DIR%"

:: Check if Python is already installed
echo Checking for Python installation...
python --version >nul 2>&1
if %errorlevel% equ 0 (
    echo Python found on system. Will use system Python when possible.
    echo 1> "%INSTALL_DIR%\use_system_python.flag"
) else (
    echo Python not found on system. Will use portable Python.
    if exist "%INSTALL_DIR%\use_system_python.flag" del "%INSTALL_DIR%\use_system_python.flag"
)

:: Copy files from network location with validation
echo Installing TimeSheet...
echo Copying main executable...
xcopy /y "%~dp0TimeSheet.exe" "%INSTALL_DIR%\" || (
    echo Failed to copy TimeSheet.exe
    goto :error
)

echo Copying assets...
xcopy /y /s /i "%~dp0assets" "%INSTALL_DIR%\assets\" || (
    echo Failed to copy assets
    goto :error
)

echo Copying viewer script...
xcopy /y "%~dp0timesheet_viewer.py" "%INSTALL_DIR%\" || (
    echo Failed to copy timesheet_viewer.py
    goto :error
)

echo Extracting portable Python environment...
echo This may take a few minutes. Please wait...

:: Create portable_python directory if it doesn't exist
if not exist "%INSTALL_DIR%\portable_python" mkdir "%INSTALL_DIR%\portable_python"

:: Copy the tar file first
echo Copying Python archive...
xcopy /y "%~dp0portable_python.tar" "%INSTALL_DIR%\" || (
    echo Failed to copy portable_python.tar
    goto :error
)

:: Extract the tar file
echo Extracting Python environment (this is much faster than copying individual files)...
tar -xf "%INSTALL_DIR%\portable_python.tar" -C "%INSTALL_DIR%\portable_python" || (
    echo Failed to extract portable Python
    goto :error
)

echo Portable Python extracted successfully.

echo Copying launch script...
xcopy /y "%~dp0launch_streamlit.bat" "%INSTALL_DIR%\" || (
    echo Failed to copy launch script
    goto :error
)

:: Create a system Python launcher script if Python is installed
if exist "%INSTALL_DIR%\use_system_python.flag" (
    echo Creating system Python launcher...
    echo @echo off > "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo setlocal enabledelayedexpansion >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo set "SCRIPT_DIR=%%~dp0" >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo cd "%%SCRIPT_DIR%%" >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo echo Installing required packages... >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo python -m pip install streamlit pandas plotly -q >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo echo Starting Streamlit viewer... >> "%INSTALL_DIR%\launch_streamlit_system.bat"
    echo python -m streamlit run timesheet_viewer.py >> "%INSTALL_DIR%\launch_streamlit_system.bat"
)

:: Verify critical files exist
if not exist "%INSTALL_DIR%\TimeSheet.exe" (
    echo ERROR: TimeSheet.exe not found in installation directory.
    goto :error
)
if not exist "%INSTALL_DIR%\assets\MACLEOD_JORDAN_LOGO.jpg" (
    echo ERROR: Logo file not found in installation directory.
    goto :error
)
if not exist "%INSTALL_DIR%\timesheet_viewer.py" (
    echo ERROR: timesheet_viewer.py not found in installation directory.
    goto :error
)
if not exist "%INSTALL_DIR%\portable_python\python.exe" (
    echo ERROR: Portable Python not found in installation directory.
    goto :error
)
if not exist "%INSTALL_DIR%\launch_streamlit.bat" (
    echo ERROR: launch_streamlit.bat not found in installation directory.
    goto :error
)

:: Create shortcuts
echo Creating shortcuts...
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%DESKTOP_SHORTCUT%'); $SC.TargetPath = '%INSTALL_DIR%\TimeSheet.exe'; $SC.WorkingDirectory = '%INSTALL_DIR%'; $SC.Save()"
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%START_MENU_SHORTCUT%'); $SC.TargetPath = '%INSTALL_DIR%\TimeSheet.exe'; $SC.WorkingDirectory = '%INSTALL_DIR%'; $SC.Save()"

:: Create version file for auto-update checking
echo 1.0.0> "%INSTALL_DIR%\version.txt"

:: Clean up the tar file after extraction
del "%INSTALL_DIR%\portable_python.tar"

echo Installation complete! You can now run TimeSheet from your desktop or start menu.
goto :end

:error
echo Installation failed! Please contact support.
echo Error details: The installation could not complete because some required files were not copied correctly.
echo Please try running the installer again with administrator privileges or contact IT support.
exit /b 1

:end
timeout /t 3 