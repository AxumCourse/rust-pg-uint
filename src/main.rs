use std::sync::Arc;

use axum::routing::{get, post};
use rust_pg_uint::{handler, state::AppState};
#[tokio::main]
async fn main() {
    let dsn = std::env::var("PG_DSN")
        .unwrap_or("postgres://postgres:postgres@127.0.0.1:5432/draft".to_string());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&dsn)
        .await
        .unwrap();
    let pool = Arc::new(pool);

    let state = Arc::new(AppState { pool });

    let app = axum::Router::new()
        .route("/", get(handler::list).post(handler::create))
        .route("/:num", get(handler::find))
        .route("/unsigned/:num", get(handler::find_unsigned))
        .route("/unsigned", post(handler::create_unsigned))
        .with_state(state);

    axum::Server::bind(&"0.0.0.0:9527".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
