use axum::{extract::Query, http::StatusCode, Json};
use axum_extra::extract::CookieJar;
use diesel::{delete, insert_into, prelude::*};
use diesel_async::RunQueryDsl;

use crate::{
    forms::login_form::LoginForm,
    models::{login_result::LoginResult, refresh_token_model::RefreshTokenModel},
    utils::{
        errors::{get_error_from_string, get_internal_error, ErrorResult},
        tokens::{get_access_token, get_refresh_token, set_token_cookies},
        users::find_user,
    },
    PG_POOL,
};

pub async fn login_web(Query(login_query): Query<LoginForm>, jar: CookieJar) -> Result<(CookieJar, Json<LoginResult>), (StatusCode, Json<ErrorResult>)> {
    let response = login(login_query.username, login_query.password, "webcall".into()).await?;

    if response.access_token.is_some() && response.refresh_token.is_some() {
        let json = response.clone();
        Ok((set_token_cookies(response.access_token.unwrap(), response.refresh_token.unwrap(), jar), Json(json)))
    } else {
        Ok((jar, Json(response)))
    }
}

pub async fn login_api(Query(login_query): Query<LoginForm>) -> Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)> {
    Ok(Json(login(login_query.username, login_query.password, "app".into()).await?))
}

async fn login(username_or_email: String, password: String, source: String) -> Result<LoginResult, (StatusCode, Json<ErrorResult>)> {
    use crate::schema::refresh_tokens;
    use crate::schema::refresh_tokens::dsl::{source as db_source, username as db_username};

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let user_search = find_user(
        Some(username_or_email.clone()),
        Some(username_or_email.clone()),
        connection,
    )
    .await
    .map_err(|err| get_internal_error(err).to_tuple())?;

    if user_search.is_none() {
        Ok(LoginResult::error(get_error_from_string(
            StatusCode::UNAUTHORIZED,
            format!("No user with the username/password {}", username_or_email),
        )))
    } else {
        let user = user_search.unwrap();
        if !user.verify_password(&password) {
            return Ok(LoginResult::error(get_error_from_string(
                StatusCode::OK,
                "Invalid username or password".into(),
            )));
        }

        let access = get_access_token(&user.username, user.level, &source)
            .map_err(|err| get_internal_error(err).to_tuple())?;

        let refresh =
            get_refresh_token(&user.username).map_err(|err| get_internal_error(err).to_tuple())?;

        let db_token = RefreshTokenModel {
            refresh_token: refresh.clone(),
            username: user.username.clone(),
            source: source.clone(),
        };

        delete(refresh_tokens::table)
            .filter(
                db_username
                    .eq(user.username.clone())
                    .and(db_source.eq(source.clone())),
            )
            .execute(connection)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;

        insert_into(refresh_tokens::table)
            .values(&db_token)
            .execute(connection)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;

        Ok(LoginResult::new(access, refresh))
    }
}