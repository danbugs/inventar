#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

pub mod models;
pub mod schema;

use schema::{users::dsl::*, categories::dsl::*};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn uid_from_uname(un: String) -> i32 {
    let connection = establish_connection();
    users
        .filter(user_name.eq(un))
        .select(schema::users::dsl::user_id)
        .first(&connection)
        .unwrap()
}

pub fn cid_from_cname(cn: Option<String>) -> Option<i32> {
    let connection = establish_connection();
    match cn {
        Some(cname) => {
            let cid = categories
            .filter(category_name.eq(cname))
            .select(category_id)
            .first(&connection)
            .unwrap();

            Some(cid)
        },
        None => None
    }
}