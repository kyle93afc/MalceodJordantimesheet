# Development Setup for Modern Timesheet Application

## Prerequisites

This modern rebuild uses Tauri + Rust + Svelte. You'll need the following tools installed:

### 1. Rust Toolchain
Install Rust via rustup:
```bash
# Windows (run in PowerShell)
Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe
./rustup-init.exe

# Or visit: https://rustup.rs/
```

### 2. Node.js and npm
Install Node.js (version 16 or higher):
```bash
# Download from: https://nodejs.org/
# Or use package manager like Chocolatey:
choco install nodejs
```

### 3. Tauri CLI
Install Tauri CLI after Rust is set up:
```bash
cargo install tauri-cli
```

### 4. Additional Dependencies (Windows)
For Windows development, you'll need:
- Microsoft Visual Studio Build Tools or Visual Studio with C++ workload
- WebView2 (usually pre-installed on Windows 10/11)

## Project Setup

1. **Clone and setup the repository:**
```bash
git clone <your-repo-url>
cd MalceodJordantimesheet
git checkout feature/tauri-rust-rebuild
```

2. **Install dependencies:**
```bash
# Install Rust dependencies
cargo check

# Install Node.js dependencies
npm install
```

3. **Run in development mode:**
```bash
npm run tauri dev
```

4. **Build for production:**
```bash
npm run tauri build
```

## Development Workflow

- **Backend (Rust)**: Located in `src-tauri/src/`
- **Frontend (Svelte)**: Located in `src/`
- **Database**: SQLite with Diesel ORM
- **Styling**: Tailwind CSS
- **Testing**: Rust unit tests + Vitest for frontend

## Branch Structure

- `main` - Current PyQt5 version
- `feature/tauri-rust-rebuild` - Modern Tauri rebuild (active development)

## First Steps

1. Ensure all prerequisites are installed
2. Run `npm run tauri dev` to start development server
3. Make changes to either Rust backend or Svelte frontend
4. Hot reload will automatically update the application