mod database;
mod models;
mod probe;
mod routes;
mod templates;

use syshawk_templating;

#[macro_use]
extern crate rocket;

use crate::probe::probe::handle_probes;
use anyhow::Result;
use rocket::{fs, http::Method, Ignite, Rocket};
use sqlx::SqlitePool;
use std::thread;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[rocket::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let _probe_handle_thread = thread::spawn(move || handle_probes());

    let pool = SqlitePool::connect("sqlite://database.db").await?;

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .allowed_methods(
            vec![Method::Get, Method::Post]
                .into_iter()
                .map(From::from)
                .collect(),
        )
        .allow_credentials(true)
        .to_cors()?;

    rocket::build()
        .attach(cors)
        .mount("/", fs::FileServer::from("./page"))
        .mount(
            "/ssr",
            routes![
                routes::ssr::home,
                routes::ssr::header,
                routes::ssr::sysinfo,
                routes::ssr::sysinfo_id
            ],
        )
        .mount(
            "/api/v1",
            routes![routes::api::sysinfo_by_id_api, routes::api::sysinfo_api],
        )
        .manage(pool)
        .launch()
        .await?;

    Ok(())
}
