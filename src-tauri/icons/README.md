# Icon Requirements

Tauri requires the following icon files:
- 32x32.png
- 128x128.png
- 128x128@2x.png (256x256)
- icon.icns (for macOS)
- icon.ico (for Windows)

Currently using the MACLEOD_JORDAN_LOGO.ico file.

To generate all required icon sizes from a single source image, you can use tools like:
- https://tauri.app/v1/guides/features/icons/
- https://www.npmjs.com/package/@tauri-apps/cli

Run: `npm run tauri icon path/to/high-res-logo.png` to generate all sizes automatically.