use axum::{extract::Query, http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use serde::{Deserialize, Serialize};

use crate::{
    models::login_result::LoginResult,
    utils::{
        errors::{get_error, get_error_from_string, get_internal_error, ErrorResult},
        tokens::{
            confirm_refresh_token, get_access_token, get_refresh_token, save_refresh_token,
            set_token_cookies, ACCESS_TOKEN, REFRESH_TOKEN,
        },
        users::find_user,
    },
    PG_POOL, TOKEN_ISSUER,
};

#[derive(Serialize, Deserialize)]
pub struct RefreshWebRequest {
    pub source: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshApiRequest {
    pub source: String,
    pub refresh: String,
}

pub async fn refresh_web(
    Query(request): Query<RefreshWebRequest>,
    jar: CookieJar,
) -> Result<CookieJar, (CookieJar, (StatusCode, Json<ErrorResult>))> {
    let RefreshWebRequest { source } = request;

    let cookie = jar.get(REFRESH_TOKEN).ok_or_else(|| {
        (
            jar.clone(),
            get_error_from_string(StatusCode::UNAUTHORIZED, "No refresh token".into()).to_tuple(),
        )
    })?;

    match refresh(&source, cookie.value()).await {
        Ok((access, refresh)) => Ok(set_token_cookies(access, refresh, jar)),
        Err(err) => Err((jar.clone().remove(cookie.clone()), err)),
    }
}

pub async fn refresh_api(
    Query(request): Query<RefreshApiRequest>,
) -> Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)> {
    let (access, refresh) = refresh(&request.source, &request.refresh).await?;

    Ok(Json(LoginResult::new(access, refresh)))
}

async fn refresh(
    source: &str,
    token: &str,
) -> Result<(String, String), (StatusCode, Json<ErrorResult>)> {
    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let token_model = confirm_refresh_token(token, &source, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?
        .ok_or_else(|| {
            get_error_from_string(StatusCode::UNAUTHORIZED, "Invalid refresh token".into())
                .to_tuple()
        })?;

    TOKEN_ISSUER
        .validate_refresh(token, &token_model.username)
        .map_err(|err| get_error(err, StatusCode::UNAUTHORIZED).to_tuple())?;

    let user = find_user(Some(token_model.username.clone()), None, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?
        .ok_or_else(|| {
            get_error_from_string(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find user".into(),
            )
            .to_tuple()
        })?;

    let access = get_access_token(&user.username, user.level, &source)
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let refresh =
        get_refresh_token(&user.username).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&refresh, &user.username, &source, true, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok((access, refresh))
}
