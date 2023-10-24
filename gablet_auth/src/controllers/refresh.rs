use axum::{http::StatusCode, Json};
use gablet_shared_api::errors::{
    get_error, get_error_from_string, get_internal_error, ErrorResult,
};

use crate::{
    models::{requests::RefreshRequest, responses::LoginResponse},
    utils::{
        tokens::{confirm_refresh_token, get_access_token, get_refresh_token, save_refresh_token},
        users::find_user,
    },
    PG_POOL, TOKEN_ISSUER,
};

pub async fn refresh(
    Json(request): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResult>)> {
    let RefreshRequest { refresh } = request;
    let pool = PG_POOL.get().unwrap().clone();

    tracing::trace!("Refreshing login {}", refresh);

    let connection = &mut pool.get().await.map_err(|err| {
        tracing::error!("Failed to get db connection: {}", err);
        get_internal_error(err).to_tuple()
    })?;

    let token_model = confirm_refresh_token(&refresh, connection)
        .await
        .map_err(|err| {
            tracing::error!("Failed to confirm refresh token {}: {}", refresh, err);
            get_internal_error(err).to_tuple()
        })?
        .ok_or_else(|| {
            get_error_from_string(StatusCode::UNAUTHORIZED, "Invalid refresh token".into())
                .to_tuple()
        })?;

    TOKEN_ISSUER
        .validate_refresh(&refresh, &token_model.username)
        .map_err(|err| get_error(err, StatusCode::UNAUTHORIZED).to_tuple())?;

    let user = find_user(Some(token_model.username.clone()), None, connection)
        .await
        .map_err(|err| {
            tracing::error!("Error while fetching user: {}", err);
            get_internal_error(err).to_tuple()
        })?
        .ok_or_else(|| {
            get_error_from_string(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to find user".into(),
            )
            .to_tuple()
        })?;

    let access = get_access_token(&user.username, user.id, user.level).map_err(|err| {
        tracing::error!("Failed to get new access token: {}", err);
        get_internal_error(err).to_tuple()
    })?;

    let refresh = get_refresh_token(&user.username).map_err(|err| {
        tracing::error!("Failed to get new refresh token: {}", err);
        get_internal_error(err).to_tuple()
    })?;

    save_refresh_token(&refresh, &user.username, true, connection)
        .await
        .map_err(|err| {
            tracing::error!("Failed to save refresh token: {}", err);
            get_internal_error(err).to_tuple()
        })?;

    tracing::trace!("Refreshed login for {}", user.username);

    Ok(Json(LoginResponse::new(access, refresh)))
}
