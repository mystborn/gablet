use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Json, TypedHeader,
};
use diesel::{prelude::*, delete};
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
    tracing::info!("Logging out");

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    TOKEN_ISSUER
        .validate_auth(auth.token())
        .map_err(|err| get_error(err, StatusCode::UNAUTHORIZED).to_tuple())?;

    delete(db_refresh_tokens)
        .filter(db_refresh_token.eq(&request.refresh))
        .execute(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(StatusCode::OK)
}
