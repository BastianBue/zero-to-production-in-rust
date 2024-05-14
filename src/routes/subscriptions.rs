use actix_web::web::Form;
use actix_web::{post, web, HttpResponse, Responder};
use chrono::Utc;
use serde::Deserialize;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
struct FormData {
    name: String,
    email: String,
}

#[post("/subscriptions")]
pub async fn subscribe(form: Form<FormData>, _connection: web::Data<PgPool>) -> impl Responder {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name,subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(_connection.get_ref())
    .await
    .expect("Failed to execute query.");
    HttpResponse::Ok().body(format!("Hello, {}, {}!", form.name, form.email))
}
