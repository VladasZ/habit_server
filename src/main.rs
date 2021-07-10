mod credientals;
mod user;

#[macro_use]
extern crate rocket;
use crate::credientals::Credentials;
use crate::user::User;
use rocket::config::Config;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
struct Token {
    token: String,
}

#[derive(Debug)]
struct Users {
    users: Mutex<Vec<User>>,
}

#[get("/users")]
fn users(users: &State<Users>) {
    dbg!(users);
}

#[post("/register", data = "<cred>")]
fn register(users: &State<Users>, cred: Json<Credentials>) -> Json<Token> {
    let mut users = users.users.lock().unwrap();
    let user = User::new(users.len(), cred.into_inner());
    dbg!(&user);
    users.push(user);

    Json::from(Token {
        token: "koken".to_string(),
    })
}

#[post("/login", data = "<cred>")]
fn login(cred: Json<Credentials>) -> Json<Token> {
    dbg!(&cred);
    Json::from(Token {
        token: "kok".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    let mut config = Config::debug_default();

    config.address = IpAddr::from_str("0.0.0.0").unwrap();
    config.port = 80;

    rocket::custom(config)
        .mount("/", routes![login, users, register])
        .manage(Users {
            users: Mutex::new(vec![]),
        })
}
