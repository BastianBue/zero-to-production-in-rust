use std::io::Error;
use zero_to_production_in_rust::startup::{db_connect, run};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    run(db_connect().await).await
}
