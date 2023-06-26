use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::get, Json, Router};
use axum_login::{
    axum_sessions::{SessionLayer},
    extractors::AuthContext,
    RequireAuthorizationLayer, PostgresStore,
};
use credentials::Credentials;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use mail_send::{SmtpClient, SmtpClientBuilder};
use models::user::{User, UserLevel};
use once_cell::sync::OnceCell;
use rand::Rng;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

pub mod controllers;
pub mod credentials;
pub mod models;
pub mod schema;
pub mod utils;

type MailServer = SmtpClient<tokio_rustls::client::TlsStream<tokio::net::TcpStream>>;
type Auth = AuthContext<usize, User, PostgresStore<User, UserLevel>, UserLevel>;
type RequireAuth = RequireAuthorizationLayer<usize, User, UserLevel>;

fn postgres_connection() -> Pool<ConnectionManager<PgConnection>> {
    let creds = Credentials::new().unwrap();

    let manager = ConnectionManager::<PgConnection>::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        creds.postgres.username,
        creds.postgres.password,
        creds.postgres.host,
        creds.postgres.port,
        creds.postgres.db
    ));

    Pool::builder().build(manager).unwrap()
}

async fn mail_connection() -> MailServer {
    let creds = Credentials::new().unwrap();

    let credentials = mail_send::Credentials::new(&creds.mail.username, &creds.mail.password);

    SmtpClientBuilder::new(&creds.mail.host, creds.mail.port)
        .implicit_tls(false)
        .credentials(credentials)
        .connect()
        .await
        .unwrap()
}

lazy_static::lazy_static! {
    pub static ref PG_POOL: Pool<ConnectionManager<PgConnection>> = postgres_connection();
    pub static ref SESSION_STORE = PostgresSessionStore::new()
}

static MAIL_SERVER: OnceCell<MailServer> = OnceCell::new();

pub async fn start() {
    // let mail_server = mail_connection().await;
    // MAIL_SERVER.set(mail_server).unwrap_or_else(|_| {
    //     panic!("Failed to set MAIL_SERVER");
    // });

    let secret = rand::thread_rng().gen::<[u8; 64]>();

    let session_store = MemoryStore::new();
    let session_layer = SessionLayer::new(session_store, &secret);

    // Initialize CORS layer
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let api_routes = Router::new()
        .route("/", get(index))
        .route("/get-message", get(get_message));

    let app = Router::new()
        .nest("/api", api_routes)
        .layer(ServiceBuilder::new().layer(cors));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
