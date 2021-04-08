#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use serde::{ Serialize, Deserialize };

use rocket_contrib::json::Json;
use std::convert::TryFrom;
use rocket_contrib::databases::diesel;

// #[database("sqlite_logs")]
// struct Db(diesel::SqliteConnection);

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Animal {
    pub mood: u32,
    pub hunger: u32,
    pub health: u32,

    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub animal: Animal,
    pub name: String,
    pub age: u8,
    pub email: String
}

#[post("/login", data = "<data>")]
fn login(mut data: Json<LoginData>) -> Json<LoginData> {
    println!("{:?}", data);
    data.email = "kok".to_string();
    data.password = "sos".to_string();
    data
}

// #[post("/login", data = "<data>")]
// fn register(db: Db, data: Json<LoginData>) -> Json<LoginData> {
//     //db.in
//
//     data
// }

#[get("/user")]
pub fn user() -> Json<User> {
    let mut user = User {
        animal: Animal {
            mood: 80,
            hunger: 30,
            health: 95,
            name: "Gragdans".to_string()
        },
        name: "Kotitka".to_string(),
        age: 23,
        email: "kotitka@gmail.com".to_string()
    };
    Json(user)
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
    //    .attach(Db::fairing())
        .mount("/", routes![hello, user, login])
        .launch();
}
