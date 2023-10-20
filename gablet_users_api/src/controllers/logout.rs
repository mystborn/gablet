use axum::{Json, http::StatusCode, TypedHeader, headers::{Authorization, authorization::Bearer}};
use gablet_shared_api::errors::{ErrorResult, get_internal_error};

use crate::{models::requests::LogoutRequest, PG_POOL};

#[axum::debug_handler]
pub async fn logout(
    TypedHeader(auth): TypedHeader<Authorization<Bearer>>,
    Json(request): Json<LogoutRequest>) -> Result<StatusCode, (StatusCode, Json<ErrorResult>)>
{
    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    

    Ok(StatusCode::OK)
}