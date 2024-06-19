use crate::domain::SubscriberEmail;
use reqwest::{Client, Url};
use serde::Serialize;

#[derive(Serialize)]
struct SendEmailPostRequestBody {
    from: String,
    to: String,
    subject: String,
    html_body: String,
    text_body: String,
}

pub struct EmailClient {
    sender: SubscriberEmail,
    http_client: Client,
    email_service_provider_url: Url,
}

impl EmailClient {
    pub fn new(sender: SubscriberEmail, email_service_provider_url: String) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            email_service_provider_url: Url::parse(&email_service_provider_url)
                .expect("invalid email service provider url"),
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        plain_body: &str,
        html_body: &str,
    ) -> Result<(), String> {
        let url = Url::join(&self.email_service_provider_url, "email").unwrap();
        let body = SendEmailPostRequestBody {
            to: recipient.as_ref().to_owned(),
            text_body: plain_body.to_owned(),
            from: self.sender.as_ref().to_owned(),
            subject: subject.to_owned(),
            html_body: html_body.to_owned(),
        };
        let _ = self.http_client.post(url).json(&body);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::mailing::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::Fake;
    use wiremock::matchers::any;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn send_email_fails_if_the_email_provider_server_is_unavailable() {
        let mock_server = MockServer::start().await;
        let sender = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let email_client = EmailClient::new(sender, mock_server.uri());

        Mock::given(any())
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let subscriber_email = SubscriberEmail::parse(SafeEmail().fake()).unwrap();
        let subject: String = Sentence(1..2).fake();
        let content: String = Paragraph(1..10).fake();

        //Act
        let _ = email_client.send_email(subscriber_email, &subject, &content, &content);
    }
}
