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

:: Verify critical files exist
if not exist "%INSTALL_DIR%\TimeSheet.exe" (
    echo ERROR: TimeSheet.exe not found in installation directory.
    goto :error
)
if not exist "%INSTALL_DIR%\assets\MACLEOD_JORDAN_LOGO.jpg" (
    echo ERROR: Logo file not found in installation directory.
    goto :error
)

:: Create shortcuts
echo Creating shortcuts...
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%DESKTOP_SHORTCUT%'); $SC.TargetPath = '%INSTALL_DIR%\TimeSheet.exe'; $SC.WorkingDirectory = '%INSTALL_DIR%'; $SC.Save()"
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%START_MENU_SHORTCUT%'); $SC.TargetPath = '%INSTALL_DIR%\TimeSheet.exe'; $SC.WorkingDirectory = '%INSTALL_DIR%'; $SC.Save()"

:: Create version file for auto-update checking
echo 1.1.0> "%INSTALL_DIR%\version.txt"

echo.
echo Installation complete! You can now run TimeSheet from your desktop or start menu.
goto :end

:error
echo Installation failed! Please contact support.
echo Error details: The installation could not complete because some required files were not copied correctly.
echo Please try running the installer again with administrator privileges or contact IT support.
exit /b 1

:end
timeout /t 5
