mod credientals;
mod habit;
mod user;

#[macro_use]
extern crate rocket;
use crate::credientals::{Credentials, Sha3Hashable};
use crate::habit::{Color, Habit, Interval};
use crate::user::User;
use rocket::config::Config;
use rocket::response::status::NotFound;
use rocket::serde::{json::Json, Serialize};
use rocket::State;
use std::net::IpAddr;
use std::str::FromStr;
use std::sync::Mutex;

#[derive(Debug, Serialize)]
struct Token {
    token: String,
}

#[derive(Debug, Default)]
struct Users {
    users: Mutex<Vec<User>>,
}

#[get("/habits")]
fn get_habits() -> Json<Vec<Habit>> {
    let habits = vec![];
    Json(habits.to_vec())
}

#[get("/user")]
fn get_user(state: &State<Users>) -> Json<User> {
    let users = state.users.lock().unwrap();
    dbg!(&users);
    Json(users.first().unwrap().clone())
}

#[patch("/user", data = "<user>")]
fn patch_user(state: &State<Users>, user: Json<User>) -> Json<User> {
    let mut users = state.users.lock().unwrap();
    let mut local_user = users.first_mut().unwrap();
    local_user.email = user.email.clone();
    Json(local_user.clone())
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
fn login(users: &State<Users>, cred: Json<Credentials>) -> Result<Json<User>, NotFound<&'static str>> {

    dbg!(&cred);

    let users = users.users.lock().unwrap();
    if let Some(user) = users.iter().find(|a| a.email == cred.email) {
       if user.password_hash == cred.password.sha3() {
            return Ok(Json(user.clone()));
       }
       return Err(NotFound("Invalid password"))
    }

    Err(NotFound("User not found"))
}

#[launch]
fn rocket() -> _ {
    let mut config = Config::debug_default();

    config.address = IpAddr::from_str("0.0.0.0").unwrap();
    config.port = 8000;

    rocket::custom(config)
        .mount(
            "/",
            routes![login, get_user, patch_user, register, get_habits],
        )
        .manage(Users::default())
}
