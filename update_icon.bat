@echo off
echo Updating icon cache and shortcuts...

REM Kill explorer to release icon cache
taskkill /F /IM explorer.exe

REM Clear icon cache
del /A:H /F "%LOCALAPPDATA%\IconCache.db"
del /F "%LOCALAPPDATA%\Microsoft\Windows\Explorer\iconcache*"

REM Restart explorer
start explorer.exe

echo Icon cache has been cleared.
echo Please restart your computer to fully apply the changes.
pause 