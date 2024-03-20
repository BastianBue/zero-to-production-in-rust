use zero_to_production_in_rust::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    run().await
}