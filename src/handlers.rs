use axum::{async_trait, extract, http, Json};
use axum::extract::{Request, State};
use axum::handler::{Handler,};
use axum::http::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct Quote {
    id: uuid::Uuid,
    book_name: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Clone)]
pub struct CreateQuote {
    book_name: String,
    quote: String,
}
impl Quote {
    fn new(book_name: String, quote: String) -> Self {
        let now = chrono::Utc::now();
        Self {
            id: uuid::Uuid::new_v4(),
            book_name,
            quote,
            inserted_at: now,
            updated_at: now,
        }
    }
}

pub async fn check_health() -> StatusCode {
    StatusCode::OK
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let quote = Quote::new(payload.book_name, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book, quote, inserted_at, updated_at)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
        .bind(&quote.id)
        .bind(&quote.book_name)
        .bind(&quote.quote)
        .bind(&quote.inserted_at)
        .bind(&quote.updated_at)
        .execute(&pool)
        .await;

    match res {
        Ok(_) => Ok((StatusCode::CREATED, Json(quote))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}