use diesel::prelude::*;
use inventar_lib::{establish_connection, models::categories::{Category, IncomingCategory, NewCategory}, schema::categories::{self, dsl::*}, uid_from_uname};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[get("/<uname>")]
pub fn read_categories(uname: String) -> Result<Json<Vec<Category>>, Status> {
    let connection = establish_connection();
    categories
        .filter(user_id.eq(uid_from_uname(uname)))
        .get_results(&connection)
        .map(|cs| Json(cs))
        .map_err(|_| Status::InternalServerError)
}

#[post("/", data="<ic>")]
pub fn create_category(ic: Json<IncomingCategory>) -> Result<Json<Category>, Status> {
    let nc = NewCategory::new(ic.into_inner());
    let connection = establish_connection();
    diesel::insert_into(categories::table)
        .values(nc)
        .get_result(&connection)
        .map(|c| Json(c))
        .map_err(|_| Status::InternalServerError)
}
