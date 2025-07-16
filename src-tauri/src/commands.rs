use tauri::State;
use anyhow::Result;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;

use crate::models::*;
use crate::AppState;

#[tauri::command]
pub async fn get_current_project(state: State<'_, AppState>) -> Result<Option<Project>, String> {
    let tracker = state.tracker.lock().await;
    let status = tracker.get_tracking_status().await;
    Ok(status.current_project)
}

#[tauri::command]
pub async fn get_projects(state: State<'_, AppState>) -> Result<Vec<Project>, String> {
    state.db.get_projects().await
        .map_err(|e| format!("Failed to get projects: {}", e))
}

#[tauri::command]
pub async fn get_time_entries(
    state: State<'_, AppState>,
    start_date: Option<String>,
    end_date: Option<String>,
) -> Result<Vec<TimeEntry>, String> {
    let start = if let Some(date_str) = start_date {
        Some(DateTime::parse_from_rfc3339(&date_str)
            .map_err(|e| format!("Invalid start date: {}", e))?
            .with_timezone(&Utc))
    } else {
        None
    };
    
    let end = if let Some(date_str) = end_date {
        Some(DateTime::parse_from_rfc3339(&date_str)
            .map_err(|e| format!("Invalid end date: {}", e))?
            .with_timezone(&Utc))
    } else {
        None
    };
    
    state.db.get_time_entries(start, end).await
        .map_err(|e| format!("Failed to get time entries: {}", e))
}

#[tauri::command]
pub async fn start_tracking(state: State<'_, AppState>) -> Result<(), String> {
    let tracker = state.tracker.lock().await;
    tracker.start_tracking().await
        .map_err(|e| format!("Failed to start tracking: {}", e))
}

#[tauri::command]
pub async fn stop_tracking(state: State<'_, AppState>) -> Result<(), String> {
    let tracker = state.tracker.lock().await;
    tracker.stop_tracking().await
        .map_err(|e| format!("Failed to stop tracking: {}", e))
}

#[tauri::command]
pub async fn get_tracking_status(state: State<'_, AppState>) -> Result<TrackingStatus, String> {
    let tracker = state.tracker.lock().await;
    Ok(tracker.get_tracking_status().await)
}

#[tauri::command]
pub async fn get_daily_summary(
    state: State<'_, AppState>,
    date: String,
) -> Result<DailySummary, String> {
    let tracker = state.tracker.lock().await;
    tracker.get_daily_summary(&date).await
        .map_err(|e| format!("Failed to get daily summary: {}", e))
}

#[tauri::command]
pub async fn get_project_statistics(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<Option<ProjectStatistics>, String> {
    let uuid = Uuid::parse_str(&project_id)
        .map_err(|e| format!("Invalid project ID: {}", e))?;
    
    let tracker = state.tracker.lock().await;
    tracker.get_project_statistics(uuid).await
        .map_err(|e| format!("Failed to get project statistics: {}", e))
}

#[tauri::command]
pub async fn export_data(
    state: State<'_, AppState>,
    request: ExportRequest,
) -> Result<String, String> {
    // Parse dates
    let start_date = if let Some(date_str) = request.start_date {
        Some(chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid start date: {}", e))?
            .and_hms_opt(0, 0, 0).unwrap()
            .and_utc())
    } else {
        None
    };
    
    let end_date = if let Some(date_str) = request.end_date {
        Some(chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .map_err(|e| format!("Invalid end date: {}", e))?
            .and_hms_opt(23, 59, 59).unwrap()
            .and_utc())
    } else {
        None
    };
    
    // Get time entries
    let entries = state.db.get_time_entries(start_date, end_date).await
        .map_err(|e| format!("Failed to get time entries: {}", e))?;
    
    // Filter by project IDs if specified
    let filtered_entries: Vec<TimeEntry> = if let Some(project_ids) = request.project_ids {
        entries.into_iter()
            .filter(|entry| project_ids.contains(&entry.project_id))
            .collect()
    } else {
        entries
    };
    
    // Export based on format
    match request.format {
        ExportFormat::JSON => {
            serde_json::to_string_pretty(&filtered_entries)
                .map_err(|e| format!("Failed to serialize JSON: {}", e))
        }
        ExportFormat::CSV => {
            export_to_csv(&filtered_entries)
                .map_err(|e| format!("Failed to export CSV: {}", e))
        }
        ExportFormat::Excel => {
            // For now, return CSV format
            // TODO: Implement actual Excel export
            export_to_csv(&filtered_entries)
                .map_err(|e| format!("Failed to export Excel: {}", e))
        }
    }
}

fn export_to_csv(entries: &[TimeEntry]) -> Result<String> {
    let mut csv_content = String::new();
    
    // Header
    csv_content.push_str("ID,Project ID,Window Title,Start Time,End Time,Duration (seconds),Is Manual,Is Idle,Created At,Updated At\n");
    
    // Data rows
    for entry in entries {
        csv_content.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{}\n",
            entry.id,
            entry.project_id,
            entry.window_title.replace(',', ";"), // Escape commas
            entry.start_time.to_rfc3339(),
            entry.end_time.map(|t| t.to_rfc3339()).unwrap_or_default(),
            entry.duration_seconds,
            entry.is_manual,
            entry.is_idle,
            entry.created_at.to_rfc3339(),
            entry.updated_at.to_rfc3339()
        ));
    }
    
    Ok(csv_content)
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Value, String> {
    // Get all settings and return as JSON object
    let mut settings = serde_json::Map::new();
    
    // Default settings
    let default_settings = vec![
        ("idle_threshold", "300"),
        ("save_interval", "30"),
        ("theme", "light"),
        ("auto_start", "true"),
        ("minimize_to_tray", "true"),
        ("show_notifications", "true"),
    ];
    
    for (key, default_value) in default_settings {
        let value = state.db.get_setting(key).await
            .map_err(|e| format!("Failed to get setting {}: {}", key, e))?
            .unwrap_or_else(|| default_value.to_string());
        settings.insert(key.to_string(), Value::String(value));
    }
    
    Ok(Value::Object(settings))
}

#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Value,
) -> Result<(), String> {
    if let Value::Object(settings_map) = settings {
        for (key, value) in settings_map {
            let value_str = match value {
                Value::String(s) => s,
                Value::Number(n) => n.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => value.to_string(),
            };
            
            state.db.set_setting(&key, &value_str).await
                .map_err(|e| format!("Failed to set setting {}: {}", key, e))?;
        }
    }
    
    Ok(())
}