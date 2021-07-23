#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use self::models::{Thing, NewThing};

pub mod schema;
pub mod models;

pub fn establish_connection () -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn insert_thing<'a>(conn: &PgConnection, name: &'a str) -> Thing {
    use schema::things;

    let new_thing = NewThing {
        thing_name: name.to_string()
    };

    diesel::insert_into(things::table)
        .values(&new_thing)
        .get_result(conn)
        .expect("Error saving new thing")
}