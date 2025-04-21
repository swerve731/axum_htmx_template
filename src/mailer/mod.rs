use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Address, Message, SmtpTransport, Transport
};
pub mod error;
use error::MailerError;

#[derive(Clone)]
pub struct Mailer {
    credentials: Credentials,
    pub noreply_email: Address,
    host: String,
    _port: u16,
    sender_name: String,
}


impl Mailer {

    pub fn new(credentials: Credentials, noreply_email: Address, host: String, port: u16, sender_name: String) -> Self {
        Mailer { 
            credentials,
            noreply_email,
            host,
            _port:port,
            sender_name
         }
    }


    pub fn send_message(&self, message: Message) -> Result<(), MailerError>{
        let relay = SmtpTransport::relay(&self.host)?;

        let mailer = relay.credentials(self.credentials.clone()).build();
        mailer.send(&message)?;

        Ok(())
    } 


    pub fn create_message(&self, body: String, reciever_email: String, reciever_name: String,subject: String) -> Result<Message,  MailerError> {
        let message = Message::builder()
            .from(Mailbox::new(Some(self.sender_name.clone()), self.noreply_email.clone()))
            .to(Mailbox::new(Some(reciever_name),  reciever_email.parse()?))
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(body)?;


        Ok(message)

    }
}

