

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file"); // Add this line



    // let message = lettre::Message::builder()
    //     .from(format!("No Reply <{}>", service.noreply_email).parse().unwrap())
    //     .to(format!("Eoin <eoinmitchell39@proton.me>").parse().unwrap())
    //     .subject("Test Email")
    //     .header(lettre::message::header::ContentType::TEXT_PLAIN)
    //     .body(String::from("This is a test email from lettre."))
    //     .unwrap();

    // service.send(message).expect("Failed to send email");


    let app = ahp::app::App::default().await;
    app.run_server().await.expect("Error Starting server");
}