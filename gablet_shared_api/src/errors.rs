use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use std::backtrace::Backtrace;
use std::error::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ErrorResult {
    pub error_code: u16,
    pub error_message: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub error_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub stack_trace: Option<String>,
}

impl ErrorResult {
    pub fn to_response<T>(&self) -> Result<T, (StatusCode, Json<ErrorResult>)> {
        Err((
            StatusCode::from_u16(self.error_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self.clone()),
        ))
    }

    pub fn to_tuple(&self) -> (StatusCode, Json<ErrorResult>) {
        (
            StatusCode::from_u16(self.error_code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self.clone()),
        )
    }
}

pub fn get_error_from_string(error_code: StatusCode, error_message: String) -> ErrorResult {
    ErrorResult {
        error_code: error_code.as_u16(),
        error_message,
        error_type: None,
        stack_trace: if cfg!(debug_assertions) {
            Some(format!("{:?}", Backtrace::capture()))
        } else {
            None
        },
    }
}

pub fn get_error_message<T: Error>(
    _err: T,
    error_code: StatusCode,
    error_message: String,
) -> ErrorResult {
    ErrorResult {
        error_code: error_code.as_u16(),
        error_message,
        stack_trace: if cfg!(debug_assertions) {
            Some(format!("{:?}", Backtrace::capture()))
        } else {
            None
        },
        error_type: if cfg!(debug_assertions) {
            Some(format!("{}", std::any::type_name::<T>()))
        } else {
            None
        },
    }
}

pub fn get_error<T: Error>(err: T, error_code: StatusCode) -> ErrorResult {
    ErrorResult {
        error_code: error_code.as_u16(),
        error_message: err.to_string(),
        stack_trace: if cfg!(debug_assertions) {
            Some(format!("{:?}", Backtrace::capture()))
        } else {
            None
        },
        error_type: if cfg!(debug_assertions) {
            Some(format!("{}", std::any::type_name::<T>()))
        } else {
            None
        },
    }
}

pub fn get_internal_error<T: Error>(err: T) -> ErrorResult {
    ErrorResult {
        error_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        error_message: err.to_string(),
        stack_trace: if cfg!(debug_assertions) {
            Some(format!("{:?}", Backtrace::capture()))
        } else {
            None
        },
        error_type: if cfg!(debug_assertions) {
            Some(format!("{}", std::any::type_name::<T>()))
        } else {
            None
        },
    }
}

pub fn get_internal_dyn_error(err: Box<dyn Error>) -> ErrorResult {
    ErrorResult {
        error_code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        error_message: err.to_string(),
        error_type: None,
        stack_trace: if cfg!(debug_assertions) {
            Some(format!("{:?}", Backtrace::capture()))
        } else {
            None
        },
    }
}
