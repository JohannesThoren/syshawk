use std::{thread};
use rocket::tokio::time;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};
use tokio::runtime::Runtime;
use syshawklib;
use anyhow::Result;
use syshawklib::system::System;
use crate::database::history::insert_history;
use crate::database::probe::fetch_probes;
use crate::models::probe::Probe;
use crate::models::status::Status;


pub fn handle_probes() -> Result<()> {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let pool = match SqlitePool::connect("sqlite://database.db").await {
            Ok(r) => { r }
            Err(e) => {
                eprintln!("{e}");
                panic!()
            }
        };
        let probes = fetch_probes(&pool).await.unwrap();

        loop {
            let probe_info = fetch_info_from_probes(&probes).await.unwrap();
            debug!("{:?}", probe_info);

            for pi in &probe_info {
                match pi.1 {
                    None => { insert_history(&pool, pi.0.clone(), pi.1.clone(), Status::UNREACHABLE).await }
                    Some(_) => { insert_history(&pool, pi.0.clone(), pi.1.clone(), Status::SUCCESS).await }
                }
            }


            tokio::time::sleep(time::Duration::from_secs(5)).await;
        }
    });

    return Ok(());
}

pub async fn fetch_info_from_probes(probes: &Vec<Probe>) -> Result<Vec<(String, Option<syshawklib::system::System>)>> {
    let mut sys_info: Vec<(String, Option<syshawklib::system::System>)> = Vec::new();

    for probe in probes {
        match reqwest::get(probe.url.clone()).await {
            Ok(r) => {
                match r.json::<syshawklib::system::System>().await {
                    Ok(system) => {
                        sys_info.push((probe.id.clone(), Some(system)))
                    }
                    Err(_) => {
                        sys_info.push((probe.id.clone(), None))
                    }
                }
            }
            Err(_) => {
                sys_info.push((probe.id.clone(), None))
            }
        };
    }

    return Ok(sys_info);
}
