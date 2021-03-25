#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use serde::{ Serialize, Deserialize };

use rocket_contrib::json::Json;


#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    pub email: String,
    pub password: String
}

#[post("/login", data = "<task>")]
fn login(mut task: Json<LoginData>) -> Json<LoginData> {
    println!("{:?}", task);
    task.email = "kok".to_string();
    task.password = "sos".to_string();
    task
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

    let cfg = rocket::config::Config::build(rocket::config::Environment::Development)
        .address("127.0.0.1")
        .port(80)
        .unwrap();

    rocket::custom(cfg)
        .mount("/", routes![hello, users, login])
        .launch();
}
