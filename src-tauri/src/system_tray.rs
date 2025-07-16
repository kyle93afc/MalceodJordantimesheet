use tauri::{CustomMenuItem, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, AppHandle, Manager};
use log::info;

pub fn create_system_tray() -> SystemTray {
    let show_hide = CustomMenuItem::new("show_hide".to_string(), "Show/Hide Window");
    let start_stop = CustomMenuItem::new("start_stop".to_string(), "Start Tracking");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let about = CustomMenuItem::new("about".to_string(), "About");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show_hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(start_stop)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    SystemTray::new().with_menu(tray_menu).with_tooltip("Modern Timesheet Tracker")
}

pub fn handle_system_tray_event(app: &AppHandle, event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::LeftClick { .. } => {
            info!("System tray left clicked");
            toggle_window_visibility(app);
        }
        SystemTrayEvent::RightClick { .. } => {
            info!("System tray right clicked");
        }
        SystemTrayEvent::DoubleClick { .. } => {
            info!("System tray double clicked");
            toggle_window_visibility(app);
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            info!("System tray menu item clicked: {}", id);
            handle_menu_item_click(app, &id);
        }
        _ => {}
    }
}

fn toggle_window_visibility(app: &AppHandle) {
    if let Some(window) = app.get_window("main") {
        match window.is_visible() {
            Ok(true) => {
                let _ = window.hide();
            }
            Ok(false) => {
                let _ = window.show();
                let _ = window.set_focus();
            }
            Err(e) => {
                eprintln!("Error checking window visibility: {}", e);
            }
        }
    }
}

fn handle_menu_item_click(app: &AppHandle, item_id: &str) {
    match item_id {
        "show_hide" => {
            toggle_window_visibility(app);
        }
        "start_stop" => {
            // TODO: Toggle tracking state
            // This would need to communicate with the time tracker service
            info!("Start/Stop tracking clicked");
        }
        "settings" => {
            // TODO: Open settings window
            info!("Settings clicked");
        }
        "about" => {
            // TODO: Show about dialog
            info!("About clicked");
        }
        "quit" => {
            info!("Quit clicked");
            app.exit(0);
        }
        _ => {}
    }
}

pub fn update_tray_menu(app: &AppHandle, is_tracking: bool) {
    let start_stop_text = if is_tracking {
        "Stop Tracking"
    } else {
        "Start Tracking"
    };
    
    let start_stop = CustomMenuItem::new("start_stop".to_string(), start_stop_text);
    let show_hide = CustomMenuItem::new("show_hide".to_string(), "Show/Hide Window");
    let settings = CustomMenuItem::new("settings".to_string(), "Settings");
    let about = CustomMenuItem::new("about".to_string(), "About");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");

    let tray_menu = SystemTrayMenu::new()
        .add_item(show_hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(start_stop)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(settings)
        .add_item(about)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    if let Some(tray) = app.tray_handle() {
        let _ = tray.set_menu(tray_menu);
    }
}

pub fn update_tray_tooltip(app: &AppHandle, current_project: Option<&str>, duration: i64) {
    let tooltip = if let Some(project) = current_project {
        format!("Modern Timesheet Tracker - {} ({})", project, format_duration(duration))
    } else {
        "Modern Timesheet Tracker - Not tracking".to_string()
    };
    
    if let Some(tray) = app.tray_handle() {
        let _ = tray.set_tooltip(&tooltip);
    }
}

fn format_duration(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    
    if hours > 0 {
        format!("{}h {:02}m {:02}s", hours, minutes, secs)
    } else if minutes > 0 {
        format!("{}m {:02}s", minutes, secs)
    } else {
        format!("{}s", secs)
    }
}