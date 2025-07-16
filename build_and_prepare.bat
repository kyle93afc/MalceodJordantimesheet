@echo off
setlocal enabledelayedexpansion

echo Building TimeSheet executable...

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

:: Check if Python is available (try 3.8 first, then any Python 3)
py -3.8 --version >nul 2>&1
if errorlevel 1 (
    echo Python 3.8 not found, trying any Python 3...
    py -3 --version >nul 2>&1
    if errorlevel 1 (
        echo Error: Python 3.x is required but not found.
        echo Please install Python from https://www.python.org/downloads/
        echo Make sure to check "Add Python to PATH" during installation.
        echo After installing, run this script again.
        timeout /t 10
        exit /b 1
    )
)

:: Clean previous builds thoroughly
echo Cleaning previous builds...
rd /s /q "dist" 2>nul
rd /s /q "build" 2>nul

:: Check for required files
if not exist "timesheetorg\assets\MACLEOD_JORDAN_LOGO.jpg" (
    echo Error: Logo file not found at timesheetorg\assets\MACLEOD_JORDAN_LOGO.jpg
    exit /b 1
)

if not exist "timesheetorg\assets\MACLEOD_JORDAN_LOGO.ico" (
    echo Error: Icon file not found at timesheetorg\assets\MACLEOD_JORDAN_LOGO.ico
    exit /b 1
)

:: Build the executable
echo Building executable...
cd timesheetorg
pyinstaller timetracker.spec
cd ..

:: Verify the build
if not exist "timesheetorg\dist\TimeTracker.exe" (
    echo Build failed! TimeTracker.exe not found.
    exit /b 1
)

:: Create network distribution folder
set "NETWORK_DIST=network_dist"
rd /s /q "%NETWORK_DIST%" 2>nul
mkdir "%NETWORK_DIST%"

:: Setup portable Python
echo Setting up portable Python...
call portable_python_setup.bat

:: Extract version from file_version_info.txt
for /f "tokens=3 delims='" %%a in ('findstr "FileVersion" file_version_info.txt') do set "VERSION=%%a"
echo Current version: %VERSION%

:: Copy files to network distribution
echo Preparing network distribution...
echo F | xcopy /y /q "timesheetorg\dist\TimeTracker.exe" "%NETWORK_DIST%\TimeSheet.exe"
xcopy /y /q "network_install.bat" "%NETWORK_DIST%\"
xcopy /y /q "README.md" "%NETWORK_DIST%\"
xcopy /y /s /i /q "timesheetorg\assets" "%NETWORK_DIST%\assets\"
xcopy /y /q "timesheetorg\timesheet_viewer.py" "%NETWORK_DIST%\"

:: Create portable Python archive using tar instead of direct copy
echo Creating portable Python archive (this is much faster than copying individual files)...
mkdir "%NETWORK_DIST%\portable_python" 2>nul
tar -cf "%NETWORK_DIST%\portable_python.tar" -C "portable_python" .

:: Create an extraction script for the portable Python
echo @echo off > "%NETWORK_DIST%\extract_python.bat"
echo echo Extracting portable Python environment... >> "%NETWORK_DIST%\extract_python.bat"
echo if not exist "portable_python\python.exe" ( >> "%NETWORK_DIST%\extract_python.bat"
echo   echo Extracting portable Python from archive... >> "%NETWORK_DIST%\extract_python.bat"
echo   tar -xf portable_python.tar -C portable_python >> "%NETWORK_DIST%\extract_python.bat"
echo   if errorlevel 1 ( >> "%NETWORK_DIST%\extract_python.bat"
echo     echo Error extracting Python environment. >> "%NETWORK_DIST%\extract_python.bat"
echo     exit /b 1 >> "%NETWORK_DIST%\extract_python.bat"
echo   ) >> "%NETWORK_DIST%\extract_python.bat"
echo   echo Portable Python environment extracted successfully. >> "%NETWORK_DIST%\extract_python.bat"
echo ) else ( >> "%NETWORK_DIST%\extract_python.bat"
echo   echo Portable Python environment already exists. >> "%NETWORK_DIST%\extract_python.bat"
echo ) >> "%NETWORK_DIST%\extract_python.bat"

xcopy /y /q "launch_streamlit.bat" "%NETWORK_DIST%\"

:: Create a sample system Python launcher script for the network distribution
echo Creating system Python launcher template...
echo @echo off > "%NETWORK_DIST%\launch_streamlit_system.bat"
echo setlocal enabledelayedexpansion >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo set "SCRIPT_DIR=%%~dp0" >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo cd "%%SCRIPT_DIR%%" >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo echo Installing required packages... >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo python -m pip install streamlit pandas plotly -q >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo echo Starting Streamlit viewer... >> "%NETWORK_DIST%\launch_streamlit_system.bat"
echo python -m streamlit run timesheet_viewer.py >> "%NETWORK_DIST%\launch_streamlit_system.bat"

:: Create version file
echo %VERSION%> "%NETWORK_DIST%\TimeSheet.exe.version"

echo.
echo Build complete! Version: %VERSION%
echo Files are ready in the %NETWORK_DIST% folder.
echo.
echo Testing executable...
"timesheetorg\dist\TimeTracker.exe"
if errorlevel 1 (
    echo WARNING: Executable test failed!
    echo Please check the application manually.
) else (
    echo Executable test successful!
)

timeout /t 5 

:: Ask if user wants to deploy to network location
echo.
set /p DEPLOY_NETWORK="Deploy to network location (W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS)? (Y/N): "
if /i "%DEPLOY_NETWORK%"=="Y" (
    echo.
    echo Deploying to network location...
    
    :: Check if network path is accessible
    if not exist "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS\" (
        echo ERROR: Network location not accessible.
        echo Please ensure you have access to W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS
        echo and try again.
        timeout /t 5
        exit /b 1
    )
    
    :: Ask about portable Python deployment
    set DEPLOY_PYTHON=N
    if exist "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS\portable_python.tar" (
        echo.
        echo Portable Python archive already exists on network location.
        set /p DEPLOY_PYTHON="Update portable Python archive? (Only needed if Python dependencies changed) (Y/N): "
    ) else (
        echo.
        echo Portable Python archive not found on network location.
        set DEPLOY_PYTHON=Y
        echo Will deploy portable Python archive (first-time setup).
    )
    
    :: Copy main application files (excluding portable Python)
    echo.
    echo Copying application files to network location...
    robocopy "%NETWORK_DIST%" "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS" /XF portable_python.tar /MIR /NP /NDL /NFL /NC /NS /MT:8
    
    :: Copy portable Python archive only if requested
    if /i "%DEPLOY_PYTHON%"=="Y" (
        echo.
        echo Copying portable Python archive (this should be much faster than before)...
        copy /Y "%NETWORK_DIST%\portable_python.tar" "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS\"
        copy /Y "%NETWORK_DIST%\extract_python.bat" "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS\"
    ) else (
        echo.
        echo Skipped portable Python archive update.
    )
    
    echo.
    echo Network deployment complete!
    echo Only changed files were copied to W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS
)

echo.
echo All operations completed.
timeout /t 3 