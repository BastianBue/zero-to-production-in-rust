use actix_web::{http::header::ContentType, test, App};
use zero_to_production_in_rust::{index};

#[actix_web::test]
async fn health_check() {
    let app = test::init_service(App::new().service(index)).await;
    let req = test::TestRequest::default()
        .insert_header(ContentType::plaintext())
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}