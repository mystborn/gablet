#![feature(lazy_cell)]

use std::{net::SocketAddr, sync::{LazyLock, OnceLock}};

use axum::{routing::{get, post}, Router};
use credentials::Credentials;
use diesel_async::{pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use gablet_tokens::TokenIssuer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::controllers::profile::{current_user};

pub mod controllers;
pub mod credentials;
pub mod models;
pub mod schema;
pub mod utils;

fn get_postgres_connection() -> String {
    let creds = Credentials::new().unwrap();
    format!(
        "postgres://{}:{}@{}:{}/{}",
        creds.postgres.username,
        creds.postgres.password,
        creds.postgres.host,
        creds.postgres.port,
        creds.postgres.db
    )
}

async fn postgres_connection() -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(get_postgres_connection());

    Pool::builder().build(manager).await.unwrap()
}

pub static PG_POOL: OnceLock<Pool<AsyncPgConnection>> = OnceLock::new();
pub static TOKEN_ISSUER: LazyLock<TokenIssuer> = LazyLock::new(|| {
    let creds = Credentials::new().unwrap();
    TokenIssuer::new(creds.auth.access_secret, creds.auth.refresh_secret)
});

pub async fn start() {
    // Initialize CORS layer
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let api_routes = Router::new()
        .route("/current_user", get(current_user));

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(ServiceBuilder::new()
            .layer(cors)
        );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}

#[tokio::main]
async fn main() {
    start().await;
}
