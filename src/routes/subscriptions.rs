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
#[tracing::instrument(
    name = "Adding a new Subscriber"
    skip(form, _connection),
    fields(
        subscriber_email = %form.email,
        subscriber_name = %form.name,
    )
)]
pub async fn subscribe(form: Form<FormData>, _connection: web::Data<PgPool>) -> impl Responder {
    tracing::info!("Subscribing {} with email {}.", form.name, form.email);
    match insert_subscriber(&_connection, &form).await {
        Ok(_) => HttpResponse::Ok().body(format!("Hello, {}, {}!", form.name, form.email)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[tracing::instrument(
    name = "Saving new subscriber details in the database",
    skip(pool, form)
)]
pub async fn insert_subscriber(pool: &PgPool, form: &FormData) -> Result<(), sqlx::Error> {
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
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}
