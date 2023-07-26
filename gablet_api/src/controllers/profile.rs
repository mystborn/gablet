use axum::{Form, http::StatusCode, response::IntoResponse, Json, extract::Query};
use axum_extra::extract::CookieJar;
use gablet_tokens::ACCESS_TOKEN;
use serde::{Deserialize, Serialize};
use gablet_shared_api::{errors::{ErrorResult, get_error_from_string, get_error, get_internal_error}, auth::UserRequest};

use crate::TOKEN_ISSUER;

#[derive(Serialize, Deserialize, Clone)]
pub struct CurrentUserResult {
    username: String
}

pub async fn current_user(jar: CookieJar, Query(user): Query<UserRequest>) -> Result<Json<CurrentUserResult>, (StatusCode, Json<ErrorResult>)> {
    let cookie = jar.get(ACCESS_TOKEN).ok_or_else(|| {
        get_error_from_string(StatusCode::UNAUTHORIZED, "Current user not logged in".into()).to_tuple()
    })?;

    TOKEN_ISSUER.validate_auth(cookie.value(), &user.username, &user.source).map_err(|err|
        get_internal_error(err).to_tuple()
    )?;

    Ok(Json(CurrentUserResult { username: "testintg".into() }))
}