

#[derive(Debug, derive_more::From)]
pub enum FeatureError {
    
    #[from]
    Auth(super::auth::error::AuthError)
}