// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, Manager, WindowEvent};
use log::{info, error};

mod models;
mod services;
mod commands;
mod utils;
mod database;
mod window_tracker;
mod system_tray;

use crate::services::TimeTracker;
use crate::commands::*;
use crate::database::Database;
use crate::system_tray::create_system_tray;

pub struct AppState {
    pub db: Arc<Database>,
    pub tracker: Arc<Mutex<TimeTracker>>,
}

#[tokio::main]
async fn main() {
    env_logger::init();
    
    info!("Starting Modern Timesheet Tracker v2.0.0");

    // Initialize database
    let db = Arc::new(Database::new().await.expect("Failed to initialize database"));
    
    // Initialize time tracker
    let tracker = Arc::new(Mutex::new(TimeTracker::new(db.clone()).await));

    let app_state = AppState {
        db: db.clone(),
        tracker: tracker.clone(),
    };

    let system_tray = create_system_tray();

    tauri::Builder::default()
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            get_current_project,
            get_projects,
            get_time_entries,
            export_data,
            get_settings,
            update_settings,
            start_tracking,
            stop_tracking,
            get_tracking_status,
            get_daily_summary,
            get_project_statistics
        ])
        .system_tray(system_tray)
        .on_system_tray_event(|app, event| {
            system_tray::handle_system_tray_event(app, event);
        })
        .on_window_event(|event| {
            match event.event() {
                WindowEvent::CloseRequested { api, .. } => {
                    // Hide window instead of closing
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
                _ => {}
            }
        })
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, _event| {
            // Handle app events
        });
}