use crate::configuration::get_configuration;
use actix_web::{App, HttpServer};

pub async fn run() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", configuration.application_port))?
        .run()
        .await
}
