use argon2::{password_hash::SaltString, Params, Argon2, Algorithm, Version, PasswordHasher, PasswordHash, PasswordVerifier};
use rand_core::OsRng;
use rocket::{post, http::{CookieJar, Status, Cookie}, serde::json::Json};

use crate::{database::Database, models::{UserDetails, User}};

#[post("/register", data = "<user_details>")]
pub async fn register(db: Database, cookies: &CookieJar<'_>, user_details: Json<UserDetails>) -> Json<User> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2_params = Params::new(15728, 2, 1, None).unwrap();
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::default(), argon2_params);
    let password_hash = argon2.hash_password(user_details.password.as_bytes(), &salt).unwrap().to_string();
    
    let user_details = UserDetails {
        password: password_hash,
        ..user_details.into_inner()
    };

    let user = db.create_user(user_details).await;
    cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
    Json(user)
}

#[post("/login", data = "<user_details>")]
pub async fn login(db: Database, cookies: &CookieJar<'_>, user_details: Json<UserDetails>) -> Status {
    let user_details = user_details.into_inner();
    let original_password = user_details.password.clone();

    match db.get_user(user_details).await {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password).unwrap();
            if Argon2::default().verify_password(original_password.as_bytes(), &parsed_hash).is_ok() {
                cookies.add_private(Cookie::new("user", serde_json::to_string(&user).unwrap()));
                return Status::Ok;
            }

            Status::NotFound
        }
        None => Status::NotFound
    }
}
