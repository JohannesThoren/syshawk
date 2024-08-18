use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRow {
    pub probe_id: String,
    pub system_info: String,
    pub time_stamp: chrono::DateTime<Utc>,
    pub status_code: u64,
}