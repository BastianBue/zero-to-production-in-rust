use newsletter::configuration::get_configuration;
use newsletter::database::migrate_db;
use newsletter::mailing::EmailClient;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    init_subscriber(get_subscriber(
        "newsletter".into(),
        "info".into(),
        std::io::stdout,
    ));

    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = migrate_db().await;

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    let email_client = EmailClient::new(
        configuration
            .email_client
            .sender()
            .expect("Invalid sender email address."),
        configuration.email_client.base_url.clone(),
        configuration.email_client.api_token.clone(),
        configuration.email_client.duration(),
    );

    run(listener, connection_pool, email_client)?.await?;
    Ok(())
}
