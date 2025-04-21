use axum::Router;
use server::ServerState;

pub mod auth;
pub mod server;
pub mod user;

pub trait WebService {
    fn api_router(state: ServerState) -> Router<ServerState>;
    fn view_router(state: ServerState) -> Router<ServerState>;
}