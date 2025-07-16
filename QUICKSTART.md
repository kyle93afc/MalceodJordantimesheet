# Quick Start Guide - Modern Timesheet Tracker

## Prerequisites Installation

### 1. Install Rust (Required)
Visit https://rustup.rs/ and download the Windows installer:
```powershell
# Or use PowerShell:
winget install Rustlang.Rust
```

### 2. Install Node.js (Required)
Visit https://nodejs.org/ and download the LTS version
```powershell
# Or use PowerShell:
winget install OpenJS.NodeJS.LTS
```

### 3. Install Visual Studio Build Tools (Required for Windows)
Download from: https://visualstudio.microsoft.com/downloads/
- Select "Desktop development with C++" workload

## Running the Application

1. **Install dependencies:**
```bash
npm install
```

2. **Install Tauri CLI:**
```bash
cargo install tauri-cli
```

3. **Run in development mode:**
```bash
npm run tauri dev
```

## Troubleshooting

### If Rust is not found:
1. Close all terminals
2. Open a new terminal after installing Rust
3. Run: `rustc --version` to verify

### If build fails:
1. Ensure Visual Studio Build Tools are installed
2. Restart your computer after installation
3. Try building again

### Missing icons error:
The app will run with placeholder icons. To fix:
1. Place your logo in `src-tauri/icons/`
2. Run: `npm run tauri icon path/to/your-logo.png`

## Build for Production
```bash
npm run tauri build
```

The executable will be in `src-tauri/target/release/`