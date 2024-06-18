use crate::domain::SubscriberEmail;
use reqwest::Client;

pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    email_service_provider_url: String,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, email_service_provider_url: String) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            email_service_provider_url,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        plain_body: &str,
        html_body: &str,
    ) -> Result<(), String> {
        todo!()
    }
}
