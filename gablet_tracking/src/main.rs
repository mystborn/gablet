#![feature(lazy_cell)]

use std::{
    net::SocketAddr,
    sync::{LazyLock, Mutex, OnceLock},
    time::Duration,
};

use axum::{
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::get,
    Router,
};
use diesel_async::{
    pooled_connection::{bb8::Pool, AsyncDieselConnectionManager},
    AsyncPgConnection,
};
use gablet_shared_api::{
    cancellation_token::CancellationSource, credentials::Credentials,
    kafka::kafka_thread::kafka_thread,
};
use gablet_tokens::TokenIssuer;
use kafka::producer::Producer;
use tower::ServiceBuilder;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

use crate::{
    controllers::metrics::{metrics_test, track_web_view},
    gablet_kafka::kafka_thread::dispatch_kafka_event,
};

mod controllers;
mod events;
mod gablet_kafka;
mod models;
mod schema;

fn get_postgres_connection() -> String {
    let creds = Credentials::new("./config/credentials.toml")
        .unwrap()
        .postgres
        .expect("Missing postgres credentials");

    format!(
        "postgres://{}:{}@{}:{}/{}",
        creds.username, creds.password, creds.host, creds.port, creds.db
    )
}

async fn postgres_connection() -> Pool<AsyncPgConnection> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(get_postgres_connection());

    Pool::builder().build(manager).await.unwrap()
}

pub static PG_POOL: OnceLock<Pool<AsyncPgConnection>> = OnceLock::new();
pub static TOKEN_ISSUER: LazyLock<TokenIssuer> = LazyLock::new(|| {
    let creds = Credentials::new("./config/credentials.toml")
        .unwrap()
        .auth
        .expect("Missing auth credentials");
    TokenIssuer::new(creds.access_secret, creds.refresh_secret)
});

pub static TRACKING_PRODUCER: LazyLock<Mutex<kafka::producer::Producer>> = LazyLock::new(|| {
    let creds = Credentials::new("./config/credentials.toml")
        .unwrap()
        .kafka
        .expect("Missing kafka credentials");
    let producer = Producer::from_hosts(creds.hosts)
        .with_ack_timeout(Duration::from_secs(2))
        .with_required_acks(kafka::producer::RequiredAcks::One)
        .create()
        .expect("Failed to create kafka producer");

    Mutex::new(producer)
});

pub async fn start() {
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("tower_http::trace::on_response", tracing::Level::TRACE)
        .with_target("tower_http::trace::on_request", tracing::Level::TRACE)
        .with_target("tower_http::trace::make_span", tracing::Level::DEBUG)
        .with_target("gablet_tracking", tracing::Level::DEBUG)
        .with_target("kafka", tracing::Level::TRACE)
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

    let app = Router::new()
        .route("/", get(metrics_test))
        .route("/tracking", get(track_web_view))
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors),
        );

    let mut cts = CancellationSource::new();
    let token = cts.token();

    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    std::thread::spawn(move || kafka_thread(token, dispatch_kafka_event));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .with_graceful_shutdown(async {
            rx.await.ok();
        })
        .await
        .unwrap();

    cts.request_cancellation();
}

#[tokio::main]
async fn main() {
    start().await;
}
