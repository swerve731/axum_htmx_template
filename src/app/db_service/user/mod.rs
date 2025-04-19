// CREATE TABLE IF NOT EXISTS users (
//     id uuid PRIMARY KEY,
//     name VARCHAR(100) NOT NULL,
//     email VARCHAR(100) UNIQUE NOT NULL,
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
//     password_hash VARCHAR(255) NOT NULL
// );

use sqlx::PgPool;

use crate::app::auth_service::api::email_login::EmailAuthenticationClaims;


#[derive(Debug)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub password_hash: String,
}


pub async fn get_user_by_email(email: &str, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, created_at, password_hash FROM users WHERE email = $1",
        email
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
} 

pub async fn get_user_by_id(id: sqlx::types::Uuid, pool: &PgPool) -> Result<Option<User>, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, name, email, created_at, password_hash FROM users WHERE id = $1",
        id
    )
    .fetch_optional(pool)
    .await?;

    Ok(user)
}

pub async fn change_user_password(id: sqlx::types::Uuid, new_password: &str, pool: &PgPool, _email_auth_claims_to_verify: EmailAuthenticationClaims) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "
        UPDATE users
        SET password_hash = $1
        WHERE id = $2
        ",
        new_password,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}