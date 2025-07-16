use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        Self { start, end }
    }
    
    pub fn duration(&self) -> Duration {
        self.end - self.start
    }
    
    pub fn duration_seconds(&self) -> i64 {
        self.duration().num_seconds()
    }
    
    pub fn contains(&self, time: DateTime<Utc>) -> bool {
        time >= self.start && time <= self.end
    }
    
    pub fn overlaps(&self, other: &TimeRange) -> bool {
        self.start < other.end && other.start < self.end
    }
}

pub fn format_duration(seconds: i64) -> String {
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

pub fn format_duration_short(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;
    
    if hours > 0 {
        format!("{}h {}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

pub fn get_today_range() -> TimeRange {
    let now = Utc::now();
    let start = now.date_naive().and_hms_opt(0, 0, 0).unwrap().and_utc();
    let end = now.date_naive().and_hms_opt(23, 59, 59).unwrap().and_utc();
    
    TimeRange::new(start, end)
}

pub fn get_week_range() -> TimeRange {
    let now = Utc::now();
    let days_from_monday = now.weekday().num_days_from_monday();
    
    let start = (now - Duration::days(days_from_monday as i64))
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    
    let end = (start + Duration::days(7))
        .date_naive()
        .and_hms_opt(23, 59, 59)
        .unwrap()
        .and_utc();
    
    TimeRange::new(start, end)
}

pub fn get_month_range() -> TimeRange {
    let now = Utc::now();
    let start = now.date_naive()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc();
    
    let next_month = if now.month() == 12 {
        now.with_year(now.year() + 1).unwrap().with_month(1).unwrap()
    } else {
        now.with_month(now.month() + 1).unwrap()
    };
    
    let end = next_month.date_naive()
        .with_day(1)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .and_utc()
        - Duration::seconds(1);
    
    TimeRange::new(start, end)
}

pub fn sanitize_filename(filename: &str) -> String {
    filename
        .chars()
        .map(|c| match c {
            '<' | '>' | ':' | '"' | '/' | '\\' | '|' | '?' | '*' => '_',
            _ => c,
        })
        .collect()
}

pub fn generate_export_filename(format: &str, start_date: Option<&str>, end_date: Option<&str>) -> String {
    let now = Utc::now();
    let timestamp = now.format("%Y%m%d_%H%M%S");
    
    match (start_date, end_date) {
        (Some(start), Some(end)) => {
            format!("timesheet_{}_{}.{}", start, end, format.to_lowercase())
        }
        (Some(start), None) => {
            format!("timesheet_from_{}.{}", start, format.to_lowercase())
        }
        (None, Some(end)) => {
            format!("timesheet_until_{}.{}", end, format.to_lowercase())
        }
        (None, None) => {
            format!("timesheet_export_{}.{}", timestamp, format.to_lowercase())
        }
    }
}

pub fn parse_project_code(window_title: &str) -> Option<String> {
    // Common project code patterns
    let patterns = vec![
        // Basic 5-6 digit project numbers
        regex::Regex::new(r"(\d{5,6})").ok()?,
        // Complex project formats (e.g., 240378-M+J-V1-XX-DR-S-30-01-C)
        regex::Regex::new(r"(\d{6}-[A-Z]+\+[A-Z]+-V\d+-[A-Z]{2}-[A-Z]{2}-[A-Z]+-\d{2}-\d{2}-[A-Z]+)").ok()?,
        // File path patterns
        regex::Regex::new(r"W:\\02-PROJECTS\\(\d{6})").ok()?,
        regex::Regex::new(r"\\\\.*\\.*\\(\d{6})").ok()?,
    ];
    
    for pattern in patterns {
        if let Some(captures) = pattern.captures(window_title) {
            if let Some(matched) = captures.get(1) {
                return Some(matched.as_str().to_string());
            }
        }
    }
    
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(3661), "1h 01m 01s");
        assert_eq!(format_duration(61), "1m 01s");
        assert_eq!(format_duration(30), "30s");
    }
    
    #[test]
    fn test_parse_project_code() {
        assert_eq!(parse_project_code("Project 123456 - Document"), Some("123456".to_string()));
        assert_eq!(parse_project_code("240378-M+J-V1-XX-DR-S-30-01-C"), Some("240378-M+J-V1-XX-DR-S-30-01-C".to_string()));
        assert_eq!(parse_project_code("W:\\02-PROJECTS\\240378\\file.txt"), Some("240378".to_string()));
        assert_eq!(parse_project_code("No project code here"), None);
    }
    
    #[test]
    fn test_sanitize_filename() {
        assert_eq!(sanitize_filename("file<>name.txt"), "file__name.txt");
        assert_eq!(sanitize_filename("normal_file.txt"), "normal_file.txt");
    }
}