use diesel::prelude::*;
use inventar_lib::{
    establish_connection,
    models::users::{LoginUser, NewUser, RegisteringUser, User},
    schema::users::{self, dsl::*},
};
use rocket::{http::Status, response::content};
use rocket_contrib::json::Json;

use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
};
use lettre::{Message, SmtpTransport, Transport};

use dotenv::dotenv;
use std::env;

use maud::html;

#[post("/register", data = "<ru>")]
pub fn register(ru: Json<RegisteringUser>) -> Result<Status, Status> {
    let nu = NewUser::new(ru.into_inner());
    let connection = establish_connection();

    // check if username already exists
    match users
        .filter(
            user_name
                .eq(&nu.user_name)
                .or(user_email.eq(&nu.user_email)),
        )
        .first::<User>(&connection)
    {
        Ok(_) => {
            return Ok(Status::Accepted);
        }
        Err(_) => {}
    }

    dotenv().ok();
    
    let host = env::var("DEV_OR_PROD").expect("DEV_OR_PROD must be set");

    let verification_link = format!(
        "{}/users/verify/{}/{}",
        host, nu.user_name, nu.verification_code
    );

    let html = html! {
        head {
            title { "Hello from Inventar!" }
        }
        div {
            h2 { "Hello from Inventar!" }
            // Substitute in the name of our recipient.
            p { "Dear " (nu.user_name) "," }
            p {
                "Click "
                a href=(verification_link) { "here" }
                " to verify your account!"
            }
        }
    };

    let email = Message::builder()
        .from("Inventar <donotreply.inventar@gmail.com>".parse().unwrap())
        .to(nu.user_email.parse().unwrap())
        .subject("Verify Your Account")
        .multipart(
            MultiPart::alternative()
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_PLAIN)
                        .body(String::from("Hello from Inventar!")),
                )
                .singlepart(
                    SinglePart::builder()
                        .header(header::ContentType::TEXT_HTML)
                        .body(html.into_string()),
                ),
        )
        .expect("failed to build email");

    dotenv().ok();
    let creds = Credentials::new(
        env::var("STMP_USERNAME").expect("STMP_USERNAME must be set"),
        env::var("STMP_PASSWORD").expect("STMP_PASSWORD must be set"),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => diesel::insert_into(users::table)
            .values(nu)
            .execute(&connection)
            .map(|_| Status::Created)
            .map_err(|_| Status::InternalServerError),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[post("/login", data = "<lu>")]
pub fn login(lu: Json<LoginUser>) -> Result<Status, Status> {
    let connection = establish_connection();
    match users
        .filter(user_name.eq(&lu.user_name).and(verified_email.eq(true)))
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

#[get("/verify/<uname>/<vcode>", format = "html")]
pub fn verify(uname: String, vcode: String) -> content::Html<&'static str> {
    let connection = establish_connection();

    let success_msg = content::Html(
        r#"
        <div style="font-family:Arial;text-align:center;">
        <h1>You're verified!</h1>
        <h3">Prooced to <a href="https://inventar.vercel.app">Inventar</a> to login!</h3>
        </div>"#
);

    let error_msg = content::Html(
        r#"
        <div style="font-family:Arial;text-align:center;">
        <h1>Something went wrong!</h1>
        <h3>Try re-registering at <a href="https://inventar.vercel.app">Inventar</a>!</h3>
        </div>"#
    );
    match users
        .filter(user_name.eq(uname).and(verification_code.eq(vcode)))
        .first::<User>(&connection)
    {
        Ok(user) => {
            match diesel::update(users.filter(user_id.eq(user.user_id)))
                .set(verified_email.eq(true))
                .execute(&connection)
            {
                Ok(_) => success_msg,
                Err(_) => error_msg
            }
        }
        Err(_) => error_msg
    }
}
