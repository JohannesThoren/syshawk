use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use syshawklib::system;

#[derive(FromRow, Debug, Clone, Serialize, Deserialize)]
pub struct HistoryRow {
    pub probe_id: String,
    pub system_info: String,
    pub time_stamp: chrono::DateTime<Utc>,
    pub status_code: u64,
}

impl HistoryRow {
    pub fn to_return_data(self) -> HistoryRowReturnData {
        HistoryRowReturnData {
            probe_id: self.probe_id,
            system_info: match serde_json::from_str(self.system_info.as_str()) {
                Ok(r) => r,
                Err(_) => None,
            },
            time_stamp: self.time_stamp,
            status_code: self.status_code,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HistoryRowReturnData {
    pub probe_id: String,
    pub system_info: Option<system::System>,
    pub time_stamp: DateTime<Utc>,
    pub status_code: u64,
}
