use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshToken {
    sub: String,
    exp: usize,
}

impl RefreshToken {
    pub fn new(username: &str, expires_in: usize) -> RefreshToken {
        RefreshToken {
            sub: username.to_owned(),
            exp: chrono::offset::Utc::now().timestamp() as usize + expires_in,
        }
    }

    pub fn valid(&self) -> bool {
        let timestamp = chrono::offset::Utc::now().timestamp() as usize;
        return timestamp < self.exp;
    }
}
