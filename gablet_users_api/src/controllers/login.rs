use axum::{http::StatusCode, Json, response::IntoResponse};
use diesel::{delete, insert_into, prelude::*};
use diesel_async::RunQueryDsl;

use crate::{
    forms::login_form::LoginForm,
    models::{login_result::LoginResult, refresh_token_model::RefreshTokenModel},
    utils::{
        errors::{get_error_from_string, get_internal_error, ErrorResult},
        tokens::{get_access_token, get_refresh_token},
        users::find_user,
    },
    PG_POOL,
};

#[axum::debug_handler]
pub async fn pong() -> impl IntoResponse {
    return "Pong".to_string()
}

#[axum::debug_handler]
pub async fn login(
    Json(request): Json<LoginForm>,
) -> Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)> {
    let LoginForm {
        username,
        password,
    } = request;
    println!("Logging in {}, {}", username, password);

    use crate::schema::refresh_tokens;
    use crate::schema::refresh_tokens::dsl::{username as db_username};
    use crate::schema::users::dsl::last_login as db_last_login;

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let user_search = find_user(Some(username.clone()), Some(username.clone()), connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    if user_search.is_none() {
        Ok(Json(LoginResult::error(get_error_from_string(
            StatusCode::UNAUTHORIZED,
            format!("No user with the username/password {}", username),
        ))))
    } else {
        let user = user_search.unwrap();
        if !user.verify_password(&password) {
            return Ok(Json(LoginResult::error(get_error_from_string(
                StatusCode::OK,
                "Invalid username or password".into(),
            ))));
        }

        let access = get_access_token(&user.username, user.id, user.level)
            .map_err(|err| get_internal_error(err).to_tuple())?;

        let refresh =
            get_refresh_token(&user.username).map_err(|err| get_internal_error(err).to_tuple())?;

        let db_token = RefreshTokenModel {
            refresh_token: refresh.clone(),
            username: user.username.clone()
        };

        diesel::update(&user)
            .set(db_last_login.eq(chrono::Utc::now().naive_utc()))
            .execute(connection)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;

        delete(refresh_tokens::table)
            .filter(
                db_username
                    .eq(user.username.clone())
            )
            .execute(connection)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;

        insert_into(refresh_tokens::table)
            .values(&db_token)
            .execute(connection)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;

        Ok(Json(LoginResult::new(access, refresh)))
    }
}
