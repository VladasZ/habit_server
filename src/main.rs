#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::sync::Mutex;

#[derive(Debug, Deserialize)]
struct Credentials {
    login: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    login: String,
    password: String,
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

    let len = users.len();

    users.push(User {
        id: len as u32,
        login: cred.login.clone(),
        password: cred.password.clone(),
    });
    dbg!(&cred);
    Json::from(Token {
        token: "kok".to_string(),
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
    rocket::build()
        .mount("/", routes![login, users, register])
        .manage(Users {
            users: Mutex::new(vec![]),
        })
}
