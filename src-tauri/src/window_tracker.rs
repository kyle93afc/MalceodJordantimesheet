use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use anyhow::Result;
use chrono::{DateTime, Utc};
use regex::Regex;
use tokio::sync::Mutex;
use log::{info, warn, error};

use crate::models::{WindowInfo, Project};
use crate::database::Database;

pub struct WindowTracker {
    db: Arc<Database>,
    project_patterns: Arc<Mutex<Vec<ProjectPattern>>>,
    last_activity: Arc<Mutex<Instant>>,
    idle_threshold: Duration,
    current_window: Arc<Mutex<Option<WindowInfo>>>,
}

#[derive(Debug, Clone)]
pub struct ProjectPattern {
    pub project_code: String,
    pub regex: Regex,
    pub priority: i32,
}

impl WindowTracker {
    pub fn new(db: Arc<Database>) -> Self {
        Self {
            db,
            project_patterns: Arc::new(Mutex::new(Vec::new())),
            last_activity: Arc::new(Mutex::new(Instant::now())),
            idle_threshold: Duration::from_secs(300), // 5 minutes
            current_window: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn initialize_patterns(&self) -> Result<()> {
        let mut patterns = self.project_patterns.lock().await;
        patterns.clear();

        // Default patterns based on the original Python implementation
        let default_patterns = vec![
            // Basic project number patterns
            (r"(\d{5,6})", "PROJECT", 1),
            // Complex project formats
            (r"(\d{6}-[A-Z]+\+[A-Z]+-V\d+-[A-Z]{2}-[A-Z]{2}-[A-Z]+-\d{2}-\d{2}-[A-Z]+)", "PROJECT", 2),
            // File path patterns
            (r"W:\\02-PROJECTS\\(\d{6})", "PROJECT", 3),
            (r"\\\\server\\projects\\(\d{6})", "PROJECT", 3),
            // Microsoft Teams fallback
            (r"Microsoft Teams", "TEAMS", 0),
        ];

        for (pattern_str, code, priority) in default_patterns {
            if let Ok(regex) = Regex::new(pattern_str) {
                patterns.push(ProjectPattern {
                    project_code: code.to_string(),
                    regex,
                    priority,
                });
            }
        }

        patterns.sort_by(|a, b| b.priority.cmp(&a.priority));
        
        info!("Initialized {} project patterns", patterns.len());
        Ok(())
    }

    pub async fn detect_active_window(&self) -> Result<Option<WindowInfo>> {
        #[cfg(target_os = "windows")]
        {
            self.detect_active_window_windows().await
        }
        #[cfg(target_os = "macos")]
        {
            self.detect_active_window_macos().await
        }
        #[cfg(target_os = "linux")]
        {
            self.detect_active_window_linux().await
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            Ok(None)
        }
    }

    #[cfg(target_os = "windows")]
    async fn detect_active_window_windows(&self) -> Result<Option<WindowInfo>> {
        use winapi::um::winuser::{GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId};
        use winapi::um::processthreadsapi::OpenProcess;
        use winapi::um::psapi::GetModuleBaseNameW;
        use winapi::um::handleapi::CloseHandle;
        use winapi::um::winnt::PROCESS_QUERY_INFORMATION;

        unsafe {
            let hwnd = GetForegroundWindow();
            if hwnd.is_null() {
                return Ok(None);
            }

            // Get window title
            let mut title_buf = [0u16; 256];
            let title_len = GetWindowTextW(hwnd, title_buf.as_mut_ptr(), title_buf.len() as i32);
            let title = if title_len > 0 {
                String::from_utf16_lossy(&title_buf[..title_len as usize])
            } else {
                String::new()
            };

            // Get process ID
            let mut process_id = 0u32;
            GetWindowThreadProcessId(hwnd, &mut process_id);

            // Get process name
            let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, process_id);
            let process_name = if !process_handle.is_null() {
                let mut name_buf = [0u16; 256];
                let name_len = GetModuleBaseNameW(process_handle, std::ptr::null_mut(), name_buf.as_mut_ptr(), name_buf.len() as u32);
                CloseHandle(process_handle);
                
                if name_len > 0 {
                    String::from_utf16_lossy(&name_buf[..name_len as usize])
                } else {
                    String::new()
                }
            } else {
                String::new()
            };

            Ok(Some(WindowInfo {
                title,
                process_name,
                process_id,
                is_active: true,
                timestamp: Utc::now(),
            }))
        }
    }

    #[cfg(target_os = "macos")]
    async fn detect_active_window_macos(&self) -> Result<Option<WindowInfo>> {
        // Implementation for macOS using active-win-pos-rs or similar
        // This is a placeholder - you would need to implement the actual macOS window detection
        warn!("macOS window detection not yet implemented");
        Ok(None)
    }

    #[cfg(target_os = "linux")]
    async fn detect_active_window_linux(&self) -> Result<Option<WindowInfo>> {
        // Implementation for Linux using X11 or Wayland
        // This is a placeholder - you would need to implement the actual Linux window detection
        warn!("Linux window detection not yet implemented");
        Ok(None)
    }

    pub async fn extract_project_code(&self, window_title: &str) -> Option<String> {
        let patterns = self.project_patterns.lock().await;
        
        for pattern in patterns.iter() {
            if let Some(captures) = pattern.regex.captures(window_title) {
                if let Some(matched) = captures.get(1) {
                    let code = matched.as_str();
                    
                    // Special handling for numeric project codes
                    if code.chars().all(|c| c.is_ascii_digit()) && code.len() >= 5 {
                        return Some(code.to_string());
                    }
                    
                    // Return the matched pattern code for other types
                    return Some(pattern.project_code.clone());
                }
            }
        }
        
        None
    }

    pub async fn get_or_create_project(&self, project_code: &str) -> Result<Project> {
        // Try to get existing project
        if let Some(project) = self.db.get_project_by_code(project_code).await? {
            return Ok(project);
        }

        // Create new project
        let project = Project {
            id: uuid::Uuid::new_v4(),
            name: format!("Project {}", project_code),
            code: project_code.to_string(),
            color: Some(self.generate_project_color(project_code)),
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        self.db.create_project(&project).await?;
        info!("Created new project: {}", project_code);
        
        Ok(project)
    }

    fn generate_project_color(&self, project_code: &str) -> String {
        // Generate a consistent color based on project code
        let mut hash = 0u32;
        for byte in project_code.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
        }
        
        let colors = [
            "#ef4444", "#f97316", "#f59e0b", "#eab308", "#84cc16",
            "#22c55e", "#10b981", "#14b8a6", "#06b6d4", "#0ea5e9",
            "#3b82f6", "#6366f1", "#8b5cf6", "#a855f7", "#d946ef",
            "#ec4899", "#f43f5e",
        ];
        
        colors[hash as usize % colors.len()].to_string()
    }

    pub async fn is_idle(&self) -> bool {
        let last_activity = self.last_activity.lock().await;
        last_activity.elapsed() > self.idle_threshold
    }

    pub async fn update_activity(&self) {
        let mut last_activity = self.last_activity.lock().await;
        *last_activity = Instant::now();
    }

    pub async fn get_current_window(&self) -> Option<WindowInfo> {
        self.current_window.lock().await.clone()
    }

    pub async fn set_current_window(&self, window: Option<WindowInfo>) {
        let mut current = self.current_window.lock().await;
        *current = window;
    }

    pub async fn start_monitoring(&self) -> Result<()> {
        info!("Starting window monitoring");
        
        // Initialize patterns
        self.initialize_patterns().await?;
        
        // Start monitoring in a separate task
        let tracker = Arc::new(self.clone());
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(2));
            
            loop {
                interval.tick().await;
                
                if let Ok(Some(window)) = tracker.detect_active_window().await {
                    tracker.set_current_window(Some(window)).await;
                    tracker.update_activity().await;
                }
            }
        });
        
        Ok(())
    }
}

impl Clone for WindowTracker {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            project_patterns: self.project_patterns.clone(),
            last_activity: self.last_activity.clone(),
            idle_threshold: self.idle_threshold,
            current_window: self.current_window.clone(),
        }
    }
}