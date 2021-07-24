#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate diesel;
extern crate inventar_lib;

use rocket::http::{RawStr, Status};
use rocket::Request;
use rocket::http::Method;
use rocket_contrib::json::Json;

use rocket_cors::{AllowedHeaders, AllowedOrigins};

use self::diesel::prelude::*;
use self::inventar_lib::*;
use self::models::*;

#[get("/")]
fn index() -> std::string::String {
    "Welcome to the inventar_api!".to_string()
}

#[post("/things/<name>")]
fn create_thing(name: &RawStr) -> Result<Status, Status> {
    let connection = establish_connection();
    insert_thing(&connection, name)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[get("/things")]
fn read_things() -> Result<Json<Vec<Thing>>, Status> {
    use inventar_lib::schema::things::dsl::*;

    let connection = establish_connection();
    things.load::<Thing>(&connection)
        .map(|ts| Json(ts))
        .map_err(|_| Status::InternalServerError)

}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

fn main() {
    let allowed_origins = AllowedOrigins::some_exact(&["https://inventar.vercel.app/", "http://localhost:5000/"]);

    // You can also deserialize this
    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Delete, Method::Put].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().expect("Failed");

    rocket::ignite()
        .mount("/", routes![index, create_thing, read_things])
        .register(catchers![not_found])
        .attach(cors)
        .launch();
}
