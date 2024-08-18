use chrono::Utc;
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct HistoryRow {
    pub probe_id: String,
    pub system_info: String,
    pub time_stamp: chrono::DateTime<Utc>,
    pub status_code: u64,
}