// use chrono::{DateTime, Duration, NaiveDateTime, Utc};
// use rand::prelude::*; // For generating the token string
// use sqlx::types::Uuid;
// use sqlx::PgPool; // Assuming PostgreSQL, change if using a different DB

// // Database Schema Comments (for reference):
// // id uuid PRIMARY KEY DEFAULT uuid_generate_v4(), -- Assuming auto-generated UUID
// // user_id uuid NOT NULL,
// // created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP NOT NULL,
// // expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
// // token VARCHAR(6) NOT NULL,
// // FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE

// #[derive(Debug, sqlx::FromRow)] // Add FromRow to easily map query results
// pub struct EmailVerificationToken {
//     pub id: Uuid,
//     pub user_id: Uuid,
//     pub created_at: NaiveDateTime,
//     pub expires_at: NaiveDateTime,
//     pub token: String,
// }

// impl EmailVerificationToken {
//     /// Creates a new email verification token, stores it in the database,
//     /// and returns the created token struct.
//     ///
//     /// # Arguments
//     ///
//     /// * `pool` - A reference to the sqlx database connection pool.
//     /// * `user_id` - The UUID of the user for whom the token is being created.
//     ///
//     /// # Returns
//     ///
//     /// A `Result` containing the newly created `EmailVerificationToken` on success,
//     /// or an `sqlx::Error` on failure.
//     /// 
//     pub const EXPIRE_TIME_MINUTES: i64 = 15;
//     pub const TOKEN_LENGTH: usize = 6;

//     pub async fn create_token(
//         pool: &PgPool,
//         user_id: Uuid,
//     ) -> Result<Self, sqlx::Error> {
//         // Generate a 6-character alphanumeric token
//         let mut rng = rand::rng();
//         let token: String  = rng.sample_iter(rand::distr::Alphanumeric).take(Self::TOKEN_LENGTH).map(|v|v as char).collect();


//         let now = Utc::now().naive_utc();
//         let expires_at = now + Duration::minutes(Self::EXPIRE_TIME_MINUTES); // Adjust duration as needed

//         // Insert the new token into the database
//         // We assume 'id' and 'created_at' have database defaults or we generate 'id' here
//         let new_token_id = Uuid::new_v4(); // Generate UUID in Rust

//         let created_token = sqlx::query_as!(
//             EmailVerificationToken,
//             r#"
//             INSERT INTO email_verification_tokens (id, user_id, expires_at, token)
//             VALUES ($1, $2, $3, $4)
//             RETURNING id, user_id, created_at, expires_at, token
//             "#,
//             new_token_id,
//             user_id,
//             expires_at,
//             token
//         )
//         .fetch_one(pool) // Use fetch_one as RETURNING guarantees one row
//         .await?;

//         Ok(created_token)
//     }

//     // You might also want functions like:
//     // pub async fn find_by_token(pool: &PgPool, token: &str) -> Result<Option<Self>, sqlx::Error> { ... }
//     // pub async fn delete_token(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> { ... }
//     // pub async fn is_expired(&self) -> bool { ... }
// }
