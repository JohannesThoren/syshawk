use sqlx::SqlitePool;
use syshawklib::cpu::Cpu;
use syshawklib::memory::Memory;
use syshawklib::system::System;
use crate::models::history_row::HistoryRow;
use anyhow::Result;
use rocket::log::private::log;

pub async fn insert_history(pool: &SqlitePool, probe_id: String, system: Option<System>, status: crate::models::status::Status) {
    let dt = chrono::Local::now();

    sqlx::query("insert into history (probe_id, system_info, time_stamp, status_code) values ($1, $2, $3, $4)")
        .bind(probe_id.clone())
        .bind(serde_json::to_string(&system).unwrap())
        .bind(dt)
        .bind(status.to_u8())
        .execute(pool).await.unwrap();
}

pub async fn fetch_latest_by_id(id: String, pool: &SqlitePool) -> Result<HistoryRow> {
    let res: HistoryRow = sqlx::query_as("select * from history where probe_id = $1 order by time_stamp desc ").bind(id).fetch_one(pool).await?;
    return Ok(res);
}
