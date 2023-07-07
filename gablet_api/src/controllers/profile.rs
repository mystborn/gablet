use axum::{Form, http::StatusCode, response::IntoResponse, Json};
use diesel::{prelude::*, insert_into};
use serde_json::{Value, json};

use crate::{models::{login_form::LoginForm, user::User, register_form::RegisterForm, simple_result::SimpleResult}, Auth, PG_POOL};

pub async fn login(mut auth: Auth, Form(login_form): Form<LoginForm>) -> Json<SimpleResult> {
    use crate::schema::users::dsl::*;

    if let Ok(connection) = &mut PG_POOL.get() {
        
        let results = users
            .filter(username.eq(login_form.username_or_email.clone()))
            .or_filter(email.eq(login_form.username_or_email.clone()))
            .limit(1)
            .select(User::as_select())
            .load(connection);

        if results.is_err() {
            return Json(SimpleResult::from_status(StatusCode::UNAUTHORIZED));
        }

        let user_list: Vec<User> = results.unwrap();

        if user_list.len() < 1 {
            return Json(SimpleResult::from_status(StatusCode::UNAUTHORIZED));
        }

        let result = auth.login(&user_list[0]).await;

        dbg!(&result);

        if result.is_err() {
            return Json(SimpleResult::from_status(StatusCode::UNAUTHORIZED));
        }

        return Json(SimpleResult::from_status(StatusCode::OK));;
    }

    return Json(SimpleResult::from_status(StatusCode::UNAUTHORIZED));
}

pub async fn register(Form(register_form): Form<RegisterForm>) -> Json<SimpleResult> {
    use crate::schema::users::dsl::*;

    if let Ok(connection) = &mut PG_POOL.get() {
        let results = users
            .filter(username.eq(register_form.username.clone()))
            .or_filter(email.eq(register_form.email.clone()))
            .limit(1)
            .select(User::as_select())
            .load(connection);

        if results.is_err() {
            return Json(SimpleResult::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }

        let user_list: Vec<User> = results.unwrap();

        if user_list.len() > 1 {
            return Json(SimpleResult::from_error(
                "A user with that email or username already exists".to_owned(),
                StatusCode::UNAUTHORIZED));
        }

        let user = User::new(&register_form.username, &register_form.password, &register_form.email);

        let create_result = insert_into(users)
            .values(&user)
            .execute(connection);

        if create_result.is_err() {
            return Json(SimpleResult::from_status(StatusCode::INTERNAL_SERVER_ERROR));
        }

        return Json(SimpleResult::from_status(StatusCode::OK));
    }

    return Json(SimpleResult::from_status(StatusCode::INTERNAL_SERVER_ERROR));
}

pub async fn current_user(mut auth: Auth) -> Json<Value> {
    match auth.current_user {
        Some(user) => Json(json!({ "user": user.username })),
        None => Json(json!({ "user": "none" }))
    }
}