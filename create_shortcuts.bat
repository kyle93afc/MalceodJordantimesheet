@echo off
echo Creating shortcuts with the correct icon...

powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $Shortcut = $WshShell.CreateShortcut([Environment]::GetFolderPath('Desktop') + '\TimeTracker.lnk'); $Shortcut.TargetPath = '%~dp0dist\TimeTracker.exe'; $Shortcut.IconLocation = '%~dp0assets\MACLEOD_JORDAN_LOGO.ico'; $Shortcut.Save()"

powershell -Command "$WshShell = New-Object -ComObject WScript.Shell; $programsPath = [Environment]::GetFolderPath('Programs'); $Shortcut = $WshShell.CreateShortcut($programsPath + '\TimeTracker.lnk'); $Shortcut.TargetPath = '%~dp0dist\TimeTracker.exe'; $Shortcut.IconLocation = '%~dp0assets\MACLEOD_JORDAN_LOGO.ico'; $Shortcut.Save()"

echo Shortcuts created successfully!
echo.
echo Please check your Desktop and Start Menu for the new shortcuts.
echo.
pause 