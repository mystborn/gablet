use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginForm {
    pub username_or_email: String,
    pub password: String
}