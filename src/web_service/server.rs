use std::str::FromStr;

use crate::{config::ServerConfig, mailer::Mailer, repo::Repository};
use axum::{
    extract::MatchedPath, http::{
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method, Request,
    }, Router
};
use lettre::{transport::smtp::authentication::Credentials, Address};
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

use super::{auth::AuthService, user::UserService, WebService};


#[derive(Clone)]
pub struct ServerState {
    pub mailer: Mailer,
    pub repo: Repository,
    config: ServerConfig
}
impl ServerState {
    pub async fn initialize() -> Self {
        let repo = Repository::new().await.expect("Could not get repo");
        let config = ServerConfig::from_file("config.toml").expect("Could not get config");

        let mailer_credentials = Credentials::new(config.mailer.username.clone(), config.mailer.password.clone());
        let mailer_email = Address::from_str(&config.mailer.sender_email.clone()).expect("CONFIG SENDER EMAIL IS NOT VALID EMAIL");
        
        let mailer = Mailer::new(mailer_credentials, mailer_email, config.mailer.host.clone(), config.mailer.port.clone(), config.full_sender_name().clone());

        ServerState {
            repo,
            config,
            mailer
        }
    }
}

pub struct Server {}

impl WebService for Server {
    fn view_router(state: ServerState) -> Router<ServerState> {
        Router::new()
            .merge(UserService::view_router(state.clone()))
            // .route("/", get(|| async {"Hello World!"}))
            .nest("/auth", AuthService::view_router(state.clone()))
            .with_state(state.clone())
    }
    
    fn api_router(state: ServerState) -> Router<ServerState> {
        Router::new()
            .merge(UserService::api_router(state.clone()))
            // .route("/", get(|| async {"Hello World!"}))
            .nest("/auth", AuthService::api_router(state.clone()))
            .with_state(state.clone())
    }
}
 
impl Server{


    pub async fn run() -> Result<(), crate::ServerError> {
        dotenvy::dotenv().expect("Failed to load .env file"); // Add this line

        let state = ServerState::initialize().await;

        let bind_address = &state.config.app.bind_address;
        Self::init_tracing();
        
        let app = Router::new()            
            .merge(Server::view_router(state.clone()))
            
            .nest("/api", Server::api_router(state.clone()))

            .layer(Self::cors_layer(state.config.app.origin.clone()))
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
            .with_state(state.clone())
            .nest_service("/static", ServeDir::new("static"));
        
        let listener = tokio::net::TcpListener::bind(&bind_address).await?; 
        tracing::info!("Listening on {}", bind_address);        
        axum::serve(listener, app.into_make_service()) 
            .await
            .expect("Failed to start server");


        Ok(())
    }




    fn cors_layer(origin: String) -> tower_http::cors::CorsLayer {
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