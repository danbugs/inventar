use diesel::prelude::*;
use inventar_lib::{establish_connection, db::models::{NewThing, Thing}, db::schema::things::{self, dsl::*}};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<t>")]
pub fn create_thing(t: Json<NewThing>) -> Result<Status, Status> {

    let connection = establish_connection();
    diesel::insert_into(things::table)
        .values(t.into_inner())
        .execute(&connection)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[get("/")]
pub fn read_things() -> Result<Json<Vec<Thing>>, Status> {
    let connection = establish_connection();
    things.load::<Thing>(&connection)
        .map(|ts| Json(ts))
        .map_err(|_| Status::InternalServerError)

}


#[delete("/<t_id>")]
pub fn delete_thing(t_id: i32) -> Result<Status, Status> {

    let connection = establish_connection();
    diesel::delete(things.filter(thing_id.eq(t_id)))
        .execute(&connection)
        .map(|_| Status::Ok)
        .map_err(|_| Status::InternalServerError)

}
