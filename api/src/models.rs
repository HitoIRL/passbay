use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Queryable, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct UserDetails {
    pub username: String,
    pub password: String,
}

pub struct Login {
    pub user_id: i32,
    pub username: String,
    pub password: String,
}
