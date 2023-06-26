use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::books)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub approved: bool,
    pub small_thumbnail: Option<String>,
    pub big_thumbnail: Option<String>
}