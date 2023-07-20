use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::Cookie;
use diesel::result::Error as DbError;
use diesel::{delete, insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gablet_users::{AuthToken, RefreshToken};
use jsonwebtoken::errors::Error as JwtError;

use crate::models::refresh_token_model::RefreshTokenModel;
use crate::schema::refresh_tokens::dsl::{
    refresh_token as db_refresh_token, refresh_tokens as db_refresh_tokens, source as db_source,
    username as db_username,
};
use crate::{models::user::UserLevel, TOKEN_ISSUER};

pub const VALIDATE_ACCOUNT: &str = "validate";
pub const ACCESS_TOKEN: &str = "access_token";
pub const REFRESH_TOKEN: &str = "refresh_token";

const ACCESS_EXPIRY: usize = 60 * 60;
const REFRESH_EXPIRY: usize = 60 * 60 * 24 * 7;
const VALIDATE_EXPIRY: usize = 60 * 60 * 24 * 10;

pub fn get_access_token(username: &str, role: UserLevel, source: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_auth(&AuthToken::new(
        username,
        &role.to_string(),
        source,
        ACCESS_EXPIRY,
    ))
}

pub fn get_refresh_token(username: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_refresh(&RefreshToken::new(username.into(), REFRESH_EXPIRY))
}

pub fn get_validate_token(username: &str, source: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_auth(&AuthToken::new(
        username,
        &UserLevel::User.to_string(),
        VALIDATE_ACCOUNT,
        VALIDATE_EXPIRY,
    ))
}

pub fn check_validate_token(token: &str, username: &str) -> Result<AuthToken, JwtError> {
    TOKEN_ISSUER.validate_auth(token, username, VALIDATE_ACCOUNT)
}

pub async fn confirm_refresh_token(
    token: &str,
    source: &str,
    connection: &mut AsyncPgConnection,
) -> Result<Option<RefreshTokenModel>, DbError> {
    let tokens: Vec<RefreshTokenModel> = db_refresh_tokens
        .filter(
            db_refresh_token
            .eq(token.to_owned())
            .and(db_source.eq(source.to_owned())),
        )
        .select(RefreshTokenModel::as_select())
        .limit(1)
        .get_results(connection)
        .await?;

    Ok(if tokens.len() > 0 { Some(tokens[0].clone()) } else { None })
}

pub async fn save_refresh_token(
    token: &str,
    username: &str,
    source: &str,
    delete_old: bool,
    connection: &mut AsyncPgConnection,
) -> Result<(), DbError> {
    if delete_old {
        delete(db_refresh_tokens)
            .filter(
                db_username
                    .eq(username.to_string())
                    .and(db_source.eq(source.to_string())),
            )
            .execute(connection)
            .await?;
    }

    let db_token = RefreshTokenModel {
        refresh_token: token.into(),
        username: username.into(),
        source: source.into(),
    };

    insert_into(db_refresh_tokens)
        .values(&db_token)
        .execute(connection)
        .await?;

    Ok(())
}

pub fn set_token_cookies(access: String, refresh: String, jar: CookieJar) -> CookieJar {
    let access_cookie = Cookie::build(ACCESS_TOKEN.to_string(), access)
        .http_only(true)
        .finish();

    let refresh_cookie = Cookie::build(REFRESH_TOKEN.to_string(), refresh)
        .http_only(true)
        .finish();

    jar.add(access_cookie).add(refresh_cookie)
}