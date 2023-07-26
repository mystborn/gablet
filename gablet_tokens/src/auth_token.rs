use serde::{Deserialize, Serialize};

/// Represents the JWT claims used for authentication.
#[derive(Serialize, Deserialize, Debug)]
pub struct AuthToken {
    aud: String,
    sub: String,
    exp: usize,
    role: String,
}

impl AuthToken {
    /// Returns an AuthToken with the given values for logging in.
    /// # Arguments
    /// 
    /// * `username` - The user that is logging in.
    /// * `role` - The user_level of the user.
    /// * `base_uri` - The base URI of the service that this token will be used for.
    /// * `expires_in` - A number of seconds that this token will be eligible for.
    pub fn new(username: &str, role: &str, base_uri: &str, expires_in: usize) -> AuthToken {
        let now = chrono::offset::Utc::now().timestamp() as usize;
        AuthToken {
            aud: base_uri.into(),
            sub: username.into(),
            exp: now + expires_in,
            role: role.into(),
        }
    }

    pub fn username(&self) -> String {
        self.sub.clone()
    }

    pub fn role(&self) -> String {
        self.role.clone()
    }
}