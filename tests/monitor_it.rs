use actix_web::{http::header::ContentType, test, App};
use serde_urlencoded;
use zero_to_production_in_rust::routes::monitor::health_check;

#[actix_web::test]
async fn health_check_it() {
    let app = test::init_service(App::new().service(health_check)).await;
    let req = test::TestRequest::get()
        .uri("/health_check")
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
