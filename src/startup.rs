use actix_web::{App, HttpServer};

pub async fn run() -> Result<(), std::io::Error> {
    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
