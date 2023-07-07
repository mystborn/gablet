use axum::{extract::Query, http::StatusCode, Json};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;
use mail_builder::MessageBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        login_result::LoginResult,
        user::{NewUser, UserLevel},
    },
    utils::{
        errors::{get_error, get_error_from_string, get_internal_error, ErrorResult},
        mail::get_mail_server,
        tokens::{get_access_token, get_refresh_token, get_validate_token, VALIDATE_ACCOUNT, save_refresh_token},
        users::find_user,
    },
    PG_POOL, TOKEN_ISSUER,
};

use crate::schema::users::dsl::users as db_users;

use crate::schema::refresh_tokens::dsl::{
    refresh_token as db_refresh_token, refresh_tokens as db_refresh_tokens,
};

type LoginResponse = Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)>;

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
    source: String,
}

#[axum::debug_handler]
pub async fn register(Query(register_request): Query<RegisterRequest>) -> Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)> {
    // Steps:
    // 1. Establish a connection
    // 2. Search for users with the given username
    //     2.1 If said user exists, return error
    // 3. Create new validate token, save in refresh table
    // 4. Create new user, save in user table
    // 5. Send validate token in email
    // 6. Create access and refresh tokens, send as response to immediately log user in.

    let RegisterRequest {
        username,
        email,
        password,
        source,
    } = register_request;

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let found_user = find_user(Some(username.clone()), Some(email.clone()), connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    if found_user.is_some() {
        return Ok(Json(LoginResult::error(get_error_from_string(
            StatusCode::OK,
            "Username or email already in use".into(),
        ))));
    }

    let user = NewUser::new(&username, &password, &email);

    let token =
        get_validate_token(&username, &source).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&token, &username, VALIDATE_ACCOUNT, false, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    // Todo: Get email stuff from config
    let mail = MessageBuilder::new()
        .from(("Gablet", "gabletservice@gmail.com"))
        .to(email.clone())
        .subject("Validate Gablet Account")
        .text_body(format!(
            "localhost:5173/user/validate?token={}&username={}",
            token.clone(),
            username.clone()
        ))
        .html_body(format!(
            "<a href=\"localhost:5173/user/validate?token={}&username={}\">Validate Account</a>",
            token.clone(),
            username.clone()
        ));

    {
        get_mail_server()
            .lock()
            .await
            .send(mail)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;
    }

    insert_into(db_users)
        .values(user)
        .execute(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let access = get_access_token(&username, UserLevel::User, &source)
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let refresh = get_refresh_token(&username).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&refresh, &username, &source, false, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?; 

    Ok(Json(LoginResult::new(access, refresh)))
}

#[derive(Serialize, Deserialize)]
pub struct ValidateRequest {
    token: String,
    username: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct ValidateResponse {
    pub success: bool,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,

    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResult>,
}

pub async fn validate_account(
    Query(request): Query<ValidateRequest>
) -> Result<Json<ValidateResponse>, (StatusCode, Json<ErrorResult>)> {
    let ValidateRequest { username, token } = request;
    let auth = TOKEN_ISSUER.validate_auth(&token, &username, VALIDATE_ACCOUNT.into());
    if auth.is_err() {
        return Ok(Json(ValidateResponse {
            success: false,
            message: Some("Invalid validation token".into()),
            error: Some(get_error(auth.unwrap_err(), StatusCode::UNAUTHORIZED)),
        }));
    }

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool
        .get()
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let user_search = find_user(Some(username.clone()), None, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    if user_search.is_none() {
        return Ok(Json(ValidateResponse {
            success: false,
            message: Some(format!("No user {} exists", username)),
            error: Some(get_error(auth.unwrap_err(), StatusCode::UNAUTHORIZED)),
        }));
    }

    let mut user = user_search.unwrap();
    if user.verified {
        return Ok(Json(ValidateResponse {
            success: false,
            message: Some("User already verified".into()),
            error: Some(get_error(auth.unwrap_err(), StatusCode::UNAUTHORIZED)),
        }));
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

    Ok(Json(ValidateResponse {
        success: true,
        message: Some(format!("Successfully registered {username}")),
        error: None,
    }))
}
