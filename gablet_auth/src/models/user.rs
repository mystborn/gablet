use chrono::NaiveDateTime;
use diesel::prelude::*;
use strum::EnumString;

use crate::utils::password::{generate_password_hash, verify_password};

#[derive(
    Debug,
    PartialEq,
    PartialOrd,
    Clone,
    Copy,
    EnumString,
    strum::Display,
    diesel_derive_enum::DbEnum,
)]
#[ExistingTypePath = "crate::schema::sql_types::UserLevel"]
pub enum UserLevel {
    #[strum(serialize = "user")]
    User,

    #[strum(serialize = "superuser")]
    Superuser,

    #[strum(serialize = "mod")]
    Mod,

    #[strum(serialize = "admin")]
    Admin,
}

#[derive(Debug, Queryable, Selectable, Insertable, Identifiable, AsChangeset, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub verified: bool,
    pub level: UserLevel,
    pub enabled: bool,
    pub created: NaiveDateTime,
    pub last_login: NaiveDateTime,
}

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        return verify_password(password, &self.password);
    }
}

#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewUser {
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub verified: bool,
    pub level: UserLevel,
}

impl NewUser {
    pub fn new(username: &str, password: &str, email: &str) -> NewUser {
        let mut user = NewUser {
            username: username.to_owned(),
            password: "".to_owned(),
            email: email.to_owned(),
            name: "".to_owned(),
            verified: false,
            level: UserLevel::User,
        };

        user.set_password(password);

        user
    }

    pub fn set_password(&mut self, password: &str) -> bool {
        if let Some(hashed) = generate_password_hash(password) {
            self.password = hashed;
            return true;
        }

        false
    }

    pub fn verify_password(&self, password: &str) -> bool {
        return verify_password(password, &self.password);
    }
}
