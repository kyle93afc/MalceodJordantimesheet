Write-Host "Creating shortcuts with the correct icon..." -ForegroundColor Green

$WshShell = New-Object -ComObject WScript.Shell
$desktopPath = [Environment]::GetFolderPath('Desktop')
$programsPath = [Environment]::GetFolderPath('Programs')

# Get the current script directory
$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$exePath = Join-Path $scriptDir "dist\TimeTracker.exe"
$iconPath = Join-Path $scriptDir "assets\MACLEOD_JORDAN_LOGO.ico"

# Create desktop shortcut
$desktopShortcut = $WshShell.CreateShortcut("$desktopPath\TimeTracker.lnk")
$desktopShortcut.TargetPath = $exePath
$desktopShortcut.IconLocation = $iconPath
$desktopShortcut.Save()

# Create Start Menu shortcut
$programsShortcut = $WshShell.CreateShortcut("$programsPath\TimeTracker.lnk")
$programsShortcut.TargetPath = $exePath
$programsShortcut.IconLocation = $iconPath
$programsShortcut.Save()

Write-Host "Shortcuts created successfully!" -ForegroundColor Green
Write-Host "Please check your Desktop and Start Menu for the new shortcuts."
Write-Host ""
Write-Host "Press any key to continue..."
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown") 