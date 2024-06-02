use config::Environment;
use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::postgres::PgPool;
use std::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let connection_pool = PgPool::connect_lazy_with(configuration.database.connect_options_db());

    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(address)?;

    init_subscriber(get_subscriber(
        "newsletter".into(),
        "info".into(),
        std::io::stdout,
    ));

    run(listener, connection_pool)?.await?;
    Ok(())
}
