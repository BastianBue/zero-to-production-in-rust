use crate::routes::monitor::health_check;
use crate::routes::subscriptions::subscribe;
use actix_web::dev::Server;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

// will get executed for each worker node
pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(subscribe)
            .service(health_check)
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
