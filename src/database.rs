use crate::configuration::get_configuration;
use sqlx::{Connection, Executor, PgConnection, PgPool};

#[tracing::instrument(name = "Migrating the database")]
pub async fn migrate_db() {
    let config = get_configuration().expect("Failed to read configuration.");

    let mut connection_root = PgConnection::connect_with(&config.database.connect_options_root())
        .await
        .expect("Failed to connect to Postgres");
    connection_root
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            config.database.database_name
        ))
        .await
        .ok();

    let connection_pool = PgPool::connect_with(config.database.connect_options_db())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");
}
