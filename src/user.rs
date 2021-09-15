use crate::credientals::Sha3Hashable;
use crate::Credentials;
use rocket::serde::{Deserialize, Serialize};
use crate::habit::Habit;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(skip)]
    pub id: usize,
    pub name: String,
    pub age: u8,
    pub email: String,
    pub birthday: String,
    pub habits: Vec<Habit>,
    pub avatarUrl: String,
    #[serde(skip)]
    pub password_hash: String,
}

impl User {
    pub fn new(id: usize, cred: Credentials) -> User {
        User {
            id,
            name: "Denis".to_string(),
            age: 30,
            email: cred.email,
            birthday: "05:05:2005".to_string(),
            habits: vec![],
            avatarUrl: "https://crates.io/assets/Cargo-Logo-Small-c39abeb466d747f3be442698662c5260.png".into(),
            password_hash: cred.password.sha3(),
        }
    }
}
