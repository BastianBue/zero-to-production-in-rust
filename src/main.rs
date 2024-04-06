use zero_to_production_in_rust::run;
use std::io::{ Error};

#[actix_web::main]
async fn main() -> Result<(), Error> {
    run().await
}