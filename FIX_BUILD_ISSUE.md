# Fix Build Issue - Visual Studio Build Tools

## The Problem
The error `Error: Io(Error { kind: NotFound, message: "program not found" })` occurs because Visual Studio Build Tools are not properly installed or configured.

## Solution 1: Install Visual Studio Build Tools (Recommended)

### Option A: Using winget (Windows Package Manager)
```powershell
# Run as Administrator
winget install Microsoft.VisualStudio.2022.BuildTools
```

### Option B: Manual Installation
1. Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
2. Run the installer
3. Select **"Desktop development with C++"** workload
4. Click Install

### Option C: Using PowerShell script
```powershell
# Run the provided script
.\install-build-tools.ps1
```

## Solution 2: Use Developer Command Prompt

1. Search for "Developer Command Prompt for VS 2022" in Start Menu
2. Open it as Administrator
3. Navigate to your project directory:
   ```cmd
   cd C:\Cursor_Files\MalceodJordantimesheet
   ```
4. Run the Tauri development server:
   ```cmd
   npm run tauri dev
   ```

## Solution 3: Alternative - Use Visual Studio Code with Rust Extension

1. Install Visual Studio Code
2. Install the Rust Extension
3. Open the project folder
4. The extension will help set up the Rust environment

## After Installation

1. **Restart your computer** (important!)
2. Open a new terminal/PowerShell window
3. Verify installation:
   ```cmd
   cl.exe
   ```
4. Run the project:
   ```cmd
   npm run tauri dev
   ```

## If Still Having Issues

Try these additional steps:

1. **Clean the build cache:**
   ```cmd
   cd src-tauri
   cargo clean
   cd ..
   ```

2. **Update Rust:**
   ```cmd
   rustup update
   ```

3. **Check Rust toolchain:**
   ```cmd
   rustup toolchain list
   rustup default stable-x86_64-pc-windows-msvc
   ```

4. **Try building with verbose output:**
   ```cmd
   npm run tauri dev -- --verbose
   ```

## Note
WebView2 is required for Tauri applications on Windows. The build tools are necessary to compile the native WebView2 bindings.