use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use chrono::{DateTime, Utc};
use tokio::sync::Mutex;
use uuid::Uuid;
use log::{info, warn, error};

use crate::models::{TimeEntry, Project, TrackingStatus, DailySummary, ProjectStatistics};
use crate::database::Database;
use crate::window_tracker::WindowTracker;

pub struct TimeTracker {
    db: Arc<Database>,
    window_tracker: WindowTracker,
    current_entry: Arc<Mutex<Option<TimeEntry>>>,
    is_tracking: Arc<Mutex<bool>>,
    last_save: Arc<Mutex<DateTime<Utc>>>,
    save_interval: Duration,
}

impl TimeTracker {
    pub async fn new(db: Arc<Database>) -> Self {
        let window_tracker = WindowTracker::new(db.clone());
        
        Self {
            db,
            window_tracker,
            current_entry: Arc::new(Mutex::new(None)),
            is_tracking: Arc::new(Mutex::new(false)),
            last_save: Arc::new(Mutex::new(Utc::now())),
            save_interval: Duration::from_secs(30),
        }
    }

    pub async fn start_tracking(&self) -> Result<()> {
        info!("Starting time tracking");
        
        let mut is_tracking = self.is_tracking.lock().await;
        *is_tracking = true;
        
        // Start window monitoring
        self.window_tracker.start_monitoring().await?;
        
        // Start the main tracking loop
        let tracker = Arc::new(self.clone());
        tokio::spawn(async move {
            tracker.tracking_loop().await;
        });
        
        Ok(())
    }

    pub async fn stop_tracking(&self) -> Result<()> {
        info!("Stopping time tracking");
        
        let mut is_tracking = self.is_tracking.lock().await;
        *is_tracking = false;
        
        // Finalize current entry
        self.finalize_current_entry().await?;
        
        Ok(())
    }

