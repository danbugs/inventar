use diesel::prelude::*;
use inventar_lib::{
    establish_connection,
    models::users::{LoginUser, NewUser, RegisteringUser, User},
    schema::users::{self, dsl::*},
};
use rocket::http::Status;
use rocket_contrib::json::Json;

use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use dotenv::dotenv;
use std::env;

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

#[get("/send_mail_to/<email>")]
pub fn send_mail_to(email: String) -> Result<Status, Status> {
    
    let email = Message::builder()
        .from("Inventar <donotreply.inventar@gmail.com>".parse().unwrap())
        .to(email.parse().unwrap())
        .subject("Verify Your Account")
        .body(String::from("SOME CODE HERE"))
        .unwrap();
    
    dotenv().ok();
    let creds = Credentials::new(env::var("STMP_USERNAME").expect("STMP_USERNAME must be set"), env::var("STMP_PASSWORD").expect("STMP_PASSWORD must be set"));
    
    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();
    
    // Send the email
    match mailer.send(&email) {
        Ok(_) => Ok(Status::Created),
        Err(_) => Err(Status::BadRequest),
    }
}