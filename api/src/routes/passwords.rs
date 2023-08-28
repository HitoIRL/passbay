use log::info;
use poem::{IntoEndpoint, Route, handler, web::{Data, Json, Path}, get};
use serde::Deserialize;
use sqlx::MySqlPool;

use crate::{models::Password, errors::ApiError};

#[handler]
async fn get_passwords(db: Data<&MySqlPool>) -> Result<Json<Vec<Password>>, ApiError> {
    let result = sqlx::query_as!(Password, "SELECT * FROM passwords")
        .fetch_all(db.0)
        .await;

    match result {
        Ok(passwords) => {
            info!("Passwords retrieved from database: {passwords:?}");
            Ok(Json(passwords))
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[handler]
async fn get_password(db: Data<&MySqlPool>, Path(filter): Path<String>) -> Result<Json<Vec<Password>>, ApiError> {
    let like = format!("%{filter}%"); // for some reason I can't use % in the query itself
    let result = sqlx::query_as!(Password, "SELECT * FROM passwords WHERE username LIKE ? OR website LIKE ?", like, like)
        .fetch_all(db.0)
        .await;

    match result {
        Ok(password) => {
            info!("Passwords retrieved from database: {password:?}");
            Ok(Json(password))
        }
        Err(_) => Err(ApiError::NoPassword),
    }
}

#[derive(Debug, Deserialize)]
struct NewPassword {
    username: String,
    password: String,
    website: String,
}

#[handler]
async fn new_password(db: Data<&MySqlPool>, Json(password): Json<NewPassword>) -> Result<(), ApiError> {
    let result = sqlx::query!("INSERT INTO passwords (username, password, website) VALUES (?, ?, ?)", password.username, password.password, password.website)
        .execute(db.0)
        .await;

    match result {
        Ok(_) => {
            info!("New password added to database: {password:?}");
            Ok(())
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

#[handler]
async fn remove_password(db: Data<&MySqlPool>, Path(id): Path<i32>) -> Result<(), ApiError> {
    let result = sqlx::query!("DELETE FROM passwords WHERE id = ?", id)
        .execute(db.0)
        .await;

    match result {
        Ok(_) => {
            info!("Password removed from database {{ id: {id} }}");
            Ok(())
        }
        Err(_) => Err(ApiError::InternalServerError),
    }
}

pub fn register_routes() -> impl IntoEndpoint {
    Route::new()
        .at("/", get(get_passwords).post(new_password))
        .at("/:id", get(get_password).delete(remove_password))
}
