use std::{net::SocketAddr, sync::OnceLock};

use axum::{
    body::Body,
    routing::{post, get},
    Router, http::{header::{AUTHORIZATION, CONTENT_TYPE}, Method},
};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use gablet_tokens::TokenIssuer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::{
    controllers::{
        refresh::refresh,
        register::{register, validate_account}, login::{login, pong},
    },
    credentials::Credentials
};

mod controllers;
mod credentials;
mod forms;
mod models;
mod schema;
mod utils;

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

fn token_issuer() -> TokenIssuer {
    let creds = Credentials::new().unwrap();
    TokenIssuer::new(creds.auth.access_secret, creds.auth.refresh_secret)
}

lazy_static::lazy_static! {
    pub static ref TOKEN_ISSUER: TokenIssuer = token_issuer();
}

pub static PG_POOL: OnceLock<Pool<AsyncPgConnection>> = OnceLock::new();

pub async fn start() {
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_target("gablet_users_api", tracing::Level::DEBUG)
        .with_target("tokio_postgres::prepare", tracing::Level::DEBUG)
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

    let api_routes: Router<(), Body> = Router::new()
        .route("/api/login", post(login))
        .route("/api/register", post(register))
        .route("/api/validate", post(validate_account))
        .route("/api/refresh", post(refresh))
        .route("/api/ping", get(pong));

    let app = Router::new()
        .merge(api_routes)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
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
