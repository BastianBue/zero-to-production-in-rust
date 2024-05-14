use crate::configuration::get_configuration;
use actix_web::{web, App, HttpServer};
use sqlx::{Connection, PgConnection, PgPool};

// will get executed for each worker node
pub async fn run(db_pool: PgPool) -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let db_connection_app_data = web::Data::new(db_pool);
    HttpServer::new(move || App::new().app_data(db_connection_app_data.clone()))
        .bind(("127.0.0.1", configuration.application_port))?
        .run()
        .await
}

pub async fn db_connect() -> PgPool {
    let connection_string = get_configuration()
        .expect("failed to load configuration")
        .database
        .connection_string();

    let pg_pool = PgPool::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    return pg_pool;
}
