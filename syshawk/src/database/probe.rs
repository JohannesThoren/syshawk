use sqlx::SqlitePool;
use anyhow::Result;
use crate::models::probe::Probe;

pub async fn fetch_probes(pool: &SqlitePool) -> Result<Vec<Probe>> {
    let result: Vec<Probe> = sqlx::query_as("select * from probes").fetch_all(pool).await?;
    return Ok(result);
}

