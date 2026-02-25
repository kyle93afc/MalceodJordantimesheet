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

:: Copy files to network distribution
echo Preparing network distribution...
echo F | xcopy /y /q "timesheetorg\dist\TimeTracker.exe" "%NETWORK_DIST%\TimeSheet.exe"
xcopy /y /q "network_install.bat" "%NETWORK_DIST%\"
xcopy /y /q "README.md" "%NETWORK_DIST%\"
xcopy /y /s /i /q "timesheetorg\assets" "%NETWORK_DIST%\assets\"

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
    
    :: Copy application files to network location
    echo.
    echo Copying application files to network location...
    robocopy "%NETWORK_DIST%" "W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS" /MIR /NP /NDL /NFL /NC /NS /MT:8
    
    echo.
    echo Network deployment complete!
    echo Files copied to W:\05-LIBRARY\SOFTWARE\MJ SCRIPTS
)

echo.
echo All operations completed.
timeout /t 3
