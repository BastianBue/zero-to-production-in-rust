use std::io::Error;
use zero_to_production_in_rust::run;

#[actix_web::main]
async fn main() -> Result<(), Error> {
    run().await
}
