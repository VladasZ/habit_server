use rocket::serde::Deserialize;
use sha3::{Digest, Sha3_256};

#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub login: String,
    pub password: String,
}

impl Credentials {
    pub fn new(login: String, password: String) -> Credentials {
        Credentials { login, password }
    }
}

pub trait Sha3Hashable {
    fn sha3(self) -> String;
}

impl Sha3Hashable for String {
    fn sha3(self) -> String {
        let mut hasher = Sha3_256::new();
        hasher.update(self);
        format!("{:x}", hasher.finalize())
    }
}
