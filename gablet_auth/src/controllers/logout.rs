use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use diesel::{delete, prelude::*};
use diesel_async::RunQueryDsl;
use gablet_shared_api::errors::{get_error, get_internal_error, ErrorResult};

use crate::{models::requests::LogoutRequest, PG_POOL, TOKEN_ISSUER};

use crate::schema::refresh_tokens::dsl::{
    refresh_token as db_refresh_token, refresh_tokens as db_refresh_tokens,
};

#[axum::debug_handler]
pub async fn logout(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<LogoutRequest>,
) -> Result<StatusCode, (StatusCode, Json<ErrorResult>)> {
    let pool = PG_POOL.get().unwrap().clone();

    tracing::trace!("Logging user out");

    let connection = &mut pool.get().await.map_err(|err| {
        tracing::error!("Failed to get db connection. {}", err);
        get_internal_error(err).to_tuple()
    })?;

    let token = TOKEN_ISSUER
        .validate_auth(auth.token())
        .map_err(|err| get_error(err, StatusCode::UNAUTHORIZED).to_tuple())?;

    delete(db_refresh_tokens)
        .filter(db_refresh_token.eq(&request.refresh))
        .execute(connection)
        .await
        .map_err(|err| {
            tracing::error!(
                "Failed to delete refresh token while logging out {}: {}",
                token.sub,
                err
            );
            get_internal_error(err).to_tuple()
        })?;

    tracing::trace!("Logged {} out", token.sub);

    Ok(StatusCode::OK)
}
