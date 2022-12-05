mod schema;
mod database;
mod models;
mod routes;

use database::Database;
use rocket::{routes, launch};

use routes::index;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Database::fairing())
        .mount("/", routes![
            index::register,
            index::login,
        ])
}
