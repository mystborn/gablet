#![feature(lazy_cell)]

use std::{net::SocketAddr, sync::{LazyLock, OnceLock}};

use axum::{routing::{get, post}, Router, http::{header::{AUTHORIZATION, CONTENT_TYPE}, Method}};
use credentials::Credentials;
use diesel_async::{pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use gablet_tokens::TokenIssuer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

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
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_target("gablet_api", tracing::Level::DEBUG)
        .with_default(tracing::Level::INFO);

    let layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(layer)
        .with(filter)
        .init();

    // Initialize CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let pool = postgres_connection().await;
    PG_POOL.set(pool).expect("Failed to set postgres pool");

    let api_routes = Router::new()
        .route("/api/profile", post(current_user));

    let web_routes = Router::new()
        .route("/web/profile", post(current_user));

    let app = Router::new()
        .merge(api_routes)
        .merge(web_routes)
        .layer(ServiceBuilder::new()
            .layer(cors)
        )
        .layer(TraceLayer::new_for_http());

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
