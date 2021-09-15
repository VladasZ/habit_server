use crate::credientals::Sha3Hashable;
use crate::Credentials;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub name: String,
    pub age: u8,
    pub email: String,
    pub birthday: String,
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
            password_hash: cred.password.sha3(),
        }
    }
}
