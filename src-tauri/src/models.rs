use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub color: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct TimeEntry {
    pub id: Uuid,
    pub project_id: Uuid,
    pub window_title: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_seconds: i64,
    pub is_manual: bool,
    pub is_idle: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Settings {
    pub id: Uuid,
    pub key: String,
    pub value: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectStatistics {
    pub project: Project,
    pub total_duration: i64,
    pub entry_count: i64,
    pub last_activity: Option<DateTime<Utc>>,
    pub daily_breakdown: Vec<DailyBreakdown>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyBreakdown {
    pub date: String,
    pub duration: i64,
    pub entry_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingStatus {
    pub is_tracking: bool,
    pub current_project: Option<Project>,
    pub current_window: Option<String>,
    pub session_duration: i64,
    pub idle_time: i64,
    pub last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailySummary {
    pub date: String,
    pub total_duration: i64,
    pub project_breakdown: Vec<ProjectDuration>,
    pub entries_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDuration {
    pub project: Project,
    pub duration: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub format: ExportFormat,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub project_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportFormat {
    CSV,
    JSON,
    Excel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub title: String,
    pub process_name: String,
    pub process_id: u32,
    pub is_active: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRule {
    pub id: Uuid,
    pub name: String,
    pub pattern: String,
    pub pattern_type: PatternType,
    pub project_id: Uuid,
    pub priority: i32,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Regex,
    Contains,
    StartsWith,
    EndsWith,
    Exact,
}

impl Default for Project {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            name: "Unknown Project".to_string(),
            code: "UNKNOWN".to_string(),
            color: None,
            is_active: true,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Default for TimeEntry {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            project_id: Uuid::new_v4(),
            window_title: String::new(),
            start_time: Utc::now(),
            end_time: None,
            duration_seconds: 0,
            is_manual: false,
            is_idle: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            key: String::new(),
            value: String::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}