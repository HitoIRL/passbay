use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Password {
    pub id: u64,
    pub username: String,
    pub password: String,
    pub website: String,
}
