use axum::{http::StatusCode, Json};
use diesel::{insert_into, prelude::*};
use diesel_async::RunQueryDsl;
use gablet_shared_api::errors::{get_error_from_string, get_internal_error, ErrorResult};
use mail_builder::MessageBuilder;
use serde::{Deserialize, Serialize};
use urlencoding::{encode, encode_binary};

use crate::{
    models::{user::{NewUser, User, UserLevel}, responses::LoginResponse, requests::RegisterRequest},
    utils::{
        mail::get_mail_server2,
        tokens::{get_access_token, get_refresh_token, get_validate_token, save_refresh_token},
        users::find_user,
    },
    PG_POOL,
};

use crate::schema::users::dsl::users as db_users;

pub async fn register(
    Json(request): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, Json<ErrorResult>)> {
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
        return Err(get_error_from_string(
            StatusCode::CONFLICT,
            "Username or email already in use".into(),
        )
        .to_tuple());
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

    Ok((StatusCode::CREATED, Json(LoginResponse::new(access, refresh))))
}
