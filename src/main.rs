mod credientals;
mod habit;
mod user;

#[macro_use]
extern crate rocket;
use crate::credientals::Credentials;
use crate::habit::{Color, Habit, Interval};
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

#[derive(Debug)]
struct Habits {
    habits: Mutex<Vec<Habit>>,
}

#[get("/habits")]
fn get_habits(state: &State<Habits>) -> Json<Vec<Habit>> {
    let habits = state.habits.lock().unwrap();
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
    local_user.login = user.login.clone();
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
fn login(cred: Json<Credentials>) -> Json<Token> {
    dbg!(&cred);
    Json::from(Token {
        token: "kok".to_string(),
    })
}

fn make_users() -> Users {
    Users {
        users: Mutex::new(vec![User {
            id: 0,
            login: "kotitka".to_string(),
            name: "Kisulenka!".to_string(),
            age: 23,
            email: "kotitka@gmail.com".to_string(),
            birthday: "26.11.1997".to_string(),
            password_hash: "a".to_string(),
        }]),
    }
}

fn make_habits() -> Habits {
    Habits {
        habits: Mutex::new(vec![
            Habit {
                time: "00:00".to_string(),
                name: "makbuk".to_string(),
                color: Color {
                    r: 15,
                    g: 15,
                    b: 15,
                },
                daily_repetitions: 10000,
                r#type: "good".to_string(),
                interval: Interval {
                    begin: "a".into(),
                    end: "b".into(),
                },
            },
            Habit {
                time: "00:15".to_string(),
                name: "iphone".to_string(),
                color: Color {
                    r: 15,
                    g: 15,
                    b: 15,
                },
                daily_repetitions: 10000,
                r#type: "good".to_string(),
                interval: Interval {
                    begin: "a".into(),
                    end: "b".into(),
                },
            },
            Habit {
                time: "00:30".to_string(),
                name: "xiaouuumiii".to_string(),
                color: Color {
                    r: 15,
                    g: 15,
                    b: 15,
                },
                daily_repetitions: 0,
                r#type: "bad".to_string(),
                interval: Interval {
                    begin: "a".into(),
                    end: "b".into(),
                },
            },
        ]),
    }
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
        .manage(make_users())
        .manage(make_habits())
}
