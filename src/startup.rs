use crate::configuration::get_configuration;
use actix_web::{web, App, HttpServer};
use sqlx::{Connection, PgConnection};

// will get executed for each worker node
pub async fn run(db_connection: PgConnection) -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let db_connection_app_data = web::Data::new(db_connection);
    HttpServer::new(move || App::new().app_data(db_connection_app_data.clone()))
        .bind(("127.0.0.1", configuration.application_port))?
        .run()
        .await
}

pub async fn db_connect() -> PgConnection {
    let connection_string = get_configuration()
        .expect("failed to load configuration")
        .database
        .connection_string();

    let postgres_connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    return postgres_connection;
}
