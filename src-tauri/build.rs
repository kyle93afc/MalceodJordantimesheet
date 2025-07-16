fn main() {
    // Set environment variable to skip problematic builds on Windows
    #[cfg(windows)]
    {
        println!("cargo:rustc-env=WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS=--disable-web-security");
    }
    
    tauri_build::build()
}