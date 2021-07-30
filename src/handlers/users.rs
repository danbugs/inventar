use diesel::prelude::*;
use inventar_lib::{
    establish_connection,
    models::users::{LoginUser, NewUser, RegisteringUser, User},
    schema::users::{self, dsl::*},
};
use rocket::http::Status;
use rocket_contrib::json::Json;

#[post("/register", data = "<ru>")]
pub fn register(ru: Json<RegisteringUser>) -> Result<Status, Status> {
    let nu = NewUser::new(ru.into_inner());
    let connection = establish_connection();

    // check if username already exists
    match users
        .filter(user_name.eq(&nu.user_name).or(user_email.eq(&nu.user_email)))
        .first::<User>(&connection)
    {
        Ok(_) => {
            return Ok(Status::Accepted);
        }
        Err(_) => {}
    }

    diesel::insert_into(users::table)
        .values(nu)
        .execute(&connection)
        .map(|_| Status::Created)
        .map_err(|_| Status::InternalServerError)
}

#[post("/login", data = "<lu>")]
pub fn login(lu: Json<LoginUser>) -> Result<Status, Status> {
    let connection = establish_connection();
    match users
        .filter(user_name.eq(&lu.user_name))
        .first::<User>(&connection)
    {
        Ok(record) => {
            if record.equal(lu.into_inner()) {
                Ok(Status::Ok)
            } else {
                Err(Status::Unauthorized)
            }
        }
        Err(_) => Err(Status::Unauthorized),
    }
}
