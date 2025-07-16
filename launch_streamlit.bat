@echo off
setlocal enabledelayedexpansion
set "SCRIPT_DIR=%~dp0"
cd "%SCRIPT_DIR%"

:: Check if portable Python exists, if not extract it
if not exist "portable_python\python.exe" (
    if exist "portable_python.tar" (
        echo Extracting portable Python environment...
        mkdir portable_python 2>nul
        tar -xf portable_python.tar -C portable_python
        if errorlevel 1 (
            echo Error extracting Python environment.
            pause
            exit /b 1
        )
        echo Portable Python environment extracted successfully.
    ) else (
        echo Error: Portable Python not found.
        pause
        exit /b 1
    )
)

:: Launch Streamlit
echo Starting Streamlit viewer...
portable_python\python.exe -m streamlit run timesheet_viewer.py 