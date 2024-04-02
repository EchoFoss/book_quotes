use axum::{extract, http, Json};
use axum::extract::{State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(Serialize,Deserialize, FromRow)]
pub struct Quote {
    id: uuid::Uuid,
    book_name: String,
    quote: String,
    inserted_at: chrono::DateTime<chrono::Utc>,
    updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct CreateQuote {
    book_name: String,
    quote: String,
}
#[derive(Deserialize, Serialize)]
pub struct UpdateQuote {
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

pub async fn check_health() -> impl IntoResponse {
    const MESSAGE: &str = "Crud usando axum e rust";
    let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });
    Json(json_response)
}

pub async fn create_quote(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateQuote>,
) -> Result<(StatusCode, Json<Quote>), StatusCode> {
    let quote = Quote::new(payload.book_name, payload.quote);

    let res = sqlx::query(
        r#"
        INSERT INTO quotes (id, book_name, quote, inserted_at, updated_at)
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
        Err(err) => {
            println!("internal server error: {}", err);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        },
    }
}

pub async fn read_quotes(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<Quote>>, StatusCode> {
    let res = sqlx::query_as::<_, Quote>("SELECT * FROM quotes")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(quotes) => Ok(Json(quotes)),
        Err(error) => {
            eprintln!("error fetching data from database {}", error);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn delete_quote_by_id(
    State(pool): State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>
) -> StatusCode {

    let res = sqlx::query(
        r#"
        delete from quotes
        where id = $1
        "#
    )
    .bind(id)
    .execute(&pool)
    .await
    .map(|res| match res.rows_affected() {
        0 => StatusCode::NOT_FOUND,
        _ => StatusCode::OK
    });

    match res {
        Ok(status) => status,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
}

pub async fn update_quotes(
    State(pool): State<PgPool>,
    extract::Path(id): extract::Path<uuid::Uuid>,
    Json(payload): Json<UpdateQuote>
) -> StatusCode {

    let now = chrono::Utc::now();

    let res = sqlx::query(
        r#"
        update quotes
        set book_name = $1, quote = $2, updated_at = $3
        where id = $4
        "#
    )
        .bind(&payload.book_name)
        .bind(&payload.quote)
        .bind(now)
        .bind(id)
        .execute(&pool)
        .await
        .map(|res| match res.rows_affected() {
            0 => StatusCode::NOT_FOUND,
            _ => StatusCode::OK,
        });

    match res {
        Ok(status) => status,
        Err(err) => {
            println!("error while updating quote with id {}, error: {}", id, err);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }

}

