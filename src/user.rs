use crate::credientals::Sha3Hashable;
use crate::Credentials;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub login: String,
    pub password_hash: String,
}

impl User {
    pub fn new(id: usize, cred: Credentials) -> User {
        User {
            id,
            login: cred.login,
            password_hash: cred.password.sha3(),
        }
    }
}
