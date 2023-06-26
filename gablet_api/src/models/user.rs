use axum_login::{AuthUser, secrecy::SecretVec};
use diesel::prelude::*;

use crate::utils::password::{generate_password_hash, verify_password};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, diesel_derive_enum::DbEnum)]
#[ExistingTypePath = "crate::schema::sql_types::UserLevel"]
pub enum UserLevel {
    User,
    Superuser,
    Mod,
    Admin,
}

#[derive(Queryable, Selectable, Clone)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub email: String,
    pub name: String,
    pub verified: bool,
    pub level: UserLevel
}

impl User {
    fn set_password(&mut self, password: &str) -> bool {
        if let Some(hashed) = generate_password_hash(password) {
            self.password = hashed;
            return true;
        }

        false
    }

    fn verify_password(&self, password: &str) -> bool {
        return verify_password(password, &self.password);
    }
}

impl AuthUser<i32, UserLevel> for User {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_password_hash(&self) -> SecretVec<u8> {
        SecretVec::new(self.password.clone().into())
    }

    fn get_role(&self) -> Option<UserLevel> {
        Some(self.level)
    }
}