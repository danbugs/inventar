use diesel::prelude::*;
use inventar_lib::{establish_connection, models::things::{IncomingThing, NewThing, Thing}, schema::things::{self, dsl::*}, uid_from_uname};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/", data = "<it>")]
pub fn create_thing(it: Json<IncomingThing>) -> Result<Status, Status> {
    let t = NewThing::new(it.into_inner());
    let connection = establish_connection();
    diesel::insert_into(things::table)
        .values(t)
        .execute(&connection)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[get("/<un>")]
pub fn read_things(un: String) -> Result<Json<Vec<Thing>>, Status> {
    let connection = establish_connection();
    things.filter(user_id.eq(uid_from_uname(un)))
        .get_results(&connection)
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
