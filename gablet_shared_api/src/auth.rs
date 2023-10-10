use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String
}