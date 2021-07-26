#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use schema::users::dsl::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn uid_from_uname(un: String) -> i32 {
    let connection = establish_connection();
    users
        .filter(user_name.eq(un))
        .select(user_id)
        .first(&connection)
        .unwrap()
}
