use core_lib::web_service::server::Server;

#[tokio::main]
async fn main() {
    Server::run().await.expect("Failed to start server");
}
