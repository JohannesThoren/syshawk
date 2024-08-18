use crate::database::{history::fetch_latest_by_id, probe::fetch_probes};
use crate::models::history_row::HistoryRow;
use chrono::{DateTime, Utc};
use rocket::response::content::RawJson;
use rocket::response::Redirect;
use rocket::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use syshawklib::system;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct HistoryRowReturnData {
    pub probe_id: String,
    pub system_info: Option<system::System>,
    pub time_stamp: DateTime<Utc>,
    pub status_code: u64,
}

#[get("/sysinfo/<id>")]
pub async fn sysinfo_by_id_api(
    id: &str,
    pool: &State<SqlitePool>,
) -> Result<RawJson<String>, Redirect> {
    match fetch_latest_by_id(id.to_string(), pool.inner()).await {
        Ok(r) => {
            let hr = HistoryRowReturnData {
                probe_id: r.probe_id,
                system_info: match serde_json::from_str(&r.system_info) {
                    Ok(r) => r,
                    Err(_) => None,
                },
                time_stamp: r.time_stamp,
                status_code: r.status_code,
            };

            return Ok(RawJson(serde_json::to_string(&hr).unwrap()));
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
            Ok(r) => {
                let hr = HistoryRowReturnData {
                    probe_id: r.probe_id,
                    system_info: match serde_json::from_str(&r.system_info) {
                        Ok(r) => r,
                        Err(_) => None,
                    },
                    time_stamp: r.time_stamp,
                    status_code: r.status_code,
                };

                hr
            }
            Err(_) => return Err(Redirect::to("/error/500.html")),
        };

        rows.push(r)
    }

    return Ok(RawJson(
        serde_json::to_string::<Vec<HistoryRowReturnData>>(&rows).unwrap(),
    ));
}
