use crate::{models::tracking::NewWebView, PG_POOL};
use diesel::insert_into;
use diesel_async::RunQueryDsl;
use std::error::Error;

pub async fn save_web_view(view: &NewWebView) -> Result<(), Box<dyn Error>> {
    use crate::schema::web_views;

    let pool = PG_POOL.get().unwrap().clone();

    let connection = &mut pool.get().await?;

    insert_into(web_views::table)
        .values(view)
        .execute(connection)
        .await?;

    Ok(())
}