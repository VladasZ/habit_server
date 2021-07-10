use crate::credientals::Sha3Hashable;
use crate::Credentials;
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub login: String,
    #[serde(skip)]
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
