use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SimpleResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    status_code: Option<u16>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    error_msg: Option<String>,
}

impl SimpleResult {
    pub fn from_status(status: StatusCode) -> SimpleResult {
        SimpleResult {
            status_code: Some(status.as_u16()),
            error_msg: None,
        }
    }

    pub fn from_error(error: String, status: StatusCode) -> SimpleResult {
        SimpleResult {
            status_code: Some(status.as_u16()),
            error_msg: Some(error),
        }
    }
}
