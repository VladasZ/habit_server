#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::Deserialize;

use rocket::http::RawStr;
use rocket_contrib::json::Json;


#[derive(Debug, Deserialize)]
struct LoginData {
    pub email: String,
    pub password: String
}

#[post("/login", data = "<task>")]
fn login(task: Json<LoginData>) -> String {
    format!("{:?}", task)
}

#[get("/users/<id>")]
pub fn users(id: u8) -> String {
    format!("Hello, {}!", id)
}

#[get("/hello")]
pub fn hello() -> &'static str {
    "Hello, outside world!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello, users, login]).launch();
}