    async fn tracking_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(2));
        
        loop {
            interval.tick().await;
            
            let is_tracking = *self.is_tracking.lock().await;
            if !is_tracking {
                break;
            }
            
            if let Err(e) = self.process_current_window().await {
                error!("Error processing current window: {}", e);
            }
            
            // Auto-save periodically
            if self.should_save().await {
                if let Err(e) = self.save_current_entry().await {
                    error!("Error saving current entry: {}", e);
                }
            }
        }
    }

    async fn process_current_window(&self) -> Result<()> {
        let window = self.window_tracker.get_current_window().await;
        
        if let Some(window_info) = window {
            // Check if we're idle
            if self.window_tracker.is_idle().await {
                self.handle_idle_state().await?;
                return Ok(());
            }
            
            // Extract project code from window title
            let project_code = self.window_tracker
                .extract_project_code(&window_info.title)
                .await
                .unwrap_or_else(|| "GENERAL".to_string());
            
            // Get or create project
            let project = self.window_tracker
                .get_or_create_project(&project_code)
                .await?;
            
            // Update or create time entry
            self.update_time_entry(&project, &window_info.title).await?;
        }
        
        Ok(())
    }

    async fn update_time_entry(&self, project: &Project, window_title: &str) -> Result<()> {
        let mut current_entry = self.current_entry.lock().await;
        
        match current_entry.as_mut() {
            Some(entry) => {
                // Check if we're still on the same project
                if entry.project_id == project.id {
                    // Update duration
                    entry.duration_seconds = (Utc::now() - entry.start_time).num_seconds();
                    entry.updated_at = Utc::now();
                } else {
                    // Finalize current entry and start new one
                    self.finalize_entry(entry).await?;
                    *current_entry = Some(self.create_new_entry(project, window_title));
                }
            }
            None => {
                // Start new entry
                *current_entry = Some(self.create_new_entry(project, window_title));
            }
        }
        
        Ok(())
    }

    fn create_new_entry(&self, project: &Project, window_title: &str) -> TimeEntry {
        TimeEntry {
            id: Uuid::new_v4(),
            project_id: project.id,
            window_title: window_title.to_string(),
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: 0,
            is_manual: false,
            is_idle: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    async fn finalize_entry(&self, entry: &mut TimeEntry) -> Result<()> {
        let now = Utc::now();
        entry.end_time = Some(now);
        entry.duration_seconds = (now - entry.start_time).num_seconds();
        entry.updated_at = now;
        
        // Only save entries with meaningful duration (at least 1 second)
        if entry.duration_seconds > 0 {
            self.db.create_time_entry(entry).await?;
            info!("Saved time entry: {} seconds for project {}", 
                entry.duration_seconds, entry.project_id);
        }
        
        Ok(())
    }

    async fn finalize_current_entry(&self) -> Result<()> {
        let mut current_entry = self.current_entry.lock().await;
        
        if let Some(mut entry) = current_entry.take() {
            self.finalize_entry(&mut entry).await?;
        }
        
        Ok(())
    }

    async fn handle_idle_state(&self) -> Result<()> {
        // If we have a current entry, mark it as idle and finalize it
        let mut current_entry = self.current_entry.lock().await;
        
        if let Some(mut entry) = current_entry.take() {
            entry.is_idle = true;
            self.finalize_entry(&mut entry).await?;
        }
        
        Ok(())
    }

    async fn should_save(&self) -> bool {
        let last_save = *self.last_save.lock().await;
        Utc::now().signed_duration_since(last_save).num_seconds() > self.save_interval.as_secs() as i64
    }

    async fn save_current_entry(&self) -> Result<()> {
        let current_entry = self.current_entry.lock().await;
        
        if let Some(entry) = current_entry.as_ref() {
            if entry.duration_seconds > 0 {
                self.db.update_time_entry(entry).await?;
                
                let mut last_save = self.last_save.lock().await;
                *last_save = Utc::now();
            }
        }
        
        Ok(())
    }

    pub async fn get_tracking_status(&self) -> TrackingStatus {
        let is_tracking = *self.is_tracking.lock().await;
        let current_entry = self.current_entry.lock().await;
        let current_window = self.window_tracker.get_current_window().await;
        
        let (current_project, session_duration) = if let Some(entry) = current_entry.as_ref() {
            let project = self.db.get_project_by_code(&entry.project_id.to_string())
                .await
                .unwrap_or(None);
            (project, entry.duration_seconds)
        } else {
            (None, 0)
        };
        
        TrackingStatus {
            is_tracking,
            current_project,
            current_window: current_window.map(|w| w.title),
            session_duration,
            idle_time: 0, // TODO: Calculate actual idle time
            last_activity: Some(Utc::now()),
        }
    }

    pub async fn get_daily_summary(&self, date: &str) -> Result<DailySummary> {
        // Parse date and get entries for that day
        let start_date = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d")?
            .and_hms_opt(0, 0, 0).unwrap()
            .and_utc();
        let end_date = start_date + chrono::Duration::days(1);
        
        let entries = self.db.get_time_entries(Some(start_date), Some(end_date)).await?;
        
        let mut project_durations = std::collections::HashMap::new();
        let mut total_duration = 0i64;
        
        for entry in &entries {
            total_duration += entry.duration_seconds;
            *project_durations.entry(entry.project_id).or_insert(0) += entry.duration_seconds;
        }
        
        let mut project_breakdown = Vec::new();
        for (project_id, duration) in project_durations {
            if let Some(project) = self.db.get_project_by_code(&project_id.to_string()).await? {
                project_breakdown.push(crate::models::ProjectDuration {
                    project,
                    duration,
                });
            }
        }
        
        Ok(DailySummary {
            date: date.to_string(),
            total_duration,
            project_breakdown,
            entries_count: entries.len() as i64,
        })
    }

    pub async fn get_project_statistics(&self, project_id: Uuid) -> Result<Option<ProjectStatistics>> {
        // This is a placeholder implementation
        // You would implement actual statistics calculation here
        Ok(None)
    }
}

impl Clone for TimeTracker {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            window_tracker: self.window_tracker.clone(),
            current_entry: self.current_entry.clone(),
            is_tracking: self.is_tracking.clone(),
            last_save: self.last_save.clone(),
            save_interval: self.save_interval,
        }
    }
}