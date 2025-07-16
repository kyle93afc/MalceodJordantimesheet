# IMMEDIATE SOLUTION - Install Visual Studio Build Tools

## The Problem
The build fails with: `failed to find tool "cl.exe": program not found`

This means you need Microsoft Visual C++ Build Tools to compile native dependencies.

## SOLUTION (Choose one method):

### Method 1: PowerShell Script (Easiest)
1. **Run PowerShell as Administrator**
2. **Execute the install script:**
   ```powershell
   .\install-vs-buildtools.ps1
   ```

### Method 2: Manual Installation (Recommended)
1. **Download Build Tools:**
   - Go to: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Download "Build Tools for Visual Studio 2022"

2. **Install with correct workload:**
   - Run the installer
   - Select **"Desktop development with C++"** workload
   - Click Install

3. **Restart your computer** (important!)

### Method 3: Using winget
```powershell
# Run as Administrator
winget install Microsoft.VisualStudio.2022.BuildTools --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

### Method 4: Using Chocolatey (if you have it)
```powershell
# Run as Administrator
choco install visualstudio2022buildtools --package-parameters "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
```

## After Installation:
1. **RESTART your computer**
2. **Open a new PowerShell window**
3. **Navigate to your project:**
   ```powershell
   cd C:\Cursor_Files\MalceodJordantimesheet
   ```
4. **Run the project:**
   ```powershell
   npm run tauri dev
   ```

## Verify Installation:
After restart, you should be able to run:
```powershell
cl.exe
```
And see the Microsoft C++ compiler help text.

## Alternative: Use Developer Command Prompt
After installation, you can also use:
1. Search for "Developer Command Prompt for VS 2022"
2. Run it as Administrator
3. Navigate to your project and run `npm run tauri dev`

---

**Note:** The Visual Studio Build Tools are required for any Windows Rust project that compiles native code. This is a one-time setup that will work for all future Tauri projects.