use db_service::get_connection_pool;
pub mod db_service;
pub mod auth_service;
pub mod view_service;


pub trait WebService {
    fn view_router(&self) -> axum::Router;
    fn api_router(&self) -> axum::Router;
}

pub struct App {
    pub pool: sqlx::PgPool,
    pub auth_service: auth_service::AuthService,
}


impl App {
    async fn default() -> Self {
        let pool = get_connection_pool().await.expect("Failed to create connection pool");
        App { 
            pool,
            auth_service: auth_service::AuthService::default()
        }
    }
}

