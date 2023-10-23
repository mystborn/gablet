use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct ValidateRequest {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RefreshRequest {
    pub refresh: String,
}

#[derive(Serialize, Deserialize)]
pub struct LogoutRequest {
    pub refresh: String,
}