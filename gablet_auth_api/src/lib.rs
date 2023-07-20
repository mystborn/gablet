use std::{net::SocketAddr, sync::{OnceLock}};

use axum::{body::Body, routing::{get, post}, Router};
use controllers::login::login_api;
use diesel_async::{pooled_connection::{bb8::Pool, AsyncDieselConnectionManager}, AsyncPgConnection};
use gablet_users::TokenIssuer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{credentials::Credentials, controllers::{login::login_web, register::{register, validate_account, register_api, validate_account_api, register_web, token_test}, refresh::{self, refresh_web, refresh_api}}, utils::mail::init_mail_server};

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
    tracing_subscriber::fmt().with_max_level(tracing::Level::DEBUG).init();
    init_mail_server().await;

    let pool = postgres_connection().await;
    PG_POOL.set(pool).expect("Failed to set postgres pool");

    let api_routes: Router<(), Body> = Router::new()
        .route("/api/login", get(login_api))
        .route("/api/register", get(register_api))
        .route("/api/validate", get(validate_account_api))
        .route("/api/refresh", get(refresh_api))
        .route("/token-test", get(token_test));

    let web_routes: Router<(), Body> = Router::new()
        .route("/web/login", get(login_web))
        .route("/web/register", get(register_web))
        .route("/web/refresh", get(refresh_web));

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .merge(api_routes)
        .merge(web_routes)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}