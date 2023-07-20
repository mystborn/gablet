use serde::{Deserialize, Serialize};

use crate::utils::errors::ErrorResult;

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct LoginResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub access_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub refresh_token: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error: Option<ErrorResult>,
}

impl LoginResult {
    pub fn new(access_token: String, refresh_token: String) -> LoginResult {
        LoginResult {
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            error: None,
        }
    }

    pub fn error(error: ErrorResult) -> LoginResult {
        LoginResult {
            access_token: None,
            refresh_token: None,
            error: Some(error),
        }
    }
}