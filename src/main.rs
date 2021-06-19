#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Debug, Deserialize)]
struct Credentials {
    login: String,
    password: String
}

#[derive(Debug, Serialize, Deserialize)]
struct Token {
    token: String
}

#[post("/login", data = "<cred>")]
fn login(cred: Json<Credentials>) -> Json<Token> {
    Json::from(Token { token: "kok".to_string() })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![login])
}