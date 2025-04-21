use std::time::Duration;

use error::RepoError;
use sqlx::postgres::PgPoolOptions;

pub mod error;
pub mod infra;
pub mod utils;


#[derive(Clone)]
pub struct Repository {
    pub pool: sqlx::PgPool,
}


impl Repository {
    pub async fn new() -> Result<Repository, RepoError>{
        let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let pool = PgPoolOptions::new()
            .max_connections(25)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&db_connection_string)
            .await?;
    
        Ok(
            Repository { pool }
        )
    }
}