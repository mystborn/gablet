use std::{net::SocketAddr, sync::{OnceLock, Mutex}};

use axum::{
    body::Body,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::{post, get},
    Router,
};
use axum_prometheus::PrometheusMetricLayer;
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use gablet_shared_api::{kafka::kafka_writer::KafkaWriter, credentials::Credentials};
use gablet_tokens::TokenIssuer;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{prelude::*, util::SubscriberInitExt};

use crate::controllers::{login::login, refresh::refresh, register::register, validate::validate_account};

mod controllers;
mod models;
mod schema;
mod utils;

static CONFIG_PATH: &'static str = "./config/credentials.toml";

fn get_postgres_connection() -> String {
    let creds = Credentials::new(CONFIG_PATH).unwrap();
    let postgres = creds.postgres.expect("Missing postgres credentials");

    format!(
        "postgres://{}:{}@{}:{}/{}",
        postgres.username,
        postgres.password,
        postgres.host,
        postgres.port,
        postgres.db
    )
}

async fn postgres_connection() -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(get_postgres_connection());

    Pool::builder().build(manager).await.unwrap()
}

fn token_issuer() -> TokenIssuer {
    let creds = Credentials::new(CONFIG_PATH).unwrap();
    let auth = creds.auth.expect("Missing auth credentials");
    TokenIssuer::new(auth.access_secret, auth.refresh_secret)
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
        .with_target("tokio_postgres::query", tracing::Level::DEBUG)
        .with_default(tracing::Level::INFO);

    let creds = Credentials::new(CONFIG_PATH).unwrap();
    let kafka_writer = Mutex::new(KafkaWriter::from_credentials(&creds));

    let console_layer = tracing_subscriber::fmt::layer();
    let json_layer = tracing_subscriber::fmt::layer().json().with_writer(kafka_writer);

    tracing_subscriber::registry()
        .with(console_layer)
        .with(json_layer)
        .with(filter)
        .init();

    // Initialize CORS layer
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE])
        .allow_methods([Method::GET, Method::POST]);

    let pool = postgres_connection().await;
    PG_POOL.set(pool).expect("Failed to set postgres pool");

    let (prometheus_layer, metrics_handle) = PrometheusMetricLayer::pair();

    let api_routes: Router<(), Body> = Router::new()
        .route("/api/login", post(login))
        .route("/api/register", post(register))
        .route("/api/validate", post(validate_account))
        .route("/api/refresh", post(refresh))
        .route("/api/metrics", get(|| async move { 
            tracing::info!("Getting metrics");
            metrics_handle.render() 
        }));

    let app = Router::new()
        .merge(api_routes)
        .layer(ServiceBuilder::new()
            .layer(TraceLayer::new_for_http())
            .layer(prometheus_layer)
            .layer(cors)
        );

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
