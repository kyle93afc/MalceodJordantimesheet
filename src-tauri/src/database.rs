use sqlx::{SqlitePool, Row};
use anyhow::Result;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::path::PathBuf;

use crate::models::*;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let data_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("modern-timesheet-tracker");
        
        tokio::fs::create_dir_all(&data_dir).await?;
        
        let db_path = data_dir.join("timesheet.db");
        let database_url = format!("sqlite://{}", db_path.to_string_lossy());
        
        let pool = SqlitePool::connect(&database_url).await?;
        
        let db = Database { pool };
        db.migrate().await?;
        
        Ok(db)
    }

    async fn migrate(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                code TEXT NOT NULL UNIQUE,
                color TEXT,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS time_entries (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                window_title TEXT NOT NULL,
                start_time TEXT NOT NULL,
                end_time TEXT,
                duration_seconds INTEGER NOT NULL DEFAULT 0,
                is_manual BOOLEAN NOT NULL DEFAULT FALSE,
                is_idle BOOLEAN NOT NULL DEFAULT FALSE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects (id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS settings (
                id TEXT PRIMARY KEY,
                key TEXT NOT NULL UNIQUE,
                value TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS project_rules (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                pattern TEXT NOT NULL,
                pattern_type TEXT NOT NULL,
                project_id TEXT NOT NULL,
                priority INTEGER NOT NULL DEFAULT 0,
                is_active BOOLEAN NOT NULL DEFAULT TRUE,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (project_id) REFERENCES projects (id)
            );
            "#,
        )
        .execute(&self.pool)
        .await?;

        // Insert default project if none exists
        let project_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects")
            .fetch_one(&self.pool)
            .await?;

        if project_count == 0 {
            let default_project = Project {
                id: Uuid::new_v4(),
                name: "General".to_string(),
                code: "GENERAL".to_string(),
                color: Some("#6366f1".to_string()),
                is_active: true,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            self.create_project(&default_project).await?;
        }

        Ok(())
    }

    pub async fn create_project(&self, project: &Project) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO projects (id, name, code, color, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(project.id.to_string())
        .bind(&project.name)
        .bind(&project.code)
        .bind(&project.color)
        .bind(project.is_active)
        .bind(project.created_at.to_rfc3339())
        .bind(project.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_projects(&self) -> Result<Vec<Project>> {
        let rows = sqlx::query("SELECT * FROM projects ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        let mut projects = Vec::new();
        for row in rows {
            projects.push(Project {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                name: row.get("name"),
                code: row.get("code"),
                color: row.get("color"),
                is_active: row.get("is_active"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
            });
        }

        Ok(projects)
    }

    pub async fn get_project_by_code(&self, code: &str) -> Result<Option<Project>> {
        let row = sqlx::query("SELECT * FROM projects WHERE code = ? LIMIT 1")
            .bind(code)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Some(Project {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                name: row.get("name"),
                code: row.get("code"),
                color: row.get("color"),
                is_active: row.get("is_active"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
            })),
            None => Ok(None),
        }
    }

    pub async fn create_time_entry(&self, entry: &TimeEntry) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO time_entries (id, project_id, window_title, start_time, end_time, duration_seconds, is_manual, is_idle, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(entry.id.to_string())
        .bind(entry.project_id.to_string())
        .bind(&entry.window_title)
        .bind(entry.start_time.to_rfc3339())
        .bind(entry.end_time.map(|t| t.to_rfc3339()))
        .bind(entry.duration_seconds)
        .bind(entry.is_manual)
        .bind(entry.is_idle)
        .bind(entry.created_at.to_rfc3339())
        .bind(entry.updated_at.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn update_time_entry(&self, entry: &TimeEntry) -> Result<()> {
        sqlx::query(
            r#"
            UPDATE time_entries 
            SET end_time = ?, duration_seconds = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(entry.end_time.map(|t| t.to_rfc3339()))
        .bind(entry.duration_seconds)
        .bind(entry.updated_at.to_rfc3339())
        .bind(entry.id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_time_entries(&self, start_date: Option<DateTime<Utc>>, end_date: Option<DateTime<Utc>>) -> Result<Vec<TimeEntry>> {
        let mut query = "SELECT * FROM time_entries WHERE 1=1".to_string();
        let mut params = Vec::new();

        if let Some(start) = start_date {
            query.push_str(" AND start_time >= ?");
            params.push(start.to_rfc3339());
        }

        if let Some(end) = end_date {
            query.push_str(" AND start_time <= ?");
            params.push(end.to_rfc3339());
        }

        query.push_str(" ORDER BY start_time DESC");

        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        let mut entries = Vec::new();
        for row in rows {
            entries.push(TimeEntry {
                id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                project_id: Uuid::parse_str(&row.get::<String, _>("project_id"))?,
                window_title: row.get("window_title"),
                start_time: DateTime::parse_from_rfc3339(&row.get::<String, _>("start_time"))?.with_timezone(&Utc),
                end_time: row.get::<Option<String>, _>("end_time")
                    .map(|s| DateTime::parse_from_rfc3339(&s).unwrap().with_timezone(&Utc)),
                duration_seconds: row.get("duration_seconds"),
                is_manual: row.get("is_manual"),
                is_idle: row.get("is_idle"),
                created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
            });
        }

        Ok(entries)
    }

    pub async fn get_setting(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM settings WHERE key = ? LIMIT 1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.map(|row| row.get("value")))
    }

    pub async fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            INSERT INTO settings (id, key, value, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                updated_at = excluded.updated_at
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(key)
        .bind(value)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}