#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::Method;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

pub mod constants;

pub mod handlers;
use self::handlers::things;
use self::handlers::catchers;

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&[constants::PROD_ADDRESS, constants::DEV_ADDRESS]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Failed");

    rocket::ignite()
        .mount("/things", routes![things::create_thing, things::read_things, things::delete_thing])
        .register(catchers![catchers::not_found])
        .attach(cors)
        .launch();
}
