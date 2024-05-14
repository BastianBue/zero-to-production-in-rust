use actix_web::{http::header::ContentType, test, App};
use serde_urlencoded;
use sqlx::{Connection, PgConnection};
use zero_to_production_in_rust::configuration::get_configuration;
use zero_to_production_in_rust::routes::subscriptions::signup;

//#[actix_web::test]
async fn signup_returns_400_with_invalid_data() {
    let app = test::init_service(App::new().service(signup)).await;
    let test_cases = vec![
        ([("name", "Bastian"), ("email", "")], "email is required"),
        ([("name", ""), ("email", "hi@gmx.de")], "name is required"),
        ([("name", ""), ("email", "")], "invalid input"),
    ];

    for (payload, _error_message) in test_cases {
        let body = serde_urlencoded::to_string(payload).unwrap();
        let req = test::TestRequest::post()
            .uri("/signup")
            .insert_header(ContentType::form_url_encoded())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(
            resp.status().as_u16(),
            400,
            "The request payload: {:?} should have failed with a 400 Bad Request",
            payload
        );
    }
}

#[actix_web::test]
async fn signup_returns_200_with_valid_data() {
    let app = test::init_service(App::new().service(signup)).await;
    let test_cases = vec![[("name", "Bastian"), ("email", "hi@gmx.de")]];

    let connection_string = get_configuration()
        .expect("failed to load configuration")
        .database
        .connection_string();

    print!("Connection string: {}", connection_string);

    let mut postgres_connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres.");

    for payload in test_cases {
        let body = serde_urlencoded::to_string(payload).unwrap();

        let req = test::TestRequest::post()
            .uri("/signup")
            .insert_header(ContentType::form_url_encoded())
            .set_payload(body)
            .to_request();

        let resp = test::call_service(&app, req).await;
        println!("Response: {:?}", resp);

        let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
            .fetch_one(&mut postgres_connection)
            .await
            .expect("Failed to fetch saved subscription");

        assert_eq!(
            resp.status().as_u16(),
            200,
            "The request payload: {:?} should have succeeded with a 200 OK",
            payload
        );
    }
}
