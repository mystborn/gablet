use diesel::{prelude::*};
use diesel::result::Error as DbError;
use diesel_async::{AsyncPgConnection, RunQueryDsl};
use crate::{schema::users::dsl::{
    email as db_email, username as db_username, users as db_users,
}, models::user::User};

pub async fn find_user(
    username: Option<String>,
    email: Option<String>,
    connection: &mut AsyncPgConnection,
) -> Result<Option<User>, DbError> {
    let mut query = db_users.into_boxed();
    if username.is_some() {
        query = query.filter(db_username.eq(username.unwrap()));

        if email.is_some() {
            query = query.or_filter(db_email.eq(email.unwrap()));
        }
    } else {
        query = query.filter(db_email.eq(email.unwrap()));
    }

    let found_user = query
        .select(User::as_select())
        .first(connection)
        .await?;

    Ok(Some(found_user))
}