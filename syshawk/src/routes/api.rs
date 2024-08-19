use crate::database::history::fetch_n_latest_by_id;
use crate::database::{history::fetch_latest_by_id, probe::fetch_probes};
use crate::models::history_row::{HistoryRow, HistoryRowReturnData};
use chrono::{DateTime, Utc};
use rocket::response::content::RawJson;
use rocket::response::Redirect;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use syshawklib::system;

#[get("/sysinfo/<id>")]
pub async fn sysinfo_by_id_api(
    id: &str,
    pool: &State<SqlitePool>,
) -> Result<RawJson<String>, Redirect> {
    match fetch_latest_by_id(id.to_string(), pool.inner()).await {
        Ok(r) => {
            return Ok(RawJson(serde_json::to_string(&r.to_return_data()).unwrap()));
        }
        Err(_) => Err(Redirect::to("/error/500.html")),
    }
}

#[get("/sysinfo")]
pub async fn sysinfo_api(pool: &State<SqlitePool>) -> Result<RawJson<String>, Redirect> {
    let probes = fetch_probes(pool.inner()).await.unwrap();
    let mut rows: Vec<HistoryRowReturnData> = Vec::new();

    for probe in probes {
        let r = match fetch_latest_by_id(probe.id.to_string(), pool.inner()).await {
            Ok(r) => r.to_return_data(),
            Err(_) => return Err(Redirect::to("/error/500.html")),
        };

        rows.push(r)
    }

    return Ok(RawJson(
        serde_json::to_string::<Vec<HistoryRowReturnData>>(&rows).unwrap(),
    ));
}

#[get("/sysinfo/<id>/history")]
pub async fn sysinfo_20_latest_by_id_api(
    id: &str,
    pool: &State<SqlitePool>,
) -> Result<RawJson<String>, Redirect> {
    let res = match fetch_n_latest_by_id(20, id.to_string(), pool.inner()).await {
        Ok(r) => r,
        Err(_) => return Err(Redirect::to("/error/500.html")),
    };

    return Ok(RawJson(
        serde_json::to_string::<Vec<HistoryRowReturnData>>(
            &res.iter()
                .map(|r| return r.clone().to_return_data())
                .collect(),
        )
        .unwrap(),
    ));
}
