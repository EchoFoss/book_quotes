mod handlers;

use axum::{Router};
use axum::routing::{get, post};
use std::env;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_path("./.env").ok();
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_owned());
    let addr = format!("0.0.0.0:{}", port);

    let database_url =
        env::var("DATABASE_URL")
            .expect("missing DATABASE_URL env");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let app = Router::new()
        .route("/", get(handlers::check_health))
        .route("/quotes", post(handlers::create_quote))
        .route("/quotes", get(handlers::read_quotes))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

