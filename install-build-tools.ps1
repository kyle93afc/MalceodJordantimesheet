# PowerShell script to install Visual Studio Build Tools

Write-Host "Installing Visual Studio Build Tools..." -ForegroundColor Green

# Check if winget is available
if (Get-Command winget -ErrorAction SilentlyContinue) {
    Write-Host "Using winget to install Build Tools..."
    winget install Microsoft.VisualStudio.2022.BuildTools --silent --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
} else {
    Write-Host "Winget not found. Please install Visual Studio Build Tools manually from:"
    Write-Host "https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Make sure to select 'Desktop development with C++' workload during installation." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "After installation, please:" -ForegroundColor Cyan
Write-Host "1. Restart your terminal/PowerShell"
Write-Host "2. Run 'npm run tauri dev' again"