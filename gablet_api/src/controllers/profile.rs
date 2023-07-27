use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use gablet_shared_api::{
    auth::UserRequest,
    errors::{get_internal_error, ErrorResult},
};
use serde::{Deserialize, Serialize};

use crate::TOKEN_ISSUER;

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentUserResult {
    username: String,
}

#[axum::debug_handler]
pub async fn current_user(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(user): Json<UserRequest>,
) -> Result<Json<CurrentUserResult>, (StatusCode, Json<ErrorResult>)> {
    let result = TOKEN_ISSUER
        .validate_auth(auth.token(), &user.username, &user.source)
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(Json(CurrentUserResult {
        username: result.username(),
    }))
}