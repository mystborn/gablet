use std::net::SocketAddr;

use axum::{response::IntoResponse, routing::{get, post}, Json, Router};
use credentials::Credentials;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection, IntoSql,
};
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::controllers::profile::{login, register, current_user};

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

fn postgres_connection() -> Pool<ConnectionManager<PgConnection>> {
    let manager = ConnectionManager::<PgConnection>::new(get_postgres_connection());

    Pool::builder().build(manager).unwrap()
}

lazy_static::lazy_static! {
    pub static ref PG_POOL: Pool<ConnectionManager<PgConnection>> = postgres_connection();
}

pub async fn start() {
    // Initialize CORS layer
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let api_routes = Router::new()
        .route("/current_user", get(current_user))
        // .route_layer(RequireAuth::login())
        .route("/login", post(login))
        .route("/register", post(register));

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
