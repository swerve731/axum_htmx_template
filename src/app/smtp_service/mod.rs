use lettre::{
    message::{self, Mailbox, MessageBuilder}, transport::smtp::authentication::Credentials, Address, Message, SmtpTransport, Transport
};
pub mod error;
use error::SmtpError;

#[derive(Clone)]
pub struct SmtpService {
    credentials: Credentials,
    pub noreply_email: Address,
    host: String,
    _port: u16,
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
            _port:port,
            sender_name
         }
    }


    pub fn send_message(&self, message: Message) -> Result<(), SmtpError>{
        let relay = SmtpTransport::relay(&self.host)?;

        let mailer = relay.credentials(self.credentials.clone()).build();
        mailer.send(&message)?;

        Ok(())
    } 


    pub fn create_message(&self, body: String, reciever_email: String, reciever_name: String,subject: String) -> Result<Message,  SmtpError> {
   //     .to(format!("Eoin <eoinmitchell39@proton.me>").parse().unwrap())
    //     .subject("Test Email")
    //     .header(lettre::message::header::ContentType::TEXT_PLAIN)
    //     .body(String::from("This is a test email from lettre."))
    //     .unwrap();

        let message = Message::builder()
            .from(Mailbox::new(Some(self.sender_name.clone()), self.noreply_email.clone()))
            .to(Mailbox::new(Some(reciever_name),  reciever_email.parse()?))
            .subject(subject)
            .body(body)?;


        Ok(message)

    }
}