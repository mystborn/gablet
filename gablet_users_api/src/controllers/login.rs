use axum::{http::StatusCode, Json};
use diesel::prelude::*;
use diesel_async::RunQueryDsl;
use gablet_shared_api::errors::{get_error_from_string, get_internal_error, ErrorResult};

use crate::models::requests::LoginRequest;
use crate::models::responses::LoginResponse;
use crate::schema::users::dsl::last_login as db_last_login;
use crate::{
    utils::{
        tokens::{get_access_token, get_refresh_token, save_refresh_token},
        users::find_user,
    },
    PG_POOL,
};

#[axum::debug_handler]
pub async fn login(
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, Json<ErrorResult>)> {
    let LoginRequest { username, password } = request;

    tracing::trace!("Logging in {}", username);

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let user = find_user(Some(username.clone()), Some(username.clone()), connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?
        .ok_or_else(|| {
            get_error_from_string(
                StatusCode::UNAUTHORIZED,
                format!("No user with the username/password {}", username),
            )
            .to_tuple()
        })?;

    if !user.verify_password(&password) {
        return Err(get_error_from_string(
            StatusCode::UNAUTHORIZED,
            "Invalid username or password".into(),
        )
        .to_tuple());
    }

    let access = get_access_token(&user.username, user.id, user.level)
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let refresh =
        get_refresh_token(&user.username).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&refresh, &user.username, false, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    diesel::update(&user)
        .set(db_last_login.eq(chrono::Utc::now().naive_utc()))
        .execute(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(Json(LoginResponse::new(access, refresh)))
}
