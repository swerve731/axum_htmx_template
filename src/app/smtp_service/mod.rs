use std::ops::Add;

use lettre::{
    message::{header::ContentType, Mailbox, MessageBuilder}, transport::smtp::{authentication::Credentials, commands::Mail, Error}, Address, Message, SmtpTransport, Transport
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
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let credentials = Credentials::new(
            std::env::var("SMTP_USERNAME")?,
            std::env::var("SMTP_PASSWORD")?,
        );
        let noreply_email = std::env::var("SMTP_FROM_NO_REPLY")?
            .parse::<Address>()
            .map_err(|_| std::env::VarError::NotUnicode("SMTP_FROM_NO_REPLY is not valid address".into()))?;
        let host = std::env::var("SMTP_HOST")?;
        let port = std::env::var("SMTP_PORT")?
            .parse::<u16>()
            .map_err(|_| std::env::VarError::NotUnicode("SMTP_PORT is not u16".into()))?;
        let sender_name = std::env::var("SMTP_SENDER_NAME")?;

        Ok(Self::new(credentials, noreply_email, host, port, sender_name))
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


    pub fn send(&self, message: Message) -> Result<(), Error>{
        let relay = SmtpTransport::relay(&self.host)?;

        let mailer = relay.credentials(self.credentials.clone()).build();
        mailer.send(&message)?;

        Ok(())
    } 

    pub fn default_message_builder(&self) -> Result<MessageBuilder, Error> {
        let builder = Message::builder()
            .from(Mailbox::new(Some(self.sender_name.clone()), self.noreply_email.clone()));

        Ok(builder)
    }
}