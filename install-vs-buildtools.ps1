# Install Visual Studio Build Tools for Tauri development
# Run this script as Administrator

Write-Host "Installing Visual Studio Build Tools..." -ForegroundColor Green
Write-Host "This will install the C++ compiler needed for Tauri development." -ForegroundColor Yellow

# Option 1: Using winget (recommended)
try {
    if (Get-Command winget -ErrorAction SilentlyContinue) {
        Write-Host "Using winget to install Build Tools..." -ForegroundColor Cyan
        winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
        Write-Host "Installation completed via winget." -ForegroundColor Green
    } else {
        throw "winget not found"
    }
} catch {
    Write-Host "Winget installation failed or not available." -ForegroundColor Red
    Write-Host "Please install manually:" -ForegroundColor Yellow
    Write-Host "1. Go to: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor White
    Write-Host "2. Download 'Build Tools for Visual Studio 2022'" -ForegroundColor White
    Write-Host "3. Run the installer" -ForegroundColor White
    Write-Host "4. Select 'Desktop development with C++' workload" -ForegroundColor White
    Write-Host "5. Click Install" -ForegroundColor White
}

Write-Host ""
Write-Host "After installation:" -ForegroundColor Cyan
Write-Host "1. RESTART your computer" -ForegroundColor Yellow
Write-Host "2. Open a new PowerShell window" -ForegroundColor Yellow
Write-Host "3. Run: npm run tauri dev" -ForegroundColor Yellow