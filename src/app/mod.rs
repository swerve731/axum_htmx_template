pub mod db_service;
pub mod auth_service;
pub mod error;

use error::AppError;

use axum::{
    Router,
    extract::MatchedPath,
    http::{
        Method,
        Request,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};

use db_service::get_connection_pool;
use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier}, cors::{self, CorsLayer}, services::ServeDir, trace::{DefaultMakeSpan, TraceLayer}
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
}

impl AppState {
    pub async fn default() -> Self {
        let pool = get_connection_pool().await.expect("Failed to create connection pool");
        AppState { pool }
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
            .nest("/auth", self.auth_service.view_router(state.clone()))
            .with_state(state.clone());

        router
    }
    fn api_router(&self, state: AppState) -> axum::Router<AppState> {
        let router = Router::new()
            .merge(self.auth_service.api_router(state.clone()));

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
        let app_origin = std::env::var("APP_ORIGIN").expect("APP_ORIGIN must be set");
        let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS must be set");
        Self::init_tracing();
        
        let app = Router::new()
            .merge(self.view_router(self.state.clone()))
            .nest("/api", self.api_router(self.state.clone()))
            .with_state(self.state.clone())
            .nest_service("/assets", ServeDir::new("assets"))
            .layer(Self::cors_layer(&app_origin))
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
                );

        
        let listener = tokio::net::TcpListener::bind(&bind_address).await?; 
        tracing::info!("Listening on {}", bind_address);        
        axum::serve(listener, app.into_make_service()) 
            .await
            .expect("Failed to start server");


        Ok(())
    }

    fn cors_layer(origin: &str) -> tower_http::cors::CorsLayer {
        CorsLayer::new()
           .allow_origin(origin.parse::<axum::http::HeaderValue>().unwrap())
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

