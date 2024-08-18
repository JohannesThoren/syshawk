use rocket::serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize, Serialize, Clone, Debug, FromRow)]
pub struct Probe {
    pub id: String,
    pub url: String,
}

