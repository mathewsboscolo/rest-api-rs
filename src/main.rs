use axum::{Router, routing::get, routing::post, Extension};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod error;
mod models;

mod utils;

static KEYS: Lazy<models::auth::Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "Your secret here".to_owned());
    models::auth::Keys::new(secret.as_bytes())
});

#[tokio::main]
async fn main() {
    let durl = std::env::var("DATAASE_URL").expect("set DATABASE_URL env variable");
    
    tracing_subscriber::registry().with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();
    
    let cors = CorsLayer::new().allow_origin(Any);
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("unable to connect");

    let app = Router::new()
        .route("/", get(controllers::info::route_info))
        .route("/login", post(controllers::auth::login))
        .route("/register", post(controllers::auth::register))
        .route("/user_profile", get(controllers::user::user_profile))
        .layer(cors)
        .layer(Extension(pool));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed");
}