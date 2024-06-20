use crate::domain::SubscriberEmail;
use reqwest::{Client, Url};
use secrecy::{ExposeSecret, Secret};
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
    email_service_provider_url: String,
    api_token: Secret<String>,
}

impl EmailClient {
    pub fn new(
        sender: SubscriberEmail,
        email_service_provider_url: String,
        api_token: Secret<String>,
    ) -> Self {
        Self {
            sender,
            http_client: Client::new(),
            email_service_provider_url,
            api_token,
        }
    }

    pub async fn send_email(
        &self,
        recipient: SubscriberEmail,
        subject: &str,
        plain_body: &str,
        html_body: &str,
    ) -> Result<(), reqwest::Error> {
        let url = format!("{}/email", &self.email_service_provider_url);
        let request_body = SendEmailPostRequestBody {
            from: self.sender.as_ref().to_string(),
            to: recipient.as_ref().to_string(),
            subject: subject.to_string(),
            html_body: html_body.to_string(),
            text_body: plain_body.to_string(),
        };
        self.http_client
            .post(&url)
            .header("X-Postmark-Server-Token", self.api_token.expose_secret())
            .json(&request_body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::domain::SubscriberEmail;
    use crate::mailing::EmailClient;
    use fake::faker::internet::en::SafeEmail;
    use fake::faker::lorem::en::{Paragraph, Sentence};
    use fake::{Fake, Faker};
    use secrecy::Secret;
    use wiremock::matchers::{header, header_exists, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    fn subject() -> String {
        Sentence(1..2).fake()
    }

    /// Generate a random email content
    fn content() -> String {
        Paragraph(1..10).fake()
    }

    /// Generate a random subscriber email
    fn email() -> SubscriberEmail {
        SubscriberEmail::parse(SafeEmail().fake()).unwrap()
    }

    /// Get a test instance of `EmailClient`.
    fn email_client(base_url: String) -> EmailClient {
        EmailClient::new(email(), base_url, Secret::new(Faker.fake()))
    }

    #[tokio::test]
    async fn send_email_fails_if_the_email_provider_server_is_unavailable() {
        let mock_server = MockServer::start().await;
        let email_client = email_client(mock_server.uri());

        Mock::given(header_exists("X-Postmark-Server-Token"))
            .and(header("Content-Type", "application/json"))
            .and(path("/email"))
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let _ = email_client
            .send_email(email(), &subject(), &content(), &content())
            .await;
    }
}
