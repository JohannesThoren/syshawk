mod probe;
mod models;
mod database;
mod routes;
mod templates;

use syshawk_templating;

#[macro_use]
extern crate rocket;

use std::thread;
use rocket::{fs, Ignite, Rocket};
use crate::probe::probe::handle_probes;
use anyhow::Result;
use sqlx::SqlitePool;

#[rocket::main]
async fn main() -> Result<()>{
    let _probe_handle_thread = thread::spawn(move || { handle_probes() });

    let pool = SqlitePool::connect("sqlite://database.db").await?;

    rocket::build()
        .mount("/", fs::FileServer::from("./page"))
        .mount("/ssr", routes![routes::ssr::home, routes::ssr::header, routes::ssr::sysinfo, routes::ssr::sysinfo_id])
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}