pub mod auth_service;
pub mod db_service;
pub mod error;
pub mod smtp_service;
pub mod user_service;
use axum::{
    extract::MatchedPath,
    http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, Request,
    },
    Router,
};
use db_service::get_connection_pool;
use error::AppError;
use tower_http::{
    cors::CorsLayer,
    services::ServeDir,
    trace::TraceLayer,
};
use tracing::info_span;
use tracing_subscriber::{
    layer::SubscriberExt,
    util::SubscriberInitExt,
};

pub trait WebService {
    fn view_router(&self, state: AppState) -> axum::Router<AppState>;
    fn api_router(&self, state: AppState) -> axum::Router<AppState>;
}



#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub smtp_service: smtp_service::SmtpService,
}

impl AppState {
    pub async fn default() -> Self {
        let pool = get_connection_pool().await.expect("Failed to create connection pool");
        let smtp_service = smtp_service::SmtpService::from_env().expect("Failed to start SMTP service");
        AppState { pool, smtp_service }
    }
}


#[derive(Clone)]
pub struct App {
    pub state: AppState,
    pub auth_service: auth_service::AuthService,
}

impl WebService for App {
    fn view_router(&self, state: AppState) -> axum::Router<AppState> {
        let router = Router::new()
            .merge(user_service::UserService{}.view_router(state.clone()))
            .nest("/auth", self.auth_service.view_router(state.clone()))
            .with_state(state.clone());

        router
    }
    
    fn api_router(&self, state: AppState) -> axum::Router<AppState> {
        let router = Router::new()
            .merge(user_service::UserService{}.api_router(state.clone()))
            .nest("/auth", self.auth_service.api_router(state.clone()));

        router
    }
}




impl App {
    pub async fn default() -> Self {
        let state = AppState::default().await;
        App { 
            state,
            auth_service: auth_service::AuthService::default()
        }
    }

    pub async fn run_server(self) -> Result<(), AppError> {
        let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");
        Self::init_tracing();
        
        let app = Router::new()
            .merge(self.view_router(self.state.clone()))
            .nest("/api", self.api_router(self.state.clone()))
            
            .layer(Self::cors_layer())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(|request: &Request<_>| {
                        // Log the matched route's path (with placeholders not filled in).
                        // Use request.uri() or OriginalUri if you want the real path.
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!(
                            "http_request",
                            method = ?request.method(),
                            matched_path,
                            some_other_field = tracing::field::Empty,
                        )
                    })
            )
            .with_state(self.state.clone())
            .nest_service("/assets", ServeDir::new("assets"));

        
        let listener = tokio::net::TcpListener::bind(&bind_address).await?; 
        tracing::info!("Listening on {}", bind_address);        
        axum::serve(listener, app.into_make_service()) 
            .await
            .expect("Failed to start server");


        Ok(())
    }

    pub fn cors_layer() -> tower_http::cors::CorsLayer {
        let origin = std::env::var("APP_ORIGIN").expect("APP_ORIGIN must be set");

        CorsLayer::new()
            .allow_origin(origin.parse::<axum::http::HeaderValue>().expect("Failed to parse APP_ORIGIN"))
        //    .allow_origin(origin.parse::<axum::http::HeaderValue>().unwrap())
           .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE]) // Allow common methods
           .allow_headers([CONTENT_TYPE, AUTHORIZATION]) // Allow common headers
    }

    fn init_tracing() {
        tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    }


   
}

