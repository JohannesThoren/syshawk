use std::future::Future;
use rocket::response::content::RawHtml;
use rocket::response::Redirect;
use rocket::State;
use sqlx::{Pool, SqlitePool};
use syshawk_templating::html::div;
use syshawk_templating::node::Node;
use crate::templates::header::header_template;
use crate::templates::sysinfo::{sys_info_card, sys_info_template};

#[get("/home")]
pub async fn home(pool: &State<SqlitePool>) -> Result<RawHtml<String>, Redirect> {
    let home = match sys_info_template(pool.inner()).await {
        Ok(r) => { r }
        Err(_) => { return Err(Redirect::to("/error/500")) }
    };

    Ok(RawHtml(home.to_html_string()))
}

#[get("/header")]
pub fn header() -> RawHtml<String> {
    RawHtml(header_template())
}

#[get("/sysinfo")]
pub async fn sysinfo(pool: &State<SqlitePool>) -> Result<RawHtml<String>, Redirect> {
    return match sys_info_template(pool.inner()).await {
        Ok(r) => { Ok(RawHtml(r.to_html_string())) }
        Err(_) => { Err(Redirect::to("/error/500.html")) }
    };
}

#[get("/sysinfo/<id>")]
pub async fn sysinfo_id(id: String, pool: &State<SqlitePool>) -> Result<RawHtml<String>, Redirect> {
    return Ok(RawHtml(sys_info_card(id, pool).await.to_html_string()));
}