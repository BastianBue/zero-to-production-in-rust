use newsletter::configuration::get_configuration;
use newsletter::database::migrate_db;
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

    run(listener, connection_pool)?.await?;
    Ok(())
}
