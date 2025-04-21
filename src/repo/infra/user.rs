// CREATE TABLE IF NOT EXISTS users (
//     id uuid PRIMARY KEY,
//     name VARCHAR(100) NOT NULL,
//     email VARCHAR(100) UNIQUE NOT NULL,
//     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
//     password_hash VARCHAR(255) NOT NULL
// );

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use uuid::Uuid;

use crate::{features::auth::claims::password_reset::PasswordResetClaim, repo::utils::{is_valid_email, is_valid_password}};

use super::super::error::RepoError;

#[derive(Debug)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub name: String,
    pub email: String,
    pub created_at: sqlx::types::chrono::NaiveDateTime,
    pub password_hash: String,
}

#[async_trait::async_trait]
pub trait UserRepo {
    async fn user_get_by_email(&self, email: &str) -> Result<Option<User>, RepoError>;
    async fn user_get_by_id(&self, id: sqlx::types::Uuid) -> Result<Option<User>, RepoError>;
    async fn user_change_password(&self, id: sqlx::types::Uuid, new_password: String, _password_reset_claims_to_verify: PasswordResetClaim) -> Result<(), RepoError>;
    async fn user_create(&self,email: &str, password: &str, name: &str) -> Result<Uuid, RepoError>;
    async fn user_check_password(&self, email: &str, password: &str) -> Result<Option<Uuid>, RepoError>;
}

#[async_trait::async_trait]
impl UserRepo for super::super::Repository {
    async fn user_get_by_email(&self, email: &str) -> Result<Option<User>, RepoError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email, created_at, password_hash FROM users WHERE email = $1",
            email
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    } 
    async fn user_get_by_id(&self, id: sqlx::types::Uuid) -> Result<Option<User>, RepoError> {
        let user = sqlx::query_as!(
            User,
            "SELECT id, name, email, created_at, password_hash FROM users WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    async fn user_change_password(&self, id: sqlx::types::Uuid, new_password: String, _password_reset_claims_to_verify: PasswordResetClaim) -> Result<(), RepoError> {
        is_valid_password(&new_password)?;

        let argon2 = argon2::Argon2::default();
        let password = new_password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        sqlx::query!(
            "
            UPDATE users
            SET password_hash = $1
            WHERE id = $2
            ",
            password_hash,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn user_create(&self, email: &str, password: &str, name: &str) -> Result<Uuid, RepoError> {

        is_valid_email(email)?;
        is_valid_password(password)?;

        
        let argon2 = argon2::Argon2::default();
        let password = password.as_bytes();
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = argon2.hash_password(password, &salt)?.to_string();

        let id = sqlx::types::uuid::Uuid::new_v4();



        let _res = sqlx::query!(
            "INSERT INTO users (id, name, email, password_hash) VALUES ($1, $2, $3, $4)",
            id,
            name,
            email,
            password_hash
        )
        .execute(&self.pool)
        .await?;

        return Ok(id);
    }
    async fn user_check_password(&self, email: &str, password: &str) -> Result<Option<Uuid>, RepoError> {
        let user = match self.user_get_by_email(email).await {
            Ok(Some(user)) => user,
            Ok(None) => return Err(RepoError::EmailNotFound),
            Err(e) => return Err(e),
        };
        
        
        let correct_password = user.password_hash;
        let password = password.as_bytes();
        let parsed_hash = PasswordHash::new(&correct_password)?;
        let res = Argon2::default().verify_password(password, &parsed_hash);
    
        match res {
            Ok(_) => Ok(Some(user.id)),
            Err(e) => {
                match e {
                    argon2::password_hash::Error::Password => Ok(None),
                    _ => Err(RepoError::PasswordHashing(e))
                }
            }    
        }
    }
}

