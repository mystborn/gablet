use diesel::{Queryable, Insertable, Selectable};

#[derive(Debug, Queryable, Insertable, Selectable, Clone)]
#[diesel(table_name = crate::schema::refresh_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct RefreshTokenModel {
    pub refresh_token: String,
    pub username: String,
    pub source: String
}