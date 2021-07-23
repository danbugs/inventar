#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;
extern crate inventar_lib;

use rocket::http::RawStr;

use self::diesel::prelude::*;
use self::inventar_lib::*;
use self::models::*;

#[get("/")]
fn index () -> std::string::String {
    "Welcome to the inventar_api!".to_string()
}

#[post("/things/<name>")]
fn create_thing (name: &RawStr) {
    let connection = establish_connection();
    insert_thing(&connection, name);
}

#[get("/things")]
fn read_things () -> std::string::String {
    use inventar_lib::schema::things::dsl::*;

    let connection = establish_connection();
    let results = things
        .load::<Thing>(&connection)
        .expect("Error loading things");

    format!("You have created {} things.", results.len())
}

fn main() {
    rocket::ignite().mount("/", routes![index, create_thing, read_things]).launch();
}
