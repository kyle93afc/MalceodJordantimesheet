@echo off
echo Installing TimeSheet Application...

:: Create program files directory if it doesn't exist
set INSTALL_DIR=%LOCALAPPDATA%\TimeSheet
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

:: Copy executable and assets
echo Copying application files...
xcopy /y /q "dist\TimeSheet.exe" "%INSTALL_DIR%"
xcopy /y /s /q "timesheetorg\assets\*" "%INSTALL_DIR%\assets\"

:: Create desktop shortcut
echo Creating desktop shortcut...
powershell -Command "$WS = New-Object -ComObject WScript.Shell; $SC = $WS.CreateShortcut('%USERPROFILE%\Desktop\TimeSheet.lnk'); $SC.TargetPath = '%INSTALL_DIR%\TimeSheet.exe'; $SC.Save()"

echo Installation complete! You can now run TimeSheet from your desktop.
pause 