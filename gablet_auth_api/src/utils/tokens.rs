use diesel::result::Error as DbError;
use diesel::{delete, insert_into, prelude::*};
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use gablet_users::{AuthToken, RefreshToken};
use jsonwebtoken::errors::Error as JwtError;

use crate::models::refresh_token_model::RefreshTokenModel;
use crate::schema::refresh_tokens::dsl::{
    refresh_tokens as db_refresh_tokens, source as db_source, username as db_username,
};
use crate::{models::user::UserLevel, TOKEN_ISSUER};

pub const VALIDATE_ACCOUNT: &str = "validate";

const ACCESS_EXPIRY: usize = 60 * 60;
const REFRESH_EXPIRY: usize = 60 * 60 * 24 * 7;
const VALIDATE_EXPIRY: usize = 60 * 60 * 24 * 10;

pub fn get_access_token(username: &str, role: UserLevel, source: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_auth(&AuthToken::new(
        username.into(),
        role.to_string(),
        source.into(),
        ACCESS_EXPIRY,
    ))
}

pub fn get_refresh_token(username: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_refresh(&RefreshToken::new(username.into(), REFRESH_EXPIRY))
}

pub fn get_validate_token(username: &str, source: &str) -> Result<String, JwtError> {
    TOKEN_ISSUER.get_auth(&AuthToken::new(
        username.into(),
        UserLevel::User.to_string(),
        VALIDATE_ACCOUNT.into(),
        VALIDATE_EXPIRY,
    ))
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
