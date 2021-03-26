#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use serde::{ Serialize, Deserialize };

use rocket_contrib::json::Json;
use rocket_contrib::databases::diesel;

// #[database("sqlite_logs")]
// struct Db(diesel::SqliteConnection);

#[derive(Debug, Serialize, Deserialize)]
struct LoginData {
    pub email: String,
    pub password: String
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
        .port(8000)
        .unwrap();

    rocket::custom(cfg)
    //    .attach(Db::fairing())
        .mount("/", routes![hello, users, login])
        .launch();
}
