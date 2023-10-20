use serde::{Serialize, Deserialize};
use gablet_shared_api::errors::ErrorResult;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LoginResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub access_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub refresh_token: Option<String>
}

impl LoginResponse {
    pub fn new(access_token: String, refresh_token: String) -> LoginResponse {
        LoginResponse {
            access_token: Some(access_token),
            refresh_token: Some(refresh_token)
        }
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct ValidateResponse {
    pub success: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResult>,
}