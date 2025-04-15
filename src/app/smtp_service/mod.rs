use std::ops::Add;

use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder}, transport::smtp::{authentication::Credentials, commands::Mail}, Address, Message, SmtpTransport, Transport
};


#[derive(Clone)]
pub struct SmtpService {
    credentials: Credentials,
    pub noreply_email: Address,
    host: String,
    port: u16,
    sender_name: String,
}


impl SmtpService {
    pub fn from_env() -> Self {
        Self::new(
            lettre::transport::smtp::authentication::Credentials::new(
                std::env::var("SMTP_USERNAME").expect("SMTP_USERNAME must be set"),
                std::env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD must be set"),
            ),
            std::env::var("SMTP_FROM_NO_REPLY").expect("SMTP_FROM_NO_REPLY must be set").parse::<Address>().expect("SMTP_FROM_NO_REPLY must be a valid email address"),
            std::env::var("SMTP_HOST").expect("SMTP_HOST must be set"),
            std::env::var("SMTP_PORT")
                .expect("SMTP_PORT must be set")
                .parse::<u16>()
                .expect("SMTP_PORT must be a valid u16"),
        std::env::var("SMTP_SENDER_NAME").expect("SMTP_FROM_NAME must be set"),


        )
    }

    pub fn new(credentials: Credentials, noreply_email: Address, host: String, port: u16, sender_name: String) -> Self {
        SmtpService { 
            credentials,
            noreply_email,
            host,
            port,
            sender_name
         }
    }


    pub fn send(&self, message: Message) -> Result<(), lettre::transport::smtp::Error>{
        let relay = SmtpTransport::relay("live.smtp.mailtrap.io")?;

        let mailer = relay.credentials(self.credentials.clone()).build();
        mailer.send(&message)?;

        Ok(())
    } 

    pub fn default_message_builder(&self) -> Result<MessageBuilder, lettre::transport::smtp::Error> {
        let builder = Message::builder()
            .from(Mailbox::new(Some(self.sender_name.clone()), self.noreply_email.clone()));

        Ok(builder)
    }
}