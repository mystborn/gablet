use axum::{http::StatusCode, Json};
use diesel::{delete, prelude::*, update};
use diesel_async::RunQueryDsl;
use gablet_shared_api::errors::{
    get_error_from_string, get_error_message, get_internal_error, ErrorResult,
};

use crate::{
    models::requests::ValidateRequest,
    utils::{tokens::check_validate_token, users::find_user},
    PG_POOL,
};

use crate::schema::users::dsl::users as db_users;

use crate::schema::refresh_tokens::dsl::{
    refresh_token as db_refresh_token, refresh_tokens as db_refresh_tokens,
};

pub async fn validate_account(
    Json(request): Json<ValidateRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResult>)> {
    let ValidateRequest { token, username } = request;
    
    check_validate_token(&token).map_err(|err| {
        get_error_message(
            err,
            StatusCode::UNAUTHORIZED,
            "Invalid validation token".into(),
        )
        .to_tuple()
    })?;

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let mut user = find_user(Some(username.clone()), None, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?
        .ok_or_else(|| {
            get_error_from_string(
                StatusCode::UNAUTHORIZED,
                format!("No user {} exists", &username),
            )
            .to_tuple()
        })?;

    if user.verified {
        return Err(get_error_from_string(
            StatusCode::UNAUTHORIZED,
            format!("User {} already verified", &username),
        )
        .to_tuple());
    }

    user.verified = true;

    update(db_users)
        .set(user)
        .execute(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    delete(db_refresh_tokens)
        .filter(db_refresh_token.eq(token))
        .execute(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(StatusCode::OK)
}
