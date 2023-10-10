use axum::{extract::Query, http::StatusCode, Json};
use diesel::{delete, insert_into, prelude::*, update};
use diesel_async::RunQueryDsl;
use gablet_tokens::VALIDATE_TOKEN;
use mail_builder::MessageBuilder;
use serde::{Deserialize, Serialize};
use urlencoding::{encode, encode_binary};

use crate::{
    models::{
        login_result::LoginResult,
        user::{NewUser, User, UserLevel},
    },
    utils::{
        errors::{get_error, get_error_from_string, get_internal_error, ErrorResult},
        mail::get_mail_server2,
        tokens::{
            check_validate_token, get_access_token, get_refresh_token, get_validate_token,
            save_refresh_token,
        },
        users::find_user,
    },
    PG_POOL,
};

use crate::schema::users::dsl::users as db_users;

use crate::schema::refresh_tokens::dsl::{
    refresh_token as db_refresh_token, refresh_tokens as db_refresh_tokens,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterRequest {
    username: String,
    email: String,
    password: String,
}

pub async fn register(
    Json(request): Json<RegisterRequest>,
) -> Result<Json<LoginResult>, (StatusCode, Json<ErrorResult>)> {
    let RegisterRequest {
        username,
        email,
        password,
    } = request;

    // Steps:
    // 1. Establish a connection
    // 2. Search for users with the given username
    //     2.1 If said user exists, return error
    // 3. Create new validate token, save in refresh table
    // 4. Create new user, save in user table
    // 5. Send validate token in email
    // 6. Create access and refresh tokens, send as response to immediately log user in.

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

    let token = get_validate_token(&username).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&token, &username, false, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let query_token = encode_binary(token.as_bytes());
    let query_username = encode(&username);

    // Todo: Get email stuff from config
    let mail = MessageBuilder::new()
        .from(("Gablet", "gabletservice@gmail.com"))
        .to(email.clone())
        .subject("Validate Gablet Account")
        .text_body(format!(
            "localhost:5173/user/validate?token={}&username={}",
            query_token.clone(),
            query_username.clone()
        ))
        .html_body(format!(
            "<a href=\"http://localhost:5173/validate?token={}&username={}\">Validate Account</a>\n\n<br><p>http://localhost:5173/validate?token={}&username={}</p><br><p>Testing!!!</p>",
            query_token.clone(),
            query_username.clone(),
            query_token.clone(),
            query_username.clone()
        ));

    {
        get_mail_server2()
            // .lock()
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?
            .send(mail)
            .await
            .map_err(|err| get_internal_error(err).to_tuple())?;
    }

    let user: Vec<User> = insert_into(db_users)
        .values(user)
        .returning(User::as_returning())
        .get_results(connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let access = get_access_token(&username, user[0].id, UserLevel::User)
        .map_err(|err| get_internal_error(err).to_tuple())?;

    let refresh = get_refresh_token(&username).map_err(|err| get_internal_error(err).to_tuple())?;

    save_refresh_token(&refresh, &username, false, connection)
        .await
        .map_err(|err| get_internal_error(err).to_tuple())?;

    Ok(Json(LoginResult::new(access, refresh)))
}

#[derive(Serialize, Deserialize)]
pub struct ValidateRequest {
    token: String,
    username: String,
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
    Json(request): Json<ValidateRequest>,
) -> Result<Json<ValidateResponse>, (StatusCode, Json<ErrorResult>)> {
    let ValidateRequest { token, username } = request;
    let auth = check_validate_token(&token, &username);
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
            message: Some(format!("No user {} exists", &username)),
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenTestResponse {
    pub success: bool,
    pub message: String,
}

// pub async fn token_test(
//     Query(query): Query<RegisterRequest>,
// ) -> Result<Json<TokenTestResponse>, (StatusCode, Json<ErrorResult>)> {
//     let RegisterRequest {
//         username, source, ..
//     } = query;

//     let pool = PG_POOL.get().unwrap().clone();

//     let connection = &mut pool
//         .get()
//         .await
//         .map_err(|err| get_internal_error(err).to_tuple())?;

//     let token = get_validate_token(&username).map_err(|err| get_internal_error(err).to_tuple())?;

//     // save_refresh_token(&token, &username, VALIDATE_ACCOUNT, false, connection)
//     //     .await
//     //     .map_err(|err| get_internal_error(err).to_tuple())?;

//     let auth = check_validate_token(&token, &username);
//     if auth.is_err() {
//         return Ok(Json(TokenTestResponse {
//             success: false,
//             message: format!(
//                 "Failed to validate token: {}",
//                 auth.unwrap_err().to_string()
//             ),
//         }));
//     }

//     Ok(Json(TokenTestResponse {
//         success: true,
//         message: "Successfully validated the token".into(),
//     }))
// }
