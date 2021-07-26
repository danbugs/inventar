#[macro_use] extern crate diesel;
#[macro_use] extern crate serde_derive;

use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

pub mod db;

pub fn establish_connection () -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
