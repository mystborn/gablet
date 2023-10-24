use serde::{Deserialize, Serialize};

/// Represents the JWT claims used for authentication.
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    pub sub: String,
    pub exp: usize,
    pub role: String,
    pub user_id: i32
}

impl AuthToken {
    /// Returns an AuthToken with the given values for logging in.
    /// # Arguments
    /// 
    /// * `username` - The user that is logging in.
    /// * `role` - The user_level of the user.
    /// * `base_uri` - The base URI of the service that this token will be used for.
    /// * `expires_in` - A number of seconds that this token will be eligible for.
    pub fn new(username: &str, user_id: i32, role: &str, expires_in: usize) -> AuthToken {
        let now = chrono::offset::Utc::now().timestamp() as usize;
        AuthToken {
            sub: username.into(),
            exp: now + expires_in,
            role: role.into(),
            user_id
        }
    }

    pub fn username(&self) -> String {
        self.sub.clone()
    }

    pub fn user_id(&self) -> i32 {
        self.user_id
    }

    pub fn role(&self) -> String {
        self.role.clone()
    }
}