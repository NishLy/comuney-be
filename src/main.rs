use models::request::user_login::UserLoginRequest;
use rocket::serde::json::Json;
pub mod models;
pub mod schema;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/login/<name>")]
fn login(name: &str) -> Json<UserLoginRequest> {
    let user_from_id = UserLoginRequest {
        email: name.to_string(),
        password: "password".to_string(),
    };
    /* ... */
    Json(user_from_id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, login])
}
