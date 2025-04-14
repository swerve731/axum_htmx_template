use std::time::Duration;

use axum::response::IntoResponse;
use sqlx::{postgres::PgPoolOptions, PgPool};


pub async fn get_connection_pool() -> Result<PgPool, DbError> {
    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(25)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_string)
        .await?;

    Ok(pool)
}

#[derive(derive_more::From, Debug)]
pub enum DbError {
    #[from]
    Sqlx(sqlx::Error),
}

impl IntoResponse for DbError {
    fn into_response(self) -> axum::response::Response {
        let status = axum::http::StatusCode::INTERNAL_SERVER_ERROR;

        let body = format!("Database error: {:?}", self);

        (status, body).into_response()
    }
}