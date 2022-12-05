use rocket_sync_db_pools::{diesel, database};
use diesel::prelude::*;

use crate::models::{UserDetails, User};

#[database("passbay")]
pub struct Database(diesel::PgConnection);

impl Database {
    pub async fn get_user(&self, user_details: UserDetails) -> Option<User> {
        use crate::schema::users::dsl::*;

        self.run(move |conn| {
            users
                .filter(username.eq(user_details.username))
                .first(conn)
                .optional()
                .unwrap()
        }).await
    }

    pub async fn create_user(&self, user_details: UserDetails) -> User {
        use crate::schema::users;

        self.run(move |conn| {
            diesel::insert_into(users::table)
                .values(&user_details)
                .get_result(conn)
                .unwrap()
        }).await
    }
}
