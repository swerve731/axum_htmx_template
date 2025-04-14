

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file"); // Add this line

    let app = ahp::app::App::default().await;
    app.run_server().await.expect("Error Starting server");
}